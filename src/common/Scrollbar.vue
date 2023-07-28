<template>
    <div class="COMScrollbar" ref="div_scrollbar">
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { ref,onMounted } from "vue";
//父组件事件
const emit=defineEmits(['onWheel','onScroll'])

//绑定滑动条
const div_scrollbar=ref();

onMounted(()=>{
    div_scrollbar.value.addEventListener('wheel',(e)=>{
        emit('onWheel',e);
    },{ passive: true })
    div_scrollbar.value.addEventListener('scroll',(e)=>{
        emit('onScroll',e);
    })
});

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