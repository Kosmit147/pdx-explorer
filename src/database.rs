pub mod dir;
mod localization;
mod parser;

use crate::core::*;
use rusqlite::params;
use std::path::Path;

pub use dir::{ContentType, DirTree};
pub use localization::Language;

#[derive(Debug)]
pub struct Database {
    connection: rusqlite::Connection,
    dir_tree: DirTree,
    // localization_database: LocalizationDatabase,
}

impl Database {
    const DATABASE_INIT_SCRIPT: &'static str = include_str!("sql/init.sql");

    pub fn new(base_path: &Path, database_file_path: &Path) -> Result<Self> {
        let dir_tree = DirTree::new(base_path)?;

        let connection = rusqlite::Connection::open(database_file_path)?;
        connection.execute_batch(Self::DATABASE_INIT_SCRIPT)?;

        Self::insert_content_types(&connection)?;
        Self::insert_dir_tree(&connection, &dir_tree)?;

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

    fn insert_content_types(connection: &rusqlite::Connection) -> Result<()> {
        let sql = "INSERT INTO content_type(type) VALUES(?1)";

        for value in ContentType::values() {
            connection.execute(sql, params![value.name()])?;
        }

        Ok(())
    }

    fn insert_dir_tree(connection: &rusqlite::Connection, dir_tree: &DirTree) -> Result<()> {
        Self::insert_node(connection, dir_tree.root())
    }

    fn insert_node(connection: &rusqlite::Connection, node: &dir::Node) -> Result<()> {
        match node {
            dir::Node::Directory {
                path,
                content_type,
                children,
                id,
            } => {
                let sql = "INSERT INTO directory(id, path, content_type) VALUES(?1, ?2, ?3)";

                connection.execute(
                    sql,
                    params![id, format!("{}", path.display()), content_type.name()],
                )?;

                for child in children {
                    Self::insert_node(connection, child)?;
                }
            }
            dir::Node::File {
                path,
                content_type,
                id,
            } => {
                let sql = "INSERT INTO file(id, path, content_type) VALUES(?1, ?2, ?3)";

                connection.execute(
                    sql,
                    params![id, format!("{}", path.display()), content_type.name()],
                )?;
            }
        }

        Ok(())
    }
}
