<template>
<div class="MainPanel" :class="global_style">
    <LeftToolPanel></LeftToolPanel>
    <CenterPanel></CenterPanel>
    <RightToolPanel></RightToolPanel>
</div>
</template>

<script setup lang="ts">
import LeftToolPanel from './MainPanel/LeftToolPanel.vue';
import CenterPanel from './MainPanel/CenterPanel.vue';
import RightToolPanel from './MainPanel/RightToolPanel.vue';
import { inject } from 'vue';


//全局主题样式
const global_style=inject("global_style");

</script>

<style scoped lang="less">
.MainPanel.dark{
    background-color: #2c2c2c;
    border: 2px #3f3f3f solid;
}
.MainPanel.white{
    background-color: #f4f3ed;
    border: 2px #e7e7e7 solid;
}
.MainPanel{
    flex-grow: 1;
    height: 100px;
    width: 100%;
    display: flex;
}
</style>
