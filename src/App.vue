<script setup lang="ts">
import Titlebar from './components/Titlebar.vue';
import Statusbar from './components/Statusbar.vue';
import { onBeforeMount, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useStyleStore } from './store/style';
import { useCursorStore } from './store/cursor';

type app_style = {
  font_size: number,
  font_weight: number,
  line_height: number,
  font_family: string,
}

const style_store = useStyleStore();
const cursor_store = useCursorStore();

onBeforeMount(async () => {
  let setting = await invoke<app_style>("cfg_app_get_style", {});
  style_store.set(setting.font_size, setting.font_weight, setting.font_family, setting.line_height);
});

onMounted(() => {
  document.oncontextmenu = () => {
    return false;
  }
  document.addEventListener("keydown", e => {
    if ((e.key === 'f' || e.key === 'F') && e.ctrlKey) {
      e.preventDefault();
    }
    if ((e.key === 'p' || e.key === 'P') && e.ctrlKey) {
      e.preventDefault();
    }
    if ((e.key === 'X' || e.key === 'x') && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
    }
  });
});
</script>

<template>
  <div class="MainWindow" :style="{ cursor: cursor_store.style }">
    <Titlebar name="寻幽"></Titlebar>
    <div class="MainPanel">
      <router-view name="LeftSidebar"></router-view>
      <router-view></router-view>
      <router-view name="RightSidebar"></router-view>
    </div>
    <Statusbar></Statusbar>
  </div>
</template>

<style scoped lang="scss">
.MainWindow {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  background: var(--base-bgc);
  overflow: hidden;
  user-select: none;

  .MainPanel {
    flex: 1;
    min-height: 0;
    width: 100%;
    display: flex;
    background: var(--base-bgc1);
  }
}
</style>
