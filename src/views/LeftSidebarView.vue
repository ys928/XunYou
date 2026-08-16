<script setup lang="ts">
import { Ref, onMounted, reactive, ref } from 'vue';
import HistoryPanel from '../components/LeftSidebar/HistoryPanel.vue'
import Catalogue from '../components/LeftSidebar/Catalogue.vue';
import Toolbox from '../components/LeftSidebar/Toolbox.vue';
import Bookmark from '../components/LeftSidebar/Bookmark.vue';
import { useCursorStore } from '../store/cursor';
import { ElIcon } from 'element-plus';

const cursor_store = useCursorStore();
const show_panel = ref(false);
const div_divid_line = ref();
const div_content = ref() as Ref<HTMLElement>;
const div_left_panel = ref();

let cur_show_panel = '';

const BarItems = reactive([
    { name: 'history', show: false },
    { name: 'catalogue', show: false },
    { name: 'toolbox', show: false },
    { name: 'bookmark', show: false }
]);

onMounted(() => {
    let posX: number;
    let panelW: number;
    let act_divid = false;
    document.addEventListener("mousedown", e => {
        if (e.target === div_divid_line.value) {
            act_divid = true;
            div_divid_line.value.style.opacity = '1';
            cursor_store.set_change(false);
            posX = e.pageX;
            panelW = div_content.value.offsetWidth;
        }
    });
    document.addEventListener("mouseup", _e => {
        act_divid = false;
        cursor_store.set_change(true);
        if (div_divid_line.value) {
            div_divid_line.value.style.opacity = '0';
        }
    });
    document.addEventListener("mousemove", e => {
        if (act_divid) {
            let diffX = e.pageX - posX;
            if (panelW + diffX < 180) {
                div_content.value.style.width = "180px";
            } else if (panelW + diffX > 420) {
                div_content.value.style.width = "420px";
            } else {
                div_content.value.style.width = (panelW + diffX) + "px";
            }
        }
    });
    div_divid_line.value.addEventListener("mouseenter", () => {
        if (cursor_store.need_change) {
            cursor_store.set_style('ew-resize')
            div_divid_line.value.style.opacity = '1';
        }
    })
    div_divid_line.value.addEventListener("mouseleave", () => {
        if (cursor_store.need_change) {
            div_divid_line.value.style.opacity = '0';
        }
    })
});

function left_panel_mo() {
    if (cursor_store.need_change) {
        cursor_store.set_style('default')
    }
}

function switch_panel(name: string) {
    if (name == cur_show_panel) {
        show_panel.value = !show_panel.value;
        return;
    }
    for (let item of BarItems) {
        item.show = false;
    }
    for (let item of BarItems) {
        if (item.name === name) {
            cur_show_panel = name;
            show_panel.value = true;
            item.show = true;
            break;
        }
    }
}

function fun_is_show(name: string) {
    for (let item of BarItems) {
        if (item.name == name) {
            return item.show;
        }
    }
    return false;
}

function is_active(name: string) {
    return show_panel.value && fun_is_show(name);
}
</script>

<template>
    <div class="LeftSidbarView" ref="div_left_panel" @mouseover="left_panel_mo">
        <div class="Toolbar">
            <div class="top">
                <button class="icon" :class="{ active: is_active('history') }" title="历史记录" @click="switch_panel('history')">
                    <el-icon size="18">
                        <svg viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
                            <path d="M504 255.531c.253 136.64-111.18 248.372-247.82 248.468c-59.015.042-113.223-20.53-155.822-54.911c-11.077-8.94-11.905-25.541-1.839-35.607l11.267-11.267c8.609-8.609 22.353-9.551 31.891-1.984C173.062 425.135 212.781 440 256 440c101.705 0 184-82.311 184-184c0-101.705-82.311-184-184-184c-48.814 0-93.149 18.969-126.068 49.932l50.754 50.754c10.08 10.08 2.941 27.314-11.313 27.314H24c-8.837 0-16-7.163-16-16V38.627c0-14.254 17.234-21.393 27.314-11.314l49.372 49.372C129.209 34.136 189.552 8 256 8c136.81 0 247.747 110.78 248 247.531zm-180.912 78.784l9.823-12.63c8.138-10.463 6.253-25.542-4.21-33.679L288 256.349V152c0-13.255-10.745-24-24-24h-16c-13.255 0-24 10.745-24 24v135.651l65.409 50.874c10.463 8.137 25.541 6.253 33.679-4.21z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
                <button class="icon" :class="{ active: is_active('catalogue') }" title="目录" @click="switch_panel('catalogue')">
                    <el-icon size="18">
                        <svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
                            <path d="M2 3.75A.75.75 0 0 1 2.75 3h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 2 3.75zm0 8a.75.75 0 0 1 .75-.75h6.5a.75.75 0 0 1 0 1.5h-6.5a.75.75 0 0 1-.75-.75zM2.75 7a.75.75 0 0 0 0 1.5h10.5a.75.75 0 0 0 0-1.5H2.75z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
                <button class="icon" :class="{ active: is_active('bookmark') }" title="书签" @click="switch_panel('bookmark')">
                    <el-icon size="18">
                        <svg viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
                            <path d="M10 14a4 4 0 1 1 4-4a4.005 4.005 0 0 1-4 4zm0-6a2 2 0 1 0 1.998 2.004A2.002 2.002 0 0 0 10 8z" fill="currentColor"></path>
                            <path d="M16.644 29.415L2.586 15.354A2 2 0 0 1 2 13.941V4a2 2 0 0 1 2-2h9.941a2 2 0 0 1 1.414.586l14.06 14.058a2 2 0 0 1 0 2.828l-9.943 9.943a2 2 0 0 1-2.829 0zM4 4v9.942L18.058 28L28 18.058L13.942 4z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
            </div>
            <div class="bottom">
                <button class="icon" :class="{ active: is_active('toolbox') }" title="工具箱" @click="switch_panel('toolbox')">
                    <el-icon size="18">
                        <svg viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path d="M6 4.5V6H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h12.003a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2H14V4.5A1.5 1.5 0 0 0 12.5 3h-5A1.5 1.5 0 0 0 6 4.5zM7.5 4h5a.5.5 0 0 1 .5.5V6H7V4.5a.5.5 0 0 1 .5-.5zM4 7h12.003a1 1 0 0 1 1 1v2H14v-.5a.5.5 0 0 0-1 0v.5H7v-.5a.5.5 0 0 0-1 0v.5H3V8a1 1 0 0 1 1-1zm9 4v.5a.5.5 0 0 0 1 0V11h3.003v4a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-4h3v.5a.5.5 0 0 0 1 0V11h6z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
            </div>
        </div>
        <div class="content" v-show="show_panel" ref="div_content">
            <HistoryPanel v-if="fun_is_show('history')"></HistoryPanel>
            <Catalogue v-else-if="fun_is_show('catalogue')"></Catalogue>
            <Bookmark v-else-if="fun_is_show('bookmark')"></Bookmark>
            <Toolbox v-else-if="fun_is_show('toolbox')"></Toolbox>
        </div>
        <div v-show="show_panel" class="divid_line" ref="div_divid_line"></div>
    </div>
</template>

<style scoped lang="scss">
.LeftSidbarView {
    display: flex;
    position: relative;
    z-index: 2;
    border-right: 1px solid var(--border-color);
    background: var(--surface);

    .Toolbar {
        width: var(--toolbar-w);
        padding: 10px 0;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        background: var(--base-bgc);
        border-right: 1px solid var(--border-subtle);

        .top,
        .bottom {
            display: flex;
            flex-direction: column;
            gap: 4px;
            align-items: center;
        }

        .icon {
            width: 34px;
            height: 34px;
            border: none;
            border-radius: var(--radius-sm);
            background: transparent;
            color: var(--text-color1);
            display: inline-flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            transition: background-color 0.18s var(--ease), color 0.18s var(--ease), transform 0.18s var(--ease);

            &:hover {
                background: var(--hover-color);
                color: var(--text-color);
            }

            &.active {
                background: var(--active-color);
                color: var(--accent-strong);
                box-shadow: inset 2px 0 0 var(--accent);
            }
        }
    }

    .content {
        width: 240px;
        height: 100%;
        min-width: 180px;
        animation: panel-enter 0.2s var(--ease);
        box-shadow: var(--shadow-sm);
    }

    .divid_line {
        width: 3px;
        margin-left: -1.5px;
        background: var(--accent);
        opacity: 0;
        transition: opacity 0.15s var(--ease);
        cursor: ew-resize;
    }
}
</style>
