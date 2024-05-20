<script setup lang="ts">

</script>

<template>
  <div class="Init_info">
    <div>Ctrl+O:打开小说</div>
    <div>Ctrl+X:关闭当前小说</div>
    <div>PageUp 或 <- :上一章</div>
    <div>PageDown 或 -> :下一章</div>
  </div>
</template>

<style scoped lang="less">
.Init_info {
  position: absolute;
  left: 50%;
  top: 50%;
  font-size: large;
  transform: translate(-50%, -50%);

  div {
    color: #757575;
    margin: 10px 0;
    opacity: 0.5;
    white-space: nowrap;
  }
}
</style>