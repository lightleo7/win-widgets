use std::{
    collections::HashMap,
    fs,
    io::{Cursor},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use zip::ZipArchive;


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCandidate {
    pub id: String,

    pub name: String,

    pub description: String,

    pub author: Option<String>,

    pub relative_path: String,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadedPack {
    pub token: String,

    pub author: String,

    pub pack: String,

    pub widgets: Vec<WidgetCandidate>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallWidgetsRequest {
    pub token: String,

    pub author: String,

    pub pack: String,

    pub widgets: Vec<String>,
}


#[derive(Debug, Deserialize)]
struct GithubRepoInfo {
    default_branch: String,
}

fn parse_github_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim().trim_end_matches('/');

    let prefix = "https://github.com/";

    let path = url
        .strip_prefix(prefix)
        .ok_or("Invalid GitHub repository URL")?;

    let mut parts = path.split('/');

    let owner = parts
        .next()
        .filter(|value| !value.is_empty())
        .ok_or("Missing GitHub owner")?;

    let repo = parts
        .next()
        .filter(|value| !value.is_empty())
        .ok_or("Missing GitHub repository name")?;

    Ok((
        owner.to_string(),
        repo.trim_end_matches(".git").to_string(),
    ))
}

fn is_widget_dir(path: &Path) -> bool {
    path.join("index.html").is_file()
        && path.join("widget.json").is_file()
}

fn find_widgets_recursive(
    root: &Path,
    current: &Path,
    author: &str,
    pack: &str,
    widgets: &mut Vec<WidgetCandidate>,
    paths: &mut HashMap<String, PathBuf>,
) -> Result<(), String> {

    let entries = fs::read_dir(current)
        .map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry
            .map_err(|e| e.to_string())?;

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if is_widget_dir(&path) {

            let widget_name = path
                .file_name()
                .ok_or("Invalid widget directory")?
                .to_string_lossy()
                .to_string();

            let widget_json_path =
                path.join("widget.json");

            let content = fs::read_to_string(
                &widget_json_path,
            )
            .map_err(|e| {
                format!(
                    "Failed to read {}: {}",
                    widget_json_path.display(),
                    e
                )
            })?;

            let json: serde_json::Value =
                serde_json::from_str(&content)
                    .map_err(|e| {
                        format!(
                            "Invalid widget.json in {}: {}",
                            path.display(),
                            e
                        )
                    })?;

            let name = json
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap_or(&widget_name)
                .to_string();

            let description = json
                .get("description")
                .and_then(|value| value.as_str())
                .unwrap_or("")
                .to_string();

            let author_name = json
                .get("author")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string());

            let relative = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?;

            let relative_path = relative
                .to_string_lossy()
                .replace('\\', "/");

            let id = format!(
                "{}/{}/{}",
                author,
                pack,
                widget_name
            );

            widgets.push(
                WidgetCandidate {
                    id: id.clone(),

                    name,

                    description,

                    author: author_name,

                    relative_path: relative_path.clone(),
                }
            );

            paths.insert(
                relative_path,
                path,
            );

            continue;
        }

        find_widgets_recursive(
            root,
            &path,
            author,
            pack,
            widgets,
            paths,
        )?;
    }

    Ok(())
}

fn get_installer_dir(
    app: &AppHandle,
) -> Result<PathBuf, String> {

    let base = app
        .path()
        .local_data_dir()
        .map_err(|e| e.to_string())?;

    let dir = base
        .join("win-widgets")
        .join(".installer");

    fs::create_dir_all(&dir)
        .map_err(|e| e.to_string())?;

    Ok(dir)
}

#[tauri::command]
pub async fn download_widget_pack(
    app: AppHandle,
    repository: String,
) -> Result<DownloadedPack, String> {

    let (author, pack) =
        parse_github_url(&repository)?;

    let client = reqwest::Client::new();

    let repo_info_url = format!(
        "https://api.github.com/repos/{}/{}",
        author,
        pack
    );

    let repo_info = client
        .get(&repo_info_url)
        .header(
            "User-Agent",
            "win-widgets",
        )
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<GithubRepoInfo>()
        .await
        .map_err(|e| e.to_string())?;

    let zip_url = format!(
        "https://codeload.github.com/{}/{}/zip/refs/heads/{}",
        author,
        pack,
        repo_info.default_branch
    );

    let bytes = client
        .get(&zip_url)
        .header(
            "User-Agent",
            "win-widgets",
        )
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let token = Uuid::new_v4()
        .to_string();

    let installer_dir =
        get_installer_dir(&app)?;

    let extract_dir =
        installer_dir.join(&token);

    fs::create_dir_all(&extract_dir)
        .map_err(|e| e.to_string())?;

    let reader =
        Cursor::new(bytes);

    let mut archive =
        ZipArchive::new(reader)
            .map_err(|e| e.to_string())?;

    for index in 0..archive.len() {

        let mut file = archive
            .by_index(index)
            .map_err(|e| e.to_string())?;

        let enclosed = match file.enclosed_name() {
            Some(path) => path.to_owned(),

            None => continue,
        };

        let output =
            extract_dir.join(enclosed);

        if file.is_dir() {

            fs::create_dir_all(&output)
                .map_err(|e| e.to_string())?;

            continue;
        }

        if let Some(parent) =
            output.parent()
        {
            fs::create_dir_all(parent)
                .map_err(|e| e.to_string())?;
        }

        let mut output_file =
            fs::File::create(&output)
                .map_err(|e| e.to_string())?;

        std::io::copy(
            &mut file,
            &mut output_file,
        )
        .map_err(|e| e.to_string())?;
    }

    let mut roots =
        fs::read_dir(&extract_dir)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().is_dir()
            });

    let repo_root = roots
        .next()
        .ok_or(
            "Repository archive is empty"
        )?
        .path();

    let mut widgets = Vec::new();

    let mut paths =
        HashMap::<String, PathBuf>::new();

    find_widgets_recursive(
        &repo_root,
        &repo_root,
        &author,
        &pack,
        &mut widgets,
        &mut paths,
    )?;

    if widgets.is_empty() {
        return Err(
            "No widgets found in repository"
                .to_string()
        );
    }

    Ok(
        DownloadedPack {
            token,

            author,

            pack,

            widgets,
        }
    )
}

fn copy_dir_recursive(
    source: &Path,
    destination: &Path,
) -> Result<(), String> {

    fs::create_dir_all(destination)
        .map_err(|e| e.to_string())?;

    for entry in fs::read_dir(source)
        .map_err(|e| e.to_string())?
    {
        let entry =
            entry.map_err(|e| e.to_string())?;

        let source_path =
            entry.path();

        let destination_path =
            destination.join(
                entry.file_name(),
            );

        if source_path.is_dir() {

            copy_dir_recursive(
                &source_path,
                &destination_path,
            )?;

        } else {

            fs::copy(
                &source_path,
                &destination_path,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn install_widgets_from_pack(
    app: AppHandle,
    request: InstallWidgetsRequest,
) -> Result<Vec<String>, String> {

    if request.widgets.is_empty() {
        return Err(
            "No widgets selected"
                .to_string()
        );
    }


    let installer_dir =
        get_installer_dir(&app)?;

    let extract_dir =
        installer_dir.join(&request.token);

    if !extract_dir.exists() {
        return Err(
            "Downloaded pack not found"
                .to_string()
        );
    }


    let mut roots =
        fs::read_dir(&extract_dir)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().is_dir()
            });

    let repo_root = roots
        .next()
        .ok_or(
            "Invalid downloaded pack"
        )?
        .path();

    let widgets_dir = app
        .path()
        .local_data_dir()
        .map_err(|e| e.to_string())?
        .join("win-widgets");


    let pack_dir = widgets_dir
        .join(&request.author)
        .join(&request.pack);

    fs::create_dir_all(&pack_dir)
        .map_err(|e| e.to_string())?;


    let mut installed = Vec::new();


    for relative_path in request.widgets {

        let relative =
            Path::new(&relative_path);

        if relative.is_absolute()
            || relative.components().any(
                |component| {
                    matches!(
                        component,
                        std::path::Component::ParentDir
                    )
                }
            )
        {
            continue;
        }


        let source =
            repo_root.join(relative);

        if !is_widget_dir(&source) {
            continue;
        }


        let widget_name = source
            .file_name()
            .ok_or(
                "Invalid widget directory"
            )?
            .to_string_lossy()
            .to_string();


        let destination =
            pack_dir.join(&widget_name);

        if destination.exists() {
            fs::remove_dir_all(
                &destination,
            )
            .map_err(|e| e.to_string())?;
        }


        copy_dir_recursive(
            &source,
            &destination,
        )?;


        installed.push(
            format!(
                "{}/{}/{}",
                request.author,
                request.pack,
                widget_name
            )
        );
    }

    fs::remove_dir_all(
        &extract_dir,
    )
    .map_err(|e| e.to_string())?;


    Ok(installed)
}

#[tauri::command]
pub async fn remove_widget(
    app: AppHandle,
    widget_id: String,
) -> Result<(), String> {
    println!("[widget] removing: {}", widget_id);

    if let Some(window) =
        app.get_webview_window(&widget_id)
    {
        println!(
            "[widget] closing: {}",
            widget_id
        );

        let _ = window.close();

        tokio::time::sleep(
            std::time::Duration::from_millis(100)
        )
        .await;
    }

    let base_dir =
        crate::manifest::get_widgets_dir(&app)?;

    if widget_id.is_empty()
        || widget_id.contains("..")
        || widget_id.contains('\\')
    {
        return Err("Invalid widget id".into());
    }

    let widget_dir =
        base_dir.join(&widget_id);

    if !widget_dir.is_dir() {
        return Err(format!(
            "Widget not found: {}",
            widget_id
        ));
    }

    println!(
        "[widget] deleting: {}",
        widget_dir.display()
    );

    std::fs::remove_dir_all(&widget_dir)
        .map_err(|e| e.to_string())?;

    println!(
        "[widget] removed: {}",
        widget_id
    );

    Ok(())
}