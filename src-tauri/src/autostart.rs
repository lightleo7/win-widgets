use std::process::Command;

use crate::logger::log;
use windows::{
    core::PCWSTR,
    Win32::{
        UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_HIDE},
    },
};

const TASK_NAME: &str = r"\WinWidgets";
const ENABLE_ARG: &str = "--winwidgets-enable-autostart";
const DISABLE_ARG: &str = "--winwidgets-disable-autostart";

pub fn maybe_run_service() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == ENABLE_ARG) {
        finish_helper(create_task());
    } else if args.iter().any(|arg| arg == DISABLE_ARG) {
        finish_helper(delete_task());
    }
}

fn finish_helper(result: Result<(), String>) {
    if let Err(error) = result {
        log("AUTOSTART", format!("Task Scheduler operation failed: {error}"));
        eprintln!("Task Scheduler operation failed: {error}");
        std::process::exit(1);
    }

    std::process::exit(0);
}

pub fn is_enabled() -> bool {
    Command::new("schtasks.exe")
        .args(["/Query", "/TN", TASK_NAME])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn set_enabled(enabled: bool) -> Result<(), String> {
    let helper_arg = if enabled { ENABLE_ARG } else { DISABLE_ARG };
    run_elevated(helper_arg)
}

fn run_elevated(argument: &str) -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("current_exe failed: {error}"))?;

    let exe_w = wide(exe.to_string_lossy().as_ref());
    let verb_w = wide("runas");
    let argument_w = wide(argument);

    unsafe {
        let result = ShellExecuteW(
            None,
            PCWSTR(verb_w.as_ptr()),
            PCWSTR(exe_w.as_ptr()),
            PCWSTR(argument_w.as_ptr()),
            PCWSTR::null(),
            SW_HIDE,
        );

        if result.0 as isize <= 32 {
            return Err(format!(
                "ShellExecuteW failed with code {}",
                result.0 as isize
            ));
        }
    }

    Ok(())
}

fn create_task() -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("current_exe failed: {error}"))?;

    let action = format!("\"{}\"", exe.to_string_lossy());

    run_schtasks(&[
        "/Create",
        "/TN",
        TASK_NAME,
        "/SC",
        "ONLOGON",
        "/TR",
        &action,
        "/RL",
        "HIGHEST",
        "/IT",
        "/F",
    ])?;

    log(
        "AUTOSTART",
        format!("Created scheduled task {TASK_NAME} -> {action}"),
    );
    Ok(())
}

fn delete_task() -> Result<(), String> {
    if !is_enabled() {
        log("AUTOSTART", format!("Scheduled task {TASK_NAME} does not exist"));
        return Ok(());
    }

    run_schtasks(&["/Delete", "/TN", TASK_NAME, "/F"])?;
    log("AUTOSTART", format!("Deleted scheduled task {TASK_NAME}"));
    Ok(())
}

fn run_schtasks(args: &[&str]) -> Result<(), String> {
    let output = Command::new("schtasks.exe")
        .args(args)
        .output()
        .map_err(|error| format!("failed to start schtasks.exe: {error}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

    Err(format!(
        "schtasks.exe {:?} failed ({}): {} {}",
        args,
        output.status,
        stdout,
        stderr
    )
    .trim()
    .to_owned())
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

#[tauri::command]
pub fn is_autostart_enabled() -> bool {
    is_enabled()
}

#[tauri::command]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    set_enabled(enabled)
}
