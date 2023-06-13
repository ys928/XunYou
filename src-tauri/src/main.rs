// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use tauri::{Manager, Size, PhysicalSize, api::file};
use log::{info,warn};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    config::init_log();
    tauri::Builder::default()
        .setup(move |app|{
            let window = app.get_window("MainWindow").unwrap();
            //初始化窗口大小
            let (w,h)=config::get_wh();
            window.set_size(Size::Physical(PhysicalSize{ width: w, height: h })).unwrap();
            window_shadows::set_shadow(&window,true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_novel,
            open_novel_txt,
            config::set_line,
            config::get_line,
            config::get_record,
            config::del_record,
            config::set_wh,
            config::get_theme,
            config::set_theme,
            config::set_novel_folder,
            config::get_novel_folder,
            config::get_setting,
            config::set_setting,
            txt_to_bzip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn open_novel_txt(filename:&str) ->Vec<String>{
    let str=std::fs::read_to_string(filename).unwrap_or_else(|e|{
        warn!("error to write config info to file,{}",e);
        panic!("error");
    });
    let str=str.replace("\r", "");
    let v:Vec<&str>=str.split("\n").collect();
    let mut ret=Vec::new();
    for i in v{
        ret.push(i.to_string());
    }
    ret
}

#[tauri::command]
fn open_novel(filename:&str) ->Vec<String>{
    info!("read file and begin decompress:{}",filename);
    let v=cfun::decmpr_bzip2_file(filename);
    info!("completed decompress and convert it to string");
    let s=String::from_utf8(v).unwrap_or_else(|e|{
        warn!("error to write config info to file,{}",e);
        panic!("error");
    });
    let s=s.replace("\r", "");
    info!("begin splite by \\r\\n");
    let v:Vec<&str>=s.split("\n").collect();
    let mut ret=Vec::new();
    for i in v{
        ret.push(i.to_string());
    }
    info!("complate read and return all data");
    ret
}
#[tauri::command]
fn txt_to_bzip(txt:&str){
    let v=cfun::cmpr_bzip2_file(&txt);
    let mut name=cfun::file_name(&txt);
    let p=std::path::Path::new(&txt);
    let p=p.parent().unwrap();
    name.drain(name.find('.').unwrap()..);
    name=name+".novel";
    let name=p.join(name);
    std::fs::write(name, v).unwrap();
}