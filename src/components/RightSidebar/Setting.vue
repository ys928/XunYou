<script setup lang="ts">
import { onBeforeMount, ref, watch } from 'vue';
import { useStyleStore } from '../../store/style';
import { ElInputNumber, ElSelect, ElOption } from 'element-plus';

const style_store = useStyleStore();

const fonts = [
    { value: '楷体', label: '楷体' },
    { value: '宋体', label: '宋体' },
    { value: '仿宋', label: '仿宋' },
    { value: '黑体', label: '黑体' },
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

<template>
    <div class="Setting panel-shell">
        <div class="panel-title">阅读设置</div>
        <div class="set_list">
            <div class="set_card">
                <div class="label">字体大小</div>
                <el-input-number v-model="font_size" :min="10" :max="25" size="small" />
            </div>
            <div class="set_card">
                <div class="label">字体粗细</div>
                <el-input-number v-model="font_weight" :min="100" :max="900" :step="100" step-strictly size="small" />
            </div>
            <div class="set_card">
                <div class="label">行高</div>
                <el-input-number v-model="line_height" :min="10" :max="25" :step="1" step-strictly size="small" />
            </div>
            <div class="set_card">
                <div class="label">正文字体</div>
                <el-select v-model="font_family" placeholder="选择字体" size="small">
                    <el-option v-for="item in fonts" :key="item.value" :label="item.label" :value="item.value" />
                </el-select>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.Setting {
    .set_list {
        padding: 4px 12px 16px;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .set_card {
        padding: 12px;
        border-radius: var(--radius-md);
        border: 1px solid var(--border-color);
        background: var(--surface-raised);
        display: flex;
        flex-direction: column;
        gap: 8px;

        .label {
            font-size: 12px;
            color: var(--text-color1);
            letter-spacing: 0.04em;
        }

        :deep(.el-input-number),
        :deep(.el-select) {
            width: 100%;
        }
    }
}
</style>
