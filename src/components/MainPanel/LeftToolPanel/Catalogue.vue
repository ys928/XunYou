<template>
<div class="Catalogue" v-show="all_panel.Catalogue" :class="global_style">

</div>
</template>

<script setup lang="ts">import { Ref, inject } from 'vue';

//相关变量类型
type type_all_pan_obj={
    'HistoryPanel':boolean,
    'Catalogue':boolean
}
/**
 * 从父组件取出的变量
 */

//全局主题样式
const global_style=inject("global_style");
//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<type_all_pan_obj>;

</script>

<style scoped lang="less">
.Catalogue.dark{
    background-color: #202020;
}
.Catalogue.white{
    background-color: #fff;
}
.Catalogue{
    width: 100%;
    height: 100%;
}
</style>