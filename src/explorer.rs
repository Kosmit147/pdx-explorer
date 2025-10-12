mod ui;

use crate::core::*;
use crate::database;
use eframe::egui;
use std::path::{Path, PathBuf};

enum CachedData {
    LocalizationKeys(Vec<database::models::LocalizationKeyKeyValue>),
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Explorer {
    #[serde(skip)]
    database: Option<database::Database>,
    #[serde(skip)]
    error: Option<Error>,

    #[serde(skip)]
    top_bar: ui::TopBar,
    #[serde(skip)]
    bottom_bar: ui::BottomBar,
    #[serde(skip)]
    file_browser: ui::FileBrowser,
    #[serde(skip)]
    language_select: ui::LanguageSelect,
    #[serde(skip)]
    data_browser: ui::DataBrowser,

    #[serde(skip)]
    selected_language: database::Language,

    #[serde(skip)]
    cached_data: Option<CachedData>,
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
            self.error
                .replace(error!("failed to obtain a path to the database file"));
            return;
        };

        let db = match database::Database::new(path, &db_path) {
            Ok(db) => db,
            Err(error) => {
                self.error.replace(error);
                return;
            }
        };

        self.database.replace(db);

        if let Err(error) = self.load_localization_keys() {
            self.error.replace(error);
        }
    }

    // This function shouldn't be called unless the database is opened.
    fn load_localization_keys(&mut self) -> Result<()> {
        let Some(db) = &mut self.database else {
            fail!("cannot load localization keys; database is not opened");
        };

        match db.get_localization_keys_for_language(self.selected_language) {
            Ok(keys) => {
                self.cached_data = Some(CachedData::LocalizationKeys(keys));
                Ok(())
            }
            Err(error) => {
                self.cached_data = None;
                Err(error)
            }
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
            let dir = self.database.as_ref().map(|db| db.dir_tree().root_path());
            let picked_dir = self.top_bar.ui(ui, dir);

            if let Some(path) = picked_dir {
                self.set_directory(&path);
            }
        });

        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            self.bottom_bar.ui(ui, self.error.as_ref());
        });

        egui::SidePanel::left("left panel").show(ctx, |ui| {
            let dir_tree = self.database.as_ref().map(|db| db.dir_tree());
            self.file_browser.ui(ui, dir_tree);
        });

        egui::SidePanel::right("right panel").show(ctx, |ui| {
            #[allow(clippy::collapsible_if)]
            if self.language_select.ui(ui, &mut self.selected_language) {
                if self.database.is_some()
                    && let Err(error) = self.load_localization_keys()
                {
                    self.error.replace(error);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.data_browser.ui(ui, self.cached_data.as_ref());
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
