<template>
<div class="View" ref="div_view">
  <div class="one_line" v-for="(item,index) in novel_show_lines">
    <div v-if="IsTitle(item)" class="title">{{item}}</div>
    <div v-else class="paragraph" :style="{'font-size':mainpan_font_size+'px',
                                    'font-weight':mainpan_font_weight,
                                    'line-height':mainpan_line_height/10+'em',
                                    'background-size':'15px '+mainpan_line_height/10+'em',
                                    }">
        {{item}}
      </div>

  </div>

</div>
</template>

<script setup lang="ts">
import { Ref, ref,onMounted, inject, watch } from 'vue';
import {dialog,event,fs, invoke} from '@tauri-apps/api';
import { GlobalTheme } from 'naive-ui';

/**
 * 自定义类型
 */
//目录类型
type type_cata_obj={
  name:string,
  line:number
};


/*
绑定标签
*/
const div_view=ref();
/*
控制内容的变量数据
*/

//程序要进行显示的小说行数内容
const novel_show_lines=ref([]) as Ref<Array<string>>;

/**
 * 取出父组件传递下来的变量
 */
//取出存放打开小说的函数变量，本组件用来存放该函数
const root_fun_open_novel=inject('root_fun_open_novel') as Ref<Function>;
//当前打开的小说名称,本组件修改它
const root_title=inject("root_title") as Ref<string>;
//用于显示当前小说阅读进度的变量，本组件用于更新这个值
const root_novel_prog=inject("root_novel_prog") as Ref<string>;
//用于控制提示消息是否显示
const cenpan_show_prompt=inject("cenpan_show_prompt") as Ref<boolean>;
//用于控制是否显示加载图标
const cenpan_show_loading=inject("cenpan_show_loading") as Ref<boolean>;
//存放处理跳转jump组件的按键处理函数
const cenpan_pro_jump_input=inject('cenpan_pro_jump_input') as Ref<Function>;
//存放所有遍历到的小说目录
const mainpan_novel_cata=inject("mainpan_novel_cata") as Ref<Array<type_cata_obj>>
//存放跳转函数
const mainpan_nov_jump_fun=inject("mainpan_nov_jump_fun") as Ref<Function>;
//字体大小
const mainpan_font_size=inject('mainpan_font_size') as Ref<number>;
//字体粗细
const mainpan_font_weight=inject("mainpan_font_weight") as Ref<number>
//行高
const mainpan_line_height=inject("mainpan_line_height") as Ref<number>;


/*
普通变量
*/
//存放读取到的所有小说内容
let novel_lines:Array<string>;
//存放视图显示出来的第一行占原小说的具体行数
let view_line:number;
//存放当前打开的小说的路径
let cur_novel_path:string;
//存放当前阅读到的章节
let cur_capter_index:number;

/**
 * 初始化函数
 */
onMounted(async ()=>{
    //初始化跳转函数
    mainpan_nov_jump_fun.value=fun_jump;
    //初始化跳转组件的按键处理函数
    cenpan_pro_jump_input.value=process_jump_input;
    //初始化打开小说的函数
    root_fun_open_novel.value=fun_open_novel;
    //监听鼠标点击事件
    document.addEventListener("click",()=>{
      if(div_view.value!==undefined&&div_view.value!==null){
        div_view.value.style.filter=""; //恢复原状
      }
    })
    //对快捷键进行处理
    document.addEventListener("keydown",async (e)=>{
    //ctrl+O：打开小说
    if(e.ctrlKey&&e.key==='o'){
      const selected=await dialog.open({
        multiple: false,
        filters: [{
        name: '小说文件',
        extensions: ['novel']
        }]
      });
      if(selected===null) return;
      //打开小说
      fun_open_novel(selected as string);
    }
    //跳转
    if(e.ctrlKey&&e.key==='g'){
      e.preventDefault();
      //设置视图为毛玻璃效果
      div_view.value.style.filter="blur(5px)";
    }
    //关闭当前小说
    if(e.ctrlKey&&e.key==='x'){
      e.preventDefault();
      let ret=await dialog.ask("确定要关闭当前小说？",{title:"提示",type:"info"});
      if(ret){
        if(root_title.value===undefined || root_title.value.length===0){
          await dialog.message("当前还没有打开小说",{title:"提示",type:"info"});
          return;
        }
        novel_show_lines.value.splice(0); //清空界面内容
        root_title.value=""; //清除当前显示的文件名
        cenpan_show_prompt.value=true; //重新显示提示信息
        if(novel_lines!==undefined){
          novel_lines.splice(0); //清空读取到的小说内容
        }
        root_novel_prog.value=""; //小说进度提示，置空
      }
    }
    });
    document.addEventListener("keyup",e=>{
      if(e.key==="PageUp"){
        if(cur_capter_index<mainpan_novel_cata.value.length){
          cur_capter_index++;
        }
      }else if(e.key==='PageDown'){
        if(cur_capter_index>0){
          cur_capter_index--;
        }
      }else if(e.key==='ArrowUp'){

      }else if(e.key==='ArrowDown'){

      }
    })
  //处理滑动
  var t:NodeJS.Timeout; //事件节流，防止频繁滚动导致界面卡顿
  div_view.value.addEventListener("wheel",async (e: any)=>{
    //body的高度设置多些，能够触发onsroll事件
    clearTimeout(t);
    t = setTimeout(function() {
      process_wheel(); //处理鼠标滑轮滚动事件
    }, 300)
  });

  //处理文件拖拽
  event.listen<Array<string>>("tauri://file-drop",(e)=>{
    let file=e.payload[0];
    fun_open_novel(file);
  })
});


//专门用于打开一个小说的函数，并将其内容显示在界面上
async function fun_open_novel(path:string){
      let b=await fs.exists(path);
      if(!b){
        await dialog.message('小说不存在！', { title: '打开失败', type: 'warning' });
        return;
      }
      //清空
      novel_show_lines.value.splice(0);
      mainpan_novel_cata.value.splice(0);
      //关闭显示提示信息
      cenpan_show_prompt.value=false; 
      
      //实现加载图案
      cenpan_show_loading.value=true;
      //console.log(novel_loading.value);
      cur_novel_path=path;
      //更新文件名
      root_title.value=get_file_name(path);
      if(path.endsWith(".novel")){
        //打开文件进行展示
        novel_lines=await invoke("open_novel",{filename:path});
      }else if(path.endsWith(".txt")){
        //打开文件进行展示
        novel_lines=await invoke("open_novel_txt",{filename:path});
      }else{
        await dialog.message('不支持该类型文件！', { title: '打开失败', type: 'warning' });
        return;
      }

      for(let i=0;i<novel_lines.length;i++){
        if(IsTitle(novel_lines[i])){
          mainpan_novel_cata.value.push({
            name:novel_lines[i],
            line:i
          });
        }
      }
      //获取该小说记录已经读到的行数（view_min,view_max）中的view_min
      view_line=await invoke("get_line",{path:path});
      //开始渲染,最多两百行，不足则渲染最后所有
      let end=view_line+200>novel_lines.length?novel_lines.length:view_line+200;
      for(let i=view_line;i<end;i++){
        novel_show_lines.value.push(novel_lines[i]);
      }
      //进度，按行显示
      root_novel_prog.value=view_line+"/"+novel_lines.length;
      cenpan_show_loading.value=false;
      // console.log(show_loading.value);
}
function IsTitle(line:string) {
  const r1 =new RegExp(/^\s*开\s*篇.*\r?\n?/); 
  if(r1.test(line)){
    return true;
  }
  const r2=new RegExp(/^\s*序\s*章.*\r?\n?/);
  if(r2.test(line)) return true;
  const r3=new RegExp(/^\s*第\s*[零一二三四五六七八九0-9]{1,7}\s*[章节幕卷集部回].*\r?\n?/);
  return r3.test(line);
}

//处理鼠标滑轮滚动事件
async function process_wheel(){
  let top=div_view.value.scrollTop;
    let buttom=top+div_view.value.clientHeight;

    let all_p=div_view.value.querySelectorAll('.one_line');
    let np=all_p.length; //当前渲染的个数
    //获得视图窗口第一行在小说novel_lines中的位置
    let cur_view_line=0;
    for(let i = 0;i<all_p.length;i++){
      if(all_p[i].offsetTop>top){
        cur_view_line=i;
        break;
      }
    }
    //向前渲染最多20条，不足20条则渲染剩下的所有
    // console.log(cur_view_line);
    // console.log(all_p);
    if(cur_view_line===0||cur_view_line===1){
      let num=view_line>20?20:view_line;
      for(let i=view_line-num;i<view_line;i++){
        //向前渲染
        novel_show_lines.value.unshift(novel_lines[i]);
        //向后删除对应多的数据
        novel_show_lines.value.pop();
      }
      view_line-=num;
      all_p=div_view.value.querySelectorAll('.one_line');
      div_view.value.scrollTop=all_p[num].offsetTop;
    }else if(all_p[np-2].offsetTop<=buttom){ //倒数第2个元素已经显示出来，需要增加
      //向后渲染50条，不足50条则渲染剩下所有
      let num=view_line+np+50>novel_lines.length?novel_lines.length-view_line+np:50;
      let end=view_line+np+num;
      //end为所有小说内容的索引
      for(let i=view_line+np;i<end;i++){
        //向后渲染
        novel_show_lines.value.push(novel_lines[i]);
        //向前删除对应多的数据
        novel_show_lines.value.shift();
      }
      view_line+=(end-view_line-np);
      all_p=div_view.value.querySelectorAll('.one_line');
      div_view.value.scrollTop=all_p[cur_view_line-num].offsetTop;
    }
    //每次滑动都要记录一下数据
    await invoke("set_line",{
      path:cur_novel_path,
      line:view_line+cur_view_line,
      allLines:novel_lines.length
    });

    //同时更新状态栏
    root_novel_prog.value=(view_line+cur_view_line)+"/"+novel_lines.length;
} 

//获取文件名
function get_file_name(path:string) {
  var idx = path.lastIndexOf('\\');
  idx = idx > -1 ? idx : path.lastIndexOf('/');
  if (idx < 0) {
    return path
  }
  path=path.substring(idx+1);
  return path.substring(0,path.lastIndexOf('.'));
}
//处理跳转对话框按键的函数
function process_jump_input(key:string,value:string){
  if(key==='Escape'){
    div_view.value.style.filter=""; //清除毛玻璃效果
  }
  if(key==='Enter'){ //按下Enter键，开启跳转
    let to_line=Number(value);
    if(novel_lines===undefined||to_line>novel_lines.length){
      return;
    }
    fun_jump(-1,to_line);
    div_view.value.style.filter=""; //清除毛玻璃效果
  }
}

function fun_jump(index:number,line:number){
    if(index !==-1){
      cur_capter_index=index;
    }
    //console.log(line);
    //清空
    novel_show_lines.value.splice(0);
    //console.log(novel_show_lines.value.length);
    //开始渲染,最多两百行，不足则渲染最后所有
    view_line=line;
    let end=view_line+200>novel_lines.length?novel_lines.length:view_line+200;
    for(let i=view_line;i<end;i++){
      novel_show_lines.value.push(novel_lines[i]);
    }
    //进度，按行显示
    //nov_prog.value=view_line+"/"+novel_lines.length;
    cenpan_show_loading.value=false;
    //滑动到第一个元素的位置
    div_view.value.scrollTop=div_view.value.firstChild.offsetTop;
    //每次跳转都要记录一下数据
    invoke("set_line",{
      path:cur_novel_path,
      line:view_line+view_line,
      allLines:novel_lines.length
    });
}

</script>

<style scoped lang="less">
.View.dark{
  &::-webkit-scrollbar-thumb{
      background-color: #959595;
      border-radius: 3px;
  }
  &::-webkit-scrollbar-track{
      background-color: #333;
  }
}
.View.white{
  &::-webkit-scrollbar-thumb{
      background-color: #ddd;
      border-radius: 3px;
  }
  &::-webkit-scrollbar-track{
      background-color: #eee;
  }
}

.View{
    height: 100%;
    background-color: var(--sbase1-bgc);
    overflow-y: auto;
    overflow-x: hidden;
    margin: 0 15px;
    padding: 0 5px;
    &::-webkit-scrollbar{
        width: 10px;
    }
    &::-webkit-scrollbar-thumb{
        background-color: var(--ssb-thumb-color);
        border-radius: 3px;
    }
    &::-webkit-scrollbar-track{
        background-color: var(--ssb-track-color);
    }
    .one_line{
      .title{
        font-size: 25px;
        color: #7F7F7F;
        text-align: center;
        margin: 15px 0;
      }
      .paragraph{
        word-break: break-all;
        background-image:url("/src/assets/line.svg");
        //background-repeat: repeat;
        color: #7F7F7F;
        user-select: text;
        &::selection {
            background:#f5eccf;
            opacity: 0.5;
        }
      }
    }
}
</style>
