use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::novel::{
    add_nov_bookmark, add_nov_record, del_nov_bookmark, get_nov_bookmarks, get_nov_encoding,
    get_nov_progress, set_nov_progress, Bookmark, Chapter, Progress,
};
use crate::txt_encoding::{self, EncodingPreview};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Novel {
    pub path: String,
    pub name: String,
    pub encoding: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenTxtResult {
    pub status: String,
    pub encoding: Option<String>,
    pub auto_detected: bool,
    pub previews: Vec<EncodingPreview>,
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
pub fn novel_open_txt(filepath: &str, encoding: Option<String>) -> Result<OpenTxtResult, String> {
    let bytes =
        std::fs::read(filepath).map_err(|e| format!("无法读取文件：{e}"))?;

    let name = Path::new(filepath)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "未命名".to_string());

    let forced = encoding
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let outcome = if let Some(label) = forced {
        txt_encoding::decode_with_label(&bytes, label)?
    } else {
        let hint = get_nov_encoding(&name);
        txt_encoding::detect_and_decode(&bytes, hint.as_deref(), is_title)?
    };

    if outcome.low_confidence && forced.is_none() {
        return Ok(OpenTxtResult {
            status: "need_pick".to_string(),
            encoding: Some(outcome.encoding),
            auto_detected: true,
            previews: txt_encoding::preview_encodings(&bytes, is_title),
        });
    }

    let mut nov = parse_novel_text(&outcome.text);
    nov.name = name;
    nov.path = filepath.to_string();
    nov.encoding = outcome.encoding.clone();

    add_nov_record(filepath, Some(&outcome.encoding));

    let mut novel = OPENED_NOVEL.lock().unwrap();
    *novel = Some(nov);

    Ok(OpenTxtResult {
        status: "ok".to_string(),
        encoding: Some(outcome.encoding),
        auto_detected: forced.is_none() && !outcome.used_hint,
        previews: Vec::new(),
    })
}

#[tauri::command]
pub fn novel_preview_encodings(filepath: &str) -> Result<Vec<EncodingPreview>, String> {
    let bytes =
        std::fs::read(filepath).map_err(|e| format!("无法读取文件：{e}"))?;
    Ok(txt_encoding::preview_encodings(&bytes, is_title))
}

fn parse_novel_text(nov_content: &str) -> Novel {
    let nov_content = nov_content.replace('\r', "");
    let lines: Vec<&str> = nov_content.split('\n').collect();

    let mut chap = Chapter::default();

    if !lines.is_empty() && !is_title(lines[0]) {
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
    nov.chapters.push(chap);
    nov
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

/// 各章段落行数，用于按当前页容量估算全书剩余页数
#[tauri::command]
pub fn novel_get_chapter_line_counts() -> Result<Vec<usize>, String> {
    let novel = OPENED_NOVEL.lock().unwrap();
    if novel.is_none() {
        return Err("还没有打开该小说".to_string());
    }
    Ok(novel
        .as_ref()
        .unwrap()
        .chapters
        .iter()
        .map(|c| c.lines.len())
        .collect())
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
