#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod config;
mod novel;
mod tool;
mod types;
use tauri::{Manager, PhysicalSize, Size};

fn main() {
    novel::init();

    //config::init_log();
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("MainWindow").unwrap();
            //初始化窗口大小
            let (w, h) = config::get_wh();
            window
                .set_size(Size::Physical(PhysicalSize {
                    width: w,
                    height: h,
                }))
                .unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tool::open_novel,
            tool::open_novel_txt,
            tool::txt_to_bzip,
            config::set_nov_prog,
            config::get_nov_prog,
            config::get_record,
            config::del_record,
            config::set_wh,
            config::get_theme,
            config::set_theme,
            config::set_novel_folder,
            config::get_novel_folder,
            config::get_setting,
            config::set_setting,
            config::get_bookmark,
            config::add_bookmark,
            config::del_bookmark,
            novel::novel_open_txt,
            novel::novel_get_chapter,
            novel::novel_get_record,
            novel::novel_get_num_chapters,
            novel::novel_get_cata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
