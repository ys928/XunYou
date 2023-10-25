use core::panic;

use crate::common;
use crate::types::*;


//获取历史小说文件记录
#[tauri::command]
pub fn get_record() -> Vec<Novel> {
    let cfg = config_info();
    cfg.record
}

//获取指定小说的行数
#[tauri::command]
pub fn get_nov_prog(path: &str) -> (u64, u64) {
    let md5 = common::md5_file(path).unwrap_or_else(|| {
        //warn!("get md5 failed:{}",path);
        panic!();
    });
    let cfg_info = config_info();
    for i in cfg_info.record {
        if i.md5 == md5 {
            return (i.cur_chapter, i.cur_line);
        }
    }
    return (0, 0);
}
#[tauri::command]
pub fn del_record(path: &str) {
    //得到当前配置文件
    let mut cfg = config_info();
    for i in 0..cfg.record.len() {
        if cfg.record[i].path == path {
            cfg.record.remove(i);
            break;
        }
    }
    record_config(cfg);
}
//记录宽高
#[tauri::command]
pub fn set_wh(w: u32, h: u32) {
    //得到当前配置文件
    let mut cfg = config_info();
    cfg.app.width = w;
    cfg.app.height = h;
    //保存到配置文件中
    record_config(cfg);
}

pub fn get_wh() -> (u32, u32) {
    //得到当前配置文件
    let cfg = config_info();
    (cfg.app.width, cfg.app.height)
}

#[tauri::command]
pub fn set_nov_prog(path: &str, line: u64, chapter: u64) {
    //info!("set_nov_prog:{},{},{}",path,line,all_lines);
    let md5 = common::md5_file(path).unwrap_or_else(|| {
        //warn!("error to get md5");
        panic!("error to get md5");
    });
    //得到当前配置文件
    let mut cfg = config_info();

    let mut f = true;
    for i in &mut cfg.record {
        if i.md5 == md5 {
            i.cur_line = line;
            i.cur_chapter = chapter;
            i.path = path.to_string();
            f = false;
            break;
        }
    }
    //新小说，需要添加
    if f {
        cfg.record.push(Novel {
            name: common::file_name(path),
            path: path.to_string(),
            md5: md5,
            cur_line: line,
            cur_chapter: chapter,
            bookmark:Vec::new()
        });
    }
    //保存到文件中
    record_config(cfg);
    //info!("set_nov_prog:{},success",path);
}

//获取配置信息
fn config_info() -> ConfigInfo {
    let cf_path =common::config_dir("XunYou");
    let p = cf_path.join("profile");
    let p = p.as_path().to_str().unwrap_or_else(|| {
        //warn!("get config file failed");
        panic!();
    });
    if !common::exist_file(p) {
        let c = ConfigInfo::default();
        //默认宽高
        let s = serde_json::to_string(&c).unwrap_or_else(|_e| {
            //warn!("serde_json failed:{}",e);
            panic!("");
        });
        std::fs::write(p, s).unwrap_or_else(|_e| {
            //warn!("write config info failed:{}",e);
            panic!("");
        });
        return c;
    }
    let cfg_info = std::fs::read_to_string(p).unwrap_or_else(|_e| {
        //warn!("get config file failed:{}",e);
        panic!("");
    });
    let app_info: ConfigInfo = serde_json::from_str(&cfg_info).unwrap_or_else(|_e| {
        //warn!("serde_json failed:{}",e);
        let c = ConfigInfo::default();
        let s = serde_json::to_string(&c).unwrap_or_else(|_e| {
            //warn!("serde_json failed:{}",e);
            panic!("");
        });
        std::fs::write(p, s).unwrap_or_else(|_e| {
            //warn!("write config info failed:{}",e);
            panic!("");
        });
        return c;
    });
    app_info
}
//保存配置信息到文件中
fn record_config(cfg: ConfigInfo) {
    let cf_path = common::config_dir("XunYou");
    let p = cf_path.join("profile");
    let p = p.as_path().to_str().unwrap_or_else(|| {
        //warn!("get config file failed");
        panic!();
    });

    let s = serde_json::to_string(&cfg).unwrap_or_else(|_e| {
        //warn!("{}",e);
        panic!("error for serde_json's to_string function");
    });
    std::fs::write(p, s).unwrap_or_else(|_e| {
        //warn!("{}",e);
        panic!("error for write config to file");
    })
}

//获取配置文件中的主题信息
#[tauri::command]
pub fn get_theme() -> String {
    let cfg = config_info();
    cfg.app.theme
}

//获取配置文件中的主题信息
#[tauri::command]
pub fn set_theme(theme: &str) {
    let mut cfg = config_info();
    cfg.app.theme = theme.to_string();
    record_config(cfg);
}
//设置存放小说的文件夹记录
#[tauri::command]
pub fn set_novel_folder(folder: &str) {
    let mut cfg = config_info();
    cfg.app.novel_folder = folder.to_string();
    record_config(cfg);
}
//获取存放小说的文件夹记录
#[tauri::command]
pub fn get_novel_folder() -> String {
    let cfg = config_info();
    cfg.app.novel_folder
}
//获取小说界面相关设置
#[tauri::command]
pub fn get_setting() -> AppSet {
    let cfg = config_info();
    cfg.appset
}
//设置小说界面相关设置
#[tauri::command]
pub fn set_setting(fs: u32, fw: u32, lh: u32,ff:&str) {
    let mut cfg = config_info();
    cfg.appset.font_size = fs;
    cfg.appset.font_weight = fw;
    cfg.appset.line_height = lh;
    cfg.appset.font_family=ff.to_string();
    record_config(cfg);
}

//获取当前小说标签
#[tauri::command]
pub fn get_bookmark(path: &str) ->Vec<Bookmark>{
    let cfg = config_info();
    for n in cfg.record{
        if n.path==path{
            return n.bookmark;
        }
    }
    Vec::new()    
}
//添加当前小说书签
#[tauri::command]
pub fn add_bookmark(path: &str,mark:Bookmark) {
    let mut cfg = config_info();
    for n in cfg.record.iter_mut(){
        if n.path==path{
            n.bookmark.push(mark);
            break;
        }
    }
    record_config(cfg);
}

//删除当前小说书签
#[tauri::command]
pub fn del_bookmark(path: &str,id:String) {
    let mut cfg = config_info();
    for n in cfg.record.iter_mut(){
        if n.path!=path{
            continue;
        }
        for b in 0..n.bookmark.len(){
            if n.bookmark[b].id==id{
                n.bookmark.remove(b);
                break;
            }
        }
        break;
    }
    record_config(cfg);
}
