<template>
<div class="Setting" v-show="all_panel.Setting" :class="global_style">
    <div class="title">设置</div>
    <div class="set_item">
        <div class="font_size" :class="global_style">
            字体大小:
            <span class="button" @click="sub_fontsize" :class="global_style">-</span>
            <span class="size" :class="global_style">{{ mainpan_font_size }}</span>
            <span class="button" @click="add_fontsize" :class="global_style">+</span>
        </div>
    </div>
</div>
</template>

<script setup lang="ts">
import { Ref, inject } from 'vue';

/**
 * 相关变量类型
 */
 type all_pan_obj={
    'SearchPanel':boolean,
    'Setting':boolean
}
/**
 * vue变量
 */

//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<all_pan_obj>;
//全局主题样式
const global_style=inject("global_style");
//字体大小
const mainpan_font_size=inject('mainpan_font_size') as Ref<number>;

/**
 * 函数
 */

//字体大小增加
function add_fontsize(){
    if(mainpan_font_size.value>=25) return;
    mainpan_font_size.value+=1;
}
//字体大小减小
function sub_fontsize(){
    if(mainpan_font_size.value<=10) return;
    mainpan_font_size.value-=1;
}
</script>

<style scoped lang="less">
.Setting.white{
    background-color: #fff;
}
.Setting.dark{
    background-color: #202020;
}
.Setting{
    display: flex;
    flex-direction: column;
    height: 100%;
    .title{
        font-size: 16px;
        margin: 5px 0;
        color: #757575;
        height: 25px;
        text-align: center;
    }
    .set_item{
        display: flex;
        flex-direction: column;
        color: #757575;
        .font_size{
            padding-left: 10px;
            .size.dark{
                background-color: #bbb;
                color: #757575;
            }
            .size.white{
                background-color: #aaa;
                color: #eee;
            }
            .size{
                display: inline-block;
                margin: 0 5px;
                border-radius: 5px;
                width: 40px;
                height: 20px;
                line-height: 20px;
                text-align: center;
            }
            .button.dark{
                color: #ccc;
                background-color: #3e3e3e;
                &:hover{
                    background-color: #5e5e5e;
                }
            }
            .button.white{
                color: #ccc;
                background-color: #7f7f7f;
                &:hover{
                    background-color: #555;
                }
            }
            .button{
                border-radius: 5px;
                display: inline-block;
                width: 25px;
                height: 20px;
                line-height: 20px;
                text-align: center;
            }
        }
    }
}
</style>