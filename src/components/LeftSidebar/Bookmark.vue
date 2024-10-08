<script setup lang="ts">
import { ref, onMounted, Ref } from 'vue';
import { useNovelStore } from '../../store/novel';
import { ElScrollbar, ElPopover } from 'element-plus';
const novel_store = useNovelStore();

//是否显示右键菜单
const is_show_menu = ref(false);
//菜单标签
const dev_menu = ref();
//绑定书签列表标签
const div_bookmarks = ref() as Ref<HTMLElement>;
//保存当前右键点击到的标签项索引
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
        //没有点击到任何一个历史记录项中，则要隐藏菜单
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

//删除一个记录项，
async function del_mark() {
    if (cur_index == -1) return;
    await novel_store.del_bookmark(cur_index);
    is_show_menu.value = false;
}
</script>

<template>
    <div class="Bookmark">
        <div class="title">书签</div>
        <div class="content" ref="div_bookmarks">
            <el-scrollbar>
                <template v-for="(item, index) in novel_store.bookmark">
                    <el-popover placement="right" title="备注" :width="200" trigger="hover" :content="item.label">
                        <template #reference>
                            <div class="mark_item" @dblclick="dclick_mark(index)">
                                <div class="label">
                                    {{ item.content }}
                                </div>
                                <div class="bottom">
                                    <span>{{ item.chapter }}章{{ item.line }}段</span>
                                    <span>{{ item.datetime }}</span>
                                </div>
                            </div>
                        </template>
                    </el-popover>
                </template>
            </el-scrollbar>
        </div>
        <div class="opt_menu" ref="dev_menu" v-show="is_show_menu">
            <div class="item" @click="del_mark">删除</div>
        </div>
    </div>
</template>

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

    .content {
        margin: 0 5px;
        flex-grow: 1;
        height: 100px;
        border-bottom: none;

        .mark_item {
            border-bottom: #7f7f7f solid 1px;
            border-top: #7f7f7f solid 1px;

            border-radius: 5px;
            cursor: pointer;
            overflow: hidden;
            margin: 5px 10px;
            padding: 2px 5px;
            color: var(--text-color1);

            &:hover {
                background-color: var(--hover-color);
            }

            &>div {
                white-space: nowrap;
            }

            .label {
                text-align: left;
            }

            .bottom {
                display: flex;
                justify-content: space-between;
                font-size: 12px;
            }

        }
    }

    .opt_menu {
        position: fixed;
        border-radius: 5px;
        padding: 3px 5px;
        width: 100px;
        background-color: var(--menu-bgc);
        color: var(--menu-color);

        .item {
            padding: 2px 10px;
            border-radius: 5px;
            cursor: pointer;

            &:hover {
                background-color: var(--mih-color);
            }
        }
    }
}
</style>