<script setup lang="ts">
import { Ref, onMounted, ref, watch } from 'vue';
import { useNovelStore } from '../../store/novel';
import { CataItem } from '../../api/novel';
import { ElScrollbar, ElInput } from 'element-plus';

const search_input = ref('');
const novel_store = useNovelStore();
const show_novel_cata = ref([]) as Ref<Array<CataItem>>;

onMounted(() => {
    show_novel_cata.value = Array.from(novel_store.cata);
})

watch(() => novel_store.cata, (v) => {
    if (!search_input.value) {
        show_novel_cata.value = Array.from(v);
    }
}, { deep: true });

function dclick_cata_item(index: number) {
    novel_store.set_show_chapter(index);
}

function search_fun(v: string) {
    if (v.length === 0) {
        show_novel_cata.value = Array.from(novel_store.cata);
    } else {
        show_novel_cata.value = novel_store.cata.filter(e => e.title!.includes(v));
    }
}
</script>

<template>
    <div class="Catalogue panel-shell">
        <div class="panel-title">目录</div>
        <div class="panel-search">
            <el-input v-model="search_input" @input="search_fun" placeholder="搜索章节" clearable size="small"></el-input>
        </div>
        <div class="panel-list">
            <el-scrollbar>
                <div
                    v-for="item in show_novel_cata"
                    :key="item.idx"
                    class="panel-item"
                    :class="{ 'is-active': novel_store.cur_ch_idx === item.idx }"
                    @dblclick="dclick_cata_item(item.idx)"
                    :title="item.title"
                >
                    {{ item.title }}
                </div>
                <div v-if="show_novel_cata.length === 0" class="panel-empty">
                    {{ novel_store.isopen ? '没有匹配的章节' : '打开小说后显示目录' }}
                </div>
            </el-scrollbar>
        </div>
    </div>
</template>
