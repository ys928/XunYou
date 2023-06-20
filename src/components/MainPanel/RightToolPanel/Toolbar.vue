<template>
<n-el class="Toolbar">
    <div class="top">
        <n-icon class="icon" size="25" color="#585858" :component="CalendarSearch20Regular" title="搜索小说" @click="switch_panel('SearchPanel')"></n-icon>
    </div>
    <div class="bottom">
        <n-icon class="icon" size="25" color="#585858" :component="Settings24Regular" title="设置" @click="switch_panel('Setting')"></n-icon>
    </div>
</n-el>
</template>

<script setup lang="ts">
import { inject,Ref } from 'vue';
import { NIcon,NEl } from 'naive-ui';
import {CalendarSearch20Regular,Settings24Regular} from "@vicons/fluent"

//相关变量类型
type all_pan_obj={
    'SearchPanel':boolean,
    'Setting':boolean
}

//全局主题样式
const global_style=inject("global_style");
//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<all_pan_obj>;

function switch_panel(name:string){
    for(let k in all_panel){
        //console.log(k);
        if(k===name){
            // @ts-ignore
            if(all_panel[k]){
                // @ts-ignore
                all_panel[k]=false;
            }else{
                // @ts-ignore
                all_panel[k]=true;
            }

        }else{
            // @ts-ignore
            all_panel[k]=false;
        }
    }

}

</script>

<style scoped lang="less">
.Toolbar{
    width: 40px;
    padding: 5px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    border-left: var(--border-color) 2px solid;
    background-color:var(--base-color);
    .icon{
        width: 30px;
            height:30px;
            padding: 3px;
            border-radius: 5px;
    }
}

</style>
