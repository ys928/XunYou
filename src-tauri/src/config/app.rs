use serde::{Deserialize, Serialize};

use super::{get_config, set_config};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo {
    pub left_panel_status: bool,  //左面板是否展开
    pub right_panel_status: bool, //右面板是否展开
    pub theme: String,            //主题
    pub novel_folder: String,     //小说文件夹
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            left_panel_status: false,
            right_panel_status: false,
            theme: "dark".to_string(),
            novel_folder: "novels".to_string(),
        }
    }
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
