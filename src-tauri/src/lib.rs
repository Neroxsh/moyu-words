mod commands;
mod db;
mod vocab;

use db::Database;
use std::sync::Mutex;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()
                .map_err(|e| format!("Failed to get app data dir: {}", e))?;
            std::fs::create_dir_all(&data_dir)
                .map_err(|e| format!("Failed to create data dir: {}", e))?;
            let db_path = data_dir.join("moyu_words.db");
            let database = Database::new(&db_path)
                .map_err(|e| format!("Failed to open database: {}", e))?;
            app.manage(AppState {
                db: Mutex::new(database),
            });

            // Set dock icon at runtime (needed for dev mode)
            #[cfg(target_os = "macos")]
            set_dock_icon();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_books,
            commands::get_book,
            commands::get_active_book,
            commands::set_active_book,
            commands::add_book_from_file,
            commands::add_builtin_book,
            commands::delete_book,
            commands::get_book_summary,
            commands::list_words,
            commands::get_archived_words,
            commands::archive_word,
            commands::restore_word,
            commands::export_archived_words_csv,
            commands::get_active_plan,
            commands::create_plan,
            commands::list_units,
            commands::get_unit_info,
            commands::mark_unit_doing,
            commands::mark_unit_done,
            commands::list_unit_words,
            commands::mark_unit_word,
            commands::get_setting,
            commands::set_setting,
            commands::get_builtin_book_list,
            commands::create_overlay_window,
            commands::close_overlay_window,
            commands::hide_overlay_window,
            commands::get_current_word,
            commands::go_prev_word,
            commands::go_next_word,
            commands::toggle_auto,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "macos")]
fn set_dock_icon() {
    use std::os::raw::c_void;
    use objc2::AnyThread;
    use objc2_foundation::NSData;
    use objc2_app_kit::NSImage;

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));
    let icon_path = exe_dir
        .and_then(|d| {
            let p = d.join("icons").join("icon.icns");
            if p.exists() { return Some(p); }
            let p2 = d.join("../../../src-tauri/icons/icon.icns");
            if p2.exists() { return Some(p2); }
            None
        });

    if let Some(path) = icon_path {
        println!("[set_dock_icon] Loading icon from: {:?}", path);
        if let Ok(data) = std::fs::read(&path) {
            println!("[set_dock_icon] Read {} bytes", data.len());
            unsafe {
                let ns_data = NSData::dataWithBytes_length(
                    data.as_ptr() as *const c_void,
                    data.len(),
                );
                let image = NSImage::initWithData(NSImage::alloc(), &ns_data);
                if let Some(ref img) = image {
                    println!("[set_dock_icon] Image loaded successfully, size: {:?}", img.size());
                    let mtm = objc2::MainThreadMarker::new().unwrap();
                    let app = objc2_app_kit::NSApplication::sharedApplication(mtm);
                    app.setApplicationIconImage(Some(img));
                    println!("[set_dock_icon] Dock icon set!");
                } else {
                    println!("[set_dock_icon] FAILED to create NSImage from data");
                }
            }
        } else {
            println!("[set_dock_icon] FAILED to read file");
        }
    } else {
        println!("[set_dock_icon] Icon file not found!");
    }
}