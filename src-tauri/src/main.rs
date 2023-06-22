// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod tool;
use tauri::{Manager, Size, PhysicalSize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    //config::init_log();
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
            tool::open_novel,
            tool::open_novel_txt,
            tool::txt_to_bzip,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}