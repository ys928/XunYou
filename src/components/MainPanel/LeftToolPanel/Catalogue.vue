<template>
<div class="Catalogue" v-show="all_panel.Catalogue">
    <div class="title">目录</div>
    <Scrollbar class="catal">
        <div v-for="(item,index) in mainpan_novel_cata" class="cata_item"  @dblclick="dclick_cata_item(index)">
            {{item.name}}
        </div>
    </Scrollbar>
</div>
</template>

<script setup lang="ts">
import { Ref, inject, ref } from 'vue';
import Scrollbar from "../../../common/Scrollbar.vue"
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
const mainpan_nov_jump_fun=inject("mainpan_nov_jump_fun") as Ref<Function>;

/**
 * 函数
 */
function dclick_cata_item(index:number){
    mainpan_nov_jump_fun.value(index,0);
}
</script>

<style scoped lang="less">
.Catalogue{
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--base-bgc);
    .title{
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }
    .catal{
        margin: 0 5px;
        flex-grow: 1;
        border-bottom: none;
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
                background-color: var(--hover-color);
            }
        }
    }
}
</style>