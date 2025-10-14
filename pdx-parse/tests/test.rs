use anyhow::Result;
use pdx_parse::parse::{self, LocalizationParseError};
use pdx_parse::{Localization, LocalizationKey};
use std::fs;

macro_rules! localization_keys {
    ($($k:expr => $v:expr,)*) => {{
        [
            $(LocalizationKey {
                key: $k.to_owned(),
                value: $v.to_owned(),
            },)*
        ]
    }}
}

fn parse_localization_file_test(
    file_path: &str,
    expected_localization: Localization,
    expected_keys: &[LocalizationKey],
) -> Result<()> {
    let file = fs::File::open(file_path)?;
    let (result_localization, result_keys) = parse::parse_localization_file(file)?;
    assert_eq!(result_localization, expected_localization);
    assert_eq!(result_keys.as_ref(), expected_keys);
    Ok(())
}

#[test]
fn basic() -> Result<()> {
    let expected_keys = localization_keys! {
        "key_0" => "Key 0",
        "key_1" => "Key \"1\"",
    };

    parse_localization_file_test(
        "tests/input/basic_l_english.yml",
        Localization::English,
        &expected_keys,
    )?;

    Ok(())
}

#[test]
fn comments() -> Result<()> {
    let expected_keys = localization_keys! {
        "key_0" => "Key 0",
        "key_1" => "Key 1",
        "key_2" => "Key 2",
        "key_3" => "Key 3",
        "key_4" => "Key 4",
    };

    parse_localization_file_test(
        "tests/input/comments_l_english.yml",
        Localization::English,
        &expected_keys,
    )?;

    Ok(())
}

#[test]
fn empty() -> Result<()> {
    // Parsing an empty file should result in an error.
    let file = fs::File::open("tests/input/empty_l_english.yml")?;
    let error = parse::parse_localization_file(file).unwrap_err();
    let error = error.downcast::<LocalizationParseError>().unwrap();
    assert!(matches!(error, LocalizationParseError::MissingSpecifier));
    Ok(())
}

#[test]
fn empty_no_bom() -> Result<()> {
    // Parsing an empty file should result in an error.
    let file = fs::File::open("tests/input/empty_no_bom_l_english.yml")?;
    let error = parse::parse_localization_file(file).unwrap_err();
    let error = error.downcast::<LocalizationParseError>().unwrap();
    assert!(matches!(error, LocalizationParseError::NotEncodedInUtf8Bom));
    Ok(())
}

#[test]
fn whitespace() -> Result<()> {
    let expected_keys = localization_keys! {
        "key_0" => "Key 0",
        "key_1" => "Key 1",
        "key_2" => "Key 2",
        "key_3" => "Key 3",
    };

    parse_localization_file_test(
        "tests/input/whitespace_l_english.yml",
        Localization::English,
        &expected_keys,
    )?;

    Ok(())
}
