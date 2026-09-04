use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use walkdir::WalkDir;

pub type WidgetSettings = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
    // ID виджета = имя папки.
    // Например:
    // win-widgets/clock/
    pub id: String,

    // Уникальное имя окна / экземпляра.
    pub window_label: String,

    pub enabled: bool,

    pub width: f64,
    pub height: f64,

    pub x: f64,
    pub y: f64,

    pub interactive: bool,

    // Сохранённые значения кастомных настроек.
    #[serde(default)]
    pub settings: WidgetSettings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WidgetInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,

    pub default_width: f64,
    pub default_height: f64,

    pub interactive: bool,

    // Описание доступных настроек из widget.json.
    #[serde(default)]
    pub settings: Vec<WidgetSettingDefinition>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetData {
    pub id: String,
    pub window_label: String,

    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,

    pub enabled: bool,

    pub width: f64,
    pub height: f64,

    pub x: f64,
    pub y: f64,

    pub interactive: bool,

    // Схема настроек из widget.json.
    pub setting_definitions: Vec<WidgetSettingDefinition>,

    // Реальные сохранённые значения.
    pub settings: WidgetSettings,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSettingOption {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSettingDefinition {
    pub key: String,

    pub label: String,

    #[serde(rename = "type")]
    pub setting_type: String,

    pub default: serde_json::Value,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub min: Option<f64>,

    #[serde(default)]
    pub max: Option<f64>,

    #[serde(default)]
    pub step: Option<f64>,

    #[serde(default)]
    pub options: Option<Vec<WidgetSettingOption>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Manifest {
    #[serde(default)]
    pub widgets: Vec<WidgetConfig>,
}

pub fn get_widgets_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .local_data_dir()
        .map(|p| p.join("win-widgets"))
        .map_err(|e| e.to_string())
}

fn read_widget_info(widget_dir: &Path) -> Result<WidgetInfo, String> {
    let info_path = widget_dir.join("widget.json");

    let content = fs::read_to_string(&info_path)
        .map_err(|e| format!("Failed to read {}: {}", info_path.display(), e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", info_path.display(), e))
}

#[tauri::command]
pub fn save_widget(app: AppHandle, widget: WidgetConfig) -> Result<(), String> {
    let base_dir = get_widgets_dir(&app)?;
    let manifest_path = base_dir.join("manifest.json");

    let content = fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;

    let mut manifest: Manifest = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let existing = manifest
        .widgets
        .iter_mut()
        .find(|item| item.window_label == widget.window_label)
        .ok_or_else(|| format!("Widget not found: {}", widget.window_label))?;

    *existing = widget;

    let content = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;

    fs::write(&manifest_path, content).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn sync_manifest(app: &AppHandle) -> Result<Manifest, String> {
    let base_dir = get_widgets_dir(app)?;

    if !base_dir.exists() {
        fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;
    }

    crate::libraries::ensure_libraries(
        app,
        &base_dir,
    )?;

    let manifest_path = base_dir.join("manifest.json");

    let mut manifest = if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?;

        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Manifest::default()
    };

    let mut found_ids = HashSet::new();

    for entry in WalkDir::new(&base_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let index_path = path.join("index.html");
        let info_path = path.join("widget.json");

        if !index_path.exists() || !info_path.exists() {
            continue;
        }

        let relative_path = path
            .strip_prefix(&base_dir)
            .map_err(|e| e.to_string())?;

        let widget_id = relative_path
            .to_string_lossy()
            .replace('\\', "/");

        found_ids.insert(widget_id.clone());

        let info = read_widget_info(path)?;

        if let Some(widget) = manifest
            .widgets
            .iter_mut()
            .find(|widget| widget.id == widget_id)
        {
            for setting in &info.settings {
                widget
                    .settings
                    .entry(setting.key.clone())
                    .or_insert_with(|| setting.default.clone());
            }

            continue;
        }

        let settings = info
            .settings
            .iter()
            .map(|setting| {
                (
                    setting.key.clone(),
                    setting.default.clone(),
                )
            })
            .collect();

        manifest.widgets.push(WidgetConfig {
            id: widget_id.clone(),
            window_label: widget_id,

            enabled: true,

            width: info.default_width,
            height: info.default_height,

            x: 40.0,
            y: 40.0,

            interactive: info.interactive,

            settings,
        });
    }

    manifest
        .widgets
        .retain(|widget| found_ids.contains(&widget.id));

    let serialized = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;

    fs::write(&manifest_path, serialized).map_err(|e| e.to_string())?;

    Ok(manifest)
}

pub fn save_manifest(
    app: &AppHandle,
    manifest: &Manifest,
) -> Result<(), String> {
    let base_dir = get_widgets_dir(app)?;
    let manifest_path = base_dir.join("manifest.json");

    let serialized =
        serde_json::to_string_pretty(manifest)
            .map_err(|e| e.to_string())?;

    fs::write(manifest_path, serialized)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn sync_and_get_manifest(
    app: AppHandle,
) -> Result<Vec<WidgetData>, String> {
    let manifest = sync_manifest(&app)?;
    let base_dir = get_widgets_dir(&app)?;
    let mut widgets = Vec::new();

    for widget in manifest.widgets {
        let widget_dir = base_dir.join(&widget.id);
        let info = read_widget_info(&widget_dir)?;

        widgets.push(WidgetData {
            id: widget.id,
            window_label: widget.window_label,
            name: info.name,
            version: info.version,
            author: info.author,
            description: info.description,
            enabled: widget.enabled,
            width: widget.width,
            height: widget.height,
            x: widget.x,
            y: widget.y,
            interactive: widget.interactive,
            setting_definitions: info.settings,
            settings: widget.settings,
        });
    }

    Ok(widgets)
}