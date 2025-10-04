use crate::Database;
use crate::Error;
use crate::database::dir;
use eframe::egui;
use std::path::{Path, PathBuf};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Explorer {
    #[serde(skip)]
    database: Option<Database>,
    #[serde(skip)]
    error: Option<Error>,

    persistent_string: String,
}

impl Explorer {
    pub const APP_ID: &'static str = "pdx-explorer";
    const DATABASE_FILE_NAME: &'static str = "db.sqlite3";

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Try to restore the app state from previous session.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn storage_dir() -> Option<PathBuf> {
        eframe::storage_dir(Self::APP_ID)
    }

    fn database_file_path() -> Option<PathBuf> {
        Some(Self::storage_dir()?.join(Self::DATABASE_FILE_NAME))
    }

    fn set_directory(&mut self, path: &Path) {
        let Some(db_path) = Self::database_file_path() else {
            self.error.replace(Error::new(
                "Failed to obtain a path to the database file.".to_owned(),
            ));
            return;
        };

        let db = match Database::new(path, &db_path) {
            Ok(db) => db,
            Err(error) => {
                self.error.replace(error);
                return;
            }
        };

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
                    self.set_directory(&path);
                }

                if let Some(db) = &self.database {
                    ui.label(format!(
                        "Selected path: {}",
                        db.dir_tree().root_path().display()
                    ));
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

    fn dir_tree(ui: &mut egui::Ui, node: &dir::Node) {
        match node {
            dir::Node::Directory {
                path,
                content_type,
                children,
                id,
            } => {
                egui::CollapsingHeader::new(format!(
                    "{} (ct: {}, id: {})",
                    path.file_name().unwrap_or_default().display(),
                    content_type,
                    id
                ))
                .show(ui, |ui| {
                    for child in children {
                        Self::dir_tree(ui, child);
                    }
                });
            }
            dir::Node::File {
                path,
                content_type,
                id,
            } => {
                ui.label(format!(
                    "{} (ct: {}, id: {})",
                    path.file_name().unwrap_or_default().display(),
                    content_type,
                    id
                ));
            }
        }
    }

    fn left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.persistent_string)
                .on_hover_text("The value in this field should persist.");

            if let Some(db) = &self.database {
                Self::dir_tree(ui, db.dir_tree().root());
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
        // let selected_language = Language::English;

        egui::CentralPanel::default().show(ctx, |_ui| {
            // let Some(database) = &self.database else {
            //     return;
            // };

            // let Some(localization_key_map) =
            //     database.localization_database().get(&selected_language)
            // else {
            //     return;
            // };

            // let available_height = ui.available_height();

            // egui_extras::TableBuilder::new(ui)
            //     .column(egui_extras::Column::auto())
            //     .column(egui_extras::Column::remainder())
            //     .striped(false)
            //     .resizable(true)
            //     .max_scroll_height(available_height)
            //     .header(20.0, |mut header| {
            //         header.col(|ui| {
            //             ui.strong("Key");
            //         });
            //         header.col(|ui| {
            //             ui.strong("Value");
            //         });
            //     })
            //     .body(|mut body| {
            //         for (key, value) in localization_key_map {
            //             body.row(20.0, |mut row| {
            //                 row.col(|ui| {
            //                     ui.label(key);
            //                 });
            //                 row.col(|ui| {
            //                     ui.label(value);
            //                 });
            //             });
            //         }
            //     });
        });
    }
}

impl eframe::App for Explorer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save the app state.
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
