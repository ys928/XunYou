<script setup lang="ts">
import { Ref, ref, onMounted, onUnmounted, nextTick, reactive, watch, shallowRef } from 'vue';
import { event, } from '@tauri-apps/api';
import { useNovelStore } from '../../store/novel';
import { useStyleStore } from '../../store/style';
import { useShowStore } from '../../store/show';
import { Bookmark } from '../../api/novel';
import { ElMessage, ElMessageBox, ElInput, ElDialog, ElButton } from 'element-plus';
import * as dialog from "@tauri-apps/plugin-dialog"
import * as fs from "@tauri-apps/plugin-fs"
import {
	ReaderPage,
	calcBookPager,
	emptyPage,
	findPageByLine,
	paginateChapter,
} from '../../reader/paginate';

const FLIP_MS = 720;
const RESIZE_WAIT_MS = 200;

const novel_store = useNovelStore();
const style_store = useStyleStore();
const show_store = useShowStore();

const ref_div_content = ref() as Ref<HTMLElement>;
const ref_book = ref() as Ref<HTMLElement>;
const ref_stage = ref() as Ref<HTMLElement>;
const ref_body = ref() as Ref<HTMLElement>;
const ref_measure = ref() as Ref<HTMLElement>;
const ref_flip = ref() as Ref<HTMLElement>;

let dev_menu = ref() as Ref<HTMLElement>;

const is_show_menu = ref(false);
const show_edit_remark = ref(false);

const bookmark: Bookmark = reactive({
	id: '',
	chapter: 0,
	line: 0,
	label: "",
	datetime: '',
	content: '',
});

let p_div: EventTarget | null;

type PageView = {
	page: ReaderPage
	pageNo: number
	remaining: number
	chapterPages: number
}

const pages = shallowRef<ReaderPage[]>([]);
const pageIndex = ref(0);
const frontView = ref<PageView>({
	page: emptyPage(),
	pageNo: 1,
	remaining: 0,
	chapterPages: 1,
});
const underView = ref<PageView>({
	page: emptyPage(),
	pageNo: 1,
	remaining: 0,
	chapterPages: 1,
});

const flipping = ref(false);
const flipDir = ref<'next' | 'prev' | null>(null);
const noAnim = ref(false);

let turning = false;
let queuedTurn: 'next' | 'prev' | null = null;
let needsRebuild = false;
let keepLineOnRebuild: number | null = null;
let resizeTimer: number | undefined;
let resizeObserver: ResizeObserver | null = null;
let lastBookW = 0;
let lastBookH = 0;
let flipWaitToken = 0;
let wheelAcc = 0;
let wheelResetTimer: number | undefined;

function isTypingTarget(target: EventTarget | null): boolean {
	if (!(target instanceof HTMLElement)) return false;
	return !!target.closest('input, textarea, .el-dialog, .el-message-box, [contenteditable="true"]');
}

onMounted(async () => {
	novel_store.set_jump_fun(fun_jump);

	document.addEventListener("click", e => {
		if (dev_menu.value !== undefined && dev_menu.value !== null && !dev_menu.value.contains(e.target as Node)) {
			is_show_menu.value = false;
		}
	})
	document.addEventListener("keydown", on_keydown);

	event.listen<Array<string>>("tauri://drag-drop", (e) => {
		let file = (e.payload as any).paths[0];
		fun_open_novel(file);
	})

	const target = ref_book.value || ref_stage.value;
	if (target) {
		resizeObserver = new ResizeObserver(() => {
			if (!ref_book.value) return;
			if (turning || flipping.value) {
				needsRebuild = true;
				return;
			}
			const w = ref_book.value.clientWidth;
			const h = ref_book.value.clientHeight;
			if (Math.abs(w - lastBookW) < 2 && Math.abs(h - lastBookH) < 2) return;
			lastBookW = w;
			lastBookH = h;
			if (resizeTimer) clearTimeout(resizeTimer);
			resizeTimer = window.setTimeout(() => {
				if (!novel_store.isopen || turning || flipping.value) return;
				scheduleRebuild(keepLineOnRebuild ?? frontView.value.page.startLine);
			}, RESIZE_WAIT_MS);
		});
		resizeObserver.observe(target);
	}
});

onUnmounted(() => {
	document.removeEventListener("keydown", on_keydown);
	resizeObserver?.disconnect();
	if (resizeTimer) clearTimeout(resizeTimer);
	if (wheelResetTimer) clearTimeout(wheelResetTimer);
});

async function on_keydown(e: KeyboardEvent) {
	if (e.ctrlKey && e.key === 'o') {
		const selected = await dialog.open({
			multiple: false,
			filters: [{
				name: '小说文件',
				extensions: ['novel', 'txt']
			}]
		});
		if (selected === null) return;
		//@ts-ignore
		fun_open_novel(selected);
		return;
	}
	if (e.ctrlKey && e.key === 'x') {
		e.preventDefault();
		const ret = await ElMessageBox.confirm("确定要关闭当前小说？", '注意', {
			confirmButtonText: '关闭',
			cancelButtonText: '取消',
			type: 'warning'
		});
		if (ret == 'confirm') {
			novel_store.close();
			pages.value = [];
			pageIndex.value = 0;
			frontView.value = { page: emptyPage(), pageNo: 1, remaining: 0, chapterPages: 1 };
			underView.value = { page: emptyPage(), pageNo: 1, remaining: 0, chapterPages: 1 };
			show_store.set_prompt(true);
			ElMessage.success("关闭成功");
		}
		return;
	}

	if (isTypingTarget(e.target)) return;
	if (e.ctrlKey || e.altKey || e.metaKey) return;

	const nextKeys = ['ArrowRight', 'ArrowDown', 'PageDown'];
	const prevKeys = ['ArrowLeft', 'ArrowUp', 'PageUp'];
	if (nextKeys.includes(e.key)) {
		e.preventDefault();
		turnPage('next');
	} else if (prevKeys.includes(e.key)) {
		e.preventDefault();
		turnPage('prev');
	}
}

async function fun_open_novel(path: string) {
	let b = await fs.exists(path);
	if (!b) {
		await dialog.message('小说不存在！', { title: '打开失败', kind: 'warning' });
		return;
	}

	show_store.set_prompt(false);
	show_store.set_loading(true);
	if (path.endsWith(".novel")) {
	} else if (path.endsWith(".txt")) {
		await novel_store.open(path);
	} else {
		await dialog.message('不支持该类型文件！', { title: '打开失败', kind: 'warning' });
		show_store.set_loading(false);
		return;
	}
	show_store.set_loading(false);
}

watch(() => style_store.style, () => {
	if (!novel_store.isopen) return;
	scheduleRebuild(frontView.value.page.startLine ?? novel_store.cur_line_idx);
}, { deep: true });

watch(() => novel_store.isopen, (open) => {
	if (!open) {
		pages.value = [];
		pageIndex.value = 0;
	}
});

function metrics() {
	const body = ref_body.value;
	if (!body) return { pageHeight: 0, linesPerPage: 1, contentWidth: 0 };
	const cs = getComputedStyle(body);
	const padY = parseFloat(cs.paddingTop) + parseFloat(cs.paddingBottom);
	const padX = parseFloat(cs.paddingLeft) + parseFloat(cs.paddingRight);
	const pageHeight = Math.max(0, body.clientHeight - padY);
	const contentWidth = Math.max(0, body.clientWidth - padX);
	const fontSize = style_store.font_size || 16;
	const lineHeightPx = fontSize * (style_store.line_height / 10);
	const linesPerPage = Math.max(1, Math.floor(pageHeight / Math.max(lineHeightPx, 1)));
	return { pageHeight, linesPerPage, contentWidth };
}

async function waitLayout() {
	await nextTick();
	await new Promise<void>((resolve) => {
		requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
	});
}

async function buildPages(): Promise<ReaderPage[]> {
	if (!novel_store.isopen || !ref_body.value || !ref_measure.value) {
		return pages.value;
	}
	await waitLayout();
	let { pageHeight, contentWidth } = metrics();
	if (pageHeight <= 0 || contentWidth <= 0) {
		await waitLayout();
		({ pageHeight, contentWidth } = metrics());
	}
	if (pageHeight <= 0 || contentWidth <= 0) {
		return pages.value;
	}
	const box = ref_measure.value;
	box.style.position = 'fixed';
	box.style.left = '-14000px';
	box.style.top = '0';
	box.style.height = 'auto';
	box.style.maxHeight = 'none';
	box.style.overflow = 'visible';
	box.style.visibility = 'hidden';
	box.style.pointerEvents = 'none';
	box.style.width = `${contentWidth}px`;
	box.style.fontSize = style_store.style.fontSize;
	box.style.lineHeight = style_store.style.lineHeight;
	box.style.fontFamily = style_store.style.fontFamily;
	box.style.fontWeight = String(style_store.style.fontWeight);
	const built = paginateChapter(
		ref_measure.value,
		novel_store.show_chapter.title,
		novel_store.show_chapter.lines,
		pageHeight,
	);
	pages.value = built;
	return built;
}

function makeView(index: number, chapterPages: number, chapterIdx: number): PageView {
	const page = pages.value[index] ?? emptyPage(novel_store.show_chapter.title);
	const paras = novel_store.show_chapter.lines.length;
	const parasPerPage = chapterPages > 0 ? Math.max(paras / chapterPages, 1 / chapterPages) : metrics().linesPerPage;
	const book = calcBookPager(
		chapterIdx,
		index,
		chapterPages,
		novel_store.chapter_line_counts,
		parasPerPage,
	);
	return {
		page,
		pageNo: index + 1,
		remaining: book.remainingPages,
		chapterPages,
	};
}

function syncPagerStore(view: PageView) {
	const paras = novel_store.show_chapter.lines.length;
	const parasPerPage = view.chapterPages > 0
		? Math.max(paras / view.chapterPages, 1 / view.chapterPages)
		: metrics().linesPerPage;
	const book = calcBookPager(
		novel_store.cur_ch_idx,
		Math.max(0, view.pageNo - 1),
		view.chapterPages,
		novel_store.chapter_line_counts,
		parasPerPage,
	);
	novel_store.set_pager_state({
		chapterPage: view.pageNo,
		chapterPageCount: view.chapterPages,
		remainingPages: book.remainingPages,
		bookPage: book.bookPage,
		bookPages: book.bookPages,
	});
	novel_store.set_line(view.page.startLine);
	keepLineOnRebuild = view.page.startLine;
}

async function rebuildPages(targetLine?: number) {
	const built = await buildPages();
	if (!built.length) return;
	const line = targetLine ?? novel_store.cur_line_idx ?? 0;
	const idx = findPageByLine(built, line);
	pageIndex.value = idx;
	const view = makeView(idx, built.length, novel_store.cur_ch_idx);
	frontView.value = view;
	underView.value = view;
	keepLineOnRebuild = view.page.startLine;
	syncPagerStore(view);
}

function scheduleRebuild(line?: number) {
	keepLineOnRebuild = line ?? frontView.value.page.startLine ?? novel_store.cur_line_idx;
	if (flipping.value || turning) {
		needsRebuild = true;
		return;
	}
	rebuildPages(keepLineOnRebuild);
}

async function waitFlip(): Promise<void> {
	const token = ++flipWaitToken;
	await new Promise<void>((resolve) => {
		const el = ref_flip.value;
		let settled = false;
		const done = () => {
			if (settled || token !== flipWaitToken) return;
			settled = true;
			el?.removeEventListener('transitionend', onEnd);
			resolve();
		};
		const onEnd = (e: TransitionEvent) => {
			if (e.propertyName.includes('transform')) done();
		};
		el?.addEventListener('transitionend', onEnd);
		window.setTimeout(done, FLIP_MS + 90);
	});
}

async function applyFlip(dir: 'next' | 'prev', dest: PageView) {
	underView.value = dest;
	noAnim.value = true;
	flipDir.value = dir;
	flipping.value = false;
	await nextTick();
	void ref_flip.value?.offsetWidth;
	noAnim.value = false;
	flipping.value = true;
	await waitFlip();

	noAnim.value = true;
	frontView.value = dest;
	underView.value = dest;
	flipping.value = false;
	flipDir.value = null;
	await nextTick();
	void ref_flip.value?.offsetWidth;
	noAnim.value = false;
}

async function turnToAdjacentChapter(dir: 'next' | 'prev') {
	const moved = dir === 'next'
		? await novel_store.next_chapter()
		: await novel_store.prev_chapter();
	if (!moved) {
		ElMessage.warning(dir === 'next' ? '已经到最后一页了' : '已经是第一页了');
		return;
	}
	const built = await buildPages();
	const idx = dir === 'next' ? 0 : Math.max(0, built.length - 1);
	pageIndex.value = idx;
	const dest = makeView(idx, built.length, novel_store.cur_ch_idx);
	await applyFlip(dir, dest);
	syncPagerStore(dest);
}

async function turnPage(dir: 'next' | 'prev') {
	if (!novel_store.isopen) {
		ElMessage.warning('你还未打开小说');
		return;
	}
	if (turning || flipping.value) {
		queuedTurn = dir;
		return;
	}
	turning = true;

	try {
		if (!pages.value.length) {
			await rebuildPages(keepLineOnRebuild ?? novel_store.cur_line_idx);
			if (!pages.value.length) return;
		}

		const chapterPages = pages.value.length;
		if (dir === 'next') {
			if (pageIndex.value < chapterPages - 1) {
				pageIndex.value += 1;
				const dest = makeView(pageIndex.value, chapterPages, novel_store.cur_ch_idx);
				await applyFlip('next', dest);
				syncPagerStore(dest);
			} else {
				const current = pages.value[pageIndex.value];
				const lastLine = novel_store.show_chapter.lines.length - 1;
				if (current && lastLine >= 0 && current.endLine < lastLine) {
					const built = await buildPages();
					const idx = findPageByLine(built, current.startLine);
					if (idx < built.length - 1) {
						pageIndex.value = idx + 1;
						const dest = makeView(pageIndex.value, built.length, novel_store.cur_ch_idx);
						await applyFlip('next', dest);
						syncPagerStore(dest);
					} else {
						await turnToAdjacentChapter('next');
					}
				} else {
					await turnToAdjacentChapter('next');
				}
			}
		} else {
			if (pageIndex.value > 0) {
				pageIndex.value -= 1;
				const dest = makeView(pageIndex.value, chapterPages, novel_store.cur_ch_idx);
				await applyFlip('prev', dest);
				syncPagerStore(dest);
			} else {
				await turnToAdjacentChapter('prev');
			}
		}

		if (needsRebuild) {
			needsRebuild = false;
			await rebuildPages(keepLineOnRebuild ?? frontView.value.page.startLine);
		}
	} finally {
		turning = false;
		if (queuedTurn) {
			const next = queuedTurn;
			queuedTurn = null;
			turnPage(next);
		}
	}
}

async function fun_jump(cur_chapter: number, cur_line: number) {
	show_store.set_prompt(false);
	keepLineOnRebuild = cur_line;
	await novel_store.set_show_chapter(cur_chapter);
	await nextTick();
	await rebuildPages(cur_line);
}

function on_wheel(e: WheelEvent) {
	if (!novel_store.isopen) return;
	e.preventDefault();
	if (isTypingTarget(e.target)) return;
	wheelAcc += e.deltaY;
	if (wheelResetTimer) clearTimeout(wheelResetTimer);
	wheelResetTimer = window.setTimeout(() => {
		wheelAcc = 0;
	}, 80);
	if (wheelAcc > 48) {
		wheelAcc = 0;
		turnPage('next');
	} else if (wheelAcc < -48) {
		wheelAcc = 0;
		turnPage('prev');
	}
}

function on_context_menu(e: MouseEvent) {
	if (!dev_menu.value) return;
	dev_menu.value.style.left = e.pageX + "px";
	dev_menu.value.style.top = e.pageY + "px";
	is_show_menu.value = true;
	p_div = e.target;
}

function on_turn_click(dir: 'next' | 'prev') {
	if (window.getSelection()?.toString()) return;
	turnPage(dir);
}

async function fun_add_bookmark() {
	const host = (p_div instanceof HTMLElement) ? p_div.closest('.line') as HTMLElement | null : null;
	const attr = host?.dataset.line;
	let cur_p = attr !== undefined ? Number(attr) : -1;
	if (cur_p < 0 || Number.isNaN(cur_p)) {
		is_show_menu.value = false;
		ElMessage.info("右键到指定的内容才可添加标签");
		return;
	}
	bookmark.id = generateUUID();
	bookmark.chapter = novel_store.cur_ch_idx;
	bookmark.line = cur_p;
	bookmark.datetime = getCurrentDateTime();
	bookmark.content = (novel_store.show_chapter.lines[cur_p] || '').trim();
	show_edit_remark.value = true;
	is_show_menu.value = false;
}

function getCurrentDateTime() {
	var currentDate = new Date();
	var year = currentDate.getFullYear();
	var month = currentDate.getMonth() + 1;
	var day = currentDate.getDate();
	var hours = String(currentDate.getHours()).padStart(2, '0');
	var minutes = String(currentDate.getMinutes()).padStart(2, '0');
	var seconds = String(currentDate.getSeconds()).padStart(2, '0');
	return year + '/' + month + '/' + day + ' ' + hours + ':' + minutes + ':' + seconds;
}

function generateUUID() {
	return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
		var r = Math.random() * 16 | 0,
			v = c === 'x' ? r : (r & 0x3 | 0x8);
		return v.toString(16);
	});
}

function onNegativeClick() {
	show_edit_remark.value = false;
	ElMessage.info('已取消添加书签');
}

async function onPositiveClick() {
	await novel_store.add_bookmark(bookmark);
	bookmark.label = "";
	ElMessage.success('成功添加书签!');
	show_edit_remark.value = false;
}
</script>

<template>
	<div class="View" @wheel="on_wheel">
		<div class="book-stage" ref="ref_stage" v-show="novel_store.isopen">
			<div class="edge-cover"></div>
			<div class="book" ref="ref_book">
				<div class="sheet under">
					<div class="sheet-inner">
						<div class="running-head">{{ underView.page.title }}</div>
						<div class="sheet-body" :style="style_store.style">
							<template v-for="(block, i) in underView.page.blocks" :key="'u' + i">
								<div v-if="block.kind === 'title'" class="title">{{ block.text }}</div>
								<div
									v-else
									class="line"
									:class="{ continued: block.continued }"
									:data-line="block.lineIndex"
								>{{ block.text }}</div>
							</template>
						</div>
						<div class="running-foot">
							<span class="rest">还剩 {{ underView.remaining }} 页</span>
							<span class="pg">{{ underView.pageNo }}</span>
							<span class="rest spacer">{{ underView.chapterPages }} 页</span>
						</div>
					</div>
				</div>
				<div
					class="sheet flip"
					ref="ref_flip"
					:class="{
						'is-flipping': flipping,
						'dir-next': flipDir === 'next',
						'dir-prev': flipDir === 'prev',
						'no-anim': noAnim,
					}"
				>
					<div class="face front">
						<div class="sheet-inner" ref="ref_div_content" @contextmenu="on_context_menu">
							<div class="running-head">{{ frontView.page.title }}</div>
							<div class="sheet-body" ref="ref_body" :style="style_store.style">
								<template v-for="(block, i) in frontView.page.blocks" :key="'f' + i">
									<div v-if="block.kind === 'title'" class="title">{{ block.text }}</div>
									<div
										v-else
										class="line"
										:class="{ continued: block.continued }"
										:data-line="block.lineIndex"
									>{{ block.text }}</div>
								</template>
							</div>
							<div class="running-foot">
								<span class="rest">还剩 {{ frontView.remaining }} 页</span>
								<span class="pg">{{ frontView.pageNo }}</span>
								<span class="rest spacer">{{ frontView.chapterPages }} 页</span>
							</div>
						</div>
						<div class="fold-glint"></div>
					</div>
					<div class="face back"></div>
				</div>
				<div class="turn-zone left" @click="on_turn_click('prev')"></div>
				<div class="turn-zone right" @click="on_turn_click('next')"></div>
			</div>
			<div class="edge-cover"></div>
		</div>
		<div
			class="pager-measure"
			ref="ref_measure"
			:style="style_store.style"
			aria-hidden="true"
		></div>
		<div class="ctx-menu" ref="dev_menu" v-show="is_show_menu">
			<div class="item" @click="fun_add_bookmark">添加书签</div>
		</div>
		<el-dialog v-model="show_edit_remark" width="420" class="bookmark-dialog">
			<template #header>
				<div>书签备注</div>
			</template>
			<el-input placeholder="填写书签备注（可选）" v-model="bookmark.label"></el-input>
			<template #footer>
				<div class="dialog-footer">
					<el-button @click="onNegativeClick">取消</el-button>
					<el-button type="primary" @click="onPositiveClick">确定</el-button>
				</div>
			</template>
		</el-dialog>
	</div>
</template>

<style scoped lang="scss">
.View {
	height: 100%;
	overflow: hidden;
	background: var(--page-bg);
}

.book-stage {
	height: 100%;
	display: flex;
	flex-direction: row;
	align-items: stretch;
	padding: 16px 0 12px;
	perspective: 2800px;
	perspective-origin: 50% 50%;
	overflow: hidden;
	position: relative;
}

.edge-cover {
	flex: 1 1 28px;
	min-width: 28px;
	background: var(--page-bg);
	z-index: 8;
	position: relative;
}

.book {
	position: relative;
	flex: 0 1 780px;
	width: min(780px, calc(100% - 56px));
	max-width: 780px;
	height: 100%;
	min-height: 0;
	overflow: hidden;
	transform-style: preserve-3d;
	z-index: 1;
	background: var(--page-bg);
}

.pager-measure {
	position: fixed;
	left: -14000px;
	top: 0;
	visibility: hidden;
	pointer-events: none;
	height: auto;
	max-height: none;
	overflow: visible;
	z-index: -1;
}

.sheet {
	position: absolute;
	inset: 0;
	border-radius: 2px 10px 10px 2px;
	background: var(--page-bg);
	transform-style: preserve-3d;
	box-shadow:
		4px 0 12px rgba(0, 0, 0, 0.12),
		0 18px 40px rgba(0, 0, 0, 0.12);
	border: 1px solid var(--border-subtle);
}

.sheet.under {
	z-index: 1;
}

.sheet.flip {
	z-index: 2;
	transform-origin: left center;
	transition: transform 0.72s cubic-bezier(0.645, 0.045, 0.355, 1);
	will-change: transform;

	&.dir-prev {
		transform-origin: right center;
	}

	&.no-anim {
		transition: none !important;
	}

	&.is-flipping.dir-next {
		transform: rotateY(-180deg);
		box-shadow: none;
	}

	&.is-flipping.dir-prev {
		transform: rotateY(180deg);
		box-shadow: none;
	}
}

.face {
	position: absolute;
	inset: 0;
	backface-visibility: hidden;
	border-radius: inherit;
	overflow: hidden;
	background: var(--page-bg);
}

.face.front {
	transform: translateZ(1px);
}

.face.back {
	transform: rotateY(180deg);
	background: var(--page-bg);
	visibility: hidden;
}

.fold-glint {
	position: absolute;
	inset: 0;
	pointer-events: none;
	opacity: 0;
	transition: none;
	background: linear-gradient(
		90deg,
		transparent 0%,
		transparent 72%,
		rgba(0, 0, 0, 0.22) 100%
	);
}

.sheet.flip.is-flipping:not(.no-anim) .fold-glint {
	opacity: 1;
}

.sheet.flip.dir-prev .fold-glint {
	background: linear-gradient(
		270deg,
		transparent 0%,
		transparent 72%,
		rgba(0, 0, 0, 0.22) 100%
	);
}

.sheet-inner {
	display: flex;
	flex-direction: column;
	height: 100%;
	background:
		linear-gradient(90deg, rgba(0, 0, 0, 0.045) 0, transparent 14px),
		var(--page-bg);
}

.running-head {
	flex-shrink: 0;
	height: 36px;
	padding: 12px 32px 0;
	font-size: 11px;
	letter-spacing: 0.12em;
	color: var(--text-color1);
	text-align: center;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
	user-select: none;
}

.sheet-body {
	flex: 1;
	min-height: 0;
	overflow: hidden;
	padding: 6px 32px 8px;
}

.running-foot {
	flex-shrink: 0;
	display: grid;
	grid-template-columns: 1fr auto 1fr;
	align-items: center;
	height: 34px;
	padding: 0 32px 10px;
	user-select: none;
	font-variant-numeric: tabular-nums;

	.rest {
		font-size: 11px;
		color: var(--text-color1);
	}

	.spacer {
		text-align: right;
	}

	.pg {
		font-size: 12px;
		color: var(--text-color);
		letter-spacing: 0.08em;
	}
}

.sheet-body,
.pager-measure {
	:deep(.title) {
		font-size: 20px;
		color: var(--text-c3);
		text-align: center;
		margin: 8px 0 28px;
		user-select: none;
		font-family: inherit;
		font-weight: inherit;
		line-height: 1.6;
	}

	:deep(.line) {
		word-break: break-all;
		color: var(--text-c3);
		user-select: text;
		font-family: inherit;
		font-weight: inherit;
		margin: 0.7em 0;
		text-indent: 2em;

		&.continued {
			text-indent: 0;
			margin-top: 0;
		}

		&::selection {
			background: var(--selected-color);
			line-height: normal;
		}
	}
}

.turn-zone {
	position: absolute;
	top: 0;
	bottom: 0;
	width: 72px;
	z-index: 4;
	cursor: pointer;

	&.left {
		left: 0;
		cursor: w-resize;
	}

	&.right {
		right: 0;
		cursor: e-resize;
	}
}
</style>
