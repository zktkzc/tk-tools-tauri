use hex::encode;
use serde::{Deserialize, Serialize};
use sha1::Digest;
use std::io::Read;
use tauri::{AppHandle, Manager, Runtime, Window};

fn md5_hash(value: &str) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(value.as_bytes());
    encode(hasher.finalize())
}

fn sha1_hash(value: &str) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(value.as_bytes());
    encode(hasher.finalize())
}

fn sha256_hash(value: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(value.as_bytes());
    encode(hasher.finalize())
}

fn sha512_hash(value: &str) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(value.as_bytes());
    encode(hasher.finalize())
}

fn sm3_hash(value: &str) -> String {
    let mut hasher = sm3::Sm3::new();
    hasher.update(value.as_bytes());
    encode(hasher.finalize())
}

#[derive(Serialize)]
pub struct HashResult {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
    pub sm3: String,
}

#[tauri::command]
fn calculate_hash(value: &str) -> HashResult {
    let md5 = md5_hash(value);
    let sha1 = sha1_hash(value);
    let sha256 = sha256_hash(value);
    let sha512 = sha512_hash(value);
    let sm3 = sm3_hash(value);

    HashResult {
        md5: md5.to_string(),
        sha1: sha1.to_string(),
        sha256: sha256.to_string(),
        sha512: sha512.to_string(),
        sm3: sm3.to_string(),
    }
}

#[tauri::command]
fn calculate_file_hash(path: &str) -> HashResult {
    // 打开文件
    let mut file = std::fs::File::open(path).unwrap();

    // 初始化各算法的哈希器
    let mut md5_hasher = md5::Md5::new();
    let mut sha1_hasher = sha1::Sha1::new();
    let mut sha256_hasher = sha2::Sha256::new();
    let mut sha512_hasher = sha2::Sha512::new();
    let mut sm3_hasher = sm3::Sm3::new();

    // 分块读取文件（每次读取 8KB，可根据需求调整）
    let mut buffer = [0; 8192];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break; // 读取完毕
        }
        let chunk = &buffer[0..n];

        // 更新所有哈希器（一次读取，多算法并行计算）
        md5_hasher.update(chunk);
        sha1_hasher.update(chunk);
        sha256_hasher.update(chunk);
        sha512_hasher.update(chunk);
        sm3_hasher.update(chunk);
    }

    // 完成计算并转换为十六进制字符串
    HashResult {
        md5: encode(md5_hasher.finalize()),
        sha1: encode(sha1_hasher.finalize()),
        sha256: encode(sha256_hasher.finalize()),
        sha512: encode(sha512_hasher.finalize()),
        sm3: encode(sm3_hasher.finalize()),
    }
}

#[tauri::command]
fn get_theme<R: Runtime>(window: Window<R>) -> String {
    match window.theme().unwrap() {
        tauri::Theme::Light => "light".to_string(),
        tauri::Theme::Dark => "dark".to_string(),
        _ => panic!("Get system theme error"),
    }
}

#[tauri::command]
fn change_theme<R: Runtime>(window: Window<R>, value: &str) {
    match value {
        "light" => window.set_theme(Some(tauri::Theme::Light)).unwrap(),
        "dark" => window.set_theme(Some(tauri::Theme::Dark)).unwrap(),
        "system" => window.set_theme(None).unwrap(),
        _ => panic!("Change theme error"),
    }
}

#[tauri::command]
fn open_dev_tools<R: Runtime>(app: AppHandle<R>) {
    app.get_webview_window("main").unwrap().open_devtools()
}

#[tauri::command]
fn get_window_always_on_top<R: Runtime>(app: AppHandle<R>) -> bool {
    app.get_webview_window("main")
        .unwrap()
        .is_always_on_top()
        .unwrap()
}

#[tauri::command]
fn set_window_always_on_top<R: Runtime>(app: AppHandle<R>, value: bool) {
    app.get_webview_window("main")
        .unwrap()
        .set_always_on_top(value)
        .unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            calculate_hash,
            calculate_file_hash,
            get_theme,
            change_theme,
            open_dev_tools,
            get_window_always_on_top,
            set_window_always_on_top
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
