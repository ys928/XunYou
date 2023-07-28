<template>
    <div class="COMScrollbar">
        <slot></slot>
    </div>
</template>

<script setup lang="ts">

</script>

<style scoped lang="less">
.COMScrollbar {
    overflow-y: auto;
    overflow-x: hidden;
    border: solid 2px var(--border-color);
    border-radius: 5px;
    color: #7f7f7f;
    padding: 0 5px;
    &::-webkit-scrollbar {
        width: 5px;
    }

    &::-webkit-scrollbar-thumb {
        background-color: var(--thumb-color);
        border-radius: 3px;
    }

    &::-webkit-scrollbar-track {
        background-color: var(--track-color);
    }
}
</style>