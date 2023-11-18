<template>
	<Scrollbar class="View" id="div_view" :style="{
		'font-family': mainpan_font_family,
		'font-weight': mainpan_font_weight,

	}" @onWheel="process_wheel" @onScroll="process_scroll">
		<template v-for="(item, index) in novel_show_lines">
			<div v-if="IsTitle(item)" class="title">{{ item }}</div>
			<div v-else class="paragraph" :style="{
				'font-size': mainpan_font_size + 'px',
				'line-height': mainpan_line_height / 10 + 'em',
				'background-size': '15px ' + mainpan_line_height / 10 + 'em'
			}">
				{{ item }}
			</div>
		</template>
		<div class="opt_menu" ref="dev_menu" v-show="is_show_menu">
			<div class="item" @click="fun_add_bookmark">添加书签</div>
		</div>
		<n-modal v-model:show="show_edit_remark" preset="dialog" title="dialog" :mask-closable="false" positive-text="确定"
			negative-text="取消" :closable="false" @positive-click="onPositiveClick" @negative-click="onNegativeClick">
			<template #header>
				<div>书签备注</div>
			</template>
			<div>
				<n-input placeholder="填写书签备注" v-model:value="bookmark.label"></n-input>
			</div>
		</n-modal>
	</Scrollbar>
</template>

<script setup lang="ts">
import { Ref, ref, onMounted, inject, nextTick, reactive } from 'vue';
import { dialog, event, fs, invoke } from '@tauri-apps/api';
import Scrollbar from "../../../common/Scrollbar.vue"
import { useDialog, useMessage, NModal, NInput } from "naive-ui"
/**
 * 自定义类型
 */
//目录类型
type type_cata_obj = {
	name: string,
	index: number
};
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
//程序要进行显示的小说行数内容
const novel_show_lines = ref([]) as Ref<Array<string>>;
//保存当前书签内容
const bookmark: book_mark = reactive({
	id: '',
	chapter: 0,
	line: 0,
	label: "",
	datetime: '',
	content: '',
});


/**
 * 取出父组件传递下来的变量
 */
//取出存放打开小说的函数变量，本组件用来存放该函数
const root_fun_open_novel = inject('root_fun_open_novel') as Ref<Function>;
//当前打开的小说名称,本组件修改它
const app_title = inject("app_title") as Ref<string>;
//用于控制提示消息是否显示
const cenpan_show_prompt = inject("cenpan_show_prompt") as Ref<boolean>;
//用于控制是否显示加载图标
const cenpan_show_loading = inject("cenpan_show_loading") as Ref<boolean>;
//存放所有遍历到的小说目录
const mainpan_novel_cata = inject("mainpan_novel_cata") as Ref<Array<type_cata_obj>>
//配合搜索功能，存放要显示的目录
const mainpan_show_novel_cata = inject('mainpan_show_novel_cata') as Ref<Array<type_cata_obj>>;
//存放跳转函数
const mainpan_nov_jump_fun = inject("mainpan_nov_jump_fun") as Ref<Function>;
//字体大小
const mainpan_font_size = inject('mainpan_font_size') as Ref<number>;
//字体粗细
const mainpan_font_weight = inject("mainpan_font_weight") as Ref<number>
//字体
const mainpan_font_family = inject("mainpan_font_family") as Ref<string>;
//行高
const mainpan_line_height = inject("mainpan_line_height") as Ref<number>;
//存放当前小说所有书签
const mainpan_bookmark = inject("mainpan_bookmark") as Ref<Array<book_mark>>
//存放当前小说路径
const mainpan_nov_path = inject("mainpan_nov_path") as Ref<string>

/*
普通变量
*/
//存放读取到的所有小说内容
let novel_lines: Array<string>;
//存放读取到的小说所有内容，并按章节划分存放，每章第一节为标题。
let novel_chapter: Array<Array<string>> = [];
//当前阅读章节
let cur_chap_num: number;
//存放当前打开的小说的路径
let cur_novel_path: string;
//存放当前用户右键点击到的div标签
let p_div: EventTarget | null;

const ndialog = useDialog();
const popmsg = useMessage();
/**
 * 初始化函数
 */
onMounted(async () => {
	// //初始化跳转函数
	mainpan_nov_jump_fun.value = fun_jump;
	// //初始化跳转组件的按键处理函数
	// cenpan_pro_jump_input.value=process_jump_input;
	//初始化打开小说的函数
	root_fun_open_novel.value = fun_open_novel;
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
					extensions: ['novel','txt']
				}]
			});
			if (selected === null) return;
			//打开小说
			fun_open_novel(selected as string);
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
					if (app_title.value === undefined || app_title.value.length === 0) {
						popmsg.info("当前没有打开小说");
						return;
					}
					novel_show_lines.value = []; //清空界面内容
					app_title.value = ""; //清除当前显示的文件名
					cenpan_show_prompt.value = true; //重新显示提示信息
					mainpan_bookmark.value = []; //清空书签
					mainpan_show_novel_cata.value = []; //清空目录
					mainpan_novel_cata.value = []; //清空目录
					if (novel_lines !== undefined) {
						novel_lines = []; //清空读取到的小说内容
					}
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

	mainpan_novel_cata.value.splice(0);
	//关闭显示提示信息
	cenpan_show_prompt.value = false;
	novel_show_lines.value.splice(0); //清空显示的章节
	cenpan_show_loading.value = true; //显示加载图案
	//console.log(novel_loading.value);
	cur_novel_path = path;
	mainpan_nov_path.value = path;
	//更新文件名
	app_title.value = get_file_name(path);
	if (path.endsWith(".novel")) {
		//打开文件进行展示
		novel_lines = await invoke("open_novel", { filename: path });
	} else if (path.endsWith(".txt")) {
		//打开文件进行展示
		novel_lines = await invoke("open_novel_txt", { filename: path });
	} else {
		await dialog.message('不支持该类型文件！', { title: '打开失败', type: 'warning' });
		return;
	}
	let chap_num = 0;
	novel_chapter.splice(0); //清空章节
	novel_chapter.push([])
	for (let i = 0; i < novel_lines.length; i++) {
		if (IsTitle(novel_lines[i])) {
			chap_num++;
			//目录
			mainpan_novel_cata.value.push({
				name: novel_lines[i].trim(),
				index: chap_num
			});
			novel_chapter.push([]);
		}
		novel_chapter[chap_num].push(novel_lines[i]);
	}

	let record = await invoke<Array<number>>("get_nov_prog", {
		path: cur_novel_path
	});
	cur_chap_num = record[0]; //从记录章节开始加载
	//如果小说第一章、第一行不为标题，则添加一个‘开始’作为标题
	let first_chap = novel_chapter[0];
	if (!IsTitle(first_chap[0])) {
		novel_chapter[0].unshift("开始");
		mainpan_novel_cata.value.unshift({
			name: '开始',
			index: 0
		})
	}
	let cur_chapter = novel_chapter[cur_chap_num];
	for (let i = 0; i < cur_chapter.length; i++) {
		novel_show_lines.value.push(cur_chapter[i]);
	}
	mainpan_show_novel_cata.value = Array.from(mainpan_novel_cata.value);
	//关闭加载图标
	cenpan_show_loading.value = false;
	//获取当前小说所有标签
	mainpan_bookmark.value = await invoke('get_bookmark', { path: cur_novel_path });
}
function IsTitle(line: string) {
	//开篇
	const r1 = new RegExp(/^\s*开\s*篇.*\r?\n?$/);
	if (r1.test(line)) {
		return true;
	}
	//序章
	const r2 = new RegExp(/^\s*序\s*章.*\r?\n?$/);
	if (r2.test(line)) return true;
	//第xxx章
	const r3 = new RegExp(/^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,10}\s*[章节幕卷集部回]\s*\r?\n?$/);
	if (r3.test(line)) return true;
	//第xxx章 章节名
	const r4 = new RegExp(/^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,10}\s*[章节幕卷集部回]\s+.*\r?\n?$/);
	if (r4.test(line)) return true;
	//Chapter xxx 章节名
	const r5 = new RegExp(/^Chapter\s*[零一二三四五六七八九十百千万0-9]{1,10}\s+.*\r?\n?$/);
	if (r5.test(line)) return true;
	return false;
}
//获取文件名
function get_file_name(path: string) {
	var idx = path.lastIndexOf('\\');
	idx = idx > -1 ? idx : path.lastIndexOf('/');
	if (idx < 0) {
		return path
	}
	path = path.substring(idx + 1);
	return path.substring(0, path.lastIndexOf('.'));
}

//翻到下一章
async function next_chapter() {
	if (cur_chap_num < novel_chapter.length - 1) {
		cur_chap_num++;
		novel_show_lines.value.splice(0);
		let cur_chap = novel_chapter[cur_chap_num];
		for (let i = 0; i < cur_chap.length; i++) {
			novel_show_lines.value.push(cur_chap[i]);
		}
		await nextTick(); //等待渲染完成
		let p1 = div_view.firstElementChild as Element;
		p1.scrollIntoView();
		//记录翻章
		invoke("set_nov_prog", {
			path: cur_novel_path,
			line: 0,
			chapter: cur_chap_num
		});
	}
}
//翻到上一章
async function prev_chapter() {
	if (cur_chap_num > 0) {
		cur_chap_num--;
		novel_show_lines.value.splice(0);
		let cur_chap = novel_chapter[cur_chap_num];
		for (let i = 0; i < cur_chap.length; i++) {
			novel_show_lines.value.push(cur_chap[i]);
		}
		//记录翻章
		invoke("set_nov_prog", {
			path: cur_novel_path,
			line: 0,
			chapter: cur_chap_num
		});
	}
}

async function fun_jump(cur_chapter: number, cur_line: number) {
	cenpan_show_loading.value = true;
	cur_chap_num = cur_chapter;
	novel_show_lines.value.splice(0);
	let cur_chap = novel_chapter[cur_chap_num];
	for (let i = 0; i < cur_chap.length; i++) {
		novel_show_lines.value.push(cur_chap[i]);
	}
	await nextTick(); //等待渲染完成
	let p1 = div_view.querySelector(`:nth-child(${cur_line + 1})`) as Element;
	p1.scrollIntoView();
	cenpan_show_loading.value = false;
	//每次跳转都要记录一下数据
	invoke("set_nov_prog", {
		path: cur_novel_path,
		line: cur_line,
		chapter: cur_chapter
	});
}
//添加书签函数
async function fun_add_bookmark() {
	if (novel_lines === undefined || novel_lines.length === 0) {
		popmsg.error("请先打开一本小说！");
		is_show_menu.value = false;
		return;
	}
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
	bookmark.chapter = cur_chap_num;
	bookmark.line = cur_p;
	bookmark.datetime = time;
	bookmark.content = novel_chapter[cur_chap_num][cur_p].trim();
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
	mainpan_bookmark.value.push({
		label: bookmark.label,
		datetime: bookmark.datetime,
		id: bookmark.id,
		chapter: bookmark.chapter,
		line: bookmark.line,
		content: bookmark.content
	});
	await invoke('add_bookmark', {
		path: cur_novel_path,
		mark: bookmark
	});
	bookmark.label = "";
	popmsg.success('成功添加书签!');
}
</script>

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

	.paragraph {
		word-break: break-all;
		color: var(--text-c3);
		user-select: text;
		font-family: inherit;
		font-weight: inherit;
		background-repeat: repeat;
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
