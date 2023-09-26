<template>
    <div class="Catalogue">
        <div class="title">目录</div>
        <div class="top_pos">
            <n-input size="tiny" round @input="search_fun" placeholder="搜目录"></n-input>
        </div>
        <Scrollbar class="catal">
            <div v-for="item in mainpan_show_novel_cata" class="cata_item" @dblclick="dclick_cata_item(item.index)">
                {{ item.name }}
            </div>
        </Scrollbar>
    </div>
</template>

<script setup lang="ts">
import { Ref, inject, watch } from 'vue';
import { NInput } from "naive-ui"
import Scrollbar from "../../../common/Scrollbar.vue"
//目录类型
type type_cata_obj = {
    name: string,
    index: number
};

/**
 * ref变量
 */
/**
 * 从父组件取出的变量
 */
//存放所有遍历到的小说目录
const mainpan_novel_cata = inject("mainpan_novel_cata") as Ref<Array<type_cata_obj>>
//存放跳转函数
const mainpan_nov_jump_fun = inject("mainpan_nov_jump_fun") as Ref<Function>;
//配合搜索功能，存放要显示的目录
const mainpan_show_novel_cata = inject('mainpan_show_novel_cata') as Ref<Array<type_cata_obj>>;

watch(mainpan_novel_cata, () => {
    mainpan_show_novel_cata.value = Array.from(mainpan_novel_cata.value);
});

/**
 * 函数
 */
function dclick_cata_item(index: number) {
    mainpan_nov_jump_fun.value(index, 0);
}

function search_fun(v: string) {
    if (v.length === 0) { //为空，显示所有内容
        mainpan_show_novel_cata.value = Array.from(mainpan_novel_cata.value);
    } else {
        mainpan_show_novel_cata.value.splice(0); //先清空
        mainpan_show_novel_cata.value = mainpan_novel_cata.value.filter(e => e.name!.includes(v));
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