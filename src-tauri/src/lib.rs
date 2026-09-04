#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use sysinfo::System;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

mod desktop;
mod manifest;
mod widgets;
mod settings;
mod libraries;
mod installer;

use manifest::{save_widget, sync_and_get_manifest};
use tauri::http::{Response, StatusCode};
use tauri::State;
use widgets::{
    close_widget, create_widget, get_monitors, hide_widget, move_widget, reload_widgets,
    resize_widget, show_widget, reload_widget
};
use settings::{get_current_widget_settings, save_widget_settings};

pub struct WidgetManager {
    widgets: Mutex<Vec<String>>,
}

#[derive(Serialize)]
struct SysMetrics {
    cpu_usage: f32,
    ram_usage: f32,
    ram_used_gb: f32,
    ram_total_gb: f32,
}

struct SystemState {
    sys: Mutex<System>,
}

impl WidgetManager {
    fn new() -> Self {
        Self {
            widgets: Mutex::new(Vec::new()),
        }
    }

    pub fn add(&self, label: String) {
        let mut widgets = self.widgets.lock().unwrap();
        if !widgets.contains(&label) {
            widgets.push(label);
        }
    }

    pub fn remove(&self, label: &str) {
        let mut widgets = self.widgets.lock().unwrap();
        widgets.retain(|x| x != label);
    }
}

#[tauri::command]
fn get_system_metrics(state: State<'_, SystemState>) -> SysMetrics {
    let mut sys = state.sys.lock().unwrap();

    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_usage();
    let total_mem = sys.total_memory() as f32;
    let used_mem = sys.used_memory() as f32;

    let ram_usage = (used_mem / total_mem) * 100.0;
    let ram_used_gb = used_mem / 1_073_741_824.0;
    let ram_total_gb = total_mem / 1_073_741_824.0;

    SysMetrics {
        cpu_usage,
        ram_usage,
        ram_used_gb,
        ram_total_gb,
    }
}

#[tauri::command]
async fn http_get(url: String) -> Result<String, String> {
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    response.text().await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_media::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(SystemState {
            sys: Mutex::new(System::new_all()),
        })
        .register_uri_scheme_protocol("widget", move |ctx, request| {
            let uri = request.uri();

            println!("[widget protocol] request: {}", uri);

            let relative_path = uri.path().trim_start_matches('/');

            let base_dir = ctx
                .app_handle()
                .path()
                .local_data_dir()
                .expect("Failed to get local data directory")
                .join("win-widgets");

            let file_path = base_dir.join(relative_path);

            println!("[widget protocol] file: {}", file_path.display());

            if !file_path.exists() {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Vec::new())
                    .unwrap();
            }

            let data = match std::fs::read(&file_path) {
                Ok(data) => data,

                Err(error) => {
                    eprintln!(
                        "[widget protocol] failed to read {}: {}",
                        file_path.display(),
                        error
                    );

                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Vec::new())
                        .unwrap();
                }
            };

            let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", mime.as_ref())
                .body(data)
                .unwrap()
        })
        .plugin(tauri_plugin_http::init())
        .manage(WidgetManager::new())
        .invoke_handler(tauri::generate_handler![
            create_widget,
            close_widget,
            show_widget,
            hide_widget,
            move_widget,
            resize_widget,
            get_monitors,
            sync_and_get_manifest,
            reload_widgets,
            get_system_metrics,
            http_get,
            save_widget,
            get_current_widget_settings,
            save_widget_settings,
            installer::download_widget_pack,
            installer::install_widgets_from_pack,
            reload_widget,
            installer::remove_widget,
        ])
        .setup(|app| {
            println!("[app] setup started");

            let current_manifest = match manifest::sync_manifest(app.handle()) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Ошибка инициализации манифеста виджетов: {e}");
                    return Ok(());
                }
            };
            let main_window = app
                .get_webview_window("main")
                .expect("main window not found");

            main_window.clone().on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();

                    let _ = main_window.hide();
                }
            });

            let show_item = MenuItem::with_id(app, "toggle", "Manage widgets", true, None::<&str>)?;
            let reload_item = MenuItem::with_id(app, "reload", "Reload widgets", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_item, &reload_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "toggle" => {
                        let main_window = app
                            .get_webview_window("main")
                            .expect("main window not found");

                        if main_window.is_visible().unwrap() {
                            main_window.hide().unwrap();
                        } else {
                            main_window.show().unwrap();
                            main_window.set_focus().unwrap();
                        }
                    }
                    "reload" => {
                        let app = app.clone();

                        tauri::async_runtime::spawn(async move {
                            if let Err(error) = reload_widgets(app).await {
                                eprintln!("Failed to reload widgets: {}", error);
                            }
                        });
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();

                        if let Some(main_window) = app.get_webview_window("main") {
                            let _ = main_window.show();
                            let _ = main_window.unminimize();
                            let _ = main_window.set_focus();
                        }
                    }
                })
                .build(app)?;

            println!("[tray] ready");

            for widget in current_manifest.widgets {
                if !widget.enabled {
                    println!("[widget] skipped (disabled): {}", widget.id);
                    continue;
                }

                let widget_url = format!("widget://localhost/{}/index.html", widget.id);

                println!("[widget] loading: {}", widget_url);

                if let Err(e) = create_widget(
                    app.handle().clone(),
                    widget.window_label,
                    widget_url,
                    widget.x,
                    widget.y,
                    widget.width,
                    widget.height,
                    widget.interactive,
                ) {
                    eprintln!("Не удалось запустить виджет: {e}");
                }
            }

            println!("[app] setup finished");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
