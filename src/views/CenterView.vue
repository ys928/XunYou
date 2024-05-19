<script setup lang="ts">
import View from '../components/Center/View.vue';
import ShowInfo from '../components/Center/ShowInfo.vue';
import { Ref, ref, onMounted, inject } from 'vue';
import { NSpin } from "naive-ui"
import { useShowStore } from '../store/show';

const show_store = useShowStore();


/**
 * 绑定标签
 */
const div_center_panel = ref();

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

<template>
    <div class="CenterView" ref="div_center_panel">
        <View></View>
        <ShowInfo v-show="show_store.show_prompt"></ShowInfo>
        <n-spin class="loading" size="medium" v-show="show_store.show_loading"></n-spin>
    </div>
</template>

<style scoped lang="less">
.CenterView {
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
