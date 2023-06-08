<template>
<div class="StatusBar" ref="div_statusbar" :class="global_style">
    <div class="left"></div>
    <div class="right">
        <span :class="global_style">{{ root_novel_prog }}</span>
    </div>
</div>
</template>

<script setup lang="ts">
import { Ref, inject, onMounted,ref } from 'vue';


/**
 * 绑定标签的变量
 */
//绑定整个状态栏
 const div_statusbar=ref();

/**
 * 取出父组件传下来的变量
 */

//全局主题样式
const global_style=inject("global_style");
//鼠标样式
const app_cursor=inject("app_cursor") as Ref<string>;
//用于显示当前小说阅读进度的变量，本组件用于显示这个值
const root_novel_prog=inject("root_novel_prog");

onMounted(()=>{
    div_statusbar.value.addEventListener("mouseenter",()=>{
        app_cursor.value="default";
    })
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
    height: 30px;
    display: flex;
    justify-content: space-between;
    line-height: 30px;
    padding: 0 20px;
    font-size: 12px;
    .right{
        span.white{
            color: #222;
        }
        span.dark{
            color: #eee;
        }
    }
}
</style>
