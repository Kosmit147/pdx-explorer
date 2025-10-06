pub mod dir;
mod localization;
mod parser;
mod schema;

use crate::core::*;
use diesel::connection::SimpleConnection;
use diesel::{Connection, ExpressionMethods, RunQueryDsl};
use dir::DirTree;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Localization,
    Unspecified,
}

impl ContentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Localization => "Localization",
            Self::Unspecified => "Unspecified",
        }
    }

    pub fn values() -> &'static [Self] {
        &[Self::Localization, Self::Unspecified]
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unspecified
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
    // localization_database: LocalizationDatabase,
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

        // let localization_database =
        //     LocalizationDatabase::new(&base_path.join("localization"))?;

        Ok(Self {
            connection,
            dir_tree,
            // localization_database,
        })
    }

    pub fn dir_tree(&self) -> &DirTree {
        &self.dir_tree
    }

    fn insert_content_types(connection: &mut diesel::SqliteConnection) -> Result<()> {
        for value in ContentType::values() {
            diesel::insert_into(schema::content_type::table)
                .values(schema::content_type::dsl::name.eq(value.name()))
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
            dir::Node::Directory {
                path,
                content_type,
                children,
                id,
            } => {
                diesel::insert_into(schema::directory::table)
                    .values((
                        schema::directory::id.eq(*id as i32),
                        schema::directory::path.eq(Self::path_to_str(path)?),
                        schema::directory::content_type.eq(content_type.name()),
                    ))
                    .execute(connection)?;

                for child in children {
                    Self::insert_node(connection, child)?;
                }
            }
            dir::Node::File {
                path,
                content_type,
                id,
            } => {
                diesel::insert_into(schema::file::table)
                    .values((
                        schema::file::id.eq(*id as i32),
                        schema::file::path.eq(Self::path_to_str(path)?),
                        schema::file::content_type.eq(content_type.name()),
                    ))
                    .execute(connection)?;
            }
        }

        Ok(())
    }

    // Helper function which returns a Result instead of Option.
    fn path_to_str(path: &Path) -> Result<&str> {
        path.to_str()
            .ok_or_else(|| error!("Path `{}` contains invalid UTF-8", path.display()))
    }
}
