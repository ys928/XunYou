<template>
<div class="CenterPanel" ref="div_center_panel" :class="global_style">
    <View></View>
    <ShowInfo></ShowInfo>
    <n-spin class="loading" size="medium" v-show="cenpan_show_loading"></n-spin>
    <Jump></Jump>
</div>
</template>

<script setup lang="ts">
import View from './CenterPanel/View.vue';
import ShowInfo from './CenterPanel/ShowInfo.vue';
import Jump from './CenterPanel/Jump.vue';
import { Ref, provide, ref,onMounted,inject } from 'vue';
import {NSpin} from "naive-ui"
/**
 * 绑定标签
 */
const div_center_panel=ref();
/**
 * 传递个子组件的变量
 */
//用于控制在未打开小说时面板上的提示消息
const cenpan_show_prompt=ref(true);
provide('cenpan_show_prompt',cenpan_show_prompt);
//用于控制全局样式
const global_style=ref() as Ref<string>;
//用于控制是否显示加载图标
const cenpan_show_loading=ref(false);
provide('cenpan_show_loading',cenpan_show_loading);
//用于绑定子组件View中的处理输入跳转输入框的函数，让Jump组件使用
const cenpan_pro_jump_input=ref();
provide('cenpan_pro_jump_input',cenpan_pro_jump_input);
//用于决定是否显示Jump跳转组件
const cenpan_show_jump=ref(false);
provide('cenpan_show_jump',cenpan_show_jump);

//鼠标样式
const app_cursor=inject("app_cursor") as Ref<string>;

onMounted(()=>{
    div_center_panel.value.addEventListener("mouseenter",()=>{
        app_cursor.value="text";
    })
    document.addEventListener("keyup",e=>{
        if((e.key==='g'||e.key==='G')&&e.ctrlKey){
            cenpan_show_jump.value=true;
        }
    })
});

</script>

<style scoped lang="less">
.CenterPanel{
    position: relative;
    flex-grow: 1;
    height: 100%;
    .loading{
        position: absolute;
        left: 50%;
        top: 50%;
        transform: translate(-50%,-50%);
    }
}


</style>
