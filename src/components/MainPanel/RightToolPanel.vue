<template>
    <div class="RightToolPanel" ref="div_right_panel">
        <div v-show="show_divid" class="divid_line" ref="div_divid_line"></div>
        <div class="content" ref="div_content" v-show="show_divid">
            <SearchPanel v-show="all_panel.SearchPanel"></SearchPanel>
            <Setting v-show="all_panel.Setting"></Setting>
        </div>
        <Toolbar></Toolbar>
    </div>
</template>

<script setup lang="ts">
import Toolbar from './RightToolPanel/Toolbar.vue';
import SearchPanel from './RightToolPanel/SearchPanel.vue';
import Setting from './RightToolPanel/Setting.vue';
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
const div_right_panel = ref();
/**
 * 取出父组件提供的变量
 */
const app_cursor = inject("app_cursor") as Ref<string>;
const app_is_change_cursor = inject("app_is_change_cursor") as Ref<boolean>;
/**
 * 提供给子组件公用
 */
const all_panel = reactive({
    'SearchPanel': false,
    'Setting': false
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

onMounted(() => {
    let posX: number; //记录当鼠标点击时的x坐标
    let panelW: number; //记录当鼠标点击时，面板的宽度
    let act_divid = false; //是否激活更改面板大小
    document.addEventListener("mousedown", e => {
        if (e.target === div_divid_line.value) {
            act_divid = true; //激活
            div_divid_line.value.style.opacity = "#0.7";
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
    div_content.value.addEventListener("mouseenter", () => {
        if (app_is_change_cursor.value) {
            app_cursor.value = "default";
        }
    })
    div_right_panel.value.addEventListener("mouseenter", () => {
        if (app_is_change_cursor.value) {
            app_cursor.value = "default";
        }
    });
});

</script>

<style scoped lang="less">
.RightToolPanel {
    display: flex;
    border-top: solid 2px var(--border-color);
    border-bottom: solid 2px var(--border-color);

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
