use crate::Error;
use crate::database::Language;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Parser {}

impl Parser {
    const LOCALIZATION_FILE_COMMENT_DELIMITER: char = '#';

    pub fn parse_localization_file(
        path: &Path,
    ) -> Result<(Language, Vec<(String, String)>), Error> {
        // This is how a localization file looks like:
        //
        // l_english:
        //  canal_suez:0 "Suez Canal"
        //  canal_panama:0 "Panama Canal"
        //
        // A localization file must be encoded in the UTF-8 with BOM format.

        let file_content = fs::read_to_string(path)?;

        // Strip BOM (first three bytes).
        // todo: Should do this in a better, cleaner way.
        let Some(file_content) = file_content.get(3..) else {
            return Ok(Default::default()); // Empty file, no BOM.
        };

        // Empty lines and lines which contain only comments are filtered out.
        let mut filtered_lines =
            file_content
                .lines()
                .enumerate()
                .filter_map(|(line_number, line)| {
                    // Strip the comments.
                    let line = match line.split_once(Self::LOCALIZATION_FILE_COMMENT_DELIMITER) {
                        Some((before_comment, _comment)) => before_comment,
                        None => line,
                    }
                    .trim();

                    if !line.is_empty() {
                        Some((line_number, line))
                    } else {
                        None
                    }
                });

        let Some((line_number, language_specifier_line)) = filtered_lines.next() else {
            return Ok(Default::default()); // Empty file.
        };

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

        let mut keys = Vec::new();

        for (line_number, line) in filtered_lines {
            keys.push(Self::parse_localization_file_line(line_number, line, path)?);
        }

        Ok((language, keys))
    }

    fn parse_localization_file_line(
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

        let Some((before_colon, after_colon)) = line.split_once(':') else {
            return Err(make_error());
        };

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
    }
}
