<template>
<div class="MainPanel" :class="mainpan_style">
    <LeftToolPanel></LeftToolPanel>
    <CenterPanel></CenterPanel>
    <RightToolPanel></RightToolPanel>
</div>
</template>

<script setup lang="ts">
import LeftToolPanel from './MainPanel/LeftToolPanel.vue';
import CenterPanel from './MainPanel/CenterPanel.vue';
import RightToolPanel from './MainPanel/RightToolPanel.vue';
import { inject,onMounted,onBeforeMount,onUnmounted,provide,ref, Ref } from 'vue';
import { invoke,event, dialog } from '@tauri-apps/api';
import {NLayout,NLayoutSider,NLayoutContent} from "naive-ui"
/**
 *  定义类型
 */
type app_setting={
    font_size:number, //font-size
    font_weight:number, //font-weight
    line_height:number //line-height
}

//全局主题样式
const mainpan_style=ref() as Ref<string>;

//存放小说目录
const mainpan_novel_cata=ref([]);
provide("mainpan_novel_cata",mainpan_novel_cata);

//存放跳转函数
// const mainpan_nov_jump_fun=ref();
// provide("mainpan_nov_jump_fun",mainpan_nov_jump_fun);

//字体大小
const mainpan_font_size=ref(16);
provide("mainpan_font_size",mainpan_font_size);
//字体粗细
const mainpan_font_weight=ref(400);
provide("mainpan_font_weight",mainpan_font_weight);
//行高
const mainpan_line_height=ref(16);
provide("mainpan_line_height",mainpan_line_height);

/**
 * 初始化操作
 */

 onBeforeMount(async ()=>{
    //取得配置文件中的设置信息
    let setting=await invoke<app_setting>("get_setting",{});
    mainpan_font_size.value=setting.font_size;
    mainpan_font_weight.value=setting.font_weight;
    mainpan_line_height.value=setting.line_height;
 });

</script>

<style scoped lang="less">
.MainPanel.dark{
    background-color: #2c2c2c;
    border: 2px #3f3f3f solid;
}
.MainPanel.white{
    background-color: #f4f3ed;
    border: 2px #e7e7e7 solid;
}
.MainPanel{
    flex-grow: 1;
    height: 100px;
    width: 100%;
    display: flex;
}
</style>
