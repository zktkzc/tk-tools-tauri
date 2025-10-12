use base64::Engine;
use hex::encode;
use serde::{Deserialize, Serialize};
use sha1::Digest;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Listener, Manager, WebviewUrl, WindowEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_store::StoreExt;
use tauri_plugin_updater::{Update, UpdaterExt};

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

fn base64_encode(value: Vec<u8>, safe: bool, pad: bool) -> String {
    if safe {
        if pad {
            base64::engine::general_purpose::URL_SAFE.encode(value)
        } else {
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value)
        }
    } else {
        if pad {
            base64::engine::general_purpose::STANDARD.encode(value)
        } else {
            base64::engine::general_purpose::STANDARD_NO_PAD.encode(value)
        }
    }
}

#[tauri::command]
fn calculate_text_base64(value: &str, safe: bool, pad: bool) -> String {
    base64_encode(value.as_bytes().to_vec(), safe, pad)
}

#[tauri::command]
fn calculate_file_base64(path: &str, safe: bool, pad: bool, flag: &str) -> String {
    let file_bytes = fs::read(path).unwrap();
    let result = base64_encode(file_bytes, safe, pad);
    let data_url = base64_to_data_url(result.as_str(), path);
    match flag {
        "null" => result,
        "data-url" => data_url,
        "css" => format!("background-image: url({})", data_url),
        "html" => format!("<img src=\"{}\" />", data_url),
        _ => result,
    }
}

fn base64_to_data_url(value: &str, path: &str) -> String {
    let mime_type = mime_guess::from_path(path).first_or_octet_stream();
    format!("data:{};base64,{}", mime_type, value)
}

#[tauri::command]
fn base64_decode(value: &str, safe: bool, pad: bool) -> Result<Vec<u8>, String> {
    let result;
    if safe {
        if pad {
            result = base64::engine::general_purpose::URL_SAFE.decode(value.as_bytes());
        } else {
            result = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(value.as_bytes());
        }
    } else {
        if pad {
            result = base64::engine::general_purpose::STANDARD.decode(value.as_bytes());
        } else {
            result = base64::engine::general_purpose::STANDARD_NO_PAD.decode(value.as_bytes());
        }
    }

    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn get_mime_type_from_base64_str(value: &str, safe: bool, pad: bool) -> String {
    let result = base64_decode(value, safe, pad);
    match result {
        Ok(bytes) => {
            if bytes.is_empty() {
                return "".to_string();
            }

            if let Some(info) = infer::get(&bytes) {
                info.mime_type().to_string()
            } else {
                "".to_string()
            }
        }
        Err(_) => "".to_string(),
    }
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
fn get_theme(app: AppHandle) -> String {
    match app.get_webview_window("main").unwrap().theme().unwrap() {
        tauri::Theme::Light => "light".to_string(),
        tauri::Theme::Dark => "dark".to_string(),
        _ => panic!("Get system theme error"),
    }
}

#[tauri::command]
fn change_theme(app: AppHandle, value: &str) {
    app.windows().values().for_each(|window| {
        match value {
            "light" => window.set_theme(Some(tauri::Theme::Light)).unwrap(),
            "dark" => window.set_theme(Some(tauri::Theme::Dark)).unwrap(),
            "system" => window.set_theme(None).unwrap(),
            _ => panic!("Change theme error"),
        }
        window.emit("theme-changed", &[] as &[()]).unwrap();
    });
}

#[tauri::command]
fn open_dev_tools(app: AppHandle, label: &str) {
    app.get_webview_window(label).unwrap().open_devtools()
}

#[tauri::command]
fn get_window_always_on_top(app: AppHandle, label: &str) -> bool {
    app.get_webview_window(label)
        .unwrap()
        .is_always_on_top()
        .unwrap()
}

#[tauri::command]
fn set_window_always_on_top(app: AppHandle, value: bool, label: &str) {
    app.get_webview_window(label)
        .unwrap()
        .set_always_on_top(value)
        .unwrap()
}

#[derive(Serialize)]
struct CheckUpdateResult {
    status: bool,
    version: String,
    body: String,
}

#[tauri::command]
async fn check_update(app: AppHandle) -> CheckUpdateResult {
    if let Some(update) = app.updater().unwrap().check().await.unwrap() {
        update_app(app, update.clone());
        return CheckUpdateResult {
            status: true,
            version: update.version,
            body: update.body.unwrap(),
        };
    }

    CheckUpdateResult {
        status: false,
        version: "".to_string(),
        body: "".to_string(),
    }
}

fn update_app(app: AppHandle, update_info: Update) {
    if let Some(update_window) = app.get_webview_window("update") {
        update_window.close().unwrap();
    }

    let window = tauri::webview::WebviewWindowBuilder::new(
        &app,
        "update",
        WebviewUrl::App(PathBuf::from("index.html#/update")),
    )
    .title("软件更新")
    .inner_size(500.0, 400.0)
    .min_inner_size(500.0, 400.0)
    .center()
    .build()
    .unwrap();

    let app_handle = app.clone();
    let update_info_clone = update_info.clone();
    window.listen("download-update", move |_| {
        let handle = app_handle.clone();
        let update_info = update_info_clone.clone();
        tauri::async_runtime::spawn(async move {
            update(handle, update_info).await.unwrap();
        });
    });

    let app_handle = app.clone();
    window.listen("cancel-update", move |_| {
        let app = app_handle.clone();
        let update_window = app.get_webview_window("update").unwrap();
        update_window.close().unwrap();
    });

    window.listen("window-ready", move |_| {
        let update_window = app.get_webview_window("update").unwrap();
        let update_info = update_info.clone();
        update_window
            .emit_to(
                "update",
                "show-update-window",
                &[CheckUpdateResult {
                    status: true,
                    version: update_info.version,
                    body: update_info.body.unwrap(),
                }],
            )
            .unwrap();
    });
}

#[derive(Serialize)]
struct DownloadPercentage {
    downloaded: usize,
    total: u64,
}

async fn update(app: AppHandle, update: Update) -> tauri_plugin_updater::Result<()> {
    let update_window = app.get_webview_window("update").unwrap();
    let mut downloaded = 0;

    update
        .download_and_install(
            |chunk_length, content_length| {
                downloaded += chunk_length;
                println!("下载中: {downloaded}/{content_length:?}");
                update_window
                    .emit_to(
                        "update",
                        "downloading-update",
                        &[DownloadPercentage {
                            downloaded,
                            total: content_length.unwrap(),
                        }],
                    )
                    .unwrap();
            },
            || {
                println!("下载完成");
                update_window
                    .emit_to("update", "download-update-complete", &[] as &[()])
                    .unwrap();
            },
        )
        .await?;

    let ans = app
        .dialog()
        .message("下载完成，点击重启软件")
        .kind(MessageDialogKind::Info)
        .title("软件更新")
        .buttons(MessageDialogButtons::OkCustom("确定".to_string()))
        .blocking_show();

    if ans {
        app.restart();
    }

    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct SettingsType {
    appearance: AppearanceType,
    system: SystemType,
}

#[derive(Deserialize, Serialize, Debug)]
struct AppearanceType {
    theme: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SystemType {
    #[serde(rename = "autoUpdate")]
    auto_update: bool,
}

/// 写入默认配置
fn write_default_settings<R: tauri::Runtime>(store: &tauri_plugin_store::Store<R>) {
    let default_settings_json = r#"
    {
        "appearance": {
            "theme": "system"
        },
        "system": {
            "autoUpdate": true
        }
    }
    "#;

    if let Ok(settings) = serde_json::from_str::<SettingsType>(default_settings_json) {
        if let Ok(settings_value) = serde_json::to_value(settings) {
            store.set("settings", settings_value);
            if let Err(err) = store.save() {
                eprintln!("保存默认配置失败: {:?}", err);
            }
        }
    }
}

/// 读取配置文件config.json
fn read_config_json_file(app: AppHandle) {
    match app.store("config.json") {
        Ok(store) => {
            match store.get("settings") {
                Some(settings_value) => {
                    match serde_json::from_value::<SettingsType>(settings_value.clone()) {
                        Ok(settings) => {
                            // 应用主题设置
                            change_theme(app.clone(), settings.appearance.theme.as_str());

                            // 软件启动时检测更新
                            if settings.system.auto_update {
                                let handle = app.clone();
                                tauri::async_runtime::spawn(async move {
                                    check_update(handle).await;
                                });
                            }
                        }
                        Err(err) => {
                            eprintln!("解析配置文件settings字段失败: {:?}", err);
                            // 解析失败，写入默认配置
                            write_default_settings(&store);
                            // 重新读取配置
                            read_config_json_file(app);
                        }
                    }
                }
                None => {
                    // 配置文件中没有settings字段，写入默认配置
                    write_default_settings(&store);
                    // 重新读取配置
                    read_config_json_file(app);
                }
            }
        }
        Err(err) => {
            eprintln!("读取配置文件失败: {:?}", err);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            calculate_hash,
            calculate_file_hash,
            get_theme,
            change_theme,
            open_dev_tools,
            get_window_always_on_top,
            set_window_always_on_top,
            check_update,
            calculate_text_base64,
            calculate_file_base64,
            base64_decode,
            get_mime_type_from_base64_str
        ])
        .setup(|app| {
            // 注册主窗体的关闭事件
            let main_window = app.get_webview_window("main").expect("获取main窗体失败");
            let app_handle = app.handle().clone();
            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();

                    let app = app_handle.clone();
                    app.exit(0);
                }
            });

            // 读取配置文件config.json
            read_config_json_file(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("软件运行失败");
}
