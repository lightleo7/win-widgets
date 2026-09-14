#[cfg(target_os = "windows")]
use std::{
    sync::OnceLock,
    thread,
    time::Duration,
};

#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::{
            CreateWindowExW,
            DefWindowProcW,
            DispatchMessageW,
            GetMessageW,
            RegisterClassW,
            TranslateMessage,
            WNDCLASSW,
        },
    },
    core::w,
};

use tauri::Manager;

#[cfg(target_os = "windows")]
const WM_DISPLAYCHANGE: u32 = 0x007E;

#[cfg(target_os = "windows")]
const WM_POWERBROADCAST: u32 = 0x0218;

#[cfg(target_os = "windows")]
const PBT_APMRESUMEAUTOMATIC: u32 = 0x0012;

#[cfg(target_os = "windows")]
const PBT_APMRESUMESUSPEND: u32 = 0x0007;

#[cfg(target_os = "windows")]
static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

#[cfg(target_os = "windows")]
pub fn start_power_listener(app: tauri::AppHandle) {
    let _ = APP_HANDLE.set(app);

    thread::spawn(|| unsafe {
        let class_name = w!("WinWidgetsPowerListener");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(power_wnd_proc),
            lpszClassName: class_name,
            ..Default::default()
        };

        RegisterClassW(&wc);

        let hwnd = match CreateWindowExW(
            Default::default(),
            class_name,
            w!("Win Widgets Power Listener"),
            Default::default(),
            0,
            0,
            0,
            0,
            None,
            None,
            None,
            None,
        ) {
            Ok(hwnd) => hwnd,
            Err(_) => return,
        };

        if hwnd.0.is_null() {
            return;
        }

        let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();

        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn power_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_POWERBROADCAST {
        match wparam.0 as u32 {
            PBT_APMRESUMEAUTOMATIC | PBT_APMRESUMESUSPEND => {
                if let Some(app) = APP_HANDLE.get() {
                    let app = app.clone();

                    thread::spawn(move || {
                        thread::sleep(Duration::from_millis(150));
                        refresh_widget_windows(&app);
                    });
                }

                return LRESULT(1);
            }

            _ => {}
        }
    }
    if msg == WM_DISPLAYCHANGE {
        if let Some(app) = APP_HANDLE.get() {
            let app = app.clone();

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(50));
                refresh_widget_windows(&app);
            });
        }

        return LRESULT(0);
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}
#[cfg(target_os = "windows")]
pub fn refresh_widget_windows(app: &tauri::AppHandle) {
    let windows: Vec<_> = app
        .webview_windows()
        .values()
        .filter(|window| window.label() != "main")
        .cloned()
        .collect();

    thread::spawn(move || {
        let mut sizes = Vec::new();

        for window in &windows {
            if let Ok(size) = window.inner_size() {
                sizes.push((window.clone(), size));

                let _ = window.set_size(tauri::Size::Physical(
                    tauri::PhysicalSize {
                        width: size.width + 1,
                        height: size.height + 1,
                    },
                ));
            }
        }

        thread::sleep(Duration::from_millis(10));

        for (window, size) in sizes {
            let _ = window.set_size(tauri::Size::Physical(size));
        }
    });
}
