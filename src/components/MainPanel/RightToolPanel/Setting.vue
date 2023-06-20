<template>
<n-el class="Setting" v-show="all_panel.Setting" style="background-color:var(--base-color)">
    <div class="title">设置</div>
    <div class="set_item">
        <div>
            <n-tag :bordered="false" size="small">字体大小:</n-tag>
            <n-input-number v-model:value="font_size" :min="10" :max="25" button-placement="both" size="tiny"></n-input-number>
        </div>
        <div>
            <n-tag :bordered="false" size="small">字体粗细:</n-tag>
            <n-input-number v-model:value="font_weight" :min="100" :max="900" :step="100" button-placement="both" size="tiny"></n-input-number>
        </div>
        <div>
            <n-tag :bordered="false" size="small">行高:</n-tag>
            <n-input-number v-model:value="line_height" :min="1" :max="2.5" :step="0.1" button-placement="both" size="tiny"></n-input-number>
        </div>
    </div>
</n-el>
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
const font_size=ref(17);
const font_weight=ref(500);
const line_height=ref(1.7);

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
watch(font_size,()=>{
    mainpan_font_size.value = font_size.value;
    save_setting();
});

watch(font_weight,()=>{
    mainpan_font_weight.value = font_weight.value;
    save_setting();
});

watch(line_height,()=>{
    mainpan_line_height.value = Number((line_height.value*10).toFixed(0));
    save_setting();
});
function save_setting(){
    invoke("set_setting",{
        fs:font_size.value,
        fw:font_weight.value,
        lh:Number((line_height.value*10).toFixed(0))
    });
}

onMounted(()=>{
    //初始化
    font_size.value=mainpan_font_size.value;
    font_weight.value=mainpan_font_weight.value;
    line_height.value=Number((mainpan_line_height.value/10).toFixed(1));
});

</script>

<style scoped lang="less">
.Setting.white{
    background-color: #fff;
}
.Setting.dark{
    background-color: #202020;
}
.Setting{
    display: flex;
    flex-direction: column;
    height: 100%;
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