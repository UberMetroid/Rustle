// Copyright (C) 2026 Jeryd
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
    pub is_dark_mode: bool,
    pub is_high_contrast: bool,
    pub is_hard_mode: bool,
    #[serde(default)]
    pub is_military_theme: bool,
}

pub fn save_preferences_to_local_storage(prefs: &StoredPreferences) {
    let _ = LocalStorage::set(PREFERENCES_KEY, prefs);
    if prefs.is_high_contrast {
        let _ = LocalStorage::set(HIGH_CONTRAST_KEY, "1");
    } else {
        LocalStorage::delete(HIGH_CONTRAST_KEY);
    }
    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item("theme", if prefs.is_dark_mode { "dark" } else { "light" });
            let _ = storage.set_item(
                "gameMode",
                if prefs.is_hard_mode { "hard" } else { "normal" },
            );
        }
    }
}

pub fn load_preferences_from_local_storage(prefers_dark: bool) -> StoredPreferences {
    if let Ok(prefs) = LocalStorage::get::<StoredPreferences>(PREFERENCES_KEY) {
        return prefs;
    }

    let is_high_contrast = get_stored_is_high_contrast_mode();
    #[allow(unused_mut)]
    let mut is_dark_mode = prefers_dark;
    #[allow(unused_mut)]
    let mut is_hard_mode = false;
    #[allow(unused_mut)]
    let mut is_military_theme = false;
    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(val)) = storage.get_item("theme") {
                is_dark_mode = val == "dark";
            }
            if let Ok(Some(val)) = storage.get_item("gameMode") {
                is_hard_mode = val == "hard";
            }
        }
    }
    StoredPreferences {
        is_dark_mode,
        is_high_contrast,
        is_hard_mode,
        is_military_theme,
    }
}

pub fn get_stored_is_high_contrast_mode() -> bool {
    let val: Result<String, _> = LocalStorage::get(HIGH_CONTRAST_KEY);
    val.map(|v| v == "1").unwrap_or(false)
}
