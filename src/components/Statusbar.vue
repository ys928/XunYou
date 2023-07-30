<template>
<div @mouseenter="app_cursor='default';" class="StatusBar"  ref="div_statusbar">
    <div class="left"></div>
    <div class="right">
    </div>
</div>

</template>

<script setup lang="ts">
import { Ref, inject, onMounted,ref } from 'vue';

/**
 * 绑定标签的变量
 */
//绑定整个状态栏

/**
 * 取出父组件传下来的变量
 */

//鼠标样式
const app_cursor=inject("app_cursor") as Ref<string>;
//用于显示当前小说阅读进度的变量，本组件用于显示这个值
//const root_novel_prog=inject("root_novel_prog");

onMounted(()=>{

});

</script>

<style scoped lang="less">
.StatusBar.dark{
    background-color: #202020;
}
.StatusBar.white{
    background-color: #fff;
}
.StatusBar{
    background-color: var(--base-bgc);
    height: 30px;
    display: flex;
    justify-content: space-between;
    line-height: 30px;
    padding: 0 20px;
    font-size: 12px;
}
</style>
