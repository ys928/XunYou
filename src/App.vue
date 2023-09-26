<template>
<n-config-provider :theme="app_style" :theme-overrides="app_style===null?lightThemeOverrides : darkThemeOverrides">
  <n-message-provider>
    <n-dialog-provider>
    <div class="MainWindow" ref="div_main_window">
      <Titlebar 
        name="寻幽" 
        :title="app_title" 
        v-model:style="app_style" 
        v-model:cursor="app_cursor"
        >
      </Titlebar>
      <MainPanel></MainPanel>
      <Statusbar></Statusbar>
    </div>
  </n-dialog-provider>
  </n-message-provider>
</n-config-provider>
</template>

<script setup lang="ts">
import Titlebar from './components/Titlebar.vue';
import MainPanel from './components/MainPanel.vue';
import Statusbar from './components/Statusbar.vue';
import { onMounted, provide, ref,watch } from 'vue';
import { event, invoke,window } from '@tauri-apps/api';
import { darkTheme,NMessageProvider,NConfigProvider, GlobalTheme,NDialogProvider} from 'naive-ui'
//用与打开文件显示在页面的函数,在view组件中为其赋值，然后给其它组件调用
const root_fun_open_novel=ref();
provide("root_fun_open_novel",root_fun_open_novel);
//用于显示当前打开的小说名称
const app_title=ref();
provide("app_title",app_title);
//用于控制当前鼠标样式
const app_cursor=ref("none");
provide("app_cursor",app_cursor);
//全局应用样式
const app_style=ref<GlobalTheme | null>(darkTheme);
provide("app_style",app_style);


/**
 * 绑定标签
 */
const div_main_window=ref();

/**
 * 全局变量
 */

//是否修改鼠标样式
const app_is_change_cursor=ref(true);
provide("app_is_change_cursor",app_is_change_cursor);
/**
 * 监视变量变化
 */
watch(app_cursor,(newv,oldv)=>{
  div_main_window.value.style.cursor=newv;
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
onMounted(()=>{
  document.oncontextmenu=()=>{
    return false;
  }
  document.addEventListener("keydown",e=>{
    if((e.key==='f'||e.key==='F')&&e.ctrlKey){
      e.preventDefault();
    }
    if((e.key==='X'||e.key==='x')&&e.ctrlKey&&e.shiftKey){
      e.preventDefault();
    }
  });

  var t:NodeJS.Timeout; //事件节流，防止频繁滚动导致界面卡顿
  type Size={
    width:number,
    height:number
  };
  const cur_win=window.getCurrent();
  event.listen<Size>("tauri://resize", (e)=>{
    clearTimeout(t);
    t = setTimeout(async function() {
      //跳过最大最小化的情况
      let f=await cur_win.isMaximized();
      if(f) return;
      f=await cur_win.isMinimized();
      if(f) return;

      invoke("set_wh",{
        w:e.payload.width,
        h:e.payload.height
      });
    }, 800)
  });
});


</script>

<style scoped lang="less">
.MainWindow{
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}
</style>