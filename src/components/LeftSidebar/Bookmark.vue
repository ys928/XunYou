<script setup lang="ts">
import { ref, onMounted, Ref } from 'vue';
import { useNovelStore } from '../../store/novel';
import { ElScrollbar, ElPopover } from 'element-plus';

const novel_store = useNovelStore();
const is_show_menu = ref(false);
const dev_menu = ref();
const div_bookmarks = ref() as Ref<HTMLElement>;
let cur_index = -1;

onMounted(() => {
    div_bookmarks.value.oncontextmenu = (e: MouseEvent) => {
        let index = -1;
        let all_history_item = div_bookmarks.value.querySelectorAll(".mark_item");
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

function dclick_mark(index: number) {
    novel_store.jump_to_bookmark(index);
}

async function del_mark() {
    if (cur_index == -1) return;
    await novel_store.del_bookmark(cur_index);
    is_show_menu.value = false;
}
</script>

<template>
    <div class="Bookmark panel-shell">
        <div class="panel-title">书签</div>
        <div class="panel-list content" ref="div_bookmarks">
            <el-scrollbar>
                <template v-if="novel_store.bookmark.length">
                    <el-popover
                        v-for="(item, index) in novel_store.bookmark"
                        :key="item.id"
                        placement="right"
                        title="备注"
                        :width="200"
                        trigger="hover"
                        :content="item.label || '无备注'"
                    >
                        <template #reference>
                            <div class="mark_item" @dblclick="dclick_mark(index)">
                                <div class="label">{{ item.content }}</div>
                                <div class="bottom">
                                    <span>第 {{ item.chapter + 1 }} 章 · 段 {{ item.line + 1 }}</span>
                                    <span>{{ item.datetime }}</span>
                                </div>
                            </div>
                        </template>
                    </el-popover>
                </template>
                <div v-else class="panel-empty">
                    暂无书签<br />在正文右键可添加
                </div>
            </el-scrollbar>
        </div>
        <div class="ctx-menu" ref="dev_menu" v-show="is_show_menu">
            <div class="item" @click="del_mark">删除书签</div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.Bookmark {
    .mark_item {
        margin: 4px 4px 8px;
        padding: 10px 12px;
        border-radius: var(--radius-md);
        border: 1px solid var(--border-color);
        background: var(--surface-raised);
        cursor: pointer;
        color: var(--text-color1);
        transition: border-color 0.18s var(--ease), background-color 0.18s var(--ease), color 0.18s var(--ease);

        &:hover {
            border-color: color-mix(in srgb, var(--accent) 45%, var(--border-color));
            background: var(--accent-soft);
            color: var(--text-color);
        }

        .label {
            overflow: hidden;
            white-space: nowrap;
            text-overflow: ellipsis;
            font-size: 13px;
            color: var(--text-color);
            margin-bottom: 6px;
        }

        .bottom {
            display: flex;
            justify-content: space-between;
            gap: 8px;
            font-size: 11px;
            opacity: 0.8;
        }
    }
}
</style>
