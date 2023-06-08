<template>
<div class="MMC">
    <img @click="appWindow.minimize" class="min" :class="global_style"  src="/src/assets/win_min.svg">
    <img @click="appWindow.toggleMaximize" class="max" :class="global_style" src="/src/assets/win_max.svg">
    <img @click="WinClose" class="close" src="/src/assets/win_close.svg">
</div>
</template>

<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { inject } from "vue";

const global_style=inject('global_style');

//处理程序退出时的情况
async function WinClose() {
    appWindow.close();
}
</script>

<style scoped lang="less">
.MMC{
    height: 30px;
    .max.dark,.min.dark{
        &:hover{
            background-color: #3e3e3e;
        }
    }
    .max.white,.min.white{
        &:hover{
            background-color: #eee;
        }
    }
    img {
        height: 30px;
    }
    .close{
        &:hover{
            background-color: #dd0000;
        }
    }
}
</style>
