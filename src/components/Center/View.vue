<script setup lang="ts">
import { Ref, ref, onMounted, nextTick, reactive, watch } from 'vue';
import { event, } from '@tauri-apps/api';
import { useNovelStore } from '../../store/novel';
import { useStyleStore } from '../../store/style';
import { useShowStore } from '../../store/show';
import { Bookmark } from '../../api/novel';
import { ElMessage, ElMessageBox, ElScrollbar, ElInput, ElDialog, ElButton } from 'element-plus';
import * as dialog from "@tauri-apps/plugin-dialog"
import * as fs from "@tauri-apps/plugin-fs"

const novel_store = useNovelStore();

const style_store = useStyleStore();

const show_store = useShowStore();

const ref_div_title = ref() as Ref<HTMLElement>;

const ref_div_content = ref() as Ref<HTMLElement>;

let dev_menu = ref() as Ref<HTMLElement>;

//控制菜单是否显示的变量
const is_show_menu = ref(false);
//控制是否显示添加备注
const show_edit_remark = ref(false);

//保存当前书签内容
const bookmark: Bookmark = reactive({
	id: '',
	chapter: 0,
	line: 0,
	label: "",
	datetime: '',
	content: '',
});

//存放当前用户右键点击到的div标签
let p_div: EventTarget | null;

onMounted(async () => {

	novel_store.set_jump_fun(fun_jump);

	ref_div_content.value.oncontextmenu = function (e) {
		dev_menu.value.style.left = e.pageX + "px";
		dev_menu.value.style.top = e.pageY + "px";
		is_show_menu.value = true;
		p_div = e.target;
	}
	document.addEventListener("click", e => {
		if (dev_menu.value !== undefined && dev_menu.value !== null && !dev_menu.value.contains(e.target as Node)) {
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
			//@ts-ignore
			fun_open_novel(selected);
			return;
		}
		//关闭当前小说
		if (e.ctrlKey && e.key === 'x') {
			e.preventDefault();
			const ret = await ElMessageBox.confirm("确定要关闭当前小说？", '注意', {
				confirmButtonText: '关闭',
				cancelButtonText: '取消',
				type: 'warning'
			});
			if (ret == 'confirm') {
				novel_store.close();
				show_store.set_prompt(true);
				ElMessage.success("关闭成功");
			}
		}

	});
	document.addEventListener("keyup", async e => {
		if (e.key === "PageUp" || e.key == "ArrowLeft") {
			e.preventDefault();
			prev_chapter();
		} else if (e.key === 'PageDown' || e.key == "ArrowRight") {
			e.preventDefault();
			next_chapter();
		}
	})

	//处理文件拖拽
	event.listen<Array<string>>("tauri://drag-drop", (e) => {
		let file = (e.payload as any).paths[0];
		fun_open_novel(file);
	})
});

//专门用于打开一个小说的函数，并将其内容显示在界面上
async function fun_open_novel(path: string) {
	let b = await fs.exists(path);
	if (!b) {
		await dialog.message('小说不存在！', { title: '打开失败', kind: 'warning' });
		return;
	}

	//关闭显示提示信息
	show_store.set_prompt(false);
	//显示加载动图
	show_store.set_loading(true);
	//更新文件名
	if (path.endsWith(".novel")) {
	} else if (path.endsWith(".txt")) {
		await novel_store.open(path);
	} else {
		await dialog.message('不支持该类型文件！', { title: '打开失败', kind: 'warning' });
		return;
	}
	show_store.set_loading(false);
}


watch(() => novel_store.cur_ch_idx, () => {
	ElMessage.success(`${novel_store.show_chapter.title}`);
});

//翻到下一章
async function next_chapter() {
	if (!novel_store.isopen) {
		ElMessage.warning('你还未打开小说');
		return;
	}

	const ret = novel_store.next_chapter();
	if (!ret) {
		ElMessage.warning('已经到最后一章了');
	} else {
		await nextTick();
		ref_div_title.value.scrollIntoView();
	}
}
//翻到上一章
async function prev_chapter() {
	if (!novel_store.isopen) {
		ElMessage.warning('你还未打开小说');
		return;
	}

	let ret = await novel_store.prev_chapter();
	if (!ret) {
		ElMessage.warning('已经是第一章了~~~');
	} else {
		await nextTick();
		ref_div_title.value.scrollIntoView();
	}

}

async function fun_jump(cur_chapter: number, cur_line: number) {
	console.log(cur_chapter, cur_line);

	show_store.set_loading(true);
	novel_store.set_show_chapter(cur_chapter);


	setTimeout(() => {
		if (scroll_line_to_view(cur_line)) {
		} else {
			ElMessage.warning('跳转到指定行失败，请检查小说是否有足够长的段落');
		}
		show_store.set_loading(false);
	}, 300);

}

function scroll_line_to_view(line: number): boolean {
	if (line == undefined) {
		return false;
	}
	let p = ref_div_content.value.querySelector(`:nth-child(${line + 1})`) as Element;
	if (!p) {
		return false;
	}
	p.scrollIntoView();
	return true;
}

//添加书签函数
async function fun_add_bookmark() {
	//获取所有段落
	let ps = ref_div_content.value.querySelectorAll('div');
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
		ElMessage.info("右键到指定的内容才可添加标签");
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
	ElMessage.info('已取消添加书签');
	show_edit_remark.value = false;
}
//模态框，确认添加书签
async function onPositiveClick() {
	await novel_store.add_bookmark(bookmark);
	bookmark.label = "";
	ElMessage.success('成功添加书签!');
	show_edit_remark.value = false;
}

let change_page = 0;
let timer: string | number | NodeJS.Timeout | undefined;

function on_scroll() {
	change_page = 0;

	//获取所有段落
	let ps = ref_div_content.value.querySelectorAll('div');
	let p_rect = ref_div_content.value.getBoundingClientRect();
	let line = 0;
	for (let i = 0; i < ps.length; i++) {
		let rect = ps[i].getBoundingClientRect();
		if (rect.top > p_rect.top) {
			line = i;
			break;
		}
	}
	novel_store.set_line(line);
}

function on_whell(e: WheelEvent) {
	change_page = e.deltaY;
	if (timer) clearTimeout(timer);
	timer = setTimeout(() => {
		if (change_page > 0) {
			next_chapter();
		} else if (change_page < 0) {
			prev_chapter();
		}
	}, 200);
}
</script>

<template>
	<div class="View" @wheel="on_whell($event)">
		<el-scrollbar @scroll="on_scroll">
			<div class="title" ref="ref_div_title">{{ novel_store.show_chapter.title }}</div>
			<div class="content" ref="ref_div_content" :style="style_store.style">
				<div class="line" v-for="(item, index) in novel_store.show_chapter.lines" :key="index">
					{{ item }}
				</div>
			</div>
		</el-scrollbar>
		<div class="opt_menu" ref="dev_menu" v-show="is_show_menu">
			<div class="item" @click="fun_add_bookmark">添加书签</div>
		</div>
		<el-dialog v-model="show_edit_remark" width="500">
			<template #header>
				<div>书签备注</div>
			</template>
			<el-input placeholder="填写书签备注" v-model="bookmark.label"></el-input>
			<template #footer>
				<div class="dialog-footer">
					<el-button @click="onNegativeClick">取消</el-button>
					<el-button type="primary" @click="onPositiveClick">确定</el-button>
				</div>
			</template>
		</el-dialog>
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