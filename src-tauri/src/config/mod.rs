use app::AppInfo;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use novel::NovelInfo;
use serde::Deserialize;
use serde::Serialize;
use style::StyleInfo;

pub mod app;
pub mod novel;
pub mod style;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfigInfo {
    pub appinfo: AppInfo,       //软件界面相关的配置信息
    pub record: Vec<NovelInfo>, //最近打开过的小说
    pub styleinfo: StyleInfo,   //相关的设置项
}

/// 获取配置文件路径，不存在则创建空配置文件
fn get_config_file_path() -> String {
    let s = dirs::config_dir().unwrap();
    let p = s.as_path().join("xunyou");
    if !p.exists() {
        std::fs::create_dir_all(&p).unwrap();
    }
    let cfg_path = p.join("config.json");
    if !cfg_path.exists() {
        std::fs::write(&cfg_path, "").unwrap();
    }
    cfg_path.to_string_lossy().to_string()
}

pub fn get_config() -> AppConfigInfo {
    let cfg_path = get_config_file_path();
    let cfg = std::fs::read_to_string(cfg_path).unwrap();
    let cfg: AppConfigInfo = serde_json::from_str(&cfg).unwrap_or_default();
    cfg
}

pub fn set_config(cfg: AppConfigInfo) {
    let cfg_path = get_config_file_path();
    let s = serde_json::to_string(&cfg).unwrap();
    std::fs::write(&cfg_path, s).unwrap();
}

// //获取历史小说文件记录
// #[tauri::command]
// pub fn get_record() -> Result<Vec<NovelInfo>, String> {
//     let cfg = config_info()?;
//     Ok(cfg.record)
// }

// //添加小说文件记录
// pub fn add_record(mut nov: NovelInfo) -> Result<(), String> {
//     let mut cfg = config_info()?;
//     for n in cfg.record.iter() {
//         if n.path == nov.path {
//             return Ok(());
//         }
//     }
//     cfg.record.push(nov);
//     record_config(cfg);
//     Ok(())
// }

// //获取指定小说的行数
// #[tauri::command]
// pub fn get_nov_prog(path: &str) -> Result<Record, String> {
//     let cfg_info = config_info()?;

//     // 按路径匹配
//     for i in cfg_info.record.iter() {
//         if i.path == path {
//             return Ok(i.record.clone());
//         }
//     }

//     return Ok(Record::default());
// }
// #[tauri::command]
// pub fn del_record(path: &str) -> Result<(), String> {
//     //得到当前配置文件
//     let mut cfg = config_info()?;
//     for i in 0..cfg.record.len() {
//         if cfg.record[i].path == path {
//             cfg.record.remove(i);
//             break;
//         }
//     }
//     record_config(cfg);
//     Ok(())
// }
// //记录宽高
// #[tauri::command]
// pub fn set_wh(w: u32, h: u32) -> Result<(), String> {
//     //得到当前配置文件
//     let mut cfg = config_info()?;
//     cfg.app.width = w;
//     cfg.app.height = h;
//     //保存到配置文件中
//     record_config(cfg);
//     Ok(())
// }

// pub fn get_wh() -> Result<(u32, u32), String> {
//     //得到当前配置文件
//     let cfg = config_info()?;
//     Ok((cfg.app.width, cfg.app.height))
// }

// #[tauri::command]
// pub fn set_nov_prog(path: &str, record: Record) -> Result<(), String> {
//     //得到当前配置文件
//     let mut cfg = config_info()?;

//     let mut f = true;
//     for i in &mut cfg.record {
//         if i.path == path {
//             i.record = record.clone();
//             f = false;
//             break;
//         }
//     }
//     //新小说，需要添加
//     if f {
//         cfg.record.push(NovelInfo {
//             name: common::file_name(path),
//             path: path.to_string(),
//             chapters: Vec::new(),
//             bookmarks: Vec::new(),
//             record,
//         });
//     }
//     //保存到文件中
//     record_config(cfg);
//     Ok(())
// }

// //获取配置信息
// fn config_info() -> Result<ConfigInfo, String> {
//     let cf_path = common::config_dir("XunYou");
//     let p = cf_path.join("profile.json");
//     let p = p.as_path().to_str();
//     if p.is_none() {
//         return Err("get config file failed".to_string());
//     }
//     let p = p.unwrap();

//     if !common::exist_file(p) {
//         let c = ConfigInfo::default();
//         //默认宽高
//         let s = serde_json::to_string(&c);
//         if s.is_err() {
//             return Err(s.unwrap_err().to_string());
//         }
//         let s = s.unwrap();

//         let ret = std::fs::write(p, s);
//         if ret.is_err() {
//             return Err(ret.unwrap_err().to_string());
//         }
//         return Ok(c);
//     }
//     let cfg_info = std::fs::read_to_string(p);
//     if cfg_info.is_err() {
//         return Err(cfg_info.unwrap_err().to_string());
//     }
//     let cfg_info = cfg_info.unwrap();
//     let app_info: ConfigInfo = serde_json::from_str(&cfg_info).unwrap_or_else(|_e| {
//         //warn!("serde_json failed:{}",e);
//         let c = ConfigInfo::default();
//         let s = serde_json::to_string(&c).unwrap_or_else(|_e| {
//             //warn!("serde_json failed:{}",e);
//             panic!("");
//         });
//         std::fs::write(p, s).unwrap_or_else(|_e| {
//             //warn!("write config info failed:{}",e);
//             panic!("");
//         });
//         return c;
//     });
//     Ok(app_info)
// }
// //保存配置信息到文件中
// fn record_config(cfg: ConfigInfo) {
//     let cf_path = common::config_dir("XunYou");
//     let p = cf_path.join("profile.json");
//     let p = p.to_str();
//     if p.is_none() {
//         warn!("get config path failed");
//         return;
//     }

//     let p = p.unwrap();

//     let s = serde_json::to_string(&cfg);
//     if s.is_err() {
//         warn!("{}", s.unwrap_err());
//         return;
//     }

//     let s = s.unwrap();

//     let ret = std::fs::write(p, s);
//     if ret.is_err() {
//         warn!("{}", ret.unwrap_err());
//     }
// }

// //获取配置文件中的主题信息
// #[tauri::command]
// pub fn get_theme() -> Result<String, String> {
//     let cfg = config_info()?;
//     Ok(cfg.app.theme)
// }

// //获取配置文件中的主题信息
// #[tauri::command]
// pub fn set_theme(theme: &str) -> Result<(), String> {
//     let mut cfg = config_info()?;
//     cfg.app.theme = theme.to_string();
//     record_config(cfg);
//     Ok(())
// }
// //设置存放小说的文件夹记录
// #[tauri::command]
// pub fn cfg_set_novel_folder(folder: &str) -> Result<(), String> {
//     let mut cfg = config_info()?;
//     cfg.app.novel_folder = folder.to_string();
//     record_config(cfg);
//     Ok(())
// }
// //获取存放小说的文件夹记录
// #[tauri::command]
// pub fn cfg_get_novel_folder() -> Result<String, String> {
//     let cfg = config_info()?;
//     Ok(cfg.app.novel_folder)
// }
// //获取小说界面相关设置
// #[tauri::command]
// pub fn get_setting() -> Result<StyleInfo, String> {
//     let cfg = config_info()?;
//     Ok(cfg.appset)
// }
// //设置小说界面相关设置
// #[tauri::command]
// pub fn set_setting(fs: u32, fw: u32, lh: u32, ff: &str) -> Result<(), String> {
//     let mut cfg = config_info()?;
//     cfg.appset.font_size = fs;
//     cfg.appset.font_weight = fw;
//     cfg.appset.line_height = lh;
//     cfg.appset.font_family = ff.to_string();
//     record_config(cfg);
//     Ok(())
// }

// //获取当前小说标签
// pub fn get_bookmark(path: &str) -> Result<Vec<Bookmark>, String> {
//     let cfg = config_info()?;
//     for n in cfg.record {
//         if n.path == path {
//             return Ok(n.bookmarks);
//         }
//     }
//     Ok(Vec::new())
// }
// //添加当前小说书签
// pub fn add_bookmark(path: &str, mark: Bookmark) -> Result<(), String> {
//     let mut cfg = config_info()?;
//     for n in cfg.record.iter_mut() {
//         if n.path == path {
//             n.bookmarks.push(mark);
//             break;
//         }
//     }
//     info!("{:?}", cfg);
//     record_config(cfg);
//     Ok(())
// }

// //删除当前小说书签
// #[tauri::command]
// pub fn del_bookmark(path: &str, id: String) -> Result<(), String> {
//     let mut cfg = config_info()?;
//     for n in cfg.record.iter_mut() {
//         if n.path != path {
//             continue;
//         }
//         for b in 0..n.bookmarks.len() {
//             if n.bookmarks[b].id == id {
//                 n.bookmarks.remove(b);
//                 break;
//             }
//         }
//         break;
//     }
//     record_config(cfg);
//     Ok(())
// }

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
