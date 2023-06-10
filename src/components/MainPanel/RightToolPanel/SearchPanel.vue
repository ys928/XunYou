<template>
<div class="SearchPanel" v-show="all_panel.SearchPanel" :class="global_style">
    <div class="title">搜索栏</div>
    <div class="top_pos">
        <input type="text" ref="input_search" @input="search_fun" class="search" :class="global_style" placeholder="搜小说">
        <img :class="global_style" @click="choose_dir" src="/src/assets/folder.svg" srcset="">
    </div>
    <div class="novel_list" ref="novel_list" :class="global_style">
        <div v-for="(item,index) in show_novel_list" class="novel_item" :class="global_style">
            <span @dblclick="dclick_novel(index)">{{item.name!.substring(0,item.name!.lastIndexOf('.'))}}</span>
        </div>
        <img v-show="show_loading" class="loading" src="/src/assets/search_load.gif" alt="">
    </div>
</div>
</template>

<script setup lang="ts">
import { Ref,ref, inject, onMounted } from 'vue';
import { FileEntry, readDir } from '@tauri-apps/api/fs';
import { dialog, invoke } from '@tauri-apps/api';
import { fs } from '@tauri-apps/api';
/**
 * 相关变量类型
 */
type all_pan_obj={
    'SearchPanel':boolean
}
/**
 * vue变量
 */

//控制面板显示与否变量
const all_panel=inject("all_panel") as Ref<all_pan_obj>;
//全局主题样式
const global_style=inject("global_style");
//控制要显示的小说列表项
let show_novel_list=ref([]) as Ref<Array<FileEntry>>;
//控制加载图标是否显示
const show_loading=ref(false);
/**
 * 绑定相关标签的变量
 */

//搜索框 
const input_search=ref();

/**
 * 普通变量
 */


//存储所有可搜索的小说项
let all_novel:Array<FileEntry>=[];

/**
 * 从父组件取出的变量
 */

//取出存放打开小说的函数变量，本组件用来使用该函数
const root_fun_open_novel=inject('root_fun_open_novel') as Ref<Function>;

async function search_fun() {
    if(input_search.value.value.length===0){ //为空，直接展示前一百条
        show_novel_list.value.splice(0); //先清空
        //优化性能，最多展示前100项
        let max=100<all_novel.length?100:all_novel.length;
        for(let i=0;i<max;i++){
            show_novel_list.value.push(all_novel[i]);
        }
        
    }else{
        show_novel_list.value.splice(0); //先清空
        let search_result=all_novel.filter(e => e.name!.includes(input_search.value.value) );
        for(let i=0;i<search_result.length;i++){
            show_novel_list.value.push(search_result[i]);
        }
    }
} 

onMounted(async ()=>{
    let p:string=await invoke("get_novel_folder",{});
    //bug,还需要判断这个文件是不是目录
    let ret=await fs.exists(p);
    if(ret){
        all_novel.splice(0); //清空
        show_novel_list.value.splice(0);
        all_novel=await readDir(p as string); //重新获取数据
        all_novel=all_novel.filter(e => e.name!.endsWith(".novel") );
        show_loading.value=false;
        //优化性能，最多展示前100项
        let max=100<all_novel.length?100:all_novel.length;
        for(let i=0;i<max;i++){
            show_novel_list.value.push(all_novel[i]);
        }
    }
});

async function choose_dir(){
    let p=await dialog.open({
        directory:true
    });
    if(p===null) return;
    show_loading.value=true;
    invoke("set_novel_folder",{folder:p});
    //console.log(all_novel);
    all_novel.splice(0); //清空
    show_novel_list.value.splice(0);
    all_novel=await readDir(p as string); //重新获取数据
    all_novel=all_novel.filter(e => e.name!.endsWith(".novel") );
    show_loading.value=false;
    //优化性能，最多展示前100项
    let max=100<all_novel.length?100:all_novel.length;
    for(let i=0;i<max;i++){
        show_novel_list.value.push(all_novel[i]);
    }
}

function dclick_novel(index:number){
    root_fun_open_novel.value(show_novel_list.value[index].path);
}

</script>

<style scoped lang="less">
.SearchPanel.white{
    background-color: #fff;
}
.SearchPanel.dark{
    background-color: #202020;
}
.SearchPanel{
    display: flex;
    flex-direction: column;
    height: 100%;
    .title{
        font-size: 16px;
        margin: 5px 0;
        color: #757575;
        height: 25px;
        text-align: center;
    }
    .top_pos{
        display: flex;
        justify-content: center;
        margin-bottom: 15px;
        padding: 0 30px;
        height: 20px;
        .search.white{
            background-color: #eee;
            &:focus{
                background-color: #fff;
                border: #666 solid 1px;
            }
        }
        .search.dark{
            background-color: #3e3e3e;
            &:focus{
                background-color: #333;
                border: #666 solid 1px;
            }
        }
        .search{
            outline: none;
            border: 0px;
            border-radius: 5px;
            flex-grow: 1;
            color: #757575;
            padding: 0 5px;
            height: 20px;
            width: 120px;
        }
        img.dark{
            &:hover{
                background-color: #555;
            }
        }
        img.white{
            &:hover{
                background-color: #ddd;
            }
        }
        img{
            width: 24px;
            height: 20px;
            display: inline;
            line-height: 20px;
            border-radius: 5px;
            margin-left: 3px;

        }
    }
    .novel_list.dark{
        border: 2px solid #3e3e3e;
        background-color: #2c2c2c;
        border-bottom: none;
        &::-webkit-scrollbar-thumb{
            background-color: #959595;
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: #333;
        }
    }
    .novel_list.white{
        background-color: #f4f3ed;
        border: 2px solid #e7e7e7;
        border-bottom: none;
        &::-webkit-scrollbar-thumb{
            background-color: #ddd;
            border-radius: 3px;
        }
        &::-webkit-scrollbar-track{
            background-color: #eee;
        }
    }
    .novel_list{
        font-size: 14px;
        overflow: auto;
        flex-grow: 1;
        text-align: left;
        padding: 0px 10px;
        border-radius: 5px;
        margin: 0 5px;
        position: relative;
        &::-webkit-scrollbar{
            width: 10px;
        }
        .novel_item.dark{
            background-color: #2e2e2e;
            border-bottom: #666 solid 2px;
            &:hover{
                background-color: #3f3f3f;
            }
        }
        .novel_item.white{
            border-bottom: #999 solid 2px;
            &:hover{
                background-color: #cfcfcf;
            }
        }
        .novel_item{
            margin: 5px 0;
            padding: 0 5px;
            white-space: nowrap;
            overflow-x: hidden;
            color: #7f7f7f;
            border-radius: 5px;
        }
        .loading{
            width: 50px;
            height: 50px;
            position: absolute;
            left: 50%;
            transform: translate(-50%,0);
        }
    }
}

</style>
