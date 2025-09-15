use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
pub struct Explorer {
    path: Option<PathBuf>,
}

impl Explorer {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
                self.path.replace(new_game_path);
            }

            if let Some(path) = &self.path {
                ui.label(format!("Selected path: {}", path.display()));
            }
        });
    }
}
