<script setup lang="ts">
import { ElDialog, ElButton } from 'element-plus';
import { useNovelStore } from '../store/novel';

const novel_store = useNovelStore();

async function choose(encoding: string) {
    await novel_store.apply_encoding(encoding);
}
</script>

<template>
    <el-dialog
        v-model="novel_store.encoding_dialog.visible"
        width="560"
        append-to-body
        class="encoding-dialog"
        :close-on-click-modal="true"
        @close="novel_store.close_encoding_dialog"
    >
        <template #header>
            <div class="dlg-title">选择文本编码</div>
        </template>
        <p class="hint">没能可靠识别编码。请对照预览点选正确的一项。</p>
        <div class="preview-list">
            <button
                v-for="item in novel_store.encoding_dialog.previews"
                :key="item.encoding"
                class="preview-card"
                :class="{
                    recommended: item.recommended,
                    current: item.encoding === novel_store.encoding && novel_store.isopen,
                }"
                type="button"
                @click="choose(item.encoding)"
            >
                <div class="card-head">
                    <span class="enc-name">{{ item.display_name }}</span>
                    <span v-if="item.recommended" class="tag">推荐</span>
                    <span v-else-if="item.encoding === novel_store.encoding && novel_store.isopen" class="tag muted">当前</span>
                </div>
                <pre class="preview">{{ item.preview || '（无预览）' }}</pre>
            </button>
        </div>
        <template #footer>
            <el-button @click="novel_store.close_encoding_dialog">取消</el-button>
        </template>
    </el-dialog>
</template>

<style scoped lang="scss">
.dlg-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-color);
}

.hint {
    margin: 0 0 12px;
    font-size: 12px;
    line-height: 1.6;
    color: var(--text-color1);
}

.preview-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 420px;
    overflow: auto;
}

.preview-card {
    text-align: left;
    border: 1px solid var(--border-color);
    background: var(--surface);
    border-radius: var(--radius-md);
    padding: 10px 12px;
    cursor: pointer;
    color: inherit;
    transition: border-color 0.15s var(--ease), background-color 0.15s var(--ease);

    &:hover {
        border-color: var(--accent);
        background: var(--surface-raised);
    }

    &.recommended {
        border-color: var(--accent);
    }
}

.card-head {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
}

.enc-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-color);
}

.tag {
    font-size: 11px;
    padding: 1px 6px;
    border-radius: 999px;
    background: var(--accent-soft);
    color: var(--accent-strong);

    &.muted {
        color: var(--text-color1);
    }
}

.preview {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-all;
    font-size: 12px;
    line-height: 1.55;
    color: var(--text-color2);
    font-family: inherit;
    max-height: 7.8em;
    overflow: hidden;
}
</style>
