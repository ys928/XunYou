<template>
<n-el class="HistoryPanel" v-show="all_panel.HistoryPanel">
    <div class="title">历史记录</div>
    <div class="novels" ref="div_record">
        <div v-for="(item,index) in records_novel" class="novel_item" @dblclick="dclick_novel(index)">
            <span @dblclick="dclick_novel(index)"  class="novel_name">
                {{item.name.substring(0,item.name.lastIndexOf('.'))}}
            </span> 
            <span @dblclick="dclick_novel(index)" class="novel_progress">
                进度:{{ (item.cur_line*100 / item.all_line).toFixed(2) }}%
            </span>
        </div>
    </div>
    <div class="opt_menu" ref="dev_menu" v-show="is_show_menu" >
        <div class="item" @click="del_record">删除</div>
    </div>
</n-el>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { Ref, inject, onMounted, ref, watch } from 'vue';
import {GlobalTheme, NEl} from "naive-ui"
//相关变量类型
type type_all_pan_obj={
    'HistoryPanel':boolean
}
type type_record_novel={
    name:string, //小说名字
    path:string, //小说文件路径
    md5:string, //小说md5值校验
    cur_line:number, //当前读到的行数
    all_line:number, //小说总行数
}

/**
 * 从父组件取出的变量
 */

//取出存放打开小说的函数变量，本组件用来使用该函数
const root_fun_open_novel=inject('root_fun_open_novel') as Ref<Function>;
//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<type_all_pan_obj>;
/**
 * 取得父变量
 */


/**
 * vue 绑定标签变量
 */

const div_record=ref();
const dev_menu=ref();

/**
 * vue变量
 */

//记录所有打开过的小说
const records_novel=ref([]) as Ref<Array<type_record_novel>>;
//是否显示右键菜单
const is_show_menu=ref(false);

/**
 * 普通变量
 */
//记录当前右键点击的历史项
let cur_index:number;

/**
 * 双击打开某个记录
 * @param index 这条记录的索引项，将在records_novel变量中取出
 */

//双击代表要打开的小说文件
function dclick_novel(index:number){
    root_fun_open_novel.value(records_novel.value[index].path);
}

/**
 * 初始化记录
 */

 onMounted(async ()=>{
    let record:Array<type_record_novel>=await invoke("get_record",{});
    for(let i of record){
        records_novel.value.push(i);
    }

    div_record.value.oncontextmenu=(e:MouseEvent)=>{
        let index=-1;
        let all_history_item=div_record.value.querySelectorAll("div");
        for(let i =0;i<all_history_item.length;i++){
            if(all_history_item[i].contains(e.target)){
                dev_menu.value.style.left=e.pageX+"px";
                dev_menu.value.style.top=e.pageY+"px";
                is_show_menu.value=true;
                index=i;
            }
        }
        //没有点击到任何一个历史记录项中，则要隐藏菜单
        if(index===-1){
            cur_index===-1;
            is_show_menu.value=false;
        }else{
            cur_index=index;
        }
    }
    document.addEventListener("click",e=>{
        if(dev_menu.value!==undefined&&dev_menu.value!==null&& !dev_menu.value.contains(e.target)){
            is_show_menu.value=false;
        }
    })
});

//删除一个记录项，
async function del_record(){
    await invoke("del_record",{
        path:records_novel.value[cur_index].path
    });

    records_novel.value.splice(cur_index,1);
    is_show_menu.value=false;
}

</script>

<style scoped lang="less">
.HistoryPanel{
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color:var(--sbase-bgc);
    .title{
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }
    .novels{
        padding: 0 5px;
        margin: 0 5px;
        overflow-y: auto;
        flex-grow: 1;
        border-radius: 5px;
        color: #7f7f7f;
        border-bottom: none;
        background-color: var(--sbase1-bgc);
        border: var(--sborder-color) solid 2px;
        &::-webkit-scrollbar{
            width: 5px;
        }
        &::-webkit-scrollbar-thumb{
            background-color: var(--ssb-thumb-color);
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: var(--ssb-track-color);
        }
        .novel_item{
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
            &:hover{
                background-color: var(--shover-color);
            }
            .novel_name{
                overflow: hidden;
            }
            .novel_progress{
                color: #4fccb0;
                margin: 0 5px;
                padding: 0 5px;
                border-radius: 5px;
            }
        }
    }
    .opt_menu{
        position: fixed;
        border-radius: 5px;
        padding: 3px 5px;
        width: 100px;
        background-color: var(--smenu-bgc);
        color: var(--smenu-color);
        .item{
            padding: 2px 10px;
            border-radius: 5px;
            &:hover{
                background-color: var(--smenu-item-hover-bgc);
            }
        }
    }
}
</style>
