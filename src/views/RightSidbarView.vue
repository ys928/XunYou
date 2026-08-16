<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import SearchPanel from '../components/RightSidebar/SearchPanel.vue';
import Setting from '../components/RightSidebar/Setting.vue';
import { useCursorStore } from '../store/cursor';
import { ElIcon } from 'element-plus';

const cursor_store = useCursorStore();
const show_panel = ref(false);
const div_divid_line = ref();
const div_content = ref();
const div_right_panel = ref();

let cur_show_panel = '';

const BarItems = reactive([
    { name: 'search', show: false },
    { name: 'setting', show: false },
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
            let diffX = posX - e.pageX;
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
    div_content.value.addEventListener("mouseenter", () => {
        if (cursor_store.need_change) {
            cursor_store.set_style('default')
        }
    })
    div_right_panel.value.addEventListener("mouseenter", () => {
        if (cursor_store.need_change) {
            cursor_store.set_style('default')
        }
    });
});

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
    <div class="RightSidbarView" ref="div_right_panel">
        <div v-show="show_panel" class="divid_line" ref="div_divid_line"></div>
        <div class="content" ref="div_content" v-show="show_panel">
            <SearchPanel v-if="fun_is_show('search')"></SearchPanel>
            <Setting v-else-if="fun_is_show('setting')"></Setting>
        </div>
        <div class="Toolbar">
            <div class="top">
                <button class="icon" :class="{ active: is_active('search') }" title="搜索小说" @click="switch_panel('search')">
                    <el-icon size="18">
                        <svg viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                            <path d="M17 5.5A2.5 2.5 0 0 0 14.5 3h-9A2.5 2.5 0 0 0 3 5.5v3.756c.318-.112.653-.19 1-.229V7h12v7.5a1.5 1.5 0 0 1-1.5 1.5H9.121l1 1H14.5a2.5 2.5 0 0 0 2.5-2.5v-9zM5.5 4h9A1.5 1.5 0 0 1 16 5.5V6H4v-.5A1.5 1.5 0 0 1 5.5 4zm1.096 12.303a3.5 3.5 0 1 1 .707-.707l2.55 2.55a.5.5 0 0 1-.707.708l-2.55-2.55zM7 13.5a2.5 2.5 0 1 0-5 0a2.5 2.5 0 0 0 5 0z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
            </div>
            <div class="bottom">
                <button class="icon" :class="{ active: is_active('setting') }" title="设置" @click="switch_panel('setting')">
                    <el-icon size="18">
                        <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path d="M12.013 2.25c.734.008 1.465.093 2.181.253a.75.75 0 0 1 .582.649l.17 1.527a1.384 1.384 0 0 0 1.928 1.116l1.4-.615a.75.75 0 0 1 .85.174a9.793 9.793 0 0 1 2.204 3.792a.75.75 0 0 1-.271.825l-1.242.916a1.38 1.38 0 0 0 .001 2.226l1.243.915a.75.75 0 0 1 .271.826a9.798 9.798 0 0 1-2.203 3.792a.75.75 0 0 1-.849.175l-1.406-.617a1.38 1.38 0 0 0-1.927 1.114l-.169 1.526a.75.75 0 0 1-.572.647a9.518 9.518 0 0 1-4.405 0a.75.75 0 0 1-.572-.647l-.17-1.524a1.382 1.382 0 0 0-1.924-1.11l-1.407.616a.75.75 0 0 1-.849-.175a9.798 9.798 0 0 1-2.203-3.796a.75.75 0 0 1 .271-.826l1.244-.916a1.38 1.38 0 0 0 0-2.226l-1.243-.914a.75.75 0 0 1-.272-.826a9.793 9.793 0 0 1 2.205-3.792a.75.75 0 0 1 .849-.174l1.4.615a1.387 1.387 0 0 0 1.93-1.118l.17-1.526a.75.75 0 0 1 .583-.65c.718-.159 1.45-.243 2.202-.252zm0 1.5a9.135 9.135 0 0 0-1.355.117l-.109.977A2.886 2.886 0 0 1 6.525 7.17l-.898-.394a8.293 8.293 0 0 0-1.348 2.317l.798.587a2.881 2.881 0 0 1 0 4.643l-.798.588c.32.842.775 1.626 1.347 2.322l.906-.397a2.882 2.882 0 0 1 4.017 2.318l.108.984c.89.15 1.799.15 2.689 0l.108-.984a2.88 2.88 0 0 1 4.02-2.322l.904.396a8.299 8.299 0 0 0 1.347-2.318l-.798-.588a2.88 2.88 0 0 1 0-4.643l.796-.587a8.293 8.293 0 0 0-1.348-2.317l-.896.393a2.884 2.884 0 0 1-4.023-2.324l-.11-.976a8.99 8.99 0 0 0-1.333-.117zM12 8.25a3.75 3.75 0 1 1 0 7.5a3.75 3.75 0 0 1 0-7.5zm0 1.5a2.25 2.25 0 1 0 0 4.5a2.25 2.25 0 0 0 0-4.5z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.RightSidbarView {
    display: flex;
    position: relative;
    z-index: 2;
    border-left: 1px solid var(--border-color);
    background: var(--surface);

    .Toolbar {
        width: var(--toolbar-w);
        padding: 10px 0;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
        background: var(--base-bgc);
        border-left: 1px solid var(--border-subtle);

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
            transition: background-color 0.18s var(--ease), color 0.18s var(--ease);

            &:hover {
                background: var(--hover-color);
                color: var(--text-color);
            }

            &.active {
                background: var(--active-color);
                color: var(--accent-strong);
                box-shadow: inset -2px 0 0 var(--accent);
            }
        }
    }

    .content {
        width: 240px;
        height: 100%;
        min-width: 180px;
        animation: panel-enter-right 0.2s var(--ease);
        box-shadow: var(--shadow-sm);
    }

    .divid_line {
        width: 3px;
        margin-right: -1.5px;
        background: var(--accent);
        opacity: 0;
        transition: opacity 0.15s var(--ease);
        cursor: ew-resize;
    }
}
</style>
