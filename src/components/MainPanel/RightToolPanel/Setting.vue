<template>
    <div class="Setting">
        <div class="title">设置</div>
        <div class="set_item">
            <div>
                <n-tag :bordered="false" size="small">字体大小:</n-tag>
                <n-input-number v-model:value="font_size" :min="10" :max="25" button-placement="both"
                    size="tiny"></n-input-number>
            </div>
            <div>
                <n-tag :bordered="false" size="small">字体粗细:</n-tag>
                <n-input-number v-model:value="font_weight" :min="100" :max="900" :step="100" button-placement="both"
                    size="tiny"></n-input-number>
            </div>
            <div>
                <n-tag :bordered="false" size="small">字体行高:</n-tag>
                <n-input-number v-model:value="line_height" :min="10" :max="25" :step="1" button-placement="both"
                    size="tiny"></n-input-number>
            </div>
            <div>
                <n-tag :bordered="false" size="small">正文字体:</n-tag>
                <n-select v-model:value="font_family" size="tiny" :options="fonts" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, ref, watch } from 'vue';
import { NInputNumber, NTag, NSelect, useMessage } from "naive-ui"
import { useStyleStore } from '../../../store/style';

const style_store = useStyleStore();

const popmsg = useMessage();

const fonts = [
    {
        value: '楷体',
        label: '楷体'
    },
    {
        value: '宋体',
        label: '宋体'
    },
    {
        value: '仿宋',
        label: '仿宋'
    },
    {
        value: '黑体',
        label: '黑体'
    },
];

const font_size = ref(0);

const font_weight = ref(0);

const line_height = ref(0);

const font_family = ref("");

watch(font_size, () => {
    style_store.set_font_size(font_size.value);
    style_store.save();
})
watch(font_weight, () => {
    style_store.set_font_weight(font_weight.value);
    style_store.save();
})
watch(line_height, () => {
    style_store.set_line_height(line_height.value);
    style_store.save();
})
watch(font_family, () => {
    style_store.set_font_family(font_family.value);
    style_store.save();
})


onBeforeMount(() => {
    font_size.value = style_store.font_size;
    font_family.value = style_store.font_family;
    font_weight.value = style_store.font_weight;
    line_height.value = style_store.line_height;
});

</script>

<style scoped lang="less">
.Setting {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--base-bgc);

    .title {
        font-size: 16px;
        margin: 5px 0;
        color: #757575;
        height: 25px;
        text-align: center;
    }

    .set_item {
        display: flex;
        flex-direction: column;
        color: #757575;
        padding: 10px;

        .n-tag {
            margin-right: 5px;
        }

        &>div {
            margin: 10px 0;
            display: flex;
            white-space: nowrap;
        }
    }
}
</style>