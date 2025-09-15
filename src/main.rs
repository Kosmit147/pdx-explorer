#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release.

use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
struct Explorer {
    game_path: Option<PathBuf>,
}

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

            if ui.button("Open Game Directory").clicked()
                && let Some(new_game_path) = rfd::FileDialog::new().pick_folder()
            {
                self.game_path.replace(new_game_path);
            }

            if let Some(game_path) = &self.game_path {
                ui.label(format!("Selected path: {}", game_path.display()));
            }
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
