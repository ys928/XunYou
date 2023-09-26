<template>
    <div class="CenterPanel" ref="div_center_panel">
        <View></View>
        <ShowInfo v-show="cenpan_show_prompt"></ShowInfo>
        <n-spin class="loading" size="medium" v-show="cenpan_show_loading"></n-spin>
    </div>
</template>

<script setup lang="ts">
import View from './CenterPanel/View.vue';
import ShowInfo from './CenterPanel/ShowInfo.vue';
import { Ref, provide, ref, onMounted, inject } from 'vue';
import { NSpin } from "naive-ui"
/**
 * 绑定标签
 */
const div_center_panel = ref();
/**
 * 取得父变量
 */

/**
 * 传递个子组件的变量
 */
//用于控制在未打开小说时面板上的提示消息
const cenpan_show_prompt = ref(true);
provide('cenpan_show_prompt', cenpan_show_prompt);
//用于控制是否显示加载图标
const cenpan_show_loading = ref(false);
provide('cenpan_show_loading', cenpan_show_loading);



//鼠标样式
const app_cursor = inject("app_cursor") as Ref<string>;
const app_is_change_cursor = inject("app_is_change_cursor") as Ref<boolean>;
onMounted(async () => {
    div_center_panel.value.addEventListener("mouseenter", () => {
        if (app_is_change_cursor.value) {
            app_cursor.value = "text";
        }
    })
    document.addEventListener("keyup", e => {

    })
});

</script>

<style scoped lang="less">
.CenterPanel {
    position: relative;
    flex-grow: 1;
    height: 100%;
    background-color: var(--base-bgc1);

    .loading {
        position: absolute;
        left: 50%;
        top: 50%;
        transform: translate(-50%, -50%);
    }
}
</style>
