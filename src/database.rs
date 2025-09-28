mod localization;
mod parser;

use crate::Error;
use localization::LocalizationDatabase;
use std::path::Path;

pub use localization::Language;

#[derive(Debug, Default)]
pub struct Database {
    localization_database: LocalizationDatabase,
}

impl Database {
    const LOCALIZATION_PATH: &'static str = "localization";

    pub fn new(base_path: &Path) -> Result<Self, Error> {
        Ok(Self {
            localization_database: LocalizationDatabase::new(
                &base_path.join(Self::LOCALIZATION_PATH),
            )?,
        })
    }
}
