use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::novel::{
    add_nov_bookmark, add_nov_record, del_nov_bookmark, get_nov_bookmarks, get_nov_progress,
    set_nov_progress, Bookmark, Chapter, Progress,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Novel {
    pub path: String,
    pub name: String,
    pub chapters: Vec<Chapter>,
}

/// 目录项
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CataItem {
    // 标题
    title: String,
    // 索引
    idx: usize,
}

static OPENED_NOVEL: Mutex<Option<Novel>> = Mutex::new(None);

/// 打开文本格式小说内容
#[tauri::command(async)]
pub fn novel_open_txt(filepath: &str) -> bool {
    let nov_content = std::fs::read_to_string(filepath);
    if nov_content.is_err() {
        return false;
    }
    let nov_content = nov_content.unwrap();

    let nov_content = nov_content.replace("\r", "");
    let lines: Vec<&str> = nov_content.split("\n").collect();

    let mut chap = Chapter::default();

    // 如果第一行不是标题，则添加一个默认标题
    if !is_title(lines[0]) {
        chap.title = "开始".to_string();
    }

    let mut nov = Novel::default();

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
    nov.name = Path::new(filepath)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    nov.path = filepath.to_string();

    // 保存在最近打开记录
    add_nov_record(filepath);

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

/// 获取小说阅读记录
#[tauri::command]
pub fn novel_get_progress() -> Result<Progress, String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    let nov = novel.as_ref().unwrap();
    let rec = get_nov_progress(&nov.name)?;
    Ok(rec)
}

#[tauri::command]
pub fn novel_get_num_chapters() -> Result<usize, String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    return Ok(novel.as_ref().unwrap().chapters.len());
}

#[tauri::command]
pub fn novel_get_cata() -> Result<Vec<CataItem>, String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    let mut cata = Vec::new();
    for (idx, c) in novel.as_ref().unwrap().chapters.iter().enumerate() {
        cata.push(CataItem {
            title: c.title.clone(),
            idx,
        });
    }
    Ok(cata)
}

#[tauri::command]
pub fn novel_get_bookmark() -> Result<Vec<Bookmark>, String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    let bookmarks = get_nov_bookmarks(&novel.as_ref().unwrap().name);
    Ok(bookmarks)
}

#[tauri::command]
pub fn novel_add_bookmark(mark: Bookmark) -> Result<(), String> {
    let mut novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    let nov = novel.as_mut().unwrap();
    add_nov_bookmark(nov.name.as_str(), mark);
    Ok(())
}

#[tauri::command]
pub fn novel_del_bookmark(id: String) -> Result<(), String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    let nov = novel.as_ref().unwrap();
    del_nov_bookmark(&nov.name, &id);
    Ok(())
}

/// 保存小说记录
#[tauri::command]
pub fn novel_set_progress(record: Progress) -> Result<(), String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    let nov = novel.as_ref().unwrap();
    set_nov_progress(&nov.name, record)?;
    Ok(())
}

/// 判断是否为标题
fn is_title(line: &str) -> bool {
    static RE_TITLE: OnceLock<Vec<Regex>> = OnceLock::new();
    RE_TITLE.get_or_init(|| {
        let res = vec![
            r"^\s*开\s*篇.*\r?\n?$", //开篇
            r"^\s*序\s*章.*\r?\n?$", //序章
            r"^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,10}\s*[章节幕卷集部回]\s*\r?\n?$", //第xxx章
            r"^\s*第\s*[零一二三四五六七八九十百千万0-9]{1,10}\s*[章节幕卷集部回]\s+.*\r?\n?$", //第xxx章 章节名
            r"^Chapter\s*[零一二三四五六七八九十百千万0-9]{1,10}\s+.*\r?\n?$", //Chapter xxx 章节名
        ];
        let mut v = Vec::new();
        for r in res {
            let r = Regex::new(r).unwrap();
            v.push(r);
        }
        v
    });
    let re_title = RE_TITLE.get().unwrap();
    for r in re_title {
        if r.is_match(line) {
            return true;
        }
    }
    return false;
}
