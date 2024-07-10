<script setup lang="ts">
import View from '../components/Center/View.vue';
import ShowInfo from '../components/Center/ShowInfo.vue';
import { ref, onMounted } from 'vue';
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
});


</script>

<template>
    <div class="CenterView" ref="div_center_panel" v-loading="show_store.show_loading">
        <View></View>
        <ShowInfo v-show="show_store.show_prompt"></ShowInfo>
    </div>
</template>

<style scoped lang="less">
.CenterView {
    position: relative;
    flex-grow: 1;
    height: 100%;
    background-color: var(--base-bgc1);
}
</style>
