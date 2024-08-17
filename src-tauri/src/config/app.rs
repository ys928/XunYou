use serde::{Deserialize, Serialize};

use super::{get_config, set_config};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo {
    pub width: u32,               //软件宽度
    pub height: u32,              //软件高度
    pub left_panel_status: bool,  //左面板是否展开
    pub right_panel_status: bool, //右面板是否展开
    pub theme: String,            //主题
    pub novel_folder: String,     //小说文件夹
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

pub fn get_window_wh() -> (u32, u32) {
    let cfg = get_config();
    (cfg.appinfo.width, cfg.appinfo.height)
}

#[tauri::command]
pub fn cfg_set_windows_wh(w: u32, h: u32) {
    let mut cfg = get_config();
    cfg.appinfo.width = w;
    cfg.appinfo.height = h;
    set_config(cfg);
}

#[tauri::command]
pub fn cfg_set_app_theme(theme: &str) {
    let mut cfg = get_config();
    cfg.appinfo.theme = theme.to_string();
    set_config(cfg);
}

#[tauri::command]
pub fn cfg_get_app_theme() -> String {
    let cfg = get_config();
    cfg.appinfo.theme
}

#[tauri::command]
pub fn cfg_set_novel_folder(folder: &str) {
    let mut cfg = get_config();
    cfg.appinfo.novel_folder = folder.to_string();
    set_config(cfg);
}
#[tauri::command]
pub fn cfg_get_novel_folder() -> String {
    let cfg = get_config();
    cfg.appinfo.novel_folder
}
