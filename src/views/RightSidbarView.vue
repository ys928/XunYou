<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { CalendarSearch20Regular, Settings24Regular } from "@vicons/fluent"
import SearchPanel from '../components/RightSidebar/SearchPanel.vue';
import Setting from '../components/RightSidebar/Setting.vue';
import { useCursorStore } from '../store/cursor';
import { ElIcon } from 'element-plus';

const cursor_store = useCursorStore();

const show_panel = ref(false);
/**
 * 绑定的标签变量
 */
const div_divid_line = ref();
const div_content = ref();
const div_right_panel = ref();

let cur_show_panel = '';

const BarItems = reactive([
    {
        name: 'search',
        show: false,
    },
    {
        name: 'setting',
        show: false,
    },
]);

onMounted(() => {
    let posX: number; //记录当鼠标点击时的x坐标
    let panelW: number; //记录当鼠标点击时，面板的宽度
    let act_divid = false; //是否激活更改面板大小
    document.addEventListener("mousedown", e => {
        if (e.target === div_divid_line.value) {
            act_divid = true; //激活
            div_divid_line.value.style.opacity = "#0.7";
            cursor_store.set_change(false);
            posX = e.pageX; //初始化坐标
            panelW = div_content.value.offsetWidth; //记录宽度
        }
    });
    document.addEventListener("mouseup", e => {
        act_divid = false; //关闭
        cursor_store.set_change(true);
        if (div_divid_line.value) {
            div_divid_line.value.style.opacity = '0';
        }
    });
    document.addEventListener("mousemove", e => {
        if (act_divid) { //如果处于激活状态，则更改面板的大小
            let diffX = posX - e.pageX;//实时计算差值
            if (panelW + diffX < 150) {
                div_content.value.style.width = "150px";
            } else if (panelW + diffX > 400) {
                div_content.value.style.width = "400px";
            } else {
                div_content.value.style.width = (panelW + diffX) + "px";
            }
        }
    });
    div_divid_line.value.addEventListener("mouseenter", () => {
        if (cursor_store.need_change) {
            cursor_store.set_style('ew-resize')
            div_divid_line.value.style.opacity = '0.7';
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

</script>

<template>
    <div class="RightSidbarView" ref="div_right_panel">
        <div v-show="show_panel" class="divid_line" ref="div_divid_line"></div>
        <div class="content" ref="div_content" v-show="show_panel">
            <SearchPanel v-if="fun_is_show('search')"></SearchPanel>
            <setting v-else-if="fun_is_show('setting')"></setting>
        </div>
        <div class="Toolbar">
            <div class="top">
                <el-icon class="icon" size="25" color="#585858" title="搜索小说" @click="switch_panel('search')">
                    <CalendarSearch20Regular />
                </el-icon>
            </div>
            <div class="bottom">
                <el-icon class="icon" size="25" color="#585858" title="设置" @click="switch_panel('setting')">
                    <Settings24Regular />
                </el-icon>
            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
.RightSidbarView {
    display: flex;
    border-top: solid 2px var(--border-color);
    border-bottom: solid 2px var(--border-color);

    .Toolbar {
        width: 40px;
        padding: 5px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        border-left: var(--border-color) 2px solid;
        background-color: var(--base-bgc);

        .icon {
            width: 30px;
            height: 30px;
            padding: 3px;
            border-radius: 5px;
            cursor: pointer;

            &:hover {
                background-color: var(--hover-color);
            }
        }
    }

    .content {
        width: 200px;
        height: 100%;
    }

    .divid_line {
        background-color: #86bfeb;
        width: 5px;
    }
}
</style>
