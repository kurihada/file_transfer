mod config;
mod device;
mod entity;
mod response;
mod router;
mod utils;

use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

use crate::router::app_window::{
    about, loading_window_close, windows_close, windows_hide, windows_maximize, windows_minimize,
};

use crate::config::config::init_config;

use crate::router::file_utils::{exist_file, get_document_notebooks, set_data_dir};
use crate::router::note_book::{
    create_note_file, create_notebook, get_note_list, read_note_file, remove_note, remove_notebook,
    save_note,
};
#[tauri::command]
async fn scan_devices() -> Vec<device::DeviceInfo> {
    match device::scan_devices() {
        Ok(devices) => devices,
        Err(_) => todo!(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_note_file,
            create_notebook,
            get_note_list,
            read_note_file,
            remove_note,
            remove_notebook,
            save_note,
            scan_devices,
            about,
            loading_window_close,
            windows_minimize,
            windows_hide,
            windows_maximize,
            windows_close,
            get_document_notebooks,
            set_data_dir,
            exist_file,
        ])
        .setup(|app| {
            let config = init_config(&app.path().app_config_dir().unwrap());
            app.manage(config.unwrap());

            let handle: &tauri::AppHandle = app.handle();
            let about_menu_item = MenuItem::new(handle, "About", true, None::<&str>)?;
            let menu = Menu::with_items(
                handle,
                &[&Submenu::with_items(
                    handle,
                    "File",
                    true,
                    &[&about_menu_item],
                )?],
            )?;
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("File Transfer")
                .inner_size(800.0, 600.0)
                .menu(menu)
                .on_menu_event(move |_, event| {
                    if event.id == about_menu_item.id() {
                        println!("{:?}", event)
                    }
                })
                .enable_clipboard_access();
            let _window = win_builder.build().unwrap();

            // 仅在 macOS 时设置透明标题栏和背景颜色
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil};
                use tauri::TitleBarStyle;

                let ns_window = _window.ns_window().unwrap() as id;
                unsafe {
                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        204.0 / 255.0,
                        255.0 / 255.0,
                        255.5 / 255.0,
                        1.0,
                    );
                    ns_window.setBackgroundColor_(bg_color);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
