<script setup lang="ts">
// import { dialog, invoke } from '@tauri-apps/api';
// import { ElButton } from 'element-plus';

// async function fun_txt_to_bzip2() {
//     const selected = await dialog.open({
//         multiple: false,
//         filters: [{
//             name: '文本文件',
//             extensions: ['txt']
//         }]
//     });
//     if (selected === null) return;
//     await invoke("txt_to_bzip", { txt: selected });
// }

</script>

<template>
    <div class="Toolbox">
        <div class="title">工具箱</div>
        <div class="tools">
            <!-- <el-button @click="fun_txt_to_bzip2">txt转换</el-button> -->
        </div>
    </div>
</template>

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