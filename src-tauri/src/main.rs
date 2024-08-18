#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod novel;

fn main() {
    config::init_log();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            config::app::cfg_get_app_theme,
            config::app::cfg_set_app_theme,
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
