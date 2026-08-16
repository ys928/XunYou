<script setup lang="ts">
import { nextTick, onMounted, ref } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { invoke } from "@tauri-apps/api/core";
import { useNovelStore } from "../store/novel";
import { useCursorStore } from "../store/cursor";
import { Sunny, Moon } from "@element-plus/icons-vue"
import { ElMessage, ElSwitch, ElIcon } from 'element-plus'

const appWindow = getCurrentWebviewWindow()
const novel_store = useNovelStore();
const cursor_store = useCursorStore();
const style_mode = ref(false);

onMounted(async () => {
    await nextTick();
    let theme = await invoke("cfg_get_app_theme");
    if (theme == "dark") {
        style_mode.value = true;
    } else {
        style_mode.value = false;
    }
    loadTheme();
});

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
    const existingLink = document.querySelector('#theme-style');
    if (existingLink) {
        existingLink.parentNode?.removeChild(existingLink);
    }
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
    document.head.appendChild(link);
}

function change_theme() {
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
            <span data-tauri-drag-region class="brand">寻幽</span>
        </div>
        <div data-tauri-drag-region class="app_title">
            <span v-if="novel_store.name" class="novel-name">{{ novel_store.name }}</span>
            <span v-else class="placeholder">静读一页书</span>
        </div>
        <div class="app_opt">
            <el-switch
                v-model="style_mode"
                @change="change_theme"
                :active-action-icon="Moon"
                :inactive-action-icon="Sunny"
                class="theme-switch"
            />
            <div class="mmc">
                <button class="win-btn min" title="最小化" @click="WinMin">
                    <el-icon size="14">
                        <svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                            <path d="M872 474H152c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h720c4.4 0 8-3.6 8-8v-60c0-4.4-3.6-8-8-8z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
                <button class="win-btn max" title="最大化" @click="WinTogMax">
                    <el-icon size="14">
                        <svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
                            <path d="M4.5 3A1.5 1.5 0 0 0 3 4.5v7A1.5 1.5 0 0 0 4.5 13h7a1.5 1.5 0 0 0 1.5-1.5v-7A1.5 1.5 0 0 0 11.5 3h-7zm0 1h7a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-.5.5h-7a.5.5 0 0 1-.5-.5v-7a.5.5 0 0 1 .5-.5z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
                <button class="win-btn close" title="关闭" @click="WinClose">
                    <el-icon size="14">
                        <svg viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg">
                            <path d="M289.94 256l95-95A24 24 0 0 0 351 127l-95 95l-95-95a24 24 0 0 0-34 34l95 95l-95 95a24 24 0 1 0 34 34l95-95l95 95a24 24 0 0 0 34-34z" fill="currentColor"></path>
                        </svg>
                    </el-icon>
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.Titlebar {
    height: var(--titlebar-h);
    min-height: var(--titlebar-h);
    padding: 0 4px 0 10px;
    display: grid;
    grid-template-columns: 1fr minmax(0, 2fr) 1fr;
    align-items: center;
    background: var(--base-bgc);
    border-bottom: 1px solid var(--border-color);

    .app_info {
        display: flex;
        align-items: center;
        gap: 8px;
        min-width: 0;

        img {
            width: 20px;
            height: 20px;
            border-radius: 4px;
        }

        .brand {
            font-size: 13px;
            font-weight: 500;
            letter-spacing: 0.08em;
            color: var(--text-color);
        }
    }

    .app_title {
        min-width: 0;
        text-align: center;
        font-size: 12px;
        color: var(--text-color1);

        .novel-name {
            display: inline-block;
            max-width: 100%;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            color: var(--text-color);
            font-weight: 500;
        }

        .placeholder {
            opacity: 0.7;
            letter-spacing: 0.08em;
        }
    }

    .app_opt {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 8px;

        .theme-switch {
            --el-switch-on-color: #2a313b;
            --el-switch-off-color: #dfe4ea;
            height: 22px;
        }

        :deep(.el-switch__core) {
            border: 1px solid var(--border-color);
        }

        .mmc {
            display: flex;
            height: var(--titlebar-h);

            .win-btn {
                width: 42px;
                height: 100%;
                border: none;
                background: transparent;
                color: var(--text-color1);
                display: inline-flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: background-color 0.15s var(--ease), color 0.15s var(--ease);

                &:hover {
                    background: var(--hover-color);
                    color: var(--text-color);
                }

                &.close:hover {
                    background: var(--error-color);
                    color: #fff;
                }
            }
        }
    }
}
</style>
