<script setup lang="ts">
import Titlebar from './components/Titlebar.vue';
import Statusbar from './components/Statusbar.vue';
import { onBeforeMount, onMounted, provide, ref, watch } from 'vue';
import { event, invoke, window } from '@tauri-apps/api';
import { darkTheme, NMessageProvider, NConfigProvider, GlobalTheme, NDialogProvider } from 'naive-ui'
import { useStyleStore } from './store/style';

type app_setting = {
  font_size: number, //font-size
  font_weight: number, //font-weight
  line_height: number, //line-height
  font_family: string, //font-family
}

//用于控制当前鼠标样式
const app_cursor = ref("none");
provide("app_cursor", app_cursor);
//全局应用样式
const app_style = ref<GlobalTheme | null>(darkTheme);
provide("app_style", app_style);

const style_store = useStyleStore();


onBeforeMount(async () => {
  //取得配置文件中的设置信息
  let setting = await invoke<app_setting>("get_setting", {});
  style_store.set(setting.font_size, setting.font_weight, setting.font_family, setting.line_height);
});

/**
 * 绑定标签
 */
const div_main_window = ref();

/**
 * 全局变量
 */

//是否修改鼠标样式
const app_is_change_cursor = ref(true);
provide("app_is_change_cursor", app_is_change_cursor);
/**
 * 监视变量变化
 */
watch(app_cursor, (newv, oldv) => {
  div_main_window.value.style.cursor = newv;
});

const lightThemeOverrides = {
  common: {
    baseColor: '#fff',
  }
}

const darkThemeOverrides = {
  common: {
    baseColor: '#202020',
  }
}
//初始化
onMounted(() => {
  document.oncontextmenu = () => {
    return false;
  }
  document.addEventListener("keydown", e => {
    if ((e.key === 'f' || e.key === 'F') && e.ctrlKey) {
      e.preventDefault();
    }
    if ((e.key === 'X' || e.key === 'x') && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
    }
  });

  var t: NodeJS.Timeout; //事件节流，防止频繁滚动导致界面卡顿
  type Size = {
    width: number,
    height: number
  };
  const cur_win = window.getCurrent();
  event.listen<Size>("tauri://resize", (e) => {
    clearTimeout(t);
    t = setTimeout(async function () {
      //跳过最大最小化的情况
      let f = await cur_win.isMaximized();
      if (f) return;
      f = await cur_win.isMinimized();
      if (f) return;

      invoke("set_wh", {
        w: e.payload.width,
        h: e.payload.height
      });
    }, 800)
  });
});


</script>

<template>
  <n-config-provider :theme="app_style"
    :theme-overrides="app_style === null ? lightThemeOverrides : darkThemeOverrides">
    <n-message-provider>
      <n-dialog-provider>
        <div class="MainWindow" ref="div_main_window">
          <Titlebar name="寻幽" v-model:style="app_style" v-model:cursor="app_cursor">
          </Titlebar>
          <div class="MainPanel">
            <router-view name="LeftSidebar"></router-view>
            <router-view></router-view>
            <router-view name="RightSidebar"></router-view>
          </div>
          <Statusbar></Statusbar>
        </div>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>


<style scoped lang="less">
.MainWindow {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;

  .MainPanel {
    background-color: var(--base-bgc1);
    flex-grow: 1;
    height: 100px;
    width: 100%;
    display: flex;
  }
}
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: '楷体';
  user-select: none;
}
</style>