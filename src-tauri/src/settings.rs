use std::collections::HashMap;

use serde_json::Value;
use tauri::AppHandle;

pub type WidgetSettings = HashMap<String, Value>;

#[tauri::command]
pub fn get_current_widget_settings(
    window: tauri::WebviewWindow,
    app: AppHandle,
) -> Result<WidgetSettings, String> {
    get_widget_settings(
        app,
        window.label().to_string(),
    )
}

pub fn get_widget_settings(
    app: AppHandle,
    window_label: String,
) -> Result<WidgetSettings, String> {
    let manifest =
        crate::manifest::sync_manifest(&app)?;

    let widget = manifest
        .widgets
        .iter()
        .find(|widget| {
            widget.window_label == window_label
        })
        .ok_or_else(|| {
            format!(
                "Widget '{}' not found",
                window_label
            )
        })?;

    Ok(widget.settings.clone())
}

#[tauri::command]
pub fn save_widget_settings(
    app: AppHandle,
    window_label: String,
    settings: WidgetSettings,
) -> Result<(), String> {
    let mut manifest =
        crate::manifest::sync_manifest(&app)?;

    let widget = manifest
        .widgets
        .iter_mut()
        .find(|widget| {
            widget.window_label == window_label
        })
        .ok_or_else(|| {
            format!(
                "Widget '{}' not found",
                window_label
            )
        })?;

    widget.settings = settings;

    crate::manifest::save_manifest(
        &app,
        &manifest,
    )
}