use core::panic;

use serde::{Deserialize, Serialize};
//初始化日志文件
#[allow(dead_code)]
fn init_log() {
    //构建日志文件路径
    let logfile = cfun::now("%Y_%m_%d.log");
    let cfpath = cfun::config_dir("XunYou");
    let p = cfpath.join(logfile);
    //确保该日志文件存在
    cfun::ensure_file(p.as_path().to_str().unwrap());
    //初始化日志文件
    cfun::log_to_file(p.to_str().unwrap());
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Novel {
    name: String,     //小说名字
    path: String,     //小说文件路径
    md5: String,      //小说md5值校验
    cur_chapter: u64, //当前章节（从0开始）
    cur_line: u64,    //当前章节中读到的行数
}

#[derive(Serialize, Deserialize)]
struct AppInfo {
    width: u32,               //软件宽度
    height: u32,              //软件高度
    left_panel_status: bool,  //左面板是否展开
    right_panel_status: bool, //右面板是否展开
    theme: String,            //主题
    novel_folder: String,     //小说文件夹
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            left_panel_status: false,
            right_panel_status: false,
            theme: "dark".to_string(),
            novel_folder: "novels".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppSet {
    font_size: u32,   //字体大小
    font_weight: u32, //字体粗细
    line_height: u32, //行高,除以10
}
impl Default for AppSet {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_weight: 400,
            line_height: 16,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct ConfigInfo {
    app: AppInfo,       //软件界面相关的配置信息
    record: Vec<Novel>, //最近打开过的小说
    appset: AppSet,     //相关的设置项
}

//获取历史小说文件记录
#[tauri::command]
pub fn get_record() -> Vec<Novel> {
    let cfg = config_info();
    cfg.record
}

//获取指定小说的行数
#[tauri::command]
pub fn get_nov_prog(path: &str) -> (u64, u64) {
    let md5 = cfun::md5_file(path).unwrap_or_else(|| {
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
    let md5 = cfun::md5_file(path).unwrap_or_else(|| {
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
            name: cfun::file_name(path),
            path: path.to_string(),
            md5: md5,
            cur_line: line,
            cur_chapter: chapter,
        });
    }
    //保存到文件中
    record_config(cfg);
    //info!("set_nov_prog:{},success",path);
}

//获取配置信息
fn config_info() -> ConfigInfo {
    let cf_path = cfun::config_dir("XunYou");
    let p = cf_path.join("profile");
    let p = p.as_path().to_str().unwrap_or_else(|| {
        //warn!("get config file failed");
        panic!();
    });
    if !cfun::exist_file(p) {
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
    let cf_path = cfun::config_dir("XunYou");
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

#[tauri::command]
pub fn get_setting() -> AppSet {
    let cfg = config_info();
    cfg.appset
}

#[tauri::command]
pub fn set_setting(fs: u32, fw: u32, lh: u32) {
    let mut cfg = config_info();
    cfg.appset.font_size = fs;
    cfg.appset.font_weight = fw;
    cfg.appset.line_height = lh;
    record_config(cfg);
}
