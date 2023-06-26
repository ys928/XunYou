<template>
<div class="Setting" v-show="all_panel.Setting">
    <div class="title">设置</div>
    <div class="set_item">
        <div>
            <n-tag :bordered="false" size="small">字体大小:</n-tag>
            <n-input-number v-model:value="mainpan_font_size" :min="10" :max="25" button-placement="both" size="tiny"></n-input-number>
        </div>
        <div>
            <n-tag :bordered="false" size="small">字体粗细:</n-tag>
            <n-input-number v-model:value="mainpan_font_weight" :min="100" :max="900" :step="100" button-placement="both" size="tiny"></n-input-number>
        </div>
        <div>
            <n-tag :bordered="false" size="small">字体行高:</n-tag>
            <n-input-number v-model:value="mainpan_line_height" :min="10" :max="25" :step="1" button-placement="both" size="tiny"></n-input-number>
        </div>
    </div>
</div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { Ref, inject, onMounted, ref,watch } from 'vue';
import {NInputNumber,NTag,NEl} from "naive-ui"
/**
 * 相关变量类型
 */
 type all_pan_obj={
    'SearchPanel':boolean,
    'Setting':boolean
}
/**
 * vue变量
 */

//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<all_pan_obj>;
//字体大小
const mainpan_font_size=inject('mainpan_font_size') as Ref<number>;
//字体粗细
const mainpan_font_weight=inject("mainpan_font_weight") as Ref<number>
//行高
const mainpan_line_height=inject("mainpan_line_height") as Ref<number>;
/**
 * 函数
 */
watch(mainpan_font_size,()=>{
    save_setting();
})
watch(mainpan_font_weight,()=>{
    save_setting();
})
watch(mainpan_line_height,()=>{
    save_setting();
})
function save_setting(){
    invoke("set_setting",{
        fs:mainpan_font_size.value,
        fw:mainpan_font_weight.value,
        lh:mainpan_line_height.value
    });
}

</script>

<style scoped lang="less">
.Setting{
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--sbase-bgc);
    .title{
        font-size: 16px;
        margin: 5px 0;
        color: #757575;
        height: 25px;
        text-align: center;
    }
    .set_item{
        display: flex;
        flex-direction: column;
        color: #757575;
        padding: 10px;
        .n-tag{
            margin-right: 5px;
        }
        & > div{
            margin: 10px 0;
            display: flex;
            white-space: nowrap;
        }
    }
}
</style>