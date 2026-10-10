// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logger;
fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        logger::log("PANIC", format!("{}", panic_info));
    }));

    win_widgets::run()
}
