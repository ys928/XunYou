<script setup lang="ts">
import { Ref, onMounted, reactive, ref } from 'vue';
import { History } from "@vicons/fa"
import { List16Filled, Toolbox20Regular } from "@vicons/fluent"
import { Tag } from "@vicons/carbon"
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
    {
        name: 'history',
        show: false
    },
    {
        name: 'catalogue',
        show: false
    },
    {
        name: 'toolbox',
        show: false
    }, {
        name: 'bookmark',
        show: false
    }
]);

onMounted(() => {
    let posX: number; //记录当鼠标点击时的x坐标
    let panelW: number; //记录当鼠标点击时，面板的宽度
    let act_divid = false; //是否激活更改面板大小
    document.addEventListener("mousedown", e => {
        if (e.target === div_divid_line.value) {
            act_divid = true; //激活
            div_divid_line.value.style.opacity = '0.7';
            cursor_store.set_change(false);
            posX = e.pageX; //初始化坐标
            panelW = div_content.value.offsetWidth; //记录宽度
        }
    });
    document.addEventListener("mouseup", _e => {
        act_divid = false; //关闭
        cursor_store.set_change(true);
        if (div_divid_line.value) {
            div_divid_line.value.style.opacity = '0';
        }
    });
    document.addEventListener("mousemove", e => {
        if (act_divid) { //如果处于激活状态，则更改面板的大小
            let diffX = e.pageX - posX;//实时计算差值
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

</script>


<template>
    <div class="LeftSidbarView" ref="div_left_panel" @mouseover="left_panel_mo">
        <div class="Toolbar">
            <div class="top">
                <el-icon class="icon" size="18" title="历史记录" @click="switch_panel('history')" color="#585858">
                    <History />
                </el-icon>
                <el-icon class="icon" size="23" title="目录" @click="switch_panel('catalogue')" color="#585858">
                    <List16Filled />
                </el-icon>
                <el-icon class="icon" size="20" title="书签" @click="switch_panel('bookmark')" color="#585858">
                    <Tag />
                </el-icon>
            </div>
            <div class="bottom">
                <el-icon class="icon" size="20" title="工具箱" @click="switch_panel('toolbox')" color="#585858">
                    <Toolbox20Regular />
                </el-icon>
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

<style scoped lang="less">
.LeftSidbarView {
    display: flex;
    border-top: solid 2px var(--border-color);
    border-bottom: solid 2px var(--border-color);

    .Toolbar {
        width: 40px;
        padding: 5px;
        display: flex;
        border-right: var(--border-color) 2px solid;
        background-color: var(--base-bgc);
        flex-direction: column;
        justify-content: space-between;

        .icon {
            width: 30px;
            height: 30px;
            padding: 5px;
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
        width: 5px;
        background-color: #86bfeb;
    }
}
</style>