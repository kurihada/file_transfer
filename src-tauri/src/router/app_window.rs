use tauri::{App, Context, Manager};
/**
 * 关闭加载窗口  显示主窗口
 * @param window
 */
#[tauri::command]
pub async fn loading_window_close(window: tauri::WebviewWindow) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_webview_window("loading-window") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_webview_window("label").unwrap().show().unwrap();
}
#[tauri::command]
pub async fn exit() {
    std::process::exit(0)
}
/**
 * 关于窗口
 * @param handle
 * @param window
 */
#[tauri::command]
pub async fn about(handle: tauri::AppHandle, window: tauri::WebviewWindow) {
    let about_window = tauri::WebviewWindowBuilder::new(
        &handle,
        "about-window", /* the unique window label */
        tauri::WebviewUrl::App("/Windows/About".into()),
    )
    .title("关于")
    .inner_size(720.0, 440.0)
    .decorations(false)
    .center()
    .resizable(false) //是否可调整大小
    .build()
    .unwrap();
}

#[tauri::command]
pub async fn windows_minimize(handle: tauri::AppHandle, label: String) {
    //获取窗口实例
    let window = handle.get_webview_window(label.as_str()).unwrap();
    window.minimize().unwrap();
}

#[tauri::command]
pub async fn windows_hide(handle: tauri::AppHandle, label: String) {
    //获取窗口实例
    let window = handle.get_webview_window(label.as_str()).unwrap();
    window.hide().unwrap();
}
#[tauri::command]
pub async fn windows_maximize(handle: tauri::AppHandle, label: String) {
    //获取窗口实例
    let window = handle.get_webview_window(label.as_str()).unwrap();
    window.maximize().unwrap();
}
/**
 * 窗口close
 * @param handle
 * @param label
 */
#[tauri::command]
pub async fn windows_close(handle: tauri::AppHandle, label: String) {
    //获取窗口实例
    let window = handle.get_webview_window(label.as_str()).unwrap();
    window.close().unwrap();
}
