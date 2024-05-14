<script setup lang="ts">
import { Ref, ref, onMounted, nextTick, reactive } from 'vue';
import { dialog, event, fs } from '@tauri-apps/api';
import { useDialog, useMessage, NModal, NInput, NScrollbar } from "naive-ui"
import { useNovelStore } from '../../../store/novel';
import { useStyleStore } from '../../../store/style';
import { useShowStore } from '../../../store/show';

const novel_store = useNovelStore();

const style_store = useStyleStore();

const show_store = useShowStore();

const ref_div_title = ref() as Ref<HTMLElement>;

const ref_div_content = ref() as Ref<HTMLElement>;

type book_mark = {
	id: string, //识别该书签的唯一id
	label: string, //该标签的额外标注信息
	chapter: Number, //所属章节
	line: Number, //所属行
	datetime: string, //创建日期
	content: string, //简短文章内容
}

/*
绑定标签
*/
let div_view: HTMLElement;
let dev_menu = ref();
/*
控制内容的变量数据
*/
//控制菜单是否显示的变量
const is_show_menu = ref(false);
//控制是否显示添加备注
const show_edit_remark = ref(false);

//保存当前书签内容
const bookmark: book_mark = reactive({
	id: '',
	chapter: 0,
	line: 0,
	label: "",
	datetime: '',
	content: '',
});

//存放当前用户右键点击到的div标签
let p_div: EventTarget | null;

const ndialog = useDialog();
const popmsg = useMessage();
/**
 * 初始化函数
 */
onMounted(async () => {
	//初始化view对象
	div_view = document.getElementById('div_view') as HTMLElement;
	div_view.oncontextmenu = function (e) {
		dev_menu.value.style.left = e.pageX + "px";
		dev_menu.value.style.top = e.pageY + "px";
		is_show_menu.value = true;
		p_div = e.target;
	}
	document.addEventListener("click", e => {
		if (dev_menu.value !== undefined && dev_menu.value !== null && !dev_menu.value.contains(e.target)) {
			is_show_menu.value = false;
		}
	})
	document.addEventListener("keydown", async (e) => {
		//ctrl+O：打开小说
		if (e.ctrlKey && e.key === 'o') {
			const selected = await dialog.open({
				multiple: false,
				filters: [{
					name: '小说文件',
					extensions: ['novel', 'txt']
				}]
			});
			if (selected === null) return;
			//打开小说
			fun_open_novel(selected as string);
			return;
		}
		//关闭当前小说
		if (e.ctrlKey && e.key === 'x') {
			e.preventDefault();
			ndialog.info({
				title: '提示',
				content: '确定要关闭当前小说？',
				positiveText: '确定',
				negativeText: '取消',
				onPositiveClick: () => {
					novel_store.close();
					popmsg.info("关闭成功");
				},
				onNegativeClick: () => {

				}
			})
		}
	});
	document.addEventListener("keyup", async e => {
		if (e.key === "PageUp") {
			prev_chapter();
		} else if (e.key === 'PageDown') {
			next_chapter();
		}
		// }else if(e.key==='ArrowUp'){

		// }else if(e.key==='ArrowDown'){

		// }
	})

	//处理文件拖拽
	event.listen<Array<string>>("tauri://file-drop", (e) => {
		let file = e.payload[0];
		fun_open_novel(file);
	})
});

//当前是否已经处于边缘状态（顶部、或者底部）
let IsEdeg: boolean = false;

let timer: NodeJS.Timeout;
//处理滑动事件
async function process_wheel(e: WheelEvent) {
	// 延迟处理wheel事件
	if (timer) {
		clearTimeout(timer);
	}
	timer = setTimeout(async () => {
		if (IsEdeg === false) {
			IsEdeg = true;
			return;
		}
		//如果已经到了边缘
		if (e.deltaY > 0) { //向下滚动，下边缘，翻到下一章
			next_chapter();
		} else if (e.deltaY < 0) { //向上滚动，上边缘，翻到上一章
			prev_chapter();
		}
	}, 300);
}
//处理页面滚动事件
async function process_scroll(e: Event) {
	IsEdeg = false; //只要有scroll事件发生，就说明没有到底部
}

//专门用于打开一个小说的函数，并将其内容显示在界面上
async function fun_open_novel(path: string) {
	let b = await fs.exists(path);
	if (!b) {
		await dialog.message('小说不存在！', { title: '打开失败', type: 'warning' });
		return;
	}

	//关闭显示提示信息
	show_store.set_prompt(false);
	//显示加载动图
	show_store.set_loading(true);
	//console.log(novel_loading.value);
	//更新文件名
	if (path.endsWith(".novel")) {
	} else if (path.endsWith(".txt")) {
		await novel_store.open(path);
	} else {
		await dialog.message('不支持该类型文件！', { title: '打开失败', type: 'warning' });
		return;
	}
	show_store.set_loading(false);
}

//翻到下一章
async function next_chapter() {
	novel_store.next_chapter();
	await nextTick();
	ref_div_title.value.scrollIntoView();
}
//翻到上一章
async function prev_chapter() {
	novel_store.prev_chapter();
	await nextTick();
	ref_div_title.value.scrollIntoView();
}

async function fun_jump(cur_chapter: number, cur_line: number) {
	show_store.set_loading(true);
	novel_store.set_show_chapter(cur_chapter);
	await nextTick(); //等待渲染完成
	let p = ref_div_content.value.querySelector(`:nth-child(${cur_line + 1})`) as Element;
	p.scrollIntoView();
	show_store.set_loading(false);
}
//添加书签函数
async function fun_add_bookmark() {
	//获取所有段落
	let ps = div_view.querySelectorAll('div');
	// 遍历子标签div
	let cur_p = -1;
	for (var i = 0; i < ps.length; i++) {
		if (ps[i] === p_div) {
			cur_p = i;
			break;
		}
	}
	if (cur_p == -1) {
		is_show_menu.value = false;
		popmsg.info("右键到指定的内容才可添加标签");
		return;
	}
	let time = getCurrentDateTime();
	let id = generateUUID();
	bookmark.id = id;
	bookmark.chapter = novel_store.cur_ch_idx;
	bookmark.line = cur_p;
	bookmark.datetime = time;
	bookmark.content = novel_store.show_chapter.lines[cur_p].trim();
	show_edit_remark.value = true;
	is_show_menu.value = false;
}

function getCurrentDateTime() {
	// 获取当前的日期时间
	var currentDate = new Date();
	// 获取年份
	var year = currentDate.getFullYear();
	// 获取月份（注意，返回的月份是从0开始的，所以要加1）
	var month = currentDate.getMonth() + 1;
	// 获取日
	var day = currentDate.getDate();
	// 获取小时
	var hours = String(currentDate.getHours()).padStart(2, '0');
	// 获取分钟
	var minutes = String(currentDate.getMinutes()).padStart(2, '0');
	// 获取秒钟
	var seconds = String(currentDate.getSeconds()).padStart(2, '0');
	// 返回格式化的日期时间
	return year + '/' + month + '/' + day + ' ' + hours + ':' + minutes + ':' + seconds;
}

function generateUUID() {
	return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
		var r = Math.random() * 16 | 0,
			v = c === 'x' ? r : (r & 0x3 | 0x8);
		return v.toString(16);
	});
}
//模态框，取消添加书签
function onNegativeClick() {
	show_edit_remark.value = false;
	popmsg.info('已取消添加书签')
}
//模态框，确认添加书签
async function onPositiveClick() {
	novel_store.add_bookmark(bookmark);
	bookmark.label = "";
	popmsg.success('成功添加书签!');
}
</script>

<template>
	<div class="View" id="div_view">
		<n-scrollbar @onWheel="process_wheel" @onScroll="process_scroll">
			<div class="title" ref="ref_div_title">{{ novel_store.show_chapter.title }}</div>
			<div class="content" ref="ref_div_content" :style="style_store.style">
				<div class="line" v-for="(item, index) in novel_store.show_chapter.lines" :key="index">
					{{ item }}
				</div>
			</div>

			<div class="opt_menu" ref="dev_menu" v-show="is_show_menu">
				<div class="item" @click="fun_add_bookmark">添加书签</div>
			</div>
			<n-modal v-model:show="show_edit_remark" preset="dialog" title="dialog" :mask-closable="false"
				positive-text="确定" negative-text="取消" :closable="false" @positive-click="onPositiveClick"
				@negative-click="onNegativeClick">
				<template #header>
					<div>书签备注</div>
				</template>
				<div>
					<n-input placeholder="填写书签备注" v-model:value="bookmark.label"></n-input>
				</div>
			</n-modal>
		</n-scrollbar>
	</div>
</template>

<style scoped lang="less">
.View {
	height: 100%;
	margin: 0 15px;
	border: none;

	&::-webkit-scrollbar {
		width: 10px;
	}

	.title {
		font-size: 25px;
		color: var(--text-c3);
		text-align: center;
		margin: 15px 0;
		user-select: none;
		font-family: inherit;
		font-weight: inherit;
	}

	.line {
		word-break: break-all;
		color: var(--text-c3);
		user-select: text;
		font-family: inherit;
		font-weight: inherit;
		margin: 15px 0;

		&::selection {
			background: var(--selected-color);
			opacity: 0.5;
			line-height: normal;
		}
	}

	.opt_menu {
		position: fixed;
		border-radius: 5px;
		padding: 3px 5px;
		width: 100px;
		background-color: var(--menu-bgc);
		color: var(--menu-color);
		cursor: pointer;

		.item {
			padding: 2px 10px;
			border-radius: 5px;

			&:hover {
				background-color: var(--mih-color);
			}
		}
	}
}
</style>