use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
};
use chrono::Local;
const MAX_LOG_SIZE: u64 = 5 * 1024 * 1024;

fn log_path() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    base.join("win-widgets")
        .join("logs")
        .join("win-widgets.log")
}

pub fn log(category: &str, message: impl AsRef<str>) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("[{}] [{}] {}", timestamp, category, message.as_ref());

    println!("{}", line);

    let path = log_path();

    if let Some(parent) = path.parent() {
        let _ = create_dir_all(parent);
    }

    if let Ok(metadata) = std::fs::metadata(&path) {
        if metadata.len() > MAX_LOG_SIZE {
            let _ = std::fs::remove_file(&path);
        }
    }

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(file, "{}", line);
    }
}