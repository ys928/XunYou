<template>
<div class="Icon">
    <img src="/src/assets/app-icon.png" alt="app-icon">
    <span data-tauri-drag-region :class="global_style">寻幽</span>
</div>
</template>

<script setup lang="ts">
import { inject,Ref } from 'vue';

/**
 * vue 变量 绑定标签
 */

/**
 * vue 变量
 */

//全局样式
const global_style=inject("global_style") as Ref<string>;
</script>

<style scoped lang="less">
.Icon{
    display: flex;
    span.dark{
        color: #fffbef;
    }
    span.white{
        color: #202020;
    }
    span {
        text-align: center;
        height: 25px;
        width: 40px;
        font-size: 14px;
    }
    img {
        height: 25px;
        width: 25px;
    }
}

</style>
