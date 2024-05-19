<script setup lang="ts">
import LeftToolPanel from './MainPanel/LeftToolPanel.vue';
import CenterPanel from './MainPanel/CenterPanel.vue';
import RightToolPanel from './MainPanel/RightToolPanel.vue';
import { onBeforeMount, provide, ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api';


/**
 *  定义类型
 */


//全局主题样式
const mainpan_style = ref() as Ref<string>;



</script>

<template>
    <div class="MainPanel" :class="mainpan_style">
        <router-view class="view left-sidebar" name="LeftSidebar"></router-view>
        <CenterPanel></CenterPanel>
        <router-view class="view right-sidebar" name="RightSidebar"></router-view>
    </div>
</template>

<style scoped lang="less">
.MainPanel {
    background-color: var(--base-bgc1);
    flex-grow: 1;
    height: 100px;
    width: 100%;
    display: flex;
}
</style>
