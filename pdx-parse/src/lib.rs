pub mod parse;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContentType {
    Indeterminate,
    Localization,
}

impl ContentType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Indeterminate => "Indeterminate",
            Self::Localization => "Localization",
        }
    }

    pub fn values() -> &'static [Self] {
        &[Self::Indeterminate, Self::Localization]
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Indeterminate
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Localization {
    // These are sorted in alphabetical order.
    BrazilianPortuguese,
    English,
    French,
    German,
    Japanese,
    Korean,
    Polish,
    Russian,
    SimplifiedChinese,
    Spanish,
    Turkish,
}

impl Localization {
    pub fn from_specifier(specifier: &str) -> Option<Self> {
        match specifier {
            "l_braz_por" => Some(Self::BrazilianPortuguese),
            "l_english" => Some(Self::English),
            "l_french" => Some(Self::French),
            "l_german" => Some(Self::German),
            "l_japanese" => Some(Self::Japanese),
            "l_korean" => Some(Self::Korean),
            "l_polish" => Some(Self::Polish),
            "l_russian" => Some(Self::Russian),
            "l_simp_chinese" => Some(Self::SimplifiedChinese),
            "l_spanish" => Some(Self::Spanish),
            "l_turkish" => Some(Self::Turkish),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::BrazilianPortuguese => "Brazilian Portuguese",
            Self::English => "English",
            Self::French => "French",
            Self::German => "German",
            Self::Japanese => "Japanese",
            Self::Korean => "Korean",
            Self::Polish => "Polish",
            Self::Russian => "Russian",
            Self::SimplifiedChinese => "Simplified Chinese",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
        }
    }

    pub fn values() -> &'static [Self] {
        &[
            Self::BrazilianPortuguese,
            Self::English,
            Self::French,
            Self::German,
            Self::Japanese,
            Self::Korean,
            Self::Polish,
            Self::Russian,
            Self::SimplifiedChinese,
            Self::Spanish,
            Self::Turkish,
        ]
    }
}

impl Default for Localization {
    fn default() -> Self {
        Self::English
    }
}

impl fmt::Display for Localization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalizationKey {
    pub key: String,
    pub value: String,
}
