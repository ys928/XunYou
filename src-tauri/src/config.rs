use core::panic;

use log::{warn};
use serde::{Serialize, Deserialize};
//初始化日志文件
pub fn init_log(){
    //构建日志文件路径
    let logfile=cfun::now("%Y_%m_%d.log");
    let cfpath=cfun::config_dir("XunYou");
    let p=cfpath.join(logfile);
    //确保该日志文件存在
    cfun::ensure_file(p.as_path().to_str().unwrap());
    //初始化日志文件
    cfun::log_to_file(p.to_str().unwrap());
}
#[derive(Serialize, Deserialize,Clone,Default)]
pub struct Novel{
    name:String, //小说名字
    path:String, //小说文件路径
    md5:String, //小说md5值校验
    cur_line:u64, //当前读到的行数
    all_line:u64, //小说总行数
}

#[derive(Serialize, Deserialize,Default)]
struct AppInfo{
    width:u32,  //软件宽度
    height:u32, //软件高度
    left_panel_status:bool, //左面板是否展开
    right_panel_status:bool, //右面板是否展开
    theme:String, //主题
    novel_folder:String, //小说文件夹
}

#[derive(Serialize, Deserialize,Default)]
struct ConfigInfo{
    app:AppInfo,    //软件界面相关的配置信息
    record:Vec<Novel> //最近打开过的小说
}

//获取历史小说文件记录
#[tauri::command]
pub fn get_record()->Vec<Novel>{
    let cfg=config_info();
    cfg.record
}

//获取指定小说的行数
#[tauri::command]
pub fn get_line(path:&str)->u64{
    let md5=cfun::md5_file(path).unwrap_or_else(||{
       warn!("get md5 failed:{}",path);
       panic!(); 
    });
    let cfg_info=config_info();
    for i in cfg_info.record{
        if i.md5==md5{
            return i.cur_line;
        }
    }
    return 0;
}
#[tauri::command]
pub fn del_record(path:&str){
    //得到当前配置文件
    let mut cfg=config_info();
    for i in 0..cfg.record.len(){
        if cfg.record[i].path==path{
            cfg.record.remove(i);
            break;
        }
    }
    record_config(cfg);
}
//记录宽高
#[tauri::command]
pub fn set_wh(w:u32,h:u32){
    //得到当前配置文件
    let mut cfg=config_info();
    cfg.app.width=w;
    cfg.app.height=h;
    //保存到配置文件中
    record_config(cfg);
}

pub fn get_wh() ->(u32,u32){
    //得到当前配置文件
    let cfg=config_info();
    (cfg.app.width,cfg.app.height)
}

#[tauri::command]
pub fn set_line(path:&str,line:u64,all_lines:u64){
    //info!("set_line:{},{},{}",path,line,all_lines);
    let md5=cfun::md5_file(path).unwrap_or_else(||{
        warn!("error to get md5");
        panic!("error to get md5");
    });
    //得到当前配置文件
    let mut cfg=config_info();

    let mut f=true;
    for i in &mut cfg.record{
        if i.md5==md5{
            i.cur_line=line;
            i.path=path.to_string();
            f=false;
            break;
        }
    }
    //新小说，需要添加
    if f {
        cfg.record.push(Novel{
            name:cfun::file_name(path),
            path:path.to_string(),
            md5:md5,
            cur_line:line,
            all_line:all_lines
        });

    }
    //保存到文件中
    record_config(cfg);
    //info!("set_line:{},success",path);
}

//获取配置信息
fn config_info()->ConfigInfo{
    let cf_path=cfun::config_dir("XunYou");
    let p=cf_path.join("profile");
    let p=p.as_path().to_str().unwrap_or_else(||{
        warn!("get config file failed");
        panic!();
    });
    if !cfun::exist_file(p){
        let mut c=ConfigInfo::default();
        //默认宽高
        c.app.width=1200;
        c.app.height=800;
        c.app.theme="dark".to_string();
        c.app.novel_folder="novels".to_string();
        let s=serde_json::to_string(&c).unwrap_or_else(|e|{
            warn!("serde_json failed:{}",e);
            panic!("");
        });
        std::fs::write(p, s).unwrap_or_else(|e|{
            warn!("write config info failed:{}",e);
            panic!("");
        });
        return c;
    }
    let cfg_info=std::fs::read_to_string(p).unwrap_or_else(|e|{
        warn!("get config file failed:{}",e);
        panic!("");
    });
    let app_info:ConfigInfo=serde_json::from_str(&cfg_info).unwrap_or_else(|e|{
        warn!("serde_json failed:{}",e);
        let mut t=ConfigInfo::default();
        t.app.theme="dark".to_string();
        t
    });
    app_info
}
//保存配置信息到文件中
fn record_config(cfg:ConfigInfo){
    let cf_path=cfun::config_dir("XunYou");
    let p=cf_path.join("profile");
    let p=p.as_path().to_str().unwrap_or_else(||{
        warn!("get config file failed");
        panic!();
    });

    let s=serde_json::to_string(&cfg).unwrap_or_else(|e|{
        warn!("{}",e);
        panic!("error for serde_json's to_string function");
    });
    std::fs::write(p,s).unwrap_or_else(|e|{
        warn!("{}",e);
        panic!("error for write config to file");
    })
}

//获取配置文件中的主题信息
#[tauri::command]
pub fn get_theme()->String{
    let cfg=config_info();
    cfg.app.theme
}

//获取配置文件中的主题信息
#[tauri::command]
pub fn set_theme(theme:&str){
    let mut cfg=config_info();
    cfg.app.theme=theme.to_string();
    record_config(cfg);
}
//设置存放小说的文件夹记录
#[tauri::command]
pub fn set_novel_folder(folder:&str){
    let mut cfg=config_info();
    cfg.app.novel_folder=folder.to_string();
    record_config(cfg);
}
//获取存放小说的文件夹记录
#[tauri::command]
pub fn get_novel_folder()->String{
    let cfg=config_info();
    cfg.app.novel_folder
}