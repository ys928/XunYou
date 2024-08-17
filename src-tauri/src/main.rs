#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod novel;
use tauri::{Manager, PhysicalSize, Size};

fn main() {
    config::init_log();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("MainWindow").unwrap();
            //初始化窗口大小
            let (w, h) = config::app::get_window_wh();
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
            config::app::cfg_get_app_theme,
            config::app::cfg_set_app_theme,
            config::app::cfg_set_windows_wh,
            config::app::cfg_set_novel_folder,
            config::app::cfg_get_novel_folder,
            config::novel::cfg_nov_del_record,
            config::novel::cfg_nov_get_records,
            config::style::cfg_app_get_style,
            config::style::cfg_app_set_style,
            novel::novel_open_txt,
            novel::novel_get_chapter,
            novel::novel_get_progress,
            novel::novel_get_num_chapters,
            novel::novel_get_cata,
            novel::novel_get_bookmark,
            novel::novel_add_bookmark,
            novel::novel_del_bookmark,
            novel::novel_set_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
