<template>
    <div class="Jump" ref="div_jump">
        <input v-if="cenpan_show_jump" v-focus @keydown="process_input($event)" 
ref="m_jump" type="text" placeholder="输入跳转行数">
    </div>
</template>

<script setup lang="ts">
import { Ref, inject, onMounted, ref } from 'vue';
//绑定Jump这个组件
const div_jump=ref();
//绑定jump这个input
const m_jump=ref();
//用于决定是否显示
const cenpan_show_jump=inject("cenpan_show_jump") as Ref<boolean>;
//存放处理跳转jump组件的按键处理函数
const cenpan_pro_jump_input=inject('cenpan_pro_jump_input') as Ref<Function>;
//让input显示时聚焦
const vFocus={
  mounted: (el:HTMLInputElement) => el.focus()
}

onMounted(()=>{
    document.addEventListener("click",async e=>{
    if(e.target===m_jump.value) return;
    cenpan_show_jump.value=false;
  });
});
//处理按键输入
function process_input(e:KeyboardEvent){
    if(e.key==='ArrowLeft'||e.key==='ArrowRight'){
        return true; 
    }
    if(e.key==='Backspace'){
        return true;
    }
    if(e.key==='Escape'){
        cenpan_show_jump.value=false;
    }
    cenpan_pro_jump_input.value(e.key,m_jump.value.value);
    if(Number.isFinite(Number(e.key))){
        return true;
    }
    if(e.key==='Enter'){ //按下Enter键，开启跳转
        cenpan_show_jump.value=false; //隐藏
    }
    e.preventDefault();
}
</script>

<style scoped lang="less">
.Jump{
    position: fixed;
    width: 200px;
    height: 25px;
    left: 50%;
    top: 10%;
    transform: translate(-50%,-50%);
    input{
        width: 200px;
        height: 25px;
        outline: none;
        padding: 0 10px;
        border-radius: 5px;
        background-color: var(--base2-bgc);
        color: var(--base2-color);
        border: var(--border-color) solid 2px;
        border-bottom: #4c5277 solid 2px;
    }

}
</style>