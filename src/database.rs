pub mod dir;
mod models;
mod parser;
mod schema;

use crate::core::*;
use diesel::connection::SimpleConnection;
use diesel::{
    BelongingToDsl, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use dir::DirTree;
use parser::Parser;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Localization,
    Indeterminate,
}

impl ContentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Localization => "Localization",
            Self::Indeterminate => "Indeterminate",
        }
    }

    pub fn values() -> &'static [Self] {
        &[Self::Localization, Self::Indeterminate]
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Indeterminate
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Language {
    English,
    BrazilianPortuguese,
    French,
    German,
    Polish,
    Russian,
    Spanish,
    Japanese,
    SimplifiedChinese,
    Korean,
    Turkish,
}

impl Language {
    pub fn from_language_specifier(specifier: &str) -> Option<Self> {
        match specifier {
            "l_english" => Some(Self::English),
            "l_braz_por" => Some(Self::BrazilianPortuguese),
            "l_french" => Some(Self::French),
            "l_german" => Some(Self::German),
            "l_polish" => Some(Self::Polish),
            "l_russian" => Some(Self::Russian),
            "l_spanish" => Some(Self::Spanish),
            "l_japanese" => Some(Self::Japanese),
            "l_simp_chinese" => Some(Self::SimplifiedChinese),
            "l_korean" => Some(Self::Korean),
            "l_turkish" => Some(Self::Turkish),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::BrazilianPortuguese => "Brazilian Portuguese",
            Self::French => "French",
            Self::German => "German",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Japanese => "Japanese",
            Self::SimplifiedChinese => "Simplified Chinese",
            Self::Korean => "Korean",
            Self::Turkish => "Turkish",
        }
    }

    pub fn values() -> &'static [Self] {
        &[
            Self::English,
            Self::BrazilianPortuguese,
            Self::French,
            Self::German,
            Self::Polish,
            Self::Russian,
            Self::Spanish,
            Self::Japanese,
            Self::SimplifiedChinese,
            Self::Korean,
            Self::Turkish,
        ]
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub struct Database {
    connection: diesel::SqliteConnection,
    dir_tree: DirTree,
}

impl Database {
    const DATABASE_INIT_SCRIPT: &'static str = include_str!("database/sql/init.sql");

    pub fn new(base_path: &Path, database_file_path: &Path) -> Result<Self> {
        let dir_tree = DirTree::new(base_path)?;

        let mut connection =
            diesel::SqliteConnection::establish(Self::path_to_str(database_file_path)?)?;

        connection.batch_execute(Self::DATABASE_INIT_SCRIPT)?;

        Self::insert_content_types(&mut connection)?;
        Self::insert_dir_tree(&mut connection, &dir_tree)?;
        Self::parse_and_insert_localization_keys(&mut connection)?;

        Ok(Self {
            connection,
            dir_tree,
        })
    }

    pub fn dir_tree(&self) -> &DirTree {
        &self.dir_tree
    }

    fn insert_content_types(connection: &mut diesel::SqliteConnection) -> Result<()> {
        for value in ContentType::values() {
            diesel::insert_into(schema::content_type::table)
                .values(models::NewContentType { name: value.name() })
                .execute(connection)?;
        }

        Ok(())
    }

    fn insert_dir_tree(
        connection: &mut diesel::SqliteConnection,
        dir_tree: &DirTree,
    ) -> Result<()> {
        Self::insert_node(connection, dir_tree.root())
    }

    fn insert_node(connection: &mut diesel::SqliteConnection, node: &dir::Node) -> Result<()> {
        match node {
            dir::Node::Directory(dir) => {
                diesel::insert_into(schema::directory::table)
                    .values(models::NewDirectory {
                        id: dir.id() as i32,
                        full_path: Self::path_to_str(dir.full_path())?,
                        relative_path: Self::path_to_str(dir.relative_path())?,
                        dir_name: Self::path_to_str(dir.dir_name())?,
                        content_type: dir.content_type().name(),
                    })
                    .execute(connection)?;

                for child in dir.children() {
                    Self::insert_node(connection, child)?;
                }
            }
            dir::Node::File(file) => {
                diesel::insert_into(schema::file::table)
                    .values(models::NewFile {
                        id: file.id() as i32,
                        full_path: Self::path_to_str(file.full_path())?,
                        relative_path: Self::path_to_str(file.relative_path())?,
                        file_name: Self::path_to_str(file.file_name())?,
                        content_type: file.content_type().name(),
                    })
                    .execute(connection)?;
            }
        }

        Ok(())
    }

    fn parse_and_insert_localization_keys(connection: &mut diesel::SqliteConnection) -> Result<()> {
        // The rules for parsing localization keys are as follows:
        //
        // 1. Localization files must be .yml format encoded in UTF-8-BOM. Otherwise, the game
        // ignores the file.
        // 2. The filename must end with _l_<language>.
        // 3. Localization files are processed in reverse alphabetical order (from Z to A), adding
        // a or 0 at the beginning of the localization file will make sure it is applied last.
        // 4. File within a 'replace' folder work slightly differently as the localization keys
        // within are checked specifically and overwrite any other identical localization keys.

        let files = Self::select_localization_files_for_parsing(connection)?;

        for file in files {
            let path = PathBuf::from(file.full_path);

            // TODO: Sort entries per language in the database.
            let (_language, keys) = Parser::parse_localization_file(&path)?;

            for (key, value) in keys {
                let new_key = models::NewLocalizationKey {
                    key: &key,
                    value: &value,
                    file_id: file.id,
                };

                diesel::insert_into(schema::localization_key::table)
                    .values(&new_key)
                    .on_conflict(schema::localization_key::key)
                    .do_update()
                    .set(&new_key)
                    .execute(connection)?;
            }
        }

        Ok(())
    }

    fn select_localization_files_for_parsing(
        connection: &mut diesel::SqliteConnection,
    ) -> Result<Vec<models::FileIdAndPath>> {
        // Localization files are processed in reverse alphabetical order (from Z to A), adding
        // a or 0 at the beginning of the localization file will make sure it is applied last.

        let content_type = models::ContentType {
            name: ContentType::Localization.name().to_owned(),
        };

        let files = models::File::belonging_to(&content_type)
            .order_by(schema::file::file_name.desc())
            .select(models::FileIdAndPath::as_select())
            .get_results(connection)?;

        Ok(files)
    }

    // Helper function which returns a Result instead of Option.
    fn path_to_str(path: &Path) -> Result<&str> {
        path.to_str()
            .ok_or_else(|| error!("path `{}` contains invalid UTF-8", path.display()))
    }
}
