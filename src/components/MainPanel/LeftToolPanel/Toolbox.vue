<template>
    <div class="Toolbox">
        <div class="title">工具箱</div>
        <div class="tools">
            <n-button @click="fun_txt_to_bzip2">txt转换</n-button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { dialog, invoke } from '@tauri-apps/api';
import { Ref, inject, ref } from 'vue';
import { NButton } from 'naive-ui';
//相关变量类型

/**
 * 从父组件取出的变量
 */

//函数

async function fun_txt_to_bzip2() {
    const selected = await dialog.open({
        multiple: false,
        filters: [{
            name: '文本文件',
            extensions: ['txt']
        }]
    });
    if (selected === null) return;
    await invoke("txt_to_bzip", { txt: selected });
}

</script>

<style scoped lang="less">
.Toolbox {
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--base-bgc);

    .title {
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }

    .tools {
        padding: 0 5px;
        margin: 0 5px;
        display: flex;
    }
}
</style>