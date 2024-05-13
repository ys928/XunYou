<template>
    <div class="Titlebar" @mouseenter="emit('update:cursor', 'default');">
        <div class="top_line"></div>
        <div data-tauri-drag-region class="content">
            <div class="app_info">
                <img src="/src/assets/app-icon.png" alt="app-icon">
                <span data-tauri-drag-region style="color: var(--text-color2);">{{ prop.name }}</span>
            </div>
            <div data-tauri-drag-region class="app_title" style="color:var(--text-color1)">
                {{ novel_store.name }}
            </div>
            <div class="app_opt">
                <n-switch :value="style_switch" @click="switch_sty">
                    <template #checked-icon>
                        <n-icon :component="SunnyOutline" />
                    </template>
                    <template #unchecked-icon>
                        <n-icon :component="Moon" />
                    </template>
                </n-switch>
                <div class="mmc">
                    <n-icon class="min" color="#7f7f7f" size="20" :component="MinusOutlined" @click="WinMin"></n-icon>
                    <n-icon class="max" color="#7f7f7f" size="20" :component="Maximize16Regular"
                        @click="WinTogMax"></n-icon>
                    <n-icon class="close" color="#7f7f7f" size="20" :component="Close" @click="WinClose"></n-icon>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";
import { NSwitch, NIcon, useMessage, darkTheme, } from "naive-ui"
import { Moon, SunnyOutline, Close } from "@vicons/ionicons5"
import { Maximize16Regular } from "@vicons/fluent"
import { MinusOutlined } from "@vicons/antd"
import { useNovelStore } from "../store/novel";


const novel_store = useNovelStore();

/**
 * 绑定相关标签的变量
 */
const prop = defineProps(['name', 'style', 'cursor']);
const emit = defineEmits(['update:style', 'update:cursor', 'pop_msg']);
/**
 * ref变量
 */
//控制开关显示
const style_switch = ref(false);
//全局应用样式
const message = useMessage();

/**
 * 普通函数
 */
function switch_sty() {
    style_switch.value = !style_switch.value;
    if (prop.style === undefined) {
        emit("update:style", darkTheme);
        message.info("黑夜模式");
        invoke("set_theme", { theme: 'dark' });
        change_theme(dark);
    } else {
        emit("update:style", null);
        message.info("白日模式");
        invoke("set_theme", { theme: 'white' });
        change_theme(light);
    }
}

onMounted(async () => {
    await nextTick();
    let theme = await invoke("get_theme", {});
    if (theme === 'dark') {
        change_theme(dark);
        style_switch.value = false;
        emit("update:style", darkTheme);
    } else {
        change_theme(light);
        style_switch.value = true;
        emit("update:style", null);
    }
});
let dark = {
    '--base-bgc': '#202020',
    '--base-bgc1': '#282828',
    '--base-bgc2': '#222',
    '--text-color': '#fff',
    '--text-color1': 'rgba(255, 255, 255, 0.52)',
    '--text-color2': '#63e2b7',
    '--text-c3': '#eaeaea',
    '--thumb-color': '#959595',
    '--track-color': '#333',
    '--border-color': '#3e3e3e',
    '--hover-color': '#3f3f3f',
    '--menu-bgc': '#4a4a4a',
    '--menu-color': '#999',
    '--mih-color': '#5f5f5f',
    '--error-color': '#f00',
    '--selected-color': '#273d59'
}
let light = {
    '--base-bgc': '#fff',
    '--base-bgc1': '#f0f0f2',
    '--base-bgc2': '#eee',
    '--text-color': '#3e3e3e',
    '--text-color1': 'rgb(118, 124, 130)',
    '--text-color2': '#18a058',
    '--text-c3': '#28313b',
    '--thumb-color': '#ddd',
    '--track-color': '#eee',
    '--border-color': '#e7e7e7',
    '--hover-color': '#cfcfcf',
    '--menu-bgc': '#b4b3bd',
    '--menu-color': '#2e2e2e',
    '--mih-color': '#aaa',
    '--error-color': '#f00',
    '--selected-color': '#c6d6f2'

}

function change_theme(theme: Object) {
    for (const [key, value] of Object.entries(theme)) {
        document.documentElement.style.setProperty(key, value);
    }
}

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

</script>

<style scoped lang="less">
.Titlebar {
    .top_line {
        height: 2px;
        background-color: var(--base-bgc);
    }

    .content {
        height: 30px;
        line-height: 30px;
        padding: 0 3px;
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

            .n-switch {
                margin: 3px 25px;
            }

            .mmc {
                height: 30px;
                line-height: 30px;

                .n-icon {
                    margin: 3px 0 0 0;
                    height: 25px;
                    width: 30px;
                    line-height: 30px;
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
}
</style>
