use crate::Database;
use crate::Error;
use crate::database::Language;
use crate::dir_tree;
use dir_tree::DirTree;
use eframe::egui;
use std::path::PathBuf;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Explorer {
    #[serde(skip)]
    dir_tree: Option<DirTree>,
    #[serde(skip)]
    database_connection: Option<rusqlite::Connection>,
    #[serde(skip)]
    database: Option<Database>,
    #[serde(skip)]
    error: Option<Error>,

    test_string: String,
}

impl Explorer {
    pub const APP_TITLE: &'static str = "pdx-explorer";

    const DATABASE_FILE_NAME: &'static str = "db.sqlite3";
    const DATABASE_INIT_SCRIPT: &'static str = include_str!("sql/init.sql");

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Try to restore the app state from previous session.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn set_directory(&mut self, path: PathBuf) {
        let dt = match DirTree::new(&path) {
            Ok(dt) => dt,
            Err(error) => {
                self.error.replace(error);
                return;
            }
        };

        let db_connection = match Self::open_and_init_database() {
            Ok(conn) => conn,
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
        self.database_connection.replace(db_connection);
        self.database.replace(db);
    }

    fn database_file_path() -> Option<PathBuf> {
        Some(eframe::storage_dir(Self::APP_TITLE)?.join(Self::DATABASE_FILE_NAME))
    }

    fn open_and_init_database() -> Result<rusqlite::Connection, Error> {
        let path = Self::database_file_path().ok_or_else(|| {
            Error::new("Failed to obtain a path to the database file.".to_owned())
        })?;

        let connection = rusqlite::Connection::open(path)?;
        connection.execute_batch(Self::DATABASE_INIT_SCRIPT)?;
        Ok(connection)
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
            ui.text_edit_singleline(&mut self.test_string)
                .on_hover_text("The value in this field should persist.");

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
            let Some(database) = &self.database else {
                return;
            };

            let Some(localization_key_map) =
                database.localization_database().get(&selected_language)
            else {
                return;
            };

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
