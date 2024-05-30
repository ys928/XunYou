use core::panic;

use log::warn;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;

use crate::common;
use crate::types::*;

//获取历史小说文件记录
#[tauri::command]
pub fn get_record() -> Result<Vec<Novel>, String> {
    let cfg = config_info()?;
    Ok(cfg.record)
}

//获取指定小说的行数
#[tauri::command]
pub fn get_nov_prog(path: &str) -> Result<(u64, u64), String> {
    let cfg_info = config_info()?;

    // 优先按路径匹配
    for i in cfg_info.record.iter() {
        if i.path == path {
            return Ok((i.cur_chapter, i.cur_line));
        }
    }

    // 其次按md5值匹配
    let md5 = common::md5_file(path).unwrap_or_default();

    for i in cfg_info.record.iter() {
        if i.md5 == md5 {
            return Ok((i.cur_chapter, i.cur_line));
        }
    }

    return Ok((0, 0));
}
#[tauri::command]
pub fn del_record(path: &str) -> Result<(), String> {
    //得到当前配置文件
    let mut cfg = config_info()?;
    for i in 0..cfg.record.len() {
        if cfg.record[i].path == path {
            cfg.record.remove(i);
            break;
        }
    }
    record_config(cfg);
    Ok(())
}
//记录宽高
#[tauri::command]
pub fn set_wh(w: u32, h: u32) -> Result<(), String> {
    //得到当前配置文件
    let mut cfg = config_info()?;
    cfg.app.width = w;
    cfg.app.height = h;
    //保存到配置文件中
    record_config(cfg);
    Ok(())
}

pub fn get_wh() -> Result<(u32, u32), String> {
    //得到当前配置文件
    let cfg = config_info()?;
    Ok((cfg.app.width, cfg.app.height))
}

#[tauri::command]
pub fn set_nov_prog(path: &str, line: u64, chapter: u64) -> Result<(), String> {
    //info!("set_nov_prog:{},{},{}",path,line,all_lines);
    let md5 = common::md5_file(path);
    if md5.is_none() {
        return Err("error to get md5".to_string());
    }
    let md5 = md5.unwrap();
    //得到当前配置文件
    let mut cfg = config_info()?;

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
            bookmark: Vec::new(),
        });
    }
    //保存到文件中
    record_config(cfg);
    Ok(())
}

//获取配置信息
fn config_info() -> Result<ConfigInfo, String> {
    let cf_path = common::config_dir("XunYou");
    let p = cf_path.join("profile.json");
    let p = p.as_path().to_str();
    if p.is_none() {
        return Err("get config file failed".to_string());
    }
    let p = p.unwrap();

    if !common::exist_file(p) {
        let c = ConfigInfo::default();
        //默认宽高
        let s = serde_json::to_string(&c);
        if s.is_err() {
            return Err(s.unwrap_err().to_string());
        }
        let s = s.unwrap();

        let ret = std::fs::write(p, s);
        if ret.is_err() {
            return Err(ret.unwrap_err().to_string());
        }
        return Ok(c);
    }
    let cfg_info = std::fs::read_to_string(p);
    if cfg_info.is_err() {
        return Err(cfg_info.unwrap_err().to_string());
    }
    let cfg_info = cfg_info.unwrap();
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
    Ok(app_info)
}
//保存配置信息到文件中
fn record_config(cfg: ConfigInfo) {
    let cf_path = common::config_dir("XunYou");
    let p = cf_path.join("profile.json");
    let p = p.to_str();
    if p.is_none() {
        warn!("get config path failed");
        return;
    }

    let p = p.unwrap();

    let s = serde_json::to_string(&cfg);
    if s.is_err() {
        warn!("{}", s.unwrap_err());
        return;
    }

    let s = s.unwrap();

    let ret = std::fs::write(p, s);
    if ret.is_err() {
        warn!("{}", ret.unwrap_err());
    }
}

//获取配置文件中的主题信息
#[tauri::command]
pub fn get_theme() -> Result<String, String> {
    let cfg = config_info()?;
    Ok(cfg.app.theme)
}

//获取配置文件中的主题信息
#[tauri::command]
pub fn set_theme(theme: &str) -> Result<(), String> {
    let mut cfg = config_info()?;
    cfg.app.theme = theme.to_string();
    record_config(cfg);
    Ok(())
}
//设置存放小说的文件夹记录
#[tauri::command]
pub fn set_novel_folder(folder: &str) -> Result<(), String> {
    let mut cfg = config_info()?;
    cfg.app.novel_folder = folder.to_string();
    record_config(cfg);
    Ok(())
}
//获取存放小说的文件夹记录
#[tauri::command]
pub fn get_novel_folder() -> Result<String, String> {
    let cfg = config_info()?;
    Ok(cfg.app.novel_folder)
}
//获取小说界面相关设置
#[tauri::command]
pub fn get_setting() -> Result<AppSet, String> {
    let cfg = config_info()?;
    Ok(cfg.appset)
}
//设置小说界面相关设置
#[tauri::command]
pub fn set_setting(fs: u32, fw: u32, lh: u32, ff: &str) -> Result<(), String> {
    let mut cfg = config_info()?;
    cfg.appset.font_size = fs;
    cfg.appset.font_weight = fw;
    cfg.appset.line_height = lh;
    cfg.appset.font_family = ff.to_string();
    record_config(cfg);
    Ok(())
}

//获取当前小说标签
pub fn get_bookmark(path: &str) -> Result<Vec<Bookmark>, String> {
    let cfg = config_info()?;
    for n in cfg.record {
        if n.path == path {
            return Ok(n.bookmark);
        }
    }
    Ok(Vec::new())
}
//添加当前小说书签
pub fn add_bookmark(path: &str, mark: Bookmark) -> Result<(), String> {
    let mut cfg = config_info()?;
    for n in cfg.record.iter_mut() {
        if n.path == path {
            n.bookmark.push(mark);
            break;
        }
    }
    record_config(cfg);
    Ok(())
}

//删除当前小说书签
#[tauri::command]
pub fn del_bookmark(path: &str, id: String) -> Result<(), String> {
    let mut cfg = config_info()?;
    for n in cfg.record.iter_mut() {
        if n.path != path {
            continue;
        }
        for b in 0..n.bookmark.len() {
            if n.bookmark[b].id == id {
                n.bookmark.remove(b);
                break;
            }
        }
        break;
    }
    record_config(cfg);
    Ok(())
}

/// config the log
pub fn init_log() {
    let con = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%H:%M:%S)} {h([{l}])} {M}:{L} => {m}{n}",
        )))
        .build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(con)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}
