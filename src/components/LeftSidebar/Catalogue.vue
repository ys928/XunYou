<script setup lang="ts">
import { Ref, onMounted, ref } from 'vue';
import { useNovelStore } from '../../store/novel';
import { CataItem } from '../../api/novel';
import { ElScrollbar, ElInput } from 'element-plus';


const search_input = ref('');

const novel_store = useNovelStore();

//配合搜索功能，存放要显示的目录
const show_novel_cata = ref([]) as Ref<Array<CataItem>>;

onMounted(() => {
    show_novel_cata.value = Array.from(novel_store.cata);
})

function dclick_cata_item(index: number) {
    novel_store.set_show_chapter(index);
}

function search_fun(v: string) {
    if (v.length === 0) { //为空，显示所有内容
        show_novel_cata.value = Array.from(novel_store.cata);
    } else {
        show_novel_cata.value.splice(0); //先清空
        show_novel_cata.value = novel_store.cata.filter(e => e.title!.includes(v));
    }
}


</script>

<template>
    <div class="Catalogue">
        <div class="title">目录</div>
        <div class="top_pos">
            <el-input v-model="search_input" @input="search_fun" placeholder="搜目录" size="small"></el-input>
        </div>
        <div class="catal">
            <el-scrollbar>
                <div v-for="item in show_novel_cata" class="cata_item" @dblclick="dclick_cata_item(item.idx)">
                    {{ item.title }}
                </div>
            </el-scrollbar>
        </div>
    </div>
</template>

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
        height: 100px;
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
            color: var(--text-color1);

            &:hover {
                background-color: var(--hover-color);
            }
        }
    }
}
</style>