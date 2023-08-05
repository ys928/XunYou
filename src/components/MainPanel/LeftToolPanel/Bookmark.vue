<template>
    <div class="Bookmark">
        <div class="title">书签</div>
        <div class="content">
        </div>
    </div>
</template>

<script setup lang="ts">
import { Ref, inject } from 'vue';


</script>

<style scoped lang="less">
.Bookmark {
    background-color: var(--base-bgc);
    height: 100%;
    display: flex;
    flex-direction: column;

    .title {
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }
}
</style>