<template>
    <div class="HistoryPanel">
        <div class="title">历史记录</div>
        <div class="top_pos">
            <n-input size="tiny" round @input="search_fun" placeholder="搜记录"></n-input>
        </div>
        <Scrollbar class="novels" id="div_history_list">
            <div v-for="(item, index) in show_records_novel" class="novel_item" @dblclick="dclick_novel(index)">
                <span class="novel_name">
                    {{ item.name.substring(0, item.name.lastIndexOf('.')) }}
                </span>
            </div>
        </Scrollbar>
        <div class="opt_menu" ref="dev_menu" v-show="is_show_menu">
            <div class="item" @click="del_record">删除</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { Ref, inject, onMounted, ref } from 'vue';
import { NInput } from "naive-ui";
import Scrollbar from "../../../common/Scrollbar.vue"
//相关变量类型
type type_record_novel = {
    name: string, //小说名字
    path: string, //小说文件路径
    md5: string, //小说md5值校验
    cur_line: number, //当前读到的行数
    all_line: number, //小说总行数
}

/**
 * 从父组件取出的变量
 */

//取出存放打开小说的函数变量，本组件用来使用该函数
const root_fun_open_novel = inject('root_fun_open_novel') as Ref<Function>;
/**
 * 取得父变量
 */


/**
 * vue 绑定标签变量
 */

let div_record: HTMLElement;
const dev_menu = ref();

/**
 * vue变量
 */

//记录所有打开过的小说
const records_novel = ref([]) as Ref<Array<type_record_novel>>;
//实际要显示的内容
const show_records_novel = ref([]) as Ref<Array<type_record_novel>>;
//是否显示右键菜单
const is_show_menu = ref(false);

/**
 * 普通变量
 */
//记录当前右键点击的历史项
let cur_index: number;

/**
 * 双击打开某个记录
 * @param index 这条记录的索引项，将在records_novel变量中取出
 */

//双击代表要打开的小说文件
function dclick_novel(index: number) {
    root_fun_open_novel.value(show_records_novel.value[index].path);
}

/**
 * 初始化记录
 */

onMounted(async () => {
    let record: Array<type_record_novel> = await invoke("get_record", {});
    for (let i of record) {
        records_novel.value.push(i);
    }
    show_records_novel.value = Array.from(records_novel.value);

    div_record = document.getElementById('div_history_list') as HTMLElement;

    div_record.oncontextmenu = (e: MouseEvent) => {
        let index = -1;
        let all_history_item = div_record.querySelectorAll("div");
        for (let i = 0; i < all_history_item.length; i++) {
            if (all_history_item[i].contains(e.target as Node)) {
                dev_menu.value.style.left = e.pageX + "px";
                dev_menu.value.style.top = e.pageY + "px";
                is_show_menu.value = true;
                index = i;
            }
        }
        //没有点击到任何一个历史记录项中，则要隐藏菜单
        if (index === -1) {
            cur_index = -1;
            is_show_menu.value = false;
        } else {
            cur_index = index;
        }
    }
    document.addEventListener("click", e => {
        if (dev_menu.value !== undefined && dev_menu.value !== null && !dev_menu.value.contains(e.target)) {
            is_show_menu.value = false;
        }
    })
});

//删除一个记录项，
async function del_record() {
    if (cur_index == -1) return;
    await invoke("del_record", {
        path: records_novel.value[cur_index].path
    });

    records_novel.value.splice(cur_index, 1);
    is_show_menu.value = false;
    cur_index = -1;
}

function search_fun(v: string) {
    if (v.length === 0) { //为空，显示所有内容
        show_records_novel.value = Array.from(records_novel.value);
    } else {
        show_records_novel.value.splice(0); //先清空
        show_records_novel.value = records_novel.value.filter(e => e.name!.includes(v));
    }
}

</script>

<style scoped lang="less">
.HistoryPanel {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--base-bgc);

    .title {
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }

    .top_pos {
        display: flex;
        justify-content: center;
        margin-bottom: 15px;
        padding: 0 20px;
    }

    .novels {
        margin: 0 5px;
        flex-grow: 1;
        border-bottom: none;

        .novel_item {
            display: flex;
            justify-content: space-between;
            white-space: nowrap;
            font-size: 13px;
            margin: 10px 5px;
            border-left: #7f7f7f solid 2px;
            border-bottom: #7f7f7f solid 1px;
            border-radius: 5px;
            height: 25px;
            line-height: 25px;
            padding-left: 5px;
            transition: background-color 0.2s;
            cursor: pointer;

            &:hover {
                background-color: var(--hover-color);
            }

            .novel_name {
                overflow: hidden;
            }

            .novel_progress {
                color: #4fccb0;
                margin: 0 5px;
                padding: 0 5px;
                border-radius: 5px;
            }
        }
    }

    .opt_menu {
        position: fixed;
        border-radius: 5px;
        padding: 3px 5px;
        width: 100px;
        background-color: var(--menu-bgc);
        color: var(--menu-color);

        .item {
            padding: 2px 10px;
            border-radius: 5px;
            cursor: pointer;

            &:hover {
                background-color: var(--mih-color);
            }
        }
    }
}
</style>
