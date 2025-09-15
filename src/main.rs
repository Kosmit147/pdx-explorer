#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release.

use eframe::egui;

#[derive(Default)]
struct Explorer {}

impl Explorer {
    const TITLE: &'static str = "pdx-explorer";

    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Explorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Pdx Explorer");
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        Explorer::TITLE,
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Explorer::new(cc)))),
    )
}
