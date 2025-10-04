use crate::core::*;
use crate::database::parser::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
