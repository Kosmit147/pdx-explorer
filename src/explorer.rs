use crate::Database;
use crate::Error;
use crate::database::Language;
use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
pub struct Explorer {
    path: Option<PathBuf>,
    database: Database,
    selected_language: Language,
    error: Option<Error>,
}

impl Explorer {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn update_path(&mut self, path: PathBuf) {
        match Database::new(&path) {
            Ok(db) => {
                self.path.replace(path);
                self.database = db;
            }
            Err(error) => {
                self.error.replace(error);
            }
        };
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.left_panel(ctx);
        self.right_panel(ctx);
        self.central_panel(ctx);
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Pdx Explorer");

                ui.separator();

                if ui.button("Open Game/Mod Directory").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_folder()
                {
                    self.update_path(path);
                }

                if let Some(path) = &self.path {
                    ui.label(format!("Selected path: {}", path.display()));
                }
            });
        });
    }

    fn bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::YELLOW, error.description());
            }
        });
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            for value in Language::values() {
                ui.selectable_value(&mut self.selected_language, *value, value.name());
            }
        });
    }

    fn right_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.label("TODO");
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let localization_database = self.database.localization_database();

            if let Some(localization_key_map) = localization_database.get(&self.selected_language) {
                let available_height = ui.available_height();

                egui_extras::TableBuilder::new(ui)
                    .column(egui_extras::Column::auto())
                    .column(egui_extras::Column::remainder())
                    .striped(false)
                    .resizable(true)
                    .max_scroll_height(available_height)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Key");
                        });
                        header.col(|ui| {
                            ui.strong("Value");
                        });
                    })
                    .body(|mut body| {
                        for (key, value) in localization_key_map {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(key);
                                });
                                row.col(|ui| {
                                    ui.label(value);
                                });
                            });
                        }
                    });
            }
        });
    }
}

impl eframe::App for Explorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
