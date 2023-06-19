<template>
<div data-tauri-drag-region ref="div_titlebar" id="Titlebar" class="Titlebar"  :class="global_style">
    <Icon></Icon>
    <div data-tauri-drag-region class="novel_name">{{root_title}}</div>
    <div class="right">
        <n-switch :value="style_switch" @click="switch_sty">
            <template #checked-icon>
                <n-icon :component="SunnyOutline"/>
            </template>
            <template #unchecked-icon>
                <n-icon :component="Moon"/>
            </template>
        </n-switch>
        <MMC></MMC>
    </div>
</div>
</template>

<script setup lang="ts">
import MMC from "./Titlebar/MMC.vue";
import Icon from "./Titlebar/Icon.vue"
import { Ref, inject, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import {NSwitch,NIcon,useMessage} from "naive-ui"
import {Moon,SunnyOutline} from "@vicons/ionicons5"
/**
 * 绑定相关标签的变量
 */
//开关标签
const div_titlebar=ref();
/**
 * ref变量
 */
//控制开关显示
const style_switch=ref(false);

/**
 * 取出父组件传递下来的变量
 */

//全局样式
const global_style=inject("global_style") as Ref<string>;
//当前打开的小说名称
const root_title=inject("root_title");
//鼠标样式
const app_cursor=inject("app_cursor") as Ref<string>;

const message = useMessage();

/**
 * 普通函数
 */
function switch_sty(){
    style_switch.value=!style_switch.value;
    if(global_style.value==="dark"){
        message.info("已切换为日照模式");
        global_style.value="white";
        invoke("set_theme",{theme:'white'});
    }else{
        global_style.value="dark";
        message.info("切换为黑夜模式");
        invoke("set_theme",{theme:'dark'});
    }
}

onMounted(async ()=>{
    div_titlebar.value.addEventListener("mouseenter",()=>{
        app_cursor.value="default";
    })

    let theme=await invoke("get_theme",{});
    if(theme==='dark'){
        style_switch.value=false;
        global_style.value="dark";
    }else{
        style_switch.value=true;
        global_style.value="white";
    }
});



</script>

<style scoped lang="less">
.Titlebar.dark{
    background-color: #202020;
}
.Titlebar.white{
    background-color: #fff;
}
.Titlebar {
    display: flex;
    justify-content: space-between;
    height: 30px;
    line-height: 30px;
    padding: 0 3px;
    color: #bbbbbb;
    user-select: none;
    
    .novel_name{
        font-size: 14px;
        color: #797979;
    }
    .right{
        display: flex;
        .n-switch{
            margin: 3px 25px;
        }
    }
}

</style>
