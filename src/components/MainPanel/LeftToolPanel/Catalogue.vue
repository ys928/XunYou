<template>
<div class="Catalogue" v-show="all_panel.Catalogue" :class="global_style">
    <div class="title">目录</div>
    <div class="catal" :class="global_style">
        <div v-for="(item,index) in mainpan_novel_cata" class="cata_item" :class="global_style" @dblclick="dclick_cata_item(index)">
            {{item.name}}
        </div>
    </div>
</div>
</template>

<script setup lang="ts">import { Ref, inject } from 'vue';

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

//全局主题样式
const global_style=inject("global_style");
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
    mainpan_nov_jump_fun.value(mainpan_novel_cata.value[index].line);
}
</script>

<style scoped lang="less">
.Catalogue.dark{
    background-color: #202020;
}
.Catalogue.white{
    background-color: #fff;
}
.Catalogue{
    height: 100%;
    display: flex;
    flex-direction: column;
    .title{
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }
    .catal.dark{
        background-color: #2c2c2c;
        border: #3e3e3e solid 2px;
        border-bottom: none;
        &::-webkit-scrollbar-thumb{
            background-color: #959595;
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: #333;
        }
    }

    .catal.white{
        background-color: #f4f3ed;
        border: #e7e7e7 solid 2px;
        border-bottom: none;
        &::-webkit-scrollbar-thumb{
            background-color: #ddd;
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: #eee;
        }
    }
    .catal{
        padding: 0 5px;
        margin: 0 5px;
        overflow-y: auto;
        flex-grow: 1;
        border-radius: 5px;
        color: #7f7f7f;
        overflow-x: hidden;
        &::-webkit-scrollbar{
            width: 5px;
        }
        .cata_item.dark{
            &:hover{
                background-color: #3f3f3f;
            }
        }
        .cata_item.white{
            background-color: #eee;
            &:hover{
                background-color: #cfcfcf;
            }
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
        }
    }
}
</style>