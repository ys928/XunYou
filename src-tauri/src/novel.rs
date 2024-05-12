use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{config::get_bookmark, types::Bookmark};

static OPENED_NOVEL: Mutex<Option<Novel>> = Mutex::new(None);

static RE_TITLE: OnceLock<Vec<Regex>> = OnceLock::new();

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Chapter {
    /// 每章标题
    pub title: String,
    /// 每章内容，按行分割
    pub lines: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Novel {
    path: String,
    name: String,
    chapters: Vec<Chapter>,
    bookmarks: Vec<Bookmark>,
}

/// 使用该模块前必须调用
pub fn init() {
    RE_TITLE.get_or_init(|| {
        let mut v = Vec::new();

        let r = Regex::new(r"^\s*开\s*篇.*\r?\n?$").unwrap();
        v.push(r);
        //序章
        let r = Regex::new(r"^\s*序\s*章.*\r?\n?$").unwrap();
        v.push(r);
        //第xxx章
        let r = Regex::new(
            r"^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,10}\s*[章节幕卷集部回]\s*\r?\n?$",
        )
        .unwrap();

        v.push(r);

        //第xxx章 章节名
        let r = Regex::new(
            r"^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,10}\s*[章节幕卷集部回]\s+.*\r?\n?$",
        )
        .unwrap();

        v.push(r);

        //Chapter xxx 章节名
        let r =
            Regex::new(r"^Chapter\s*[零一二三四五六七八九十百千万0-9]{1,10}\s+.*\r?\n?$").unwrap();
        v.push(r);

        v
    });
}
/// 打开文本格式小说内容
#[tauri::command(async)]
pub fn novel_open_txt(filepath: &str) -> bool {
    let str = std::fs::read_to_string(filepath);
    if str.is_err() {
        return false;
    }

    let filename = Path::new(filepath)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let mut nov = Novel {
        path: filepath.to_string(),
        name: filename,
        chapters: Vec::new(),
        bookmarks: Vec::new(),
    };

    let str = str.unwrap();
    let str = str.replace("\r", "");
    let lines: Vec<&str> = str.split("\n").collect();

    let mut chap = Chapter::default();

    // 如果第一行不是标题，则添加一个默认标题
    if !is_title(lines[0]) {
        chap.title = "开始".to_string();
    }

    for line in lines {
        if is_title(line) {
            let c = chap;
            nov.chapters.push(c);

            chap = Chapter::default();
            chap.title = line.to_string();
            continue;
        }
        chap.lines.push(line.to_string());
    }
    // 最后一章
    nov.chapters.push(chap);

    // 获取书签
    nov.bookmarks = get_bookmark(filepath);

    let mut novel = OPENED_NOVEL.lock().unwrap();
    *novel = Some(nov);
    true
}

/// 获取指定小说章节
#[tauri::command]
pub fn novel_get_chapter(idx: usize) -> Result<Chapter, String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }

    let nov = novel.as_ref().unwrap();

    if nov.chapters.len() <= idx {
        return Err("超出章节范围".to_string());
    }
    let chap = nov.chapters[idx].clone();
    Ok(chap)
}

/// 判断是否为标题
fn is_title(line: &str) -> bool {
    let re_title = RE_TITLE.get().unwrap();
    for r in re_title {
        if r.is_match(line) {
            return true;
        }
    }
    return false;
}
