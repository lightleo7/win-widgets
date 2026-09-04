use crate::WidgetManager;
use serde::Serialize;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use crate::manifest;

#[cfg(target_os = "windows")]
// use std::{
//     collections::HashMap,
//     sync::{LazyLock, Mutex},
// };

// use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, WindowFromPoint};

// #[cfg(target_os = "windows")]
// use windows::Win32::{
//     Foundation::{HWND, LPARAM, LRESULT, WPARAM},
//     UI::WindowsAndMessaging::{
//         CallWindowProcW, EnumChildWindows, GetClassNameW, SetWindowLongPtrW, GWL_WNDPROC, HTCLIENT,
//         WM_MOUSEMOVE, WM_NCHITTEST, WNDPROC,
//     },
// };

use std::time::Duration;

async fn reload_widget_internal(
    app: &AppHandle,
    widget: manifest::WidgetConfig,
) -> Result<(), String> {

    println!("[widget] reloading: {}", widget.id);

    // Если виджет отключён — закрываем его окно,
    // но заново не создаём.
    if !widget.enabled {
        if let Some(window) =
            app.get_webview_window(&widget.window_label)
        {
            println!(
                "[widget] closing disabled: {}",
                widget.window_label
            );

            let _ = window.close();
        }

        return Ok(());
    }

    // Закрываем существующее окно.
    if let Some(window) =
        app.get_webview_window(&widget.window_label)
    {
        println!(
            "[widget] closing: {}",
            widget.window_label
        );

        let _ = window.close();

        // Даём WebView нормально завершиться
        // перед созданием нового.
        tokio::time::sleep(
            Duration::from_millis(100)
        ).await;
    }

    let url = format!(
        "widget://localhost/{}/index.html",
        widget.id
    );

    println!(
        "[widget] creating: {}",
        url
    );

    create_widget(
        app.clone(),
        widget.window_label,
        url,
        widget.x,
        widget.y,
        widget.width,
        widget.height,
        widget.interactive,
    )?;

    println!(
        "[widget] reload complete: {}",
        widget.id
    );

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct MonitorInfo {
    pub name: Option<String>,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
}

fn get_widget(app: &AppHandle, label: &str) -> Result<WebviewWindow, String> {
    app.get_webview_window(label)
        .ok_or_else(|| format!("Widget '{label}' not found"))
}

#[tauri::command]
pub fn create_widget(
    app: AppHandle,
    label: String,
    mut url: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    interactive: bool,
) -> Result<(), String> {
    if app.get_webview_window(&label).is_some() {
        return Err(format!("Widget '{label}' already exists"));
    }

    if !url.starts_with("widget://")
        && !url.starts_with("asset://")
        && !url.starts_with("http://")
        && !url.starts_with("https://")
        && !url.starts_with("file:///")
    {
        let normalized_path = url.replace('\\', "/");

        url = format!("widget://localhost/{}", normalized_path);
    }

    println!("[widget] opening URL: {url}");

    let parsed_url = url
        .parse::<url::Url>()
        .map_err(|e| format!("Invalid widget URL '{url}': {e}"))?;

    let webview_url = match parsed_url.scheme() {
        "http" | "https" | "file" => WebviewUrl::External(parsed_url),

        "asset" | "widget" => WebviewUrl::CustomProtocol(parsed_url),

        scheme => {
            return Err(format!("Unsupported widget URL scheme: {scheme}"));
        }
    };

    let window = WebviewWindowBuilder::new(&app, &label, webview_url)
        .title("")
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .closable(false)
        .skip_taskbar(true)
        .always_on_top(false)
        .inner_size(width, height)
        .position(x, y)
        .visible(true)
        .build()
        .map_err(|e| format!("Failed to create widget '{label}': {e}"))?;

    let _hwnd = window.hwnd().map_err(|e| e.to_string())?;

    window
        .set_ignore_cursor_events(!interactive)
        .map_err(|e| e.to_string())?;

    println!("[widget] window created: {label}");

    crate::desktop::attach_above_icons(&window)?;

    println!("[widget] attached to desktop: {label}");

    let app_clone = app.clone();
    let label_clone = label.clone();

    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Destroyed = event {
            if let Some(manager) = app_clone.try_state::<WidgetManager>() {
                manager.remove(&label_clone);
            }
        }
    });

    if let Some(manager) = app.try_state::<WidgetManager>() {
        manager.add(label.clone());
    }

    Ok(())
}

#[tauri::command]
pub fn close_widget(app: AppHandle, label: String) -> Result<(), String> {
    let window = get_widget(&app, &label)?;

    window
        .close()
        .map_err(|e| format!("Failed to close widget '{label}': {e}"))?;

    if let Some(manager) = app.try_state::<WidgetManager>() {
        manager.remove(&label);
    }

    println!("[widget] closed: {label}");
    Ok(())
}

#[tauri::command]
pub fn show_widget(app: AppHandle, label: String) -> Result<(), String> {
    let window = get_widget(&app, &label)?;
    window
        .show()
        .map_err(|e| format!("Failed to show widget '{label}': {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn hide_widget(app: AppHandle, label: String) -> Result<(), String> {
    let window = get_widget(&app, &label)?;
    window
        .hide()
        .map_err(|e| format!("Failed to hide widget '{label}': {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn move_widget(app: AppHandle, label: String, x: f64, y: f64) -> Result<(), String> {
    let window = get_widget(&app, &label)?;
    window
        .set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }))
        .map_err(|e| format!("Failed to move widget '{label}': {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn resize_widget(app: AppHandle, label: String, width: f64, height: f64) -> Result<(), String> {
    let window = get_widget(&app, &label)?;
    window
        .set_size(tauri::Size::Logical(tauri::LogicalSize { width, height }))
        .map_err(|e| format!("Failed to resize widget '{label}': {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_monitors(app: AppHandle) -> Result<Vec<MonitorInfo>, String> {
    let windows = app.webview_windows();
    let window = windows.values().next().ok_or("No windows available")?;
    let monitors = window
        .available_monitors()
        .map_err(|e| format!("Failed to get monitors: {e}"))?;

    Ok(monitors
        .into_iter()
        .map(|monitor| {
            let position = monitor.position();
            let size = monitor.size();
            MonitorInfo {
                name: monitor.name().cloned(),
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
                scale_factor: monitor.scale_factor(),
            }
        })
        .collect())
}

#[tauri::command]
pub async fn reload_widget(
    app: AppHandle,
    widget_id: String,
) -> Result<(), String> {

    println!(
        "[widget] requested reload: {}",
        widget_id
    );

    let manifest =
        crate::manifest::sync_manifest(&app)?;

    let widget = manifest
        .widgets
        .into_iter()
        .find(|widget| widget.id == widget_id)
        .ok_or_else(|| {
            format!(
                "Widget not found: {}",
                widget_id
            )
        })?;

    reload_widget_internal(
        &app,
        widget,
    ).await
}

#[tauri::command]
pub async fn reload_widgets(
    app: AppHandle,
) -> Result<(), String> {

    println!("[widgets] reloading...");

    let manifest =
        crate::manifest::sync_manifest(&app)?;

    println!(
        "[widgets] manifest reloaded, found {} widgets",
        manifest.widgets.len()
    );

    for widget in manifest.widgets {
        reload_widget_internal(
            &app,
            widget,
        ).await?;
    }

    println!("[widgets] reload complete");

    Ok(())
}