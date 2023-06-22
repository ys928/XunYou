<template>
<n-el class="Toolbar">
    <div class="top">
        <n-icon class="icon" size="18" :component="History" title="历史记录" @click="switch_panel('HistoryPanel')" color="#585858"></n-icon>
        <n-icon class="icon" size="23" :component="List16Filled" title="目录"  @click="switch_panel('Catalogue')" color="#585858"></n-icon>
    </div>
    <div class="bottom">
        <n-icon class="icon" size="20" :component="Toolbox20Regular" title="工具箱" @click="switch_panel('Toolbox')" color="#585858"></n-icon>
    </div>
</n-el>
</template>

<script setup lang="ts">
import { Ref, inject } from 'vue';
import {History} from "@vicons/fa"
import {List16Filled,Toolbox20Regular} from "@vicons/fluent"
import { NIcon,NEl } from 'naive-ui';

//相关变量类型
type all_pan_obj={
    'HistoryPanel':boolean,
    'Catalogue':boolean,
    'Toolbox':boolean
}
//控制面板显示与否变量
const all_panel=inject<all_pan_obj>("all_panel");

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
    border-right: var(--border-color) 2px solid;
    background-color:var(--base-color);
    flex-direction: column;
    justify-content: space-between;
    .icon{
        width: 30px;
        height: 30px;
        padding: 5px;
        border-radius: 5px;
        &:hover{
            background-color: var(--info-color-hover);
        }
    }
}

</style>
