use serde::{Deserialize, Serialize};

use super::{get_config, set_config};

#[derive(Serialize, Deserialize, Debug)]
pub struct StyleInfo {
    pub font_size: u32,      //字体大小
    pub font_weight: u32,    //字体粗细
    pub line_height: u32,    //行高,除以10
    pub font_family: String, //字体
}
impl Default for StyleInfo {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_weight: 400,
            line_height: 16,
            font_family: "楷体".to_string(),
        }
    }
}

#[tauri::command]
pub fn cfg_app_get_style() -> StyleInfo {
    let cfg = get_config();
    cfg.styleinfo
}

#[tauri::command]
pub fn cfg_app_set_style(style: StyleInfo) {
    let mut cfg = get_config();
    cfg.styleinfo = style;
    set_config(cfg);
}
