use serde::{Deserialize, Serialize};
use std::path::Path;

use super::{get_config, set_config};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bookmark {
    pub id: String,       //识别该书签的唯一id
    pub label: String,    //该标签的额外标注信息
    pub chapter: u64,     //所属章节
    pub line: u64,        //所属行
    pub datetime: String, //创建日期
    pub content: String,  //简短内容，用于快捷展示
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Chapter {
    /// 每章标题
    pub title: String,
    /// 每章内容，按行分割
    pub lines: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Progress {
    /// 章节索引
    pub chapter: u64,
    /// 章节内的行数
    pub line: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NovelInfo {
    pub path: String,
    pub name: String,
    pub bookmarks: Vec<Bookmark>,
    pub record: Progress,
}

fn get_novel_record(name: &str) -> Option<NovelInfo> {
    let cfg = get_config();
    for r in cfg.record {
        if r.name == name {
            return Some(r);
        }
    }
    None
}

fn set_novel_record(nov: NovelInfo) {
    let mut cfg = get_config();
    for r in cfg.record.iter_mut() {
        if r.name == nov.name {
            *r = nov;
            break;
        }
    }
    set_config(cfg);
}

#[tauri::command]
pub fn cfg_nov_get_records() -> Vec<NovelInfo> {
    let cfg = get_config();
    cfg.record
}

/// 将指定路径的小说添加到配置文件历史记录中
pub fn add_nov_record(path: &str) {
    let mut cfg = get_config();

    let name = Path::new(path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let pos = cfg.record.iter().position(|r| r.name == name);
    if pos.is_none() {
        cfg.record.push(NovelInfo {
            path: path.to_string(),
            name,
            bookmarks: Vec::new(),
            record: Progress::default(),
        });
    } else {
        cfg.record[pos.unwrap()].path = path.to_string();
    }

    set_config(cfg);
}

/// 删除指定名字的小说记录
#[tauri::command]
pub fn cfg_nov_del_record(name: &str) {
    let mut cfg = get_config();
    let pos = cfg.record.iter().position(|r| r.name == name);
    if pos.is_some() {
        cfg.record.remove(pos.unwrap());
        set_config(cfg);
    }
}

/// 记录小说的阅读进度
pub fn set_nov_progress(name: &str, record: Progress) -> Result<(), String> {
    let nov = get_novel_record(name);
    if nov.is_none() {
        return Err("没有找到该小说".to_string());
    }

    let mut nov = nov.unwrap();

    nov.record = record;

    set_novel_record(nov);
    Ok(())
}

/// 读取小说的阅读进度
pub fn get_nov_progress(name: &str) -> Result<Progress, &'static str> {
    let nov = get_novel_record(name);
    if nov.is_none() {
        return Err("没有找到该小说");
    }

    let nov = nov.unwrap();
    Ok(nov.record)
}

pub fn get_nov_bookmarks(name: &str) -> Vec<Bookmark> {
    let nov = get_novel_record(name);
    if nov.is_none() {
        return Vec::new();
    }
    return nov.unwrap().bookmarks;
}

pub fn add_nov_bookmark(name: &str, bookmark: Bookmark) {
    let nov = get_novel_record(name);
    if nov.is_none() {
        return;
    }
    let mut nov = nov.unwrap();
    nov.bookmarks.push(bookmark);
    set_novel_record(nov);
}

pub fn del_nov_bookmark(name: &str, id: &str) {
    let nov = get_novel_record(name);
    if nov.is_none() {
        return;
    }
    let mut nov = nov.unwrap();
    let pos = nov.bookmarks.iter().position(|b| b.id == id);
    if pos.is_some() {
        nov.bookmarks.remove(pos.unwrap());
        set_novel_record(nov);
    }
}
