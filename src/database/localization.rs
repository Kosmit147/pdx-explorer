use crate::core::*;
use crate::database::Language;
use crate::database::parser::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type LocalizationKeyMap = HashMap<String, String>;

#[derive(Debug, Default)]
struct LocalizationMap {
    map: HashMap<Language, LocalizationKeyMap>,
}

impl LocalizationMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_localization_key_map(&mut self, language: Language) -> &mut LocalizationKeyMap {
        self.map.entry(language).or_default()
    }
}

#[derive(Debug, Default)]
pub struct LocalizationDatabase {
    localization_map: LocalizationMap,
}

impl LocalizationDatabase {
    pub fn new(localization_path: &Path) -> Result<Self> {
        let mut localization_map = LocalizationMap::new();
        Self::add_localization_keys_from_dir(&mut localization_map, localization_path)?;
        Ok(Self { localization_map })
    }

    fn add_localization_keys_from_dir(
        localization_map: &mut LocalizationMap,
        path: &Path,
    ) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let sub_path = entry.path();

            if sub_path.is_dir() {
                Self::add_localization_keys_from_dir(localization_map, &sub_path)?
            } else {
                let (language, keys) = Parser::parse_localization_file(&sub_path)?;
                let localization_key_map = localization_map.get_localization_key_map(language);

                for (key, value) in keys {
                    localization_key_map.insert(key, value);
                }
            }
        }

        Ok(())
    }
}
