<template>
<div class="Toolbox" v-show="all_panel.Toolbox" :class="global_style">
    <div class="title">工具箱</div>
    <div class="tools">
        <n-button @click="fun_txt_to_bzip2">txt转换</n-button>
    </div>
</div>
</template>

<script setup lang="ts">
import { dialog, invoke } from '@tauri-apps/api';
import { Ref, inject } from 'vue';
import { NButton } from 'naive-ui';
//相关变量类型
type type_all_pan_obj={
    'Toolbox':boolean
}
/**
 * 从父组件取出的变量
 */
//全局主题样式
const global_style=inject("global_style");
//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<type_all_pan_obj>;

//函数

async function fun_txt_to_bzip2(){
    const selected=await dialog.open({
        multiple: false,
        filters: [{
        name: '文本文件',
        extensions: ['txt']
        }]
      });
      if(selected===null) return;
      await invoke("txt_to_bzip",{txt:selected});
}

</script>

<style scoped lang="less">
.Toolbox.dark{
    background-color: #202020;
}
.Toolbox.white{
    background-color: #fff;
}
.Toolbox{
    height: 100%;
    display: flex;
    flex-direction: column;
    .title{
        width: 100%;
        text-align: center;
        font-size: 16px;
        margin: 10px 0;
        color: #aaa;
    }
    .tools.dark{
        background-color: #2c2c2c;
        border: #3e3e3e solid 2px;
        border-bottom: none;
        &::-webkit-scrollbar-thumb{
            background-color: #959595;
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: #333;
        }
    }

    .tools.white{
        background-color: #f4f3ed;
        border: #e7e7e7 solid 2px;
        border-bottom: none;
        &::-webkit-scrollbar-thumb{
            background-color: #ddd;
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: #eee;
        }
    }
    .tools{
        padding: 0 5px;
        margin: 0 5px;
        overflow-y: auto;
        flex-grow: 1;
        border-radius: 5px;
        color: #7f7f7f;
        display: flex;
        .item.dark{
            background-color: #2f2f2f;
            &:hover{
                background-color: #3f3f3f;
            }
        }
        .item.white{
            background-color: #eee;
            &:hover{
                background-color: #cfcfcf;
            }
        }
        .item{
            padding: 5px;
            margin: 5px;
            height: 25px;
            text-align: center;
            border-radius: 5px;
        }
    }
}
</style>