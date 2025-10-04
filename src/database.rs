pub mod dir;
mod localization;
mod parser;

use crate::Error;
use std::path::Path;

pub use dir::DirTree;
pub use localization::Language;

#[derive(Debug)]
pub struct Database {
    connection: rusqlite::Connection,
    dir_tree: DirTree,
    // localization_database: LocalizationDatabase,
}

impl Database {
    const DATABASE_INIT_SCRIPT: &'static str = include_str!("sql/init.sql");

    pub fn new(base_path: &Path, database_file_path: &Path) -> Result<Self, Error> {
        let dir_tree = DirTree::new(base_path)?;

        let connection = rusqlite::Connection::open(database_file_path)?;
        connection.execute_batch(Self::DATABASE_INIT_SCRIPT)?;

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
}
