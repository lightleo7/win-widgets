#[cfg(target_os = "windows")]
use windows::{
    core::s,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            FindWindowA,
            FindWindowExA,
            GetParent,
            GetWindowRect,
            SetParent,
            SetWindowPos,
            HWND_TOP,
            SWP_NOACTIVATE,
            SWP_NOSIZE,
        },
    },
};

#[cfg(target_os = "windows")]
pub fn attach_above_icons(
    window: &tauri::WebviewWindow,
    x: f64,
    y: f64,
) -> Result<(), String> {
    let hwnd = window.hwnd().map_err(|e| e.to_string())?;

    unsafe {
        let progman = FindWindowA(s!("Progman"), None)
            .map_err(|e| e.to_string())?;

        let mut defview =
            FindWindowExA(
                Some(progman),
                None,
                s!("SHELLDLL_DefView"),
                None,
            )
            .unwrap_or(HWND::default());

        if defview.is_invalid() {
            let mut worker = HWND::default();

            loop {
                worker = FindWindowExA(
                    None,
                    Some(worker),
                    s!("WorkerW"),
                    None,
                )
                .unwrap_or(HWND::default());

                if worker.is_invalid() {
                    break;
                }

                let candidate = FindWindowExA(
                    Some(worker),
                    None,
                    s!("SHELLDLL_DefView"),
                    None,
                )
                .unwrap_or(HWND::default());

                if !candidate.is_invalid() {
                    defview = candidate;
                    break;
                }
            }
        }

        if defview.is_invalid() {
            return Err("SHELLDLL_DefView not found".to_string());
        }

        println!("[desktop] SHELLDLL_DefView: {:?}", defview.0);

        let parent = GetParent(defview)
            .map_err(|e| e.to_string())?;

        if parent.is_invalid() {
            return Err("SHELLDLL_DefView parent not found".to_string());
        }

        println!("[desktop] DefView parent: {:?}", parent.0);

        let mut parent_rect = Default::default();

        GetWindowRect(parent, &mut parent_rect)
            .map_err(|e| e.to_string())?;

        let parent_x = x as i32 - parent_rect.left;
        let parent_y = y as i32 - parent_rect.top;

        SetParent(hwnd, Some(parent))
            .map_err(|e| e.to_string())?;

        SetWindowPos(
            hwnd,
            Some(HWND_TOP),
            parent_x,
            parent_y,
            0,
            0,
            SWP_NOSIZE | SWP_NOACTIVATE,
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}