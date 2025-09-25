use crate::Error;
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

#[derive(Default)]
pub struct Database {
    localization_database: HashMap<Language, LocalizationKeyMap>,
}

impl Database {
    const LOCALIZATION_PATH: &'static str = "localization";
    const LOCALIZATION_FILE_COMMENT_DELIMITER: char = '#';

    pub fn new(base_path: &Path) -> Result<Self, Error> {
        let mut db = Self::default();
        db.initialize(base_path)?;
        Ok(db)
    }

    pub fn localization_database(&self) -> &HashMap<Language, LocalizationKeyMap> {
        &self.localization_database
    }

    fn initialize(&mut self, base_path: &Path) -> Result<(), Error> {
        self.retrieve_localization_keys_from_dir(&base_path.join(Self::LOCALIZATION_PATH))
    }

    fn get_localization_key_map_for_language(&mut self, lang: Language) -> &mut LocalizationKeyMap {
        self.localization_database.entry(lang).or_default()
    }

    fn retrieve_localization_keys_from_dir(&mut self, path: &Path) -> Result<(), Error> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let sub_path = entry.path();

            if sub_path.is_dir() {
                self.retrieve_localization_keys_from_dir(&sub_path)?
            } else {
                self.retrieve_localization_keys_from_file(&sub_path)?
            }
        }

        Ok(())
    }

    // This is how a localization file looks like:
    // l_english:
    //  canal_suez:0 "Suez Canal"
    //  canal_panama:0 "Panama Canal"

    fn retrieve_localization_keys_from_file(&mut self, path: &Path) -> Result<(), Error> {
        let contents = fs::read_to_string(path)?;

        // Strip BOM (first three bytes).
        // todo: Should do this in a better, cleaner way.
        let contents = contents.get(3..);

        if contents.is_none() {
            return Ok(()); // Empty file, no BOM.
        }

        // Empty lines and lines which contain only comments are filtered out.
        let mut filtered_lines =
            contents
                .unwrap()
                .lines()
                .enumerate()
                .filter_map(|(line_number, line)| {
                    // Strip the comments.
                    let line = match line.split_once(Self::LOCALIZATION_FILE_COMMENT_DELIMITER) {
                        Some((before_comment, _comment)) => before_comment,
                        None => line,
                    };

                    let line = line.trim();

                    if line.is_empty() {
                        None
                    } else {
                        Some((line_number, line))
                    }
                });

        let language_specifier_line = filtered_lines.next();

        if language_specifier_line.is_none() {
            return Ok(()); // Empty file.
        }

        let (line_number, language_specifier_line) = language_specifier_line.unwrap();

        // Last character in the first line should be a colon.
        let language_specifier = match language_specifier_line.char_indices().next_back() {
            Some((colon_idx, ':')) => Ok(&language_specifier_line[..colon_idx]),
            _ => Err(Error::with_file_reference(
                path,
                line_number + 1,
                &format!(
                    "Failed to find the language specifier in line `{}`",
                    language_specifier_line
                ),
            )),
        }?;

        let language = Language::from_language_specifier(language_specifier).ok_or_else(|| {
            Error::with_file_reference(
                path,
                line_number + 1,
                &format!("Unrecognized language specifier: `{}`", language_specifier),
            )
        })?;

        let localization_key_map = self.get_localization_key_map_for_language(language);

        for (line_number, line) in filtered_lines {
            let (key, value) = Self::extract_localization_key_from_line(line_number, line, path)?;
            localization_key_map.insert(key, value);
        }

        Ok(())
    }

    fn extract_localization_key_from_line(
        line_number: usize,
        line: &str,
        path: &Path,
    ) -> Result<(String, String), Error> {
        let make_error = || {
            Error::with_file_reference(
                path,
                line_number + 1,
                &format!("Failed to parse localization key in line `{}`", line),
            )
        };

        // This is how a single line in the localization file looks like
        // (notice that there's one space at the beginning of the line;
        // this is how these files are generally structured, though it appears
        // that in most games the files work correctly even if they don't follow this convention).
        // The number after the colon is optional and is used to keep track of revisions.
        //
        //  canal_suez:0 "Suez Canal"

        if let Some((before_colon, after_colon)) = line.split_once(':') {
            // Skip the revision number and whitespace at the beginning.
            let after_colon =
                after_colon.trim_start_matches(|c: char| c.is_ascii_digit() || c.is_whitespace());

            // Make sure the first and the last character are quotes.
            let mut after_colon_chars = after_colon.chars();

            let should_be_first_quote = after_colon_chars.next().ok_or_else(make_error)?;
            let should_be_last_quote = after_colon_chars.next_back().ok_or_else(make_error)?;

            if should_be_first_quote != '"' || should_be_last_quote != '"' {
                return Err(make_error());
            }

            let key = before_colon.to_owned();
            let value: String = after_colon_chars.collect();
            Ok((key, value))
        } else {
            Err(make_error())
        }
    }
}
