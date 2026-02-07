use serde::{Deserialize, Serialize};

/// Supported UI languages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Uk,
    En,
    Pl,
    De,
    Fr,
}

impl Language {
    /// Return the ISO language code.
    pub fn code(&self) -> &'static str {
        match self {
            Language::Uk => "uk",
            Language::En => "en",
            Language::Pl => "pl",
            Language::De => "de",
            Language::Fr => "fr",
        }
    }
}
