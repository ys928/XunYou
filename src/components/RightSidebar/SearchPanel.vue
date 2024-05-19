<script setup lang="ts">
import { Ref, ref, onMounted } from 'vue';
import { FileEntry, readDir } from '@tauri-apps/api/fs';
import { dialog, invoke } from '@tauri-apps/api';
import { fs } from '@tauri-apps/api';
import { NIcon, NInput, NSpin, NScrollbar } from "naive-ui"
import { FolderMinus } from "@vicons/tabler"
import { useNovelStore } from '../../store/novel';

const novel_store = useNovelStore();

//控制要显示的小说列表项
let show_novel_list = ref([]) as Ref<Array<FileEntry>>;
//控制加载图标是否显示
const show_loading = ref(false);

//存储所有可搜索的小说项
let all_novel: Array<FileEntry> = [];

async function search_fun(v: string) {
    if (v.length === 0) { //为空，直接展示前一百条
        show_novel_list.value.splice(0); //先清空
        //优化性能，最多展示前100项
        let max = 100 < all_novel.length ? 100 : all_novel.length;
        for (let i = 0; i < max; i++) {
            show_novel_list.value.push(all_novel[i]);
        }

    } else {
        show_novel_list.value.splice(0); //先清空
        let search_result = all_novel.filter(e => e.name!.includes(v));
        for (let i = 0; i < search_result.length; i++) {
            show_novel_list.value.push(search_result[i]);
        }
    }
}

onMounted(async () => {
    let p: string = await invoke("get_novel_folder", {});
    //bug,还需要判断这个文件是不是目录
    let ret = await fs.exists(p);
    if (ret) {
        all_novel.splice(0); //清空
        show_novel_list.value.splice(0);
        all_novel = await readDir(p as string); //重新获取数据
        all_novel = all_novel.filter(e => e.name!.endsWith(".novel") || e.name!.endsWith(".txt"));
        show_loading.value = false;
        //优化性能，最多展示前100项
        let max = 100 < all_novel.length ? 100 : all_novel.length;
        for (let i = 0; i < max; i++) {
            show_novel_list.value.push(all_novel[i]);
        }
    }
});

async function choose_dir() {
    let p = await dialog.open({
        directory: true
    });
    if (p === null) return;
    show_loading.value = true;
    invoke("set_novel_folder", { folder: p });
    //console.log(all_novel);
    all_novel.splice(0); //清空
    show_novel_list.value.splice(0);
    all_novel = await readDir(p as string); //重新获取数据
    all_novel = all_novel.filter(e => e.name!.endsWith(".novel") || e.name!.endsWith(".txt"));
    show_loading.value = false;
    //优化性能，最多展示前100项
    let max = 100 < all_novel.length ? 100 : all_novel.length;
    for (let i = 0; i < max; i++) {
        show_novel_list.value.push(all_novel[i]);
    }
}

function dclick_novel(index: number) {
    novel_store.open(show_novel_list.value[index].path);
}

</script>

<template>
    <div class="SearchPanel">
        <div class="title">搜索栏</div>
        <div class="top_pos">
            <n-input size="tiny" round @input="search_fun" placeholder="搜小说"></n-input>
            <n-icon class="icon" size="20" color="#787878" :component="FolderMinus" @click="choose_dir"></n-icon>
        </div>
        <n-scrollbar class="novel_list">
            <div v-for="(item, index) in show_novel_list" class="novel_item" @dblclick="dclick_novel(index)">
                {{ item.name!.substring(0, item.name!.lastIndexOf('.')) }}
            </div>
            <n-spin class="loading" size="medium" v-show="show_loading"></n-spin>
        </n-scrollbar>
    </div>
</template>

<style scoped lang="less">
.SearchPanel {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 0 10px;
    background-color: var(--base-bgc);

    .title {
        font-size: 16px;
        margin: 5px 0;
        color: #757575;
        height: 25px;
        text-align: center;
    }

    .top_pos {
        display: flex;
        justify-content: center;
        margin-bottom: 15px;
        padding: 0 20px;
    }

    .icon {
        width: 24px;
        height: 20px;
        display: inline;
        line-height: 20px;
        border-radius: 5px;
        margin-left: 3px;
        cursor: pointer;

        &:hover {
            background-color: var(--hover-color);
        }
    }

    .novel_list {
        font-size: 14px;
        flex-grow: 1;
        border-radius: 5px;
        border-bottom: none;
        position: relative;

        .novel_item {
            margin: 10px 10px 10px 5px;
            border-radius: 8px;
            padding: 5px 8px;
            border: var(--border-color) solid 1px;
            overflow-x: hidden;
            white-space: nowrap;
            transition: border 0.3s, color 0.3s;
            cursor: pointer;
            color: var(--text-color1);

            &:hover {
                border: #36ad6a solid 1px;
                color: #36ad6a;
            }
        }

        .loading {
            width: 50px;
            height: 50px;
            position: absolute;
            left: 50%;
            transform: translate(-50%, 0);
        }
    }
}
</style>
