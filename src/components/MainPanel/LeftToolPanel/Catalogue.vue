<template>
    <div class="Catalogue">
        <div class="title">目录</div>
        <div class="top_pos">
            <n-input size="tiny" round @input="search_fun" placeholder="搜目录"></n-input>
        </div>
        <n-scrollbar class="catal">
            <div v-for="item in show_novel_cata" class="cata_item" @dblclick="dclick_cata_item(item.index)">
                {{ item.name }}
            </div>
        </n-scrollbar>
    </div>
</template>

<script setup lang="ts">
import { Ref, ref, watch } from 'vue';
import { NInput, NScrollbar } from "naive-ui"
//目录类型
type type_cata_obj = {
    name: string,
    index: number
};

//存放所有遍历到的小说目录
const novel_cata = ref() as Ref<Array<type_cata_obj>>
//存放跳转函数
const nov_jump_fun = ref() as Ref<Function>;
//配合搜索功能，存放要显示的目录
const show_novel_cata = ref() as Ref<Array<type_cata_obj>>;

watch(novel_cata, () => {
    show_novel_cata.value = Array.from(novel_cata.value);
});

/**
 * 函数
 */
function dclick_cata_item(index: number) {
    nov_jump_fun.value(index, 0);
}

function search_fun(v: string) {
    if (v.length === 0) { //为空，显示所有内容
        show_novel_cata.value = Array.from(novel_cata.value);
    } else {
        show_novel_cata.value.splice(0); //先清空
        show_novel_cata.value = novel_cata.value.filter(e => e.name!.includes(v));
    }
}


</script>

<style scoped lang="less">
.Catalogue {
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

    .top_pos {
        display: flex;
        justify-content: center;
        margin-bottom: 15px;
        padding: 0 20px;
    }

    .catal {
        margin: 0 5px;
        flex-grow: 1;
        border-bottom: none;

        .cata_item {
            display: flex;
            justify-content: space-between;
            white-space: nowrap;
            overflow: hidden;
            font-size: 13px;
            margin: 5px 0px;
            border-bottom: #7f7f7f solid 1px;
            border-radius: 5px;
            height: 25px;
            line-height: 25px;
            padding-left: 5px;
            cursor: pointer;

            &:hover {
                background-color: var(--hover-color);
            }
        }
    }
}
</style>