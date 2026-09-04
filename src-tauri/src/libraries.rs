use std::{
    fs,
    path::Path,
};

use tauri::{
    AppHandle,
    Manager,
};

pub fn ensure_libraries(
    app: &AppHandle,
    widgets_dir: &Path,
) -> Result<(), String> {
    let bundled_dir = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?
        .join("resources")
        .join("libraries");

    let target_dir = widgets_dir.join("_libraries");

    fs::create_dir_all(&target_dir)
        .map_err(|e| e.to_string())?;

    copy_directory(
        &bundled_dir,
        &target_dir,
    )?;

    Ok(())
}

fn copy_directory(
    source: &Path,
    target: &Path,
) -> Result<(), String> {
    if !source.exists() {
        return Err(format!(
            "Libraries directory not found: {}",
            source.display()
        ));
    }

    fs::create_dir_all(target)
        .map_err(|e| e.to_string())?;

    for entry in fs::read_dir(source)
        .map_err(|e| e.to_string())?
    {
        let entry = entry
            .map_err(|e| e.to_string())?;

        let source_path = entry.path();
        let target_path =
            target.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory(
                &source_path,
                &target_path,
            )?;
        } else {
            fs::copy(
                &source_path,
                &target_path,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}