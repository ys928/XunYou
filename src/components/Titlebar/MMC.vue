<template>
<div class="MMC">
    <n-icon class="min" size="20" :component="MinusOutlined" @click="WinMin"></n-icon>
    <n-icon class="max" size="20" :component="Maximize16Regular" @click="WinTogMax"></n-icon>
    <n-icon class="close" size="20" :component="Close" @click="WinClose"></n-icon>
</div>
</template>

<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { inject } from "vue";
import { NIcon } from "naive-ui";
import {Close} from "@vicons/ionicons5"
import {Maximize16Regular} from "@vicons/fluent"
import {MinusOutlined} from "@vicons/antd"


const global_style=inject('global_style');

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
.MMC{
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
            background-color: #3e3e3e;
        }
    }
    .max.white,.min.white{
        &:hover{
            background-color: #eee;
        }
    }
    .close{
        &:hover{
            background-color: #dd0000;
        }
    }
}
</style>
