use std::{
    ffi::c_void,
    os::windows::ffi::OsStrExt,
    ptr::null_mut,
    sync::OnceLock,
    thread,
    sync::atomic::{AtomicIsize, Ordering},
    time::Duration,
};

use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{CloseHandle, GetLastError, HANDLE},
        Security::{
            DuplicateTokenEx, SecurityImpersonation, TokenPrimary, TOKEN_ALL_ACCESS,
        },
        System::{
            Environment::{CreateEnvironmentBlock, DestroyEnvironmentBlock},
            RemoteDesktop::{
                WTSGetActiveConsoleSessionId,
                WTSQueryUserToken,
            },
            Services::{
                CloseServiceHandle,
                ControlService,
                CreateServiceW,
                DeleteService,
                OpenSCManagerW,
                OpenServiceW,
                RegisterServiceCtrlHandlerExW,
                SetServiceStatus,
                StartServiceCtrlDispatcherW,
                StartServiceW,
                SERVICE_ALL_ACCESS,
                SERVICE_AUTO_START,
                SERVICE_CONTROL_SESSIONCHANGE,
                SERVICE_CONTROL_STOP,
                SERVICE_DEMAND_START,
                SERVICE_ERROR_NORMAL,
                SERVICE_RUNNING,
                SERVICE_START_PENDING,
                SERVICE_STATUS,
                SERVICE_STATUS_HANDLE,
                SERVICE_STOPPED,
                SERVICE_STOP_PENDING,
                SERVICE_TABLE_ENTRYW,
                SERVICE_WIN32_OWN_PROCESS,
                SC_MANAGER_CONNECT,
                SC_MANAGER_CREATE_SERVICE,
            },
            Threading::{
                CreateProcessAsUserW,
                CREATE_UNICODE_ENVIRONMENT,
                PROCESS_INFORMATION,
                STARTUPINFOW,
            },
        },
        UI::{
            Shell::ShellExecuteW,
            WindowsAndMessaging::SW_HIDE,
        },
    },
};

const SERVICE_NAME: &str = "WinWidgets";
const SERVICE_DISPLAY_NAME: &str = "Win Widgets";

const INSTALL_ARG: &str = "--winwidgets-install-service";
const REMOVE_ARG: &str = "--winwidgets-remove-service";
const SERVICE_ARG: &str = "--winwidgets-service";
static SERVICE_STATUS_HANDLE_GLOBAL: AtomicIsize = AtomicIsize::new(0);
const WTS_SESSION_LOGON: u32 = 0x00000005;

pub fn maybe_run_service() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == SERVICE_ARG) {
        run_service();
        std::process::exit(0);
    }

    if args.iter().any(|a| a == INSTALL_ARG) {
        let result = install_service();

        if let Err(e) = result {
            eprintln!("Install service failed: {e}");
        }

        std::process::exit(0);
    }

    if args.iter().any(|a| a == REMOVE_ARG) {
        let result = remove_service();

        if let Err(e) = result {
            eprintln!("Remove service failed: {e}");
        }

        std::process::exit(0);
    }
}

pub fn is_enabled() -> bool {
    unsafe {
        let manager = match OpenSCManagerW(
            None,
            None,
            SC_MANAGER_CONNECT,
        ) {
            Ok(handle) => handle,
            Err(_) => return false,
        };

        let name = wide(SERVICE_NAME);

        let service = OpenServiceW(
            manager,
            PCWSTR(name.as_ptr()),
            0x0004,
        );

        let result = service.is_ok();

        if let Ok(service) = service {
            let _ = CloseServiceHandle(service);
        }

        let _ = CloseServiceHandle(manager);

        result
    }
}

pub fn set_enabled(enabled: bool) -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if enabled {
        if args.iter().any(|a| a == INSTALL_ARG) {
            return install_service();
        }

        run_elevated(INSTALL_ARG)?;
    } else {
        if args.iter().any(|a| a == REMOVE_ARG) {
            return remove_service();
        }

        run_elevated(REMOVE_ARG)?;
    }

    Ok(())
}

fn run_elevated(argument: &str) -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("current_exe failed: {e}"))?;

    let exe_w = wide(exe.to_string_lossy().as_ref());
    let argument_w = wide(argument);

    unsafe {
        let result = ShellExecuteW(
            None,
            PCWSTR(wide("runas").as_ptr()),
            PCWSTR(exe_w.as_ptr()),
            PCWSTR(argument_w.as_ptr()),
            PCWSTR::null(),
            SW_HIDE,
        );

        if result.0 as isize <= 32 {
            return Err(format!(
                "ShellExecuteW failed: {}",
                result.0 as isize
            ));
        }
    }

    Ok(())
}

fn install_service() -> Result<(), String> {
    unsafe {
        let manager = OpenSCManagerW(
            None,
            None,
            SC_MANAGER_CONNECT | SC_MANAGER_CREATE_SERVICE,
        )
        .map_err(|e| format!("OpenSCManagerW failed: {e}"))?;

        let exe = std::env::current_exe()
            .map_err(|e| format!("current_exe failed: {e}"))?;

        let command = format!(
            "\"{}\" {}",
            exe.to_string_lossy(),
            SERVICE_ARG
        );

        let service_name = wide(SERVICE_NAME);
        let display_name = wide(SERVICE_DISPLAY_NAME);
        let command_w = wide(&command);

        let service = CreateServiceW(
            manager,
            PCWSTR(service_name.as_ptr()),
            PCWSTR(display_name.as_ptr()),
            SERVICE_ALL_ACCESS,
            SERVICE_WIN32_OWN_PROCESS,
            SERVICE_AUTO_START,
            SERVICE_ERROR_NORMAL,
            PCWSTR(command_w.as_ptr()),
            None,
            None,
            None,
            None,
            None,
        );

        let service = match service {
            Ok(service) => service,

            Err(e) => {
                let service = OpenServiceW(
                    manager,
                    PCWSTR(service_name.as_ptr()),
                    SERVICE_ALL_ACCESS,
                )
                .map_err(|_| {
                    format!("CreateServiceW failed: {e}")
                })?;

                service
            }
        };
        
        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(manager);
    }

    Ok(())
}

fn remove_service() -> Result<(), String> {
    unsafe {
        let manager = OpenSCManagerW(
            None,
            None,
            SC_MANAGER_CONNECT,
        )
        .map_err(|e| format!("OpenSCManagerW failed: {e}"))?;

        let service_name = wide(SERVICE_NAME);

        let service = OpenServiceW(
            manager,
            PCWSTR(service_name.as_ptr()),
            SERVICE_ALL_ACCESS,
        )
        .map_err(|e| format!("OpenServiceW failed: {e}"))?;

        let mut status = SERVICE_STATUS::default();

        let _ = ControlService(
            service,
            SERVICE_CONTROL_STOP,
            &mut status,
        );

        DeleteService(service)
            .map_err(|e| format!("DeleteService failed: {e}"))?;

        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(manager);
    }

    Ok(())
}

fn run_service() {
    unsafe {
        let service_name = wide(SERVICE_NAME);

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: PWSTR(service_name.as_ptr() as *mut u16),
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        let _ = StartServiceCtrlDispatcherW(table.as_ptr());
    }
}

unsafe extern "system" fn service_main(
    _argc: u32,
    _argv: *mut PWSTR,
) {
    let status_handle = match RegisterServiceCtrlHandlerExW(
        PCWSTR(wide(SERVICE_NAME).as_ptr()),
        Some(service_control_handler),
        None,
    ) {
        Ok(handle) => handle,
        Err(_) => return,
    };

    SERVICE_STATUS_HANDLE_GLOBAL.store(
        status_handle.0 as isize,
        Ordering::Relaxed,
    );

    let mut status = SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: SERVICE_START_PENDING,
        dwControlsAccepted: SERVICE_CONTROL_SESSIONCHANGE
            | SERVICE_CONTROL_STOP,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 3000,
    };

    let _ = SetServiceStatus(
        status_handle,
        &status,
    );

    status.dwCurrentState = SERVICE_RUNNING;
    status.dwWaitHint = 0;

    let _ = SetServiceStatus(
        status_handle,
        &status,
    );

    launch_for_active_session();

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

unsafe extern "system" fn service_control_handler(
    control: u32,
    event_type: u32,
    event_data: *mut c_void,
    _context: *mut c_void,
) -> u32 {
    match control {
        SERVICE_CONTROL_STOP => {
            let raw = SERVICE_STATUS_HANDLE_GLOBAL.load(Ordering::Relaxed);

            if raw != 0 {
                let handle = SERVICE_STATUS_HANDLE(raw as *mut c_void);

                let mut status = SERVICE_STATUS {
                    dwServiceType: SERVICE_WIN32_OWN_PROCESS,
                    dwCurrentState: SERVICE_STOP_PENDING,
                    dwControlsAccepted: 0,
                    dwWin32ExitCode: 0,
                    dwServiceSpecificExitCode: 0,
                    dwCheckPoint: 0,
                    dwWaitHint: 1000,
                };

                let _ = SetServiceStatus(
                    handle,
                    &status,
                );

                status.dwCurrentState = SERVICE_STOPPED;

                let _ = SetServiceStatus(
                    handle,
                    &status,
                );
            }

            std::process::exit(0);
        }

        SERVICE_CONTROL_SESSIONCHANGE => {
            if event_type == WTS_SESSION_LOGON {
                launch_for_active_session();
            }
        }

        _ => {}
    }

    0
}

fn launch_for_active_session() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(300));

        unsafe {
            let session_id = WTSGetActiveConsoleSessionId();

            if session_id == u32::MAX {
                return;
            }

            let mut impersonation_token = HANDLE::default();

            if WTSQueryUserToken(
                session_id,
                &mut impersonation_token,
            )
            .is_err()
            {
                return;
            }

            let mut primary_token = HANDLE::default();

            if DuplicateTokenEx(
                impersonation_token,
                TOKEN_ALL_ACCESS,
                None,
                SecurityImpersonation,
                TokenPrimary,
                &mut primary_token,
            )
            .is_err()
            {
                let _ = CloseHandle(impersonation_token);
                return;
            }

            let exe = match std::env::current_exe() {
                Ok(exe) => exe,
                Err(_) => {
                    let _ = CloseHandle(primary_token);
                    let _ = CloseHandle(impersonation_token);
                    return;
                }
            };

            let mut command_line: Vec<u16> = format!(
                "\"{}\"",
                exe.to_string_lossy()
            )
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

            let mut startup_info = STARTUPINFOW::default();
            startup_info.cb =
                std::mem::size_of::<STARTUPINFOW>() as u32;

            let mut process_info = PROCESS_INFORMATION::default();

            let mut environment = null_mut();

            let _ = CreateEnvironmentBlock(
                &mut environment,
                Some(primary_token),
                false,
            );

            let result = CreateProcessAsUserW(
                Some(primary_token),
                PCWSTR::null(),
                Some(PWSTR(command_line.as_mut_ptr())),
                None,
                None,
                false,
                CREATE_UNICODE_ENVIRONMENT,
                Some(environment),
                PCWSTR::null(),
                &startup_info,
                &mut process_info,
            );

            if !environment.is_null() {
                let _ = DestroyEnvironmentBlock(environment);
            }

            let _ = CloseHandle(primary_token);
            let _ = CloseHandle(impersonation_token);

            if result.is_ok() {
                let _ = CloseHandle(process_info.hProcess);
                let _ = CloseHandle(process_info.hThread);
            }
        }
    });
}

fn wide(value: &str) -> Vec<u16> {
    value
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

#[tauri::command]
pub fn is_autostart_enabled() -> bool {
    is_enabled()
}

#[tauri::command]
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    set_enabled(enabled)
}