<script setup lang="ts">
</script>

<template>
    <div class="Toolbox panel-shell">
        <div class="panel-title">工具箱</div>
        <div class="panel-empty tools">
            <div class="badge">即将开放</div>
            <p>格式转换与实用小工具会放在这里</p>
        </div>
    </div>
</template>

<style scoped lang="scss">
.Toolbox {
    .tools {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
        padding-top: 48px;
    }

    .badge {
        padding: 4px 10px;
        border-radius: 999px;
        background: var(--accent-soft);
        color: var(--accent-strong);
        font-size: 12px;
        font-weight: 600;
        letter-spacing: 0.04em;
    }

    p {
        max-width: 160px;
        line-height: 1.6;
    }
}
</style>
