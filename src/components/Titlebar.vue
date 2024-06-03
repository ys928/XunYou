<script setup lang="ts">
import { nextTick, onMounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";
import { NIcon } from "naive-ui"
import { Close } from "@vicons/ionicons5"
import { Maximize16Regular } from "@vicons/fluent"
import { MinusOutlined } from "@vicons/antd"
import { useNovelStore } from "../store/novel";
import { useCursorStore } from "../store/cursor";
import { Sunny, Moon } from "@element-plus/icons-vue"
import { ElMessage, ElSwitch } from 'element-plus'

const novel_store = useNovelStore();

const cursor_store = useCursorStore();

const style_mode = ref(false); //默认为亮模式：false

onMounted(async () => {
    await nextTick();
    let theme = await invoke("get_theme", {});
    if (theme == "dark") {
        style_mode.value = true;
    } else {
        style_mode.value = false;
    }
    loadTheme();
});

//处理程序退出时的情况
async function WinClose() {
    appWindow.close();
}
async function WinMin() {
    appWindow.minimize();
}
async function WinTogMax() {
    appWindow.toggleMaximize();
}

function loadTheme() {
    // // 获取之前已经加载的主题样式
    const existingLink = document.querySelector('#theme-style');
    // 如果已存在样式表，移除之前的样式
    if (existingLink) {
        existingLink.parentNode?.removeChild(existingLink);
    }
    // 创建新的样式表
    const link = document.createElement('link');
    link.id = 'theme-style';
    link.rel = 'stylesheet';
    link.type = 'text/css';
    if (style_mode.value == false) {
        localStorage.setItem('style_mode', 'light');
        document.documentElement.classList.remove('dark');
        document.documentElement.classList.add('light');
    } else {
        localStorage.setItem('style_mode', 'dark');
        document.documentElement.classList.remove('light');
        document.documentElement.classList.add('dark');
    }
    // 将新的样式表添加到 head 中
    document.head.appendChild(link);
}

function change_theme(){
    if (style_mode.value == false) {
        ElMessage.success("白日模式");
    } else {
        ElMessage.success("黑夜模式");
    }
    loadTheme();
}

</script>

<template>
    <div class="Titlebar" data-tauri-drag-region @mouseenter="cursor_store.set_style('default')">
        <div class="app_info">
            <img src="/src/assets/app-icon.png" alt="app-icon">
            <span data-tauri-drag-region style="color: var(--text-color2);">寻幽</span>
        </div>
        <div data-tauri-drag-region class="app_title" style="color:var(--text-color1)">
            {{ novel_store.name }}
        </div>
        <div class="app_opt">
            <el-switch v-model="style_mode" @change="change_theme" :active-action-icon="Moon" :inactive-action-icon="Sunny"
                style="--el-switch-on-color: #2C2C2C; --el-switch-off-color: #F2F2F2;">
            </el-switch>
            <div class="mmc">
                <n-icon class="min" color="#7f7f7f" size="20" :component="MinusOutlined" @click="WinMin"></n-icon>
                <n-icon class="max" color="#7f7f7f" size="20" :component="Maximize16Regular"
                    @click="WinTogMax"></n-icon>
                <n-icon class="close" color="#7f7f7f" size="20" :component="Close" @click="WinClose"></n-icon>
            </div>
        </div>
    </div>
</template>

<style scoped lang="less">
.Titlebar {
    height: 30px;
    line-height: 30px;
    padding-left: 3px;
    display: flex;
    justify-content: space-between;
    background-color: var(--base-bgc);

    .app_info {
        display: flex;

        img {
            width: 25px;
            height: 25px;
        }

        span {
            text-align: center;
            height: 25px;
            width: 40px;
            font-size: 14px;
        }
    }

    .app_opt {
        display: flex;

        .el-switch {
            margin: 0 20px;
        }

        :deep(.el-switch) {
            .el-switch__core {
                border: solid 1px #474747;

                .el-switch__action {
                    .el-icon {
                        color: #000;

                    }
                }
            }
        }

        :deep(.el-switch.is-checked) {
            .el-switch__core {
                border: solid 1px #474747;

                .el-switch__action {
                    background-color: #141414;

                    .el-icon {
                        color: #ccc;

                    }
                }
            }
        }

        .mmc {
            height: 30px;
            line-height: 30px;
            display: flex;

            .n-icon {
                height: 30px;
                width: 30px;
                display: flex;
                justify-content: center;
                align-items: center;
            }

            .max,
            .min {
                &:hover {
                    background-color: var(--hover-color);
                }
            }

            .close {
                &:hover {
                    background-color: var(--error-color);
                }
            }
        }
    }
}
</style>
