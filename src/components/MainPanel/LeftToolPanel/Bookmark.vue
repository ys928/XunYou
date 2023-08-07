<template>
    <div class="Bookmark">
        <div class="title">书签</div>
        <Scrollbar class="content">

        </Scrollbar>
    </div>
</template>

<script setup lang="ts">
import { Ref, inject } from 'vue';
import Scrollbar from "../../../common/Scrollbar.vue"
type book_mark={
    id:String, //识别该书签的唯一id
    label:String, //该标签的额外标注信息
    chapter:Number, //所属章节
    line:Number, //所属行
    datetime:String //创建日期
}
//存放当前小说所有书签
const mainpan_bookmark=inject("mainpan_bookmark") as Ref<Array<book_mark>>

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
    .content{
        margin: 0 5px;
        flex-grow: 1;
        border-bottom: none;
    }
}
</style>