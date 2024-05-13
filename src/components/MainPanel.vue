<script setup lang="ts">
import LeftToolPanel from './MainPanel/LeftToolPanel.vue';
import CenterPanel from './MainPanel/CenterPanel.vue';
import RightToolPanel from './MainPanel/RightToolPanel.vue';
import { onBeforeMount, provide, ref, Ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { useStyleStore } from '../store/style';

const style_store = useStyleStore();

/**
 *  定义类型
 */
type app_setting = {
    font_size: number, //font-size
    font_weight: number, //font-weight
    line_height: number, //line-height
    font_family: string, //font-family
}

//全局主题样式
const mainpan_style = ref() as Ref<string>;

onBeforeMount(async () => {
    //取得配置文件中的设置信息
    let setting = await invoke<app_setting>("get_setting", {});
    style_store.set(setting.font_size, setting.font_weight, setting.font_family, setting.line_height);
});

</script>

<template>
    <div class="MainPanel" :class="mainpan_style">
        <LeftToolPanel></LeftToolPanel>
        <CenterPanel></CenterPanel>
        <RightToolPanel></RightToolPanel>
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
