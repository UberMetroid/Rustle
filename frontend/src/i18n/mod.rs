// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

mod de;
mod en;
mod es;
mod fr;
mod ja;
mod pt;
mod ru;
mod zh;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    English,
    Chinese,
    Spanish,
    German,
    Japanese,
    French,
    Portuguese,
    Russian,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Chinese => "zh",
            Self::Spanish => "es",
            Self::German => "de",
            Self::Japanese => "ja",
            Self::French => "fr",
            Self::Portuguese => "pt",
            Self::Russian => "ru",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "简体中文",
            Self::Spanish => "Español",
            Self::German => "Deutsch",
            Self::Japanese => "日本語",
            Self::French => "Français",
            Self::Portuguese => "Português",
            Self::Russian => "Русский",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "zh" => Self::Chinese,
            "es" => Self::Spanish,
            "de" => Self::German,
            "ja" => Self::Japanese,
            "fr" => Self::French,
            "pt" => Self::Portuguese,
            "ru" => Self::Russian,
            _ => Self::English,
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::English,
            Self::Chinese,
            Self::Spanish,
            Self::German,
            Self::Japanese,
            Self::French,
            Self::Portuguese,
            Self::Russian,
        ]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Translations {
    pub game_title: &'static str,
    pub win_messages: &'static [&'static str],
    pub game_copied: &'static str,
    pub not_enough_letters: &'static str,
    pub word_not_found: &'static str,
    pub hard_mode_alert: &'static str,
    pub enter: &'static str,
    pub delete: &'static str,
    pub statistics: &'static str,
    pub guess_distribution: &'static str,
    pub new_word: &'static str,
    pub share: &'static str,
    pub share_failure: &'static str,
    pub transfer: &'static str,
    pub transfer_desc: &'static str,
    pub total_tries: &'static str,
    pub success_rate: &'static str,
    pub current_streak: &'static str,
    pub best_streak: &'static str,
    pub discourage_browser: &'static str,
    pub datepicker_title: &'static str,
    pub datepicker_choose: &'static str,
    pub logout: &'static str,
}

pub fn get_translations(lang: Language) -> Translations {
    match lang {
        Language::Chinese => zh::translations(),
        Language::Spanish => es::translations(),
        Language::German => de::translations(),
        Language::Japanese => ja::translations(),
        Language::French => fr::translations(),
        Language::Portuguese => pt::translations(),
        Language::Russian => ru::translations(),
        _ => en::translations(),
    }
}

// Self-contained storage helper
pub struct StorageService;
impl StorageService {
    pub fn get_item(_key: &str) -> String {
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                return storage.get_item(_key).ok().flatten().unwrap_or_default();
            }
        }
        String::new()
    }

    pub fn set_item(_key: &str, _value: &str) {
        #[cfg(target_arch = "wasm32")]
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.set_item(_key, _value);
            }
        }
    }
}

pub fn get_saved_language() -> Language {
    let stored = StorageService::get_item("language");
    if !stored.is_empty() {
        Language::from_code(&stored)
    } else {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window() {
            if let Some(nav) = window.navigator().language() {
                let nav = nav.to_lowercase();
                if nav.starts_with("zh") {
                    return Language::Chinese;
                } else if nav.starts_with("es") {
                    return Language::Spanish;
                } else if nav.starts_with("de") {
                    return Language::German;
                } else if nav.starts_with("ja") {
                    return Language::Japanese;
                } else if nav.starts_with("fr") {
                    return Language::French;
                } else if nav.starts_with("pt") {
                    return Language::Portuguese;
                } else if nav.starts_with("ru") {
                    return Language::Russian;
                }
            }
        }
        Language::English
    }
}

pub fn save_language(lang: Language) {
    StorageService::set_item("language", lang.code());
}

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub language: Language,
    pub translations: Translations,
}

pub fn get_correct_word_message(lang: Language, solution: &str) -> String {
    match lang {
        Language::Spanish => format!("La palabra era {}", solution),
        Language::Chinese => format!("单词是 {}", solution),
        Language::German => format!("Das Wort war {}", solution),
        Language::Japanese => format!("正解は {} でした", solution),
        Language::French => format!("Le mot était {}", solution),
        Language::Portuguese => format!("A palavra era {}", solution),
        Language::Russian => format!("Слово было {}", solution),
        _ => format!("The word was {}", solution),
    }
}
