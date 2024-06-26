use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bookmark {
    pub id: String,       //识别该书签的唯一id
    pub label: String,    //该标签的额外标注信息
    pub chapter: u64,     //所属章节
    pub line: u64,        //所属行
    pub datetime: String, //创建日期
    pub content: String,  //简短内容，用于快捷展示
}


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

#[derive(Serialize, Deserialize, Debug)]
pub struct AppSet {
    pub font_size: u32,      //字体大小
    pub font_weight: u32,    //字体粗细
    pub line_height: u32,    //行高,除以10
    pub font_family: String, //字体
}
impl Default for AppSet {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_weight: 400,
            line_height: 16,
            font_family: "楷体".to_string(),
        }
    }
}

