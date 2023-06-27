<template>
<div @mouseenter="app_cursor='default'">
    <n-el tag="div"  data-tauri-drag-region justify="space-between" class="Titlebar" style="background-color:var(--base-color)">
        <div class="app_info">
            <img src="/src/assets/app-icon.png" alt="app-icon">
            <n-el data-tauri-drag-region tag="span" style="color: var(--primary-color);">寻幽</n-el>
        </div>
        <n-el class="app_title" style="color:var(--text-color-3)">
            {{root_title}}
        </n-el>
        <div class="app_opt">
            <n-switch :value="style_switch" @click="switch_sty">
                <template #checked-icon>
                    <n-icon :component="SunnyOutline"/>
                </template>
                <template #unchecked-icon>
                    <n-icon :component="Moon"/>
                </template>
            </n-switch>
            <div class="mmc">
                <n-icon class="min" color="#7f7f7f" size="20" :component="MinusOutlined" @click="WinMin"></n-icon>
                <n-icon class="max" color="#7f7f7f" size="20" :component="Maximize16Regular" @click="WinTogMax"></n-icon>
                <n-icon class="close" color="#7f7f7f" size="20" :component="Close" @click="WinClose"></n-icon>
            </div>
        </div>
    </n-el>
</div>
</template>

<script setup lang="ts">
import { Ref, inject, onMounted, reactive, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";
import {NSwitch,NIcon,useMessage, GlobalTheme,darkTheme,NImage,NEl} from "naive-ui"
import {Moon,SunnyOutline,Close} from "@vicons/ionicons5"
import {Maximize16Regular} from "@vicons/fluent"
import {MinusOutlined} from "@vicons/antd"
/**
 * 绑定相关标签的变量
 */

/**
 * ref变量
 */
//控制开关显示
const style_switch=ref(false);
/**
 * 取出父组件传递下来的变量
 */

//当前打开的小说名称
const root_title=inject("root_title");
//鼠标样式
const app_cursor=inject("app_cursor") as Ref<string>;
//全局应用样式
const app_style=inject("app_style") as Ref<GlobalTheme | null>;
const message = useMessage();
/**
 * 普通函数
 */
function switch_sty(){
    style_switch.value=!style_switch.value;
    if(app_style.value===null){
        dark_theme();
        app_style.value=darkTheme;
        invoke("set_theme",{theme:'dark'});
        message.info("黑夜模式");
    }else{
        sun_theme();
        app_style.value=null;
        message.info("白日模式");
        invoke("set_theme",{theme:'white'});
    }
}

onMounted(async ()=>{
    let theme=await invoke("get_theme",{});
    if(theme==='dark'){
        dark_theme();
        style_switch.value=false;
        app_style.value=darkTheme;
    }else{
        sun_theme();
        style_switch.value=true;
        app_style.value=null;
    }
});

function dark_theme(){
    document.documentElement.style.setProperty('--sbase-bgc', '#202020');
    document.documentElement.style.setProperty('--sbase1-bgc', '#2c2c2c');
    document.documentElement.style.setProperty('--ssb-thumb-color', '#959595');
    document.documentElement.style.setProperty('--ssb-track-color', '#333');
    document.documentElement.style.setProperty('--sborder-color', '#3e3e3e');
    document.documentElement.style.setProperty('--shover-color', '#3f3f3f');
    document.documentElement.style.setProperty('--smenu-bgc', '#4a4a4a');
    document.documentElement.style.setProperty('--smenu-color', '#999');
    document.documentElement.style.setProperty('--smenu-item-hover-bgc', '#5f5f5f');



}
function sun_theme(){
    document.documentElement.style.setProperty('--sbase-bgc', '#fff');
    document.documentElement.style.setProperty('--sbase1-bgc', '#f4f3ed');
    document.documentElement.style.setProperty('--ssb-thumb-color', '#ddd');
    document.documentElement.style.setProperty('--ssb-track-color', '#eee');
    document.documentElement.style.setProperty('--sborder-color', '#e7e7e7');
    document.documentElement.style.setProperty('--shover-color', '#cfcfcf');
    document.documentElement.style.setProperty('--smenu-bgc', '#b4b3bd');
    document.documentElement.style.setProperty('--smenu-color', '#2e2e2e');
    document.documentElement.style.setProperty('--smenu-item-hover-bgc', '#aaa');
}
//处理程序退出时的情况
async function WinClose() {
    appWindow.close();
}
async function WinMin(){
    appWindow.minimize();
}
async function WinTogMax(){
    appWindow.toggleMaximize();
}

</script>

<style scoped lang="less">
.Titlebar {
    height: 30px;
    line-height: 30px;
    padding: 0 3px;
    display: flex;
    justify-content: space-between;
    .app_info{
        display: flex;
        img{
            width: 25px;
            height: 25px;
        }
        span {
            text-align: center;
            height: 25px;
            width: 40px;
            font-size: 14px;
        }
    }
    .app_opt{
        display: flex;
        .n-switch{
            margin: 3px 25px;
        }
        .mmc{
            height: 30px;
            line-height: 30px;
            .n-icon{
                margin: 3px 0 0 0;
                height: 25px;
                width: 30px;
                line-height: 30px;
            }
            .max,.min{
                &:hover{
                    background-color: var(--info-color-hover);
                }
            }
            .close{
                &:hover{
                    background-color: var(--error-color-hover);
                }
            }
        }
    }
}

</style>
