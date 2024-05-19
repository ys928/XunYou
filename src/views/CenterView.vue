<script setup lang="ts">
import View from '../components/Center/View.vue';
import ShowInfo from '../components/Center/ShowInfo.vue';
import { Ref, ref, onMounted, inject } from 'vue';
import { NSpin } from "naive-ui"
import { useShowStore } from '../store/show';
import { useCursorStore } from '../store/cursor';

const show_store = useShowStore();

const cursor_store = useCursorStore();

const div_center_panel = ref();

onMounted(async () => {

    div_center_panel.value.addEventListener("mouseenter", () => {
        if (cursor_store.need_change) {
            cursor_store.set_style('text')
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
