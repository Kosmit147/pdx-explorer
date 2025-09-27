#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release.

use pdx_explorer::Explorer;

fn main() -> eframe::Result {
    eframe::run_native(
        Explorer::APP_TITLE,
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Explorer::new(cc)))),
    )
}
