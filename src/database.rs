pub mod dir;
mod localization;
mod parser;
mod schema;

use crate::core::*;
use diesel::connection::SimpleConnection;
use diesel::{Connection, ExpressionMethods, RunQueryDsl};
pub use dir::{ContentType, DirTree};
pub use localization::Language;
use std::path::Path;

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
            diesel::SqliteConnection::establish(database_file_path.to_str().ok_or_else(|| {
                error!(
                    "Database file path `{}` contains invalid UTF-8",
                    database_file_path.display()
                )
            })?)?;

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
                // TODO

                for child in children {
                    Self::insert_node(connection, child)?;
                }
            }
            dir::Node::File {
                path,
                content_type,
                id,
            } => {
                // TODO
            }
        }

        Ok(())
    }
}
