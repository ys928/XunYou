<template>
<div class="Setting" v-show="all_panel.Setting" :class="global_style">
    <div class="title">设置</div>
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
}
</style>