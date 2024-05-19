<script setup lang="ts">
import { useCursorStore } from '../store/cursor';

const cursor_store = useCursorStore();

</script>

<template>
    <div @mouseenter="cursor_store.set_style('default')" class="StatusBar" ref="div_statusbar">
        <div class="left"></div>
        <div class="right">
        </div>
    </div>

</template>

<style scoped lang="less">
.StatusBar.dark {
    background-color: #202020;
}

.StatusBar.white {
    background-color: #fff;
}

.StatusBar {
    background-color: var(--base-bgc);
    height: 30px;
    display: flex;
    justify-content: space-between;
    line-height: 30px;
    padding: 0 20px;
    font-size: 12px;
}
</style>
