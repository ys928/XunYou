<script setup lang="ts">
import { Ref, ref, onMounted } from 'vue';
import { useNovelStore } from '../../store/novel';
import { FS, NovelInfo } from '../../api/fs';
import { ElScrollbar, ElInput, ElIcon } from 'element-plus';
import { Folder } from '@element-plus/icons-vue'
import * as dialog from "@tauri-apps/plugin-dialog"
import * as fs from "@tauri-apps/plugin-fs"

const novel_store = useNovelStore();
let show_novel_list = ref([]) as Ref<Array<NovelInfo>>;
const show_loading = ref(false);
const search_input = ref('');
let all_novel: Array<NovelInfo> = [];

async function search_fun(v: string) {
    if (v.length === 0) {
        show_novel_list.value.splice(0);
        let max = 100 < all_novel.length ? 100 : all_novel.length;
        for (let i = 0; i < max; i++) {
            show_novel_list.value.push(all_novel[i]);
        }
    } else {
        show_novel_list.value.splice(0);
        let search_result = all_novel.filter(e => e.name!.includes(v));
        for (let i = 0; i < search_result.length; i++) {
            show_novel_list.value.push(search_result[i]);
        }
    }
}

onMounted(async () => {
    let p = await FS.get_root_path();
    let ret = await fs.exists(p);
    if (ret) {
        all_novel.splice(0);
        show_novel_list.value.splice(0);
        all_novel = await FS.read_dir(p as string);
        all_novel = all_novel.filter(e => e.name!.endsWith(".novel") || e.name!.endsWith(".txt"));
        show_loading.value = false;
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

    FS.set_root_path(p as string);

    all_novel.splice(0);
    show_novel_list.value.splice(0);

    all_novel = await FS.read_dir(p as string);
    all_novel = all_novel.filter(e => e.name!.endsWith(".novel") || e.name!.endsWith(".txt"));

    show_loading.value = false;
    let max = 100 < all_novel.length ? 100 : all_novel.length;
    for (let i = 0; i < max; i++) {
        show_novel_list.value.push(all_novel[i]);
    }
}

function dclick_novel(index: number) {
    novel_store.open(show_novel_list.value[index].path);
}

function display_name(name: string) {
    const i = name.lastIndexOf('.');
    return i > 0 ? name.substring(0, i) : name;
}
</script>

<template>
    <div class="SearchPanel panel-shell">
        <div class="panel-title">书库</div>
        <div class="panel-search top_pos">
            <el-input v-model="search_input" @input="search_fun" placeholder="搜索小说" clearable size="small"></el-input>
            <button class="dir-btn" @click="choose_dir" title="选择目录">
                <el-icon size="16"><Folder /></el-icon>
            </button>
        </div>
        <div class="panel-list novel_list" v-loading="show_loading">
            <el-scrollbar>
                <div
                    v-for="(item, index) in show_novel_list"
                    :key="item.path"
                    class="panel-item"
                    :class="{ 'is-active': novel_store.path === item.path }"
                    @dblclick="dclick_novel(index)"
                    :title="display_name(item.name!)"
                >
                    {{ display_name(item.name!) }}
                </div>
                <div v-if="!show_loading && show_novel_list.length === 0" class="panel-empty">
                    还没有可搜索的小说<br />点击文件夹图标选择目录
                </div>
            </el-scrollbar>
        </div>
    </div>
</template>

<style scoped lang="scss">
.SearchPanel {
    .top_pos {
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .dir-btn {
        width: 28px;
        height: 28px;
        flex-shrink: 0;
        border: 1px solid var(--border-color);
        border-radius: var(--radius-sm);
        background: var(--surface-raised);
        color: var(--text-color1);
        display: inline-flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: background-color 0.18s var(--ease), color 0.18s var(--ease), border-color 0.18s var(--ease);

        &:hover {
            background: var(--accent-soft);
            color: var(--accent-strong);
            border-color: color-mix(in srgb, var(--accent) 40%, var(--border-color));
        }
    }
}
</style>
