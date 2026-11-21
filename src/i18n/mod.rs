mod about_updater;
mod audit_timeline;
mod core;
mod inspector;
mod notifications;
mod speed_router;
mod storage;
mod tooltips;
mod wifi_upnp;

use serde::{Deserialize, Serialize};
use std::fmt;

/// The 10 most spoken languages in the world (by total speaker count)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    Spanish,
    #[default]
    English,
    Chinese,
    Hindi,
    Arabic,
    French,
    Bengali,
    Portuguese,
    Russian,
    Urdu,
}

impl Language {
    pub const ALL: &[Language] = &[
        Language::Spanish,
        Language::English,
        Language::Chinese,
        Language::Hindi,
        Language::Arabic,
        Language::French,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
    ];

    pub fn native_name(&self) -> &'static str {
        match self {
            Language::Spanish => "Español",
            Language::English => "English",
            Language::Chinese => "中文 (Chino)",
            Language::Hindi => "हिन्दी (Hindi)",
            Language::Arabic => "العربية (Árabe)",
            Language::French => "Français (Francés)",
            Language::Bengali => "বাংলা (Bengalí)",
            Language::Portuguese => "Português",
            Language::Russian => "Русский (Ruso)",
            Language::Urdu => "اردو (Urdu)",
        }
    }

    #[allow(dead_code)]
    pub fn code(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Hindi => "hi",
            Language::Arabic => "ar",
            Language::French => "fr",
            Language::Bengali => "bn",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Urdu => "ur",
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.native_name())
    }
}

pub struct Translations;


pub(super) fn extract_target_prefix<'a>(original: &'a str, patterns: &[&str]) -> &'a str {
    for pat in patterns {
        if let Some(idx) = original.find(pat) {
            return original[..idx].trim();
        }
    }
    original.trim_end_matches('.').trim()
}
