export type PageBlock = {
	kind: 'title' | 'line'
	text: string
	lineIndex: number
	continued: boolean
}

export type ReaderPage = {
	blocks: PageBlock[]
	startLine: number
	endLine: number
	title: string
}

export function emptyPage(title = ''): ReaderPage {
	return {
		blocks: [],
		startLine: 0,
		endLine: 0,
		title,
	}
}

function pushPage(pages: ReaderPage[], blocks: PageBlock[], title: string) {
	if (!blocks.length) return
	const lineBlocks = blocks.filter((b) => b.kind === 'line')
	const startLine = lineBlocks.length ? lineBlocks[0].lineIndex : 0
	const endLine = lineBlocks.length ? lineBlocks[lineBlocks.length - 1].lineIndex : startLine
	pages.push({
		blocks: blocks.slice(),
		startLine,
		endLine,
		title,
	})
}

function preferBreak(text: string, idx: number): number {
	if (idx <= 0 || idx >= text.length) return idx
	const from = Math.max(0, idx - 24)
	const window = text.slice(from, idx)
	const punct = /[。！？；，、…,:.!?;\s」』》"'”’]/
	for (let i = window.length - 1; i >= Math.max(0, window.length - 18); i--) {
		if (punct.test(window[i])) {
			return from + i + 1
		}
	}
	return idx
}

function makeEl(block: PageBlock): HTMLElement {
	const el = document.createElement('div')
	el.className = block.kind === 'title' ? 'title' : (block.continued ? 'line continued' : 'line')
	el.textContent = block.text
	return el
}

function takeFittingText(
	container: HTMLElement,
	el: HTMLElement,
	text: string,
	maxHeight: number,
): number {
	let lo = 0
	let hi = text.length
	let best = 0
	while (lo <= hi) {
		const mid = (lo + hi) >> 1
		el.textContent = text.slice(0, mid)
		if (container.scrollHeight <= maxHeight + 1) {
			best = mid
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return best
}

/**
 * 把当前章节按可视高度切成多页。measurer 必须与正文同宽、同样式，且不能放在 3D/overflow 裁剪容器里。
 */
export function paginateChapter(
	measurer: HTMLElement,
	title: string,
	lines: string[],
	pageHeight: number,
): ReaderPage[] {
	const pages: ReaderPage[] = []
	if (pageHeight <= 8) {
		const blocks: PageBlock[] = [
			{ kind: 'title', text: title, lineIndex: -1, continued: false },
		]
		lines.forEach((text, lineIndex) => {
			blocks.push({ kind: 'line', text, lineIndex, continued: false })
		})
		pushPage(pages, blocks, title)
		return pages.length ? pages : [emptyPage(title)]
	}

	const queue: PageBlock[] = [
		{ kind: 'title', text: title, lineIndex: -1, continued: false },
	]
	for (let i = 0; i < lines.length; i++) {
		queue.push({ kind: 'line', text: lines[i], lineIndex: i, continued: false })
	}

	measurer.innerHTML = ''
	let acc: PageBlock[] = []

	const flush = () => {
		pushPage(pages, acc, title)
		acc = []
		measurer.innerHTML = ''
	}

	const pageUsed = () => measurer.scrollHeight

	while (queue.length) {
		const block = queue.shift() as PageBlock
		const el = makeEl(block)
		measurer.appendChild(el)

		if (pageUsed() <= pageHeight + 1) {
			acc.push(block)
			continue
		}

		// 当前页已有内容：这一块放到下一页
		if (acc.length) {
			measurer.removeChild(el)
			flush()
			queue.unshift(block)
			continue
		}

		// 空页都放不下：必须拆开这一块
		if (block.kind === 'title') {
			acc.push(block)
			flush()
			continue
		}

		const remain = pageHeight
		const n = takeFittingText(measurer, el, block.text, remain)
		let cut = n > 0 ? preferBreak(block.text, n) : 1
		if (cut <= 0) cut = Math.max(n, 1)
		if (cut > block.text.length) cut = block.text.length
		if (cut >= block.text.length) {
			acc.push(block)
			flush()
			continue
		}

		const head: PageBlock = {
			kind: 'line',
			text: block.text.slice(0, cut),
			lineIndex: block.lineIndex,
			continued: block.continued,
		}
		const tail: PageBlock = {
			kind: 'line',
			text: block.text.slice(cut),
			lineIndex: block.lineIndex,
			continued: true,
		}
		el.textContent = head.text
		acc.push(head)
		flush()
		queue.unshift(tail)
	}

	flush()
	if (!pages.length) {
		pages.push({
			blocks: [{ kind: 'title', text: title, lineIndex: -1, continued: false }],
			startLine: 0,
			endLine: 0,
			title,
		})
	}
	return pages
}

export function findPageByLine(pages: ReaderPage[], line: number): number {
	if (!pages.length) return 0
	for (let i = 0; i < pages.length; i++) {
		if (pages[i].blocks.some((b) => b.kind === 'line' && b.lineIndex === line)) {
			return i
		}
	}
	if (line <= pages[0].startLine) return 0
	return pages.length - 1
}

export function calcBookPager(
	chapterIdx: number,
	pageIdx: number,
	currentChapterPages: number,
	lineCounts: number[],
	parasPerPage: number,
) {
	const ppp = Math.max(parasPerPage, 0.25)
	const est = (n: number) => (n <= 0 ? 1 : Math.max(1, Math.ceil(n / ppp)))

	let before = 0
	for (let i = 0; i < chapterIdx; i++) {
		before += est(lineCounts[i] ?? 0)
	}

	let remaining = Math.max(0, currentChapterPages - pageIdx - 1)
	for (let i = chapterIdx + 1; i < lineCounts.length; i++) {
		remaining += est(lineCounts[i] ?? 0)
	}

	const bookPage = before + pageIdx + 1
	return {
		remainingPages: remaining,
		bookPage,
		bookPages: bookPage + remaining,
	}
}
