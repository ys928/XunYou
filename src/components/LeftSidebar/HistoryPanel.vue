<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, reactive, Ref, ref } from 'vue';
import { useNovelStore } from '../../store/novel';
import { ElScrollbar, ElInput } from 'element-plus';

const novel_store = useNovelStore();

type type_record_novel = {
    name: string,
    path: string,
}

const ref_div_history_list = ref() as Ref<HTMLElement>;
const dev_menu = ref();
const search_input = ref('');
const records_novel = reactive([]) as Array<type_record_novel>;
const show_records_novel = reactive([]) as Array<type_record_novel>;
const is_show_menu = ref(false);
let cur_index: number;

function dclick_novel(index: number) {
    novel_store.open(show_records_novel[index].path);
}

onMounted(async () => {
    let record: Array<type_record_novel> = await invoke("cfg_nov_get_records", {});
    for (let i of record) {
        records_novel.push(i);
    }
    show_records_novel.push(...records_novel);

    ref_div_history_list.value.oncontextmenu = (e: MouseEvent) => {
        let index = -1;
        let all_history_item = ref_div_history_list.value.querySelectorAll(".panel-item");

        for (let i = 0; i < all_history_item.length; i++) {
            if (all_history_item[i].contains(e.target as Node)) {
                dev_menu.value.style.left = e.pageX + "px";
                dev_menu.value.style.top = e.pageY + "px";
                is_show_menu.value = true;
                index = i;
            }
        }
        if (index === -1) {
            cur_index = -1;
            is_show_menu.value = false;
        } else {
            cur_index = index;
        }
    }
    document.addEventListener("click", e => {
        if (dev_menu.value !== undefined && dev_menu.value !== null && !dev_menu.value.contains(e.target)) {
            is_show_menu.value = false;
        }
    })
});

async function del_record() {
    if (cur_index == -1) return;

    await invoke("cfg_nov_del_record", {
        name: records_novel[cur_index].name
    });

    records_novel.splice(cur_index, 1);
    show_records_novel.splice(cur_index, 1);
    is_show_menu.value = false;
    cur_index = -1;
}

function search_fun(v: string) {
    if (v.length === 0) {
        show_records_novel.length = 0;
        show_records_novel.push(...records_novel);
    } else {
        show_records_novel.length = 0;
        show_records_novel.push(...records_novel.filter(e => e.name!.includes(v)));
    }
}
</script>

<template>
    <div class="HistoryPanel panel-shell">
        <div class="panel-title">历史记录</div>
        <div class="panel-search">
            <el-input v-model="search_input" @input="search_fun" placeholder="搜索记录" clearable size="small"></el-input>
        </div>
        <div class="panel-list">
            <el-scrollbar>
                <div ref="ref_div_history_list">
                    <div
                        v-for="(item, index) in show_records_novel"
                        :key="item.path + index"
                        class="panel-item"
                        :class="{ 'is-active': novel_store.path === item.path }"
                        @dblclick="dclick_novel(index)"
                        :title="item.name"
                    >
                        {{ item.name }}
                    </div>
                    <div v-if="show_records_novel.length === 0" class="panel-empty">
                        暂无阅读记录<br />打开一本小说开始寻幽
                    </div>
                </div>
            </el-scrollbar>
        </div>
        <div class="ctx-menu" ref="dev_menu" v-show="is_show_menu">
            <div class="item" @click="del_record">删除记录</div>
        </div>
    </div>
</template>
