<script setup lang="ts">
import { computed } from 'vue';
import { useCursorStore } from '../store/cursor';
import { useNovelStore } from '../store/novel';

const cursor_store = useCursorStore();
const novel_store = useNovelStore();

const progressText = computed(() => {
    if (!novel_store.isopen || novel_store.cata.length === 0) {
        return '';
    }
    return `${novel_store.cur_ch_idx + 1} / ${novel_store.cata.length}`;
});

const progressPercent = computed(() => {
    if (!novel_store.isopen || novel_store.cata.length === 0) {
        return 0;
    }
    return Math.round(((novel_store.cur_ch_idx + 1) / novel_store.cata.length) * 100);
});
</script>

<template>
    <div @mouseenter="cursor_store.set_style('default')" class="StatusBar" ref="div_statusbar">
        <div class="left">
            <template v-if="novel_store.isopen">
                <span class="chip">{{ novel_store.name }}</span>
                <span class="sep"></span>
                <span class="muted truncate">{{ novel_store.show_chapter.title || '未命名章节' }}</span>
            </template>
            <template v-else>
                <span class="muted">准备就绪 · Ctrl+O 打开小说</span>
            </template>
        </div>
        <div class="right" v-if="novel_store.isopen">
            <span class="muted">{{ progressText }}</span>
            <div class="progress" :title="`进度 ${progressPercent}%`">
                <div class="fill" :style="{ width: progressPercent + '%' }"></div>
            </div>
            <span class="pct">{{ progressPercent }}%</span>
        </div>
    </div>
</template>

<style scoped lang="scss">
.StatusBar {
    height: var(--statusbar-h);
    min-height: var(--statusbar-h);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 0 14px;
    font-size: 11px;
    background: var(--base-bgc);
    border-top: 1px solid var(--border-color);
    color: var(--text-color1);

    .left,
    .right {
        display: flex;
        align-items: center;
        gap: 10px;
        min-width: 0;
    }

    .left {
        flex: 1;
    }

    .chip {
        max-width: 180px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        padding: 2px 8px;
        border-radius: 999px;
        background: var(--hover-color);
        color: var(--text-color);
        font-weight: 500;
    }

    .sep {
        width: 1px;
        height: 10px;
        background: var(--border-color);
        flex-shrink: 0;
    }

    .muted {
        color: var(--text-color1);
    }

    .truncate {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .progress {
        width: 72px;
        height: 3px;
        border-radius: 999px;
        background: var(--track-color);
        overflow: hidden;

        .fill {
            height: 100%;
            border-radius: inherit;
            background: var(--text-color1);
            transition: width 0.25s var(--ease);
        }
    }

    .pct {
        width: 28px;
        text-align: right;
        font-variant-numeric: tabular-nums;
        color: var(--text-color);
    }
}
</style>
