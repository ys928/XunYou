<template>
    <div class="MainPanel" :class="mainpan_style">
        <LeftToolPanel></LeftToolPanel>
        <CenterPanel></CenterPanel>
        <RightToolPanel></RightToolPanel>
    </div>
</template>

<script setup lang="ts">
import LeftToolPanel from './MainPanel/LeftToolPanel.vue';
import CenterPanel from './MainPanel/CenterPanel.vue';
import RightToolPanel from './MainPanel/RightToolPanel.vue';
import { onBeforeMount, provide, ref, Ref } from 'vue';
import { invoke} from '@tauri-apps/api';
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

//存放小说目录
const mainpan_novel_cata = ref([]);
provide("mainpan_novel_cata", mainpan_novel_cata);
//要进行显示的小说目录（配合搜索功能）
const mainpan_show_novel_cata = ref([]);
provide("mainpan_show_novel_cata", mainpan_show_novel_cata);

//存放跳转函数
const mainpan_nov_jump_fun = ref();
provide("mainpan_nov_jump_fun", mainpan_nov_jump_fun);
//存放当前小说路径
const mainpan_nov_path = ref("");
provide("mainpan_nov_path", mainpan_nov_path);
//字体大小
const mainpan_font_size = ref(16);
provide("mainpan_font_size", mainpan_font_size);
//字体粗细
const mainpan_font_weight = ref(400);
provide("mainpan_font_weight", mainpan_font_weight);
//字体
const mainpan_font_family = ref();
provide("mainpan_font_family", mainpan_font_family);
//行高
const mainpan_line_height = ref(16);
provide("mainpan_line_height", mainpan_line_height);
//存放当前打开的小说所有书签
const mainpan_bookmark = ref([]);
provide('mainpan_bookmark', mainpan_bookmark)

/**
 * 初始化操作
 */

onBeforeMount(async () => {
    //取得配置文件中的设置信息
    let setting = await invoke<app_setting>("get_setting", {});
    mainpan_font_size.value = setting.font_size;
    mainpan_font_weight.value = setting.font_weight;
    mainpan_line_height.value = setting.line_height;
    mainpan_font_family.value = setting.font_family;
});

</script>

<style scoped lang="less">
.MainPanel {
    background-color: var(--base-bgc1);
    flex-grow: 1;
    height: 100px;
    width: 100%;
    display: flex;
}
</style>
