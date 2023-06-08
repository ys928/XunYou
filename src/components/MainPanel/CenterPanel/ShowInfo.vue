<template>
<div class="Init_info" v-show="cenpan_show_prompt">
  <div>Ctrl+O:打开小说</div>
  <div>Ctrl+X:关闭当前小说</div>
  <div>Ctrl+G:跳转指定行数</div>
</div>
</template>

<script setup lang="ts">
import { inject, ref } from 'vue';


/**
 * 取出父组件传递下来的变量
 */
//用于控制提示消息是否显示
const cenpan_show_prompt=inject("cenpan_show_prompt");

</script>


<style scoped lang="less">
.Init_info{
  position: absolute;
  left: 50%;
  top: 50%;
  font-size: large;
  transform: translate(-50%,-50%);
  div{
    color: #757575;
    margin: 10px 0;
    opacity: 0.5;
    white-space:nowrap;
  }
}
</style>