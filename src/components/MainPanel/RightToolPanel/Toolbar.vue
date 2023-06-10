<template>
<div class="Toolbar" :class="global_style">
    <div class="top">
        <img src="/src/assets/search.svg" @click="switch_panel('SearchPanel')" title="搜索小说" >
    </div>
    <div class="bottom">
        <img src="/src/assets/setting.svg" title="设置" @click="switch_panel('Setting')">
    </div>
</div>
</template>

<script setup lang="ts">
import { inject,Ref } from 'vue';
//相关变量类型
type all_pan_obj={
    'SearchPanel':boolean,
    'Setting':boolean
}

//全局主题样式
const global_style=inject("global_style");
//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<all_pan_obj>;

function switch_panel(name:string){
    for(let k in all_panel){
        //console.log(k);
        if(k===name){
            // @ts-ignore
            if(all_panel[k]){
                // @ts-ignore
                all_panel[k]=false;
            }else{
                // @ts-ignore
                all_panel[k]=true;
            }

        }else{
            // @ts-ignore
            all_panel[k]=false;
        }
    }

}

</script>

<style scoped lang="less">
.Toolbar.dark{
    border-left: #1d1d1d solid 2px;
    background-color: #202020;
    img{
        &:hover{
            background-color: #444;
        }
    }
}
.Toolbar.white{
    background-color: #fff;
    border-left: 2px #e7e7e7 solid;
    img{
        &:hover{
            background-color: #ddd;
        }
    }
}
.Toolbar{
    width: 40px;
    padding: 5px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    .top{
        img{
            width: 30px;
            height:30px;
            padding: 3px;
            border-radius: 5px;
        }
    }
    .bottom{
        img{
            width: 30px;
            height:30px;
            padding: 3px;
            border-radius: 5px;
        }
    }

}

</style>
