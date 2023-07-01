<template>
<n-el class="Catalogue" v-show="all_panel.Catalogue">
    <div class="title">目录</div>
    <div class="catal">
        <div v-for="(item,index) in mainpan_novel_cata" class="cata_item"  @dblclick="dclick_cata_item(index)">
            {{item.name}}
        </div>
    </div>
</n-el>
</template>

<script setup lang="ts">
import { Ref, inject, ref } from 'vue';
import {NEl} from "naive-ui"
//相关变量类型
type type_all_pan_obj={
    'HistoryPanel':boolean,
    'Catalogue':boolean
}
//目录类型
type type_cata_obj={
  name:string,
  line:number
};
/**
 * 从父组件取出的变量
 */

//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<type_all_pan_obj>;
//存放所有遍历到的小说目录
const mainpan_novel_cata=inject("mainpan_novel_cata") as Ref<Array<type_cata_obj>>
//存放跳转函数
//const mainpan_nov_jump_fun=inject("mainpan_nov_jump_fun") as Ref<Function>;

/**
 * 函数
 */
function dclick_cata_item(index:number){
    //mainpan_nov_jump_fun.value(index,mainpan_novel_cata.value[index].line);
}
</script>

<style scoped lang="less">
.Catalogue{
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--sbase-bgc);
    .title{
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }
    .catal{
        padding: 0 5px;
        margin: 0 5px;
        overflow-y: auto;
        flex-grow: 1;
        border-radius: 5px;
        color: #7f7f7f;
        background: var(--sbase1-bgc);
        overflow-x: hidden;
        border-bottom: none;
        border: solid 2px var(--sborder-color);
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
        .cata_item{
            display: flex;
            justify-content: space-between;
            white-space: nowrap;
            overflow: hidden;
            font-size: 13px;
            margin: 5px 0px;
            border-bottom: #7f7f7f solid 1px;
            border-radius: 5px;
            height: 25px;
            line-height: 25px;
            padding-left: 5px;
            &:hover{
                background-color: var(--shover-color);
            }
        }
    }
}
</style>