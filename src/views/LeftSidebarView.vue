<script setup lang="ts">
import Toolbar from '../components/LeftSidebar/Toolbar.vue';
import HistoryPanel from '../components/LeftSidebar/HistoryPanel.vue'
import Catalogue from '../components/LeftSidebar/Catalogue.vue';
import Toolbox from '../components/LeftSidebar/Toolbox.vue';
import Bookmark from '../components/LeftSidebar/Bookmark.vue';
import { Ref, inject, onMounted, provide, reactive, ref, watch } from 'vue';
/**
 * ref变量
 */
const show_divid = ref(false);
/**
 * 绑定的标签变量
 */
const div_divid_line = ref();
const div_content = ref();
const div_left_panel = ref();
/**
 * 取出父组件提供的变量
 */
const app_cursor = inject("app_cursor") as Ref<string>;
const app_is_change_cursor = inject("app_is_change_cursor") as Ref<boolean>;
/**
 * 提供给子组件公用
 */
const all_panel = reactive({
    'HistoryPanel': false,
    'Catalogue': false,
    'Toolbox': false,
    'Tag': false,
});
provide("all_panel", all_panel);

/**
 * 监视vue变量
 */

watch(all_panel, (newval, oldval) => {
    //控制调整大小的分割线是否显示
    let b = true;
    Object.entries(all_panel).map(kv => {
        if (kv[1]) {
            show_divid.value = true;
            b = false;
        }
    });
    if (b) {
        show_divid.value = false;
    }

});

/**
 * 初始化
 */

onMounted(() => {
    let posX: number; //记录当鼠标点击时的x坐标
    let panelW: number; //记录当鼠标点击时，面板的宽度
    let act_divid = false; //是否激活更改面板大小
    document.addEventListener("mousedown", e => {
        if (e.target === div_divid_line.value) {
            act_divid = true; //激活
            div_divid_line.value.style.opacity = '0.7';
            app_is_change_cursor.value = false;
            posX = e.pageX; //初始化坐标
            panelW = div_content.value.offsetWidth; //记录宽度
        }
    });
    document.addEventListener("mouseup", e => {
        act_divid = false; //关闭
        app_is_change_cursor.value = true;
        div_divid_line.value.style.opacity = '0';
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
        if (app_is_change_cursor.value) {
            app_cursor.value = "ew-resize";
            div_divid_line.value.style.opacity = '0.7';
        }
    })
    div_divid_line.value.addEventListener("mouseleave", () => {
        if (app_is_change_cursor.value) {
            div_divid_line.value.style.opacity = '0';
        }
    })
});

function left_panel_mo() {
    if (app_is_change_cursor.value) {
        app_cursor.value = "default";
    }
}

</script>


<template>
    <div class="LeftSidbarView" ref="div_left_panel" @mouseover="left_panel_mo">
        <Toolbar></Toolbar>
        <div class="content" v-show="show_divid" ref="div_content">
            <HistoryPanel v-show="all_panel.HistoryPanel"></HistoryPanel>
            <Catalogue v-show="all_panel.Catalogue"></Catalogue>
            <Toolbox v-show="all_panel.Toolbox"></Toolbox>
            <Bookmark v-show="all_panel.Tag"></Bookmark>
        </div>
        <div v-show="show_divid" class="divid_line" ref="div_divid_line"></div>
    </div>
</template>

<style scoped lang="less">
.LeftSidbarView {
    display: flex;
    border-top: solid 2px var(--border-color);
    border-bottom: solid 2px var(--border-color);

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