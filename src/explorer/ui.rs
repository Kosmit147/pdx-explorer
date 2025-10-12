use crate::core::*;
use crate::database::{self, dir};
use crate::explorer::CachedData;
use eframe::egui;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct TopBar {}

impl TopBar {
    #[must_use]
    pub fn ui(&mut self, ui: &mut egui::Ui, dir: Option<&Path>) -> Option<PathBuf> {
        let mut new_dir = None;

        ui.horizontal(|ui| {
            ui.label("Pdx Explorer");

            ui.separator();

            if ui.button("Open Game/Mod Directory").clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                new_dir = Some(path);
            }

            if let Some(dir) = dir {
                ui.label(format!("Selected path: {}", dir.display()));
            }
        });

        new_dir
    }
}

#[derive(Default)]
pub struct BottomBar {}

impl BottomBar {
    pub fn ui(&mut self, ui: &mut egui::Ui, error: Option<&Error>) {
        if let Some(error) = error {
            ui.colored_label(
                egui::Color32::YELLOW,
                format!("Error: {}.", error.description()),
            );
        }
    }
}

#[derive(Default)]
pub struct FileBrowser {}

impl FileBrowser {
    pub fn ui(&mut self, ui: &mut egui::Ui, dir_tree: Option<&dir::DirTree>) {
        if let Some(dir_tree) = dir_tree {
            Self::dir_tree(ui, dir_tree.root());
        }
    }

    fn dir_tree(ui: &mut egui::Ui, node: &dir::Node) {
        match node {
            dir::Node::Directory(dir) => {
                egui::CollapsingHeader::new(format!(
                    "{} (ct: {}, id: {})",
                    dir.dir_name().display(),
                    dir.content_type(),
                    dir.id()
                ))
                .show(ui, |ui| {
                    for child in dir.children() {
                        Self::dir_tree(ui, child);
                    }
                });
            }
            dir::Node::File(file) => {
                ui.label(format!(
                    "{} (ct: {}, id: {})",
                    file.file_name().display(),
                    file.content_type(),
                    file.id()
                ));
            }
        }
    }
}

#[derive(Default)]
pub struct LanguageSelect {}

impl LanguageSelect {
    pub fn ui(&mut self, ui: &mut egui::Ui, language: &mut database::Language) -> bool {
        let mut value_changed = false;

        for value in database::Language::values() {
            value_changed |= ui
                .selectable_value(language, *value, value.name())
                .changed();
        }

        value_changed
    }
}

#[derive(Default)]
pub struct DataBrowser {}

impl DataBrowser {
    pub fn ui(&mut self, ui: &mut egui::Ui, data: Option<&CachedData>) {
        match data {
            Some(CachedData::LocalizationKeys(keys)) => self.localization_keys(ui, keys),
            None => {}
        }
    }

    fn localization_keys(
        &mut self,
        ui: &mut egui::Ui,
        keys: &[database::models::LocalizationKeyKeyValue],
    ) {
        let available_height = ui.available_height();

        egui_extras::TableBuilder::new(ui)
            .column(egui_extras::Column::auto())
            .column(egui_extras::Column::auto())
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
                for key in keys {
                    let (key, value) = (&key.key, &key.value);

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
}
