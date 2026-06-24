// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rustle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rustle.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

#[cfg(not(target_arch = "wasm32"))]
mod mock_storage {
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        static STORAGE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }

    pub struct LocalStorage;

    impl LocalStorage {
        pub fn set<T: serde::Serialize>(key: &str, value: T) -> Result<(), String> {
            let serialized = serde_json::to_string(&value).map_err(|e| e.to_string())?;
            STORAGE.with(|s| {
                s.borrow_mut().insert(key.to_string(), serialized);
            });
            Ok(())
        }

        pub fn get<T: serde::de::DeserializeOwned>(key: &str) -> Result<T, String> {
            STORAGE.with(|s| {
                if let Some(val) = s.borrow().get(key) {
                    serde_json::from_str(val).map_err(|e| e.to_string())
                } else {
                    Err("Key not found".to_string())
                }
            })
        }

        pub fn delete(key: &str) {
            STORAGE.with(|s| {
                s.borrow_mut().remove(key);
            });
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
use mock_storage::LocalStorage;

const GAME_STATE_KEY: &str = "gameState";
const ARCHIVE_GAME_STATE_KEY: &str = "archiveGameState";
const HIGH_CONTRAST_KEY: &str = "highContrast";
const GAME_STAT_KEY: &str = "gameStats";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StoredGameState {
    pub guesses: Vec<String>,
    pub solution: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameStats {
    #[serde(rename = "winDistribution")]
    pub win_distribution: Vec<i32>,
    #[serde(rename = "gamesFailed")]
    pub games_failed: i32,
    #[serde(rename = "currentStreak")]
    pub current_streak: i32,
    #[serde(rename = "bestStreak")]
    pub best_streak: i32,
    #[serde(rename = "totalGames")]
    pub total_games: i32,
    #[serde(rename = "successRate")]
    pub success_rate: i32,
}

pub fn save_game_state_to_local_storage(is_latest_game: bool, game_state: &StoredGameState) {
    let key = if is_latest_game {
        GAME_STATE_KEY
    } else {
        ARCHIVE_GAME_STATE_KEY
    };
    let _ = LocalStorage::set(key, game_state);
}

pub fn load_game_state_from_local_storage(is_latest_game: bool) -> Option<StoredGameState> {
    let key = if is_latest_game {
        GAME_STATE_KEY
    } else {
        ARCHIVE_GAME_STATE_KEY
    };
    LocalStorage::get(key).ok()
}

pub fn save_stats_to_local_storage(game_stats: &GameStats) {
    let _ = LocalStorage::set(GAME_STAT_KEY, game_stats);
}

pub fn load_stats_from_local_storage() -> Option<GameStats> {
    LocalStorage::get(GAME_STAT_KEY).ok()
}

const PREFERENCES_KEY: &str = "userPreferences";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StoredPreferences {
    pub theme: String,
    pub is_hard_mode: bool,
}

pub fn save_preferences_to_local_storage(prefs: &StoredPreferences) {
    let _ = LocalStorage::set(PREFERENCES_KEY, prefs);
    LocalStorage::delete(HIGH_CONTRAST_KEY);
    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item("theme", &prefs.theme);
            let _ = storage.set_item(
                "gameMode",
                if prefs.is_hard_mode { "hard" } else { "normal" },
            );
        }
    }
}

pub fn load_preferences_from_local_storage(prefers_dark: bool) -> StoredPreferences {
    #[derive(Deserialize)]
    struct LegacyPreferences {
        theme: Option<String>,
        is_dark_mode: Option<bool>,
        is_high_contrast: Option<bool>,
        is_hard_mode: Option<bool>,
        is_military_theme: Option<bool>,
    }

    if let Ok(legacy) = LocalStorage::get::<LegacyPreferences>(PREFERENCES_KEY) {
        let raw_theme = if let Some(t) = legacy.theme {
            t
        } else if legacy.is_military_theme.unwrap_or(false)
            || legacy.is_high_contrast.unwrap_or(false)
        {
            "nord".to_string()
        } else if legacy.is_dark_mode.unwrap_or(prefers_dark) {
            "dark".to_string()
        } else {
            "light".to_string()
        };
        let theme = match raw_theme.as_str() {
            "light" => "brinstar".to_string(),
            "dark" => "crateria".to_string(),
            "nord" => "maridia".to_string(),
            "dracula" => "wrecked_ship".to_string(),
            "sepia" => "norfair".to_string(),
            t => t.to_string(),
        };
        let is_hard_mode = legacy.is_hard_mode.unwrap_or(false);
        let mut prefs = StoredPreferences {
            theme,
            is_hard_mode,
        };
        migrate_holiday_theme(&mut prefs);
        return prefs;
    }

    #[allow(unused_mut)]
    let mut theme = if prefers_dark {
        "crateria".to_string()
    } else {
        "brinstar".to_string()
    };
    #[allow(unused_mut)]
    let mut is_hard_mode = false;
    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(val)) = storage.get_item("theme") {
                theme = match val.as_str() {
                    "light" => "brinstar".to_string(),
                    "dark" => "crateria".to_string(),
                    "nord" => "maridia".to_string(),
                    "dracula" => "wrecked_ship".to_string(),
                    "sepia" => "norfair".to_string(),
                    t => t.to_string(),
                };
            }
            if let Ok(Some(val)) = storage.get_item("gameMode") {
                is_hard_mode = val == "hard";
            }
        }
    }
    let mut prefs = StoredPreferences {
        theme,
        is_hard_mode,
    };

    migrate_holiday_theme(&mut prefs);
    prefs
}

fn migrate_holiday_theme(prefs: &mut StoredPreferences) {
    let date = crate::helpers::words::get_game_date();
    let holiday_info = crate::helpers::holidays::get_holiday_for_date(date);
    if let Some((prefix, _)) = holiday_info {
        if !crate::helpers::holidays::is_holiday_theme(&prefs.theme) {
            prefs.theme = prefix.to_string();
        }
    } else if crate::helpers::holidays::is_holiday_theme(&prefs.theme) {
        prefs.theme = "crateria".to_string();
    }
}
