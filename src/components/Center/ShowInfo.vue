<script setup lang="ts">
</script>

<template>
  <div class="Init_info">
    <div class="shortcuts">
      <div class="row"><span class="keys">Ctrl + O</span><span class="desc">打开小说</span></div>
      <div class="row"><span class="keys">Ctrl + X</span><span class="desc">关闭当前</span></div>
      <div class="row"><span class="keys">PageUp / ←</span><span class="desc">上一章</span></div>
      <div class="row"><span class="keys">PageDown / →</span><span class="desc">下一章</span></div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.Init_info {
  position: absolute;
  inset: 0;
  display: grid;
  place-items: center;
  pointer-events: none;
  background: var(--page-bg);

  .shortcuts {
    display: grid;
    grid-template-columns: auto auto;
    column-gap: 28px;
    row-gap: 14px;
  }

  .row {
    display: grid;
    grid-template-columns: 120px 64px;
    align-items: baseline;
    gap: 16px;
    font-size: 13px;
    line-height: 1.4;
  }

  .keys {
    text-align: right;
    color: var(--text-color1);
    font-variant-numeric: tabular-nums;
  }

  .desc {
    text-align: left;
    color: var(--text-color);
  }
}
</style>
