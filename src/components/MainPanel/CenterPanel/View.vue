<template>
	<Scrollbar class="View" id="div_view" @onWheel="process_wheel" @onScroll="process_scroll">
		<div class="one_line" v-for="(item, index) in novel_show_lines">
			<div v-if="IsTitle(item)" class="title">{{ item }}</div>
			<div v-else class="paragraph" :style="{
				'font-size': mainpan_font_size + 'px',
				'font-weight': mainpan_font_weight,
				'line-height': mainpan_line_height / 10 + 'em',
				'background-size': '15px ' + mainpan_line_height / 10 + 'em',
			}">
				{{ item }}
			</div>

		</div>
	</Scrollbar>
</template>

<script setup lang="ts">
import { Ref, ref, onMounted, inject, watch, nextTick } from 'vue';
import { dialog, event, fs, invoke } from '@tauri-apps/api';
import Scrollbar from "../../../common/Scrollbar.vue"
import { useDialog,useMessage } from "naive-ui"
/**
 * 自定义类型
 */
//目录类型
type type_cata_obj = {
	name: string,
	line: number
};


/*
绑定标签
*/
const div_view = ref();
/*
控制内容的变量数据
*/

//程序要进行显示的小说行数内容
const novel_show_lines = ref([]) as Ref<Array<string>>;

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
//存放跳转函数
const mainpan_nov_jump_fun = inject("mainpan_nov_jump_fun") as Ref<Function>;
//字体大小
const mainpan_font_size = inject('mainpan_font_size') as Ref<number>;
//字体粗细
const mainpan_font_weight = inject("mainpan_font_weight") as Ref<number>
//行高
const mainpan_line_height = inject("mainpan_line_height") as Ref<number>;


/*
普通变量
*/
//存放读取到的所有小说内容
let novel_lines: Array<string>;
//存放读取到的小说所有内容，并按章节划分存放，每章第一节为标题。
let novel_chapter: Array<Array<string>> = [];
//当前阅读章节
let cur_chap_num: number;
//存放视图显示出来的第一行占原小说的具体行数
let view_line: number;
//存放当前打开的小说的路径
let cur_novel_path: string;

const ndialog = useDialog();
const nmessage = useMessage();
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
	div_view.value = document.getElementById('div_view');

	document.addEventListener("keydown", async (e) => {
		//ctrl+O：打开小说
		if (e.ctrlKey && e.key === 'o') {
			const selected = await dialog.open({
				multiple: false,
				filters: [{
					name: '小说文件',
					extensions: ['novel']
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
						nmessage.info("当前没有打开小说");
						return;
					}
					novel_show_lines.value.splice(0); //清空界面内容
					app_title.value = ""; //清除当前显示的文件名
					cenpan_show_prompt.value = true; //重新显示提示信息
					if (novel_lines !== undefined) {
						novel_lines.splice(0); //清空读取到的小说内容
					}
					nmessage.info("关闭成功");
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
			//目录
			mainpan_novel_cata.value.push({
				name: novel_lines[i].trim(),
				line: i
			});
			chap_num++;
			novel_chapter.push([]);
		}
		novel_chapter[chap_num].push(novel_lines[i]);
	}

	let record = await invoke<Array<number>>("get_nov_prog", {
		path: cur_novel_path
	});
	cur_chap_num = record[0]; //从记录章节开始加载
	//如果小说第一章、第一行不为标题，则添加一个‘开篇’作为标题
	let first_chap = novel_chapter[0];
	if (!IsTitle(first_chap[0])) {
		novel_chapter[0].unshift("开始");
		mainpan_novel_cata.value.unshift({
			name: '开始',
			line: 0
		})
	}


	let cur_chapter = novel_chapter[cur_chap_num];
	for (let i = 0; i < cur_chapter.length; i++) {
		novel_show_lines.value.push(cur_chapter[i]);
	}
	//关闭加载图标
	cenpan_show_loading.value = false;
}
function IsTitle(line: string) {
	const r1 = new RegExp(/^\s*开\s*篇.*\r?\n?$/);
	if (r1.test(line)) {
		return true;
	}
	const r2 = new RegExp(/^\s*序\s*章.*\r?\n?$/);
	if (r2.test(line)) return true;
	const r3 = new RegExp(/^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,7}\s*[章节幕卷集部回].{0,10}\r?\n?$/);
	if (r3.test(line)) return true;
	const r4 = new RegExp(/^Chapter\s*[零一二三四五六七八九十百千万0-9]{1,7}.{0,10}\r?\n?$/);
	if (r4.test(line)) return true;
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
		let p1 = div_view.value.firstElementChild;
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
	let p1 = div_view.value.querySelector(`:nth-child(${cur_line + 1})`);
	p1.scrollIntoView();
	cenpan_show_loading.value = false;
	//每次跳转都要记录一下数据
	invoke("set_nov_prog", {
		path: cur_novel_path,
		line: cur_line,
		chapter: cur_chapter
	});
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

	.one_line {
		.title {
			font-size: 25px;
			color: #7F7F7F;
			text-align: center;
			margin: 15px 0;
			user-select: text;
		}

		.paragraph {
			word-break: break-all;
			background-image: url("/src/assets/line.svg");
			color: #7F7F7F;
			user-select: text;

			&::selection {
				background: #f5eccf;
				opacity: 0.5;
			}
		}
	}
}
</style>
