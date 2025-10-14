pub mod dir;
pub mod models;
mod schema;

use anyhow::{Context, Result, bail};
use diesel::connection::SimpleConnection;
use diesel::{
    BelongingToDsl, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use pdx_parse::parse;
use pdx_parse::{ContentType, Localization};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Path `{0}` contains invalid UTF-8")]
    PathContainsInvalidUtf8(PathBuf),
}

pub struct Database {
    connection: diesel::SqliteConnection,
    dir_tree: dir::Tree,
}

impl Database {
    const DATABASE_INIT_SCRIPT: &'static str = include_str!("database/sql/init.sql");

    pub fn new(base_path: &Path, database_file_path: &Path) -> Result<Self> {
        let dir_tree = dir::Tree::new(base_path).with_context(|| {
            format!(
                "While creating a directory tree with root path `{}`",
                base_path.display()
            )
        })?;

        let connection_context = || "While establishing a connection with the database";

        let mut connection = diesel::SqliteConnection::establish(
            Self::path_to_str(database_file_path).with_context(connection_context)?,
        )
        .with_context(connection_context)?;

        connection
            .batch_execute(Self::DATABASE_INIT_SCRIPT)
            .with_context(|| "While initializing the database")?;

        Self::insert_localizations(&mut connection)
            .with_context(|| "While inserting localizations")?;
        Self::insert_content_types(&mut connection)
            .with_context(|| "While inserting content types")?;
        Self::insert_dir_tree(&mut connection, &dir_tree)
            .with_context(|| "While inserting directory tree")?;

        Self::parse_localization(&mut connection).with_context(|| "While parsing localization")?;

        Ok(Self {
            connection,
            dir_tree,
        })
    }

    pub fn dir_tree(&self) -> &dir::Tree {
        &self.dir_tree
    }

    pub fn get_keys_for_localization(
        &mut self,
        localization: Localization,
    ) -> Result<Box<[models::LocalizationKeyKeyValue]>> {
        let localization = models::Localization {
            name: localization.name().to_owned(),
        };

        let keys = models::LocalizationKey::belonging_to(&localization)
            .select(models::LocalizationKeyKeyValue::as_select())
            .get_results(&mut self.connection)?;

        Ok(keys.into_boxed_slice())
    }

    fn insert_localizations(connection: &mut diesel::SqliteConnection) -> Result<()> {
        for value in Localization::values() {
            diesel::insert_into(schema::localization::table)
                .values(models::NewLocalization { name: value.name() })
                .execute(connection)?;
        }

        Ok(())
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
        dir_tree: &dir::Tree,
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

    fn parse_localization(connection: &mut diesel::SqliteConnection) -> Result<()> {
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

        for file_description in files {
            let mut file = fs::File::open(&file_description.full_path).with_context(|| {
                format!(
                    "While opening localization file `{}` for parsing",
                    &file_description.full_path
                )
            })?;

            let (localization, keys) =
                parse::parse_localization_file(&mut file).with_context(|| {
                    format!(
                        "While parsing localization file `{}`",
                        &file_description.full_path
                    )
                })?;

            for key in keys {
                let new_key = models::NewLocalizationKey {
                    key: &key.key,
                    value: &key.value,
                    file_id: file_description.id,
                    localization: localization.name(),
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
    ) -> Result<Box<[models::FileIdPath]>> {
        // Localization files are processed in reverse alphabetical order (from Z to A), adding
        // a or 0 at the beginning of the localization file will make sure it is applied last.

        let content_type = models::ContentType {
            name: ContentType::Localization.name().to_owned(),
        };

        let files = models::File::belonging_to(&content_type)
            .order_by(schema::file::file_name.desc())
            .select(models::FileIdPath::as_select())
            .get_results(connection)?;

        Ok(files.into_boxed_slice())
    }

    // Helper function which returns a Result instead of Option.
    fn path_to_str(path: &Path) -> Result<&str> {
        let Some(path) = path.to_str() else {
            bail!(DatabaseError::PathContainsInvalidUtf8(path.to_path_buf()))
        };

        Ok(path)
    }
}
