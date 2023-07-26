<template>
<div @mouseenter="app_cursor='default'">
    <div class="top_line"></div>
    <div data-tauri-drag-region class="Titlebar">
        <div class="app_info">
            <img src="/src/assets/app-icon.png" alt="app-icon">
            <n-el data-tauri-drag-region tag="span" style="color: var(--primary-color);">寻幽</n-el>
        </div>
        <n-el data-tauri-drag-region class="app_title" style="color:var(--text-color-3)">
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
    </div>
</div>
</template>

<script setup lang="ts">
import { Ref, inject, nextTick, onMounted, reactive, ref } from "vue";
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
        app_style.value=darkTheme;
        invoke("set_theme",{theme:'dark'});
        message.info("黑夜模式");
        change_theme(dark);
    }else{
        app_style.value=null;
        message.info("白日模式");
        invoke("set_theme",{theme:'white'});
        change_theme(light);
    }
}

onMounted(async ()=>{
    await nextTick();
    let theme=await invoke("get_theme",{});
    if(theme==='dark'){
        change_theme(dark);
        style_switch.value=false;
        app_style.value=darkTheme;
    }else{
        change_theme(light);
        style_switch.value=true;
        app_style.value=null;
    }
});
let dark={
    '--base-bgc':'#202020',
    '--base-bgc1':'#2c2c2c',
    '--base-bgc2':'#222',
    '--txt-color':'#fff',
    '--thumb-color':'#959595',
    '--track-color':'#333',
    '--border-color':'#3e3e3e',
    '--hover-color':'#3f3f3f',
    '--menu-bgc':'#4a4a4a',
    '--menu-color':'#999',
    '--mih-color':'#5f5f5f',
    '--error-color':'#f00'
}
let light={
    '--base-bgc':'#fff',
    '--base-bgc1':'#f4f3ed',
    '--base-bgc2':'#eee',
    '--txt-color':'#3e3e3e',
    '--thumb-color':'#ddd',
    '--track-color':'#eee',
    '--border-color':'#e7e7e7',
    '--hover-color':'#cfcfcf',
    '--menu-bgc':'#b4b3bd',
    '--menu-color':'#2e2e2e',
    '--mih-color':'#aaa',
    '--error-color':'#f00'
}

function change_theme(theme:Object) {
    for (const [key, value] of Object.entries(theme)) {
        document.documentElement.style.setProperty(key, value);
    }
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
.top_line{
    height: 2px;
    background-color: var(--base-bgc);
}
.Titlebar {
    height: 30px;
    line-height: 30px;
    padding: 0 3px;
    display: flex;
    justify-content: space-between;
    background-color: var(--base-bgc);
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
                    background-color: var(--hover-color);
                }
            }
            .close{
                &:hover{
                    background-color: var(--error-color);
                }
            }
        }
    }
}

</style>
