use crate::Database;
use crate::Error;
use crate::database::Language;
use crate::dir_tree;
use dir_tree::DirTree;
use eframe::egui;
use std::path::PathBuf;

#[derive(Default)]
pub struct Explorer {
    dir_tree: Option<DirTree>,
    database: Option<Database>,
    error: Option<Error>,
}

impl Explorer {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn set_directory(&mut self, path: PathBuf) {
        let dt = match DirTree::new(&path) {
            Ok(dt) => dt,
            Err(error) => {
                self.error.replace(error);
                return;
            }
        };

        let db = match Database::new(&path) {
            Ok(db) => db,
            Err(error) => {
                self.error.replace(error);
                return;
            }
        };

        self.dir_tree.replace(dt);
        self.database.replace(db);
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
                    self.set_directory(path);
                }

                if let Some(dt) = &self.dir_tree {
                    ui.label(format!("Selected path: {}", dt.root_path().display()));
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

    fn dir_tree(ui: &mut egui::Ui, node: &dir_tree::Node) {
        match node {
            dir_tree::Node::Directory(dir) => {
                egui::CollapsingHeader::new(format!(
                    "{}",
                    dir.path().file_name().unwrap_or_default().display()
                ))
                .show(ui, |ui| {
                    for child in dir.children() {
                        Self::dir_tree(ui, child);
                    }
                });
            }
            dir_tree::Node::File(file) => {
                ui.label(format!(
                    "{}",
                    file.path().file_name().unwrap_or_default().display()
                ));
            }
        }
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            if let Some(dt) = &self.dir_tree {
                Self::dir_tree(ui, dt.root());
            }
        });
    }

    fn right_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.label("TODO");
        });
    }

    fn central_panel(&mut self, ctx: &egui::Context) {
        // todo: Language selection.
        let selected_language = Language::English;

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.database.is_none() {
                return;
            }

            let localization_database = self.database.as_ref().unwrap().localization_database();

            if let Some(localization_key_map) = localization_database.get(&selected_language) {
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
