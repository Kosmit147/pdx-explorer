use crate::{Localization, LocalizationKey};
use anyhow::{Context, Result, bail, ensure};
use std::io::{self, BufRead, BufReader, Read};
use thiserror::Error;
use unicode_bom::Bom;

#[derive(Debug, Error)]
pub enum LocalizationParseError {
    #[error("Missing localization specifier")]
    MissingSpecifier,
    #[error("Localization specifier not terminated with a colon")]
    SpecifierNotTerminatedWithAColon,
    #[error("Unrecognized localization specifier")]
    UnrecognizedSpecifier,
    #[error("Localization file not encoded in UTF-8 with BOM format")]
    NotEncodedInUtf8Bom,
    #[error("Missing colon separating key and value in a localization file line")]
    MissingKeyValueColon,
    #[error("Missing opening quote before the value in a localization file line")]
    MissingOpeningQuote,
    #[error("Missing closing quote after the value in a localization file line")]
    MissingClosingQuote,
}

const LOCALIZATION_FILE_COMMENT_DELIMITER: char = '#';

pub fn parse_localization_file(file: impl Read) -> Result<(Localization, Box<[LocalizationKey]>)> {
    // This is how a localization file looks like:
    //
    // l_english:
    //  canal_suez:0 "Suez Canal"
    //  canal_panama:0 "Panama Canal"
    //
    // A localization file must be encoded in the UTF-8 with BOM format. Otherwise, the game
    // simply ignores the file.

    let mut reader = BufReader::new(file);
    ensure_utf8_bom(&mut reader)?;

    let mut lines = reader.lines().enumerate();
    let (line_number, localization_specifier_line) = get_localization_specifier_line(&mut lines)?;
    let localization = parse_localization_specifier_line(&localization_specifier_line)
        .with_context(|| line_context(line_number))?;

    let mut keys = Vec::new();

    for (line_number, line) in lines {
        let context = || line_context(line_number);
        let line = line.with_context(context)?;

        match filter_localization_file_line(line) {
            Some(line) => keys.push(parse_localization_key_line(&line).with_context(context)?),
            None => continue,
        };
    }

    Ok((localization, keys.into_boxed_slice()))
}

// Advances the iterator skipping all lines which are either empty or contain only comments.
// Returns an error if there are no non-empty lines found.
fn get_localization_specifier_line(
    lines: &mut impl Iterator<Item = (usize, io::Result<String>)>,
) -> Result<(usize, String)> {
    while let Some((line_number, line)) = lines.next() {
        let line = line.with_context(|| line_context(line_number))?;

        if let Some(line) = filter_localization_file_line(line) {
            return Ok((line_number, line));
        }
    }

    bail!(LocalizationParseError::MissingSpecifier)
}

// line must be trimmed.
fn parse_localization_specifier_line(line: &str) -> Result<Localization> {
    debug_assert!(str_is_trimmed(line));

    // Last character in the first line should be a colon.
    let localization_specifier = match line.char_indices().next_back() {
        Some((colon_idx, ':')) => &line[..colon_idx],
        Some(_) => bail!(LocalizationParseError::SpecifierNotTerminatedWithAColon),
        _ => bail!(LocalizationParseError::MissingSpecifier),
    };

    let Some(localization) = Localization::from_specifier(localization_specifier) else {
        bail!(LocalizationParseError::UnrecognizedSpecifier)
    };

    Ok(localization)
}

// line must be trimmed.
fn parse_localization_key_line(line: &str) -> Result<LocalizationKey> {
    debug_assert!(str_is_trimmed(line));

    // This is how a single line in the localization file looks like (notice that there's one
    // space at the beginning of the line; this is how these files are generally structured,
    // though it appears that in most games the files work correctly even if they don't have
    // this space). The number after the colon is optional and is used to keep track of
    // revisions.
    //
    //  canal_suez:0 "Suez Canal"

    let Some((before_colon, after_colon)) = line.split_once(':') else {
        bail!(LocalizationParseError::MissingKeyValueColon)
    };

    // Skip the revision number and whitespace at the beginning.
    let after_colon =
        after_colon.trim_start_matches(|c: char| c.is_ascii_digit() || c.is_whitespace());

    let mut after_colon_chars = after_colon.chars();

    ensure!(
        after_colon_chars.next() == Some('"'),
        LocalizationParseError::MissingOpeningQuote
    );

    ensure!(
        after_colon_chars.next_back() == Some('"'),
        LocalizationParseError::MissingClosingQuote
    );

    let key = before_colon.to_owned();
    let value = after_colon_chars.collect();

    Ok(LocalizationKey { key, value })
}

// Empty lines and lines which contain only comments are filtered out. Whitespace is trimmed.
// TODO: This shouldn't create a new String.
fn filter_localization_file_line(mut line: String) -> Option<String> {
    line = strip_localization_file_line_comment(line).trim().to_owned();
    if !line.is_empty() { Some(line) } else { None }
}

// TODO: This shouldn't create a new String.
fn strip_localization_file_line_comment(line: String) -> String {
    match line.split_once(LOCALIZATION_FILE_COMMENT_DELIMITER) {
        Some((before_comment, _comment)) => before_comment.to_owned(),
        None => line,
    }
}

fn str_is_trimmed(str: &str) -> bool {
    !str.starts_with(char::is_whitespace) && !str.ends_with(char::is_whitespace)
}

fn ensure_utf8_bom(reader: &mut impl BufRead) -> Result<()> {
    let mut bom_buffer: [u8; 3] = Default::default();

    // If we encounter an EOF while trying to read the first 3 bytes, we treat that as a "not
    // encoded in UTF-8 with BOM" error.
    if let Err(error) = reader.read_exact(&mut bom_buffer) {
        if let io::ErrorKind::UnexpectedEof = error.kind() {
            bail!(LocalizationParseError::NotEncodedInUtf8Bom)
        } else {
            bail!(error);
        }
    }

    match Bom::from(bom_buffer.as_slice()) {
        Bom::Utf8 => Ok(()),
        _ => bail!(LocalizationParseError::NotEncodedInUtf8Bom),
    }
}

fn line_context(line_number: usize) -> String {
    format!("While parsing line {line_number}")
}
