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

//! Configuration parameters and localized strings.
//! Consolidates application rules, alerts timing, cryptographic keys, and UI text elements.
// Game parameters
pub const MAX_CHALLENGES: usize = 6;
pub const ALERT_TIME_MS: u32 = 2000;
pub const LONG_ALERT_TIME_MS: u32 = 10000;
pub const REVEAL_TIME_MS: u32 = 350;
pub const WELCOME_INFO_MODAL_MS: u32 = 350;
pub const DISCOURAGE_INAPP_BROWSERS: bool = true;
pub const ENABLE_MIGRATE_STATS: bool = true;
pub const BLOWFISH_KEY: &str = "xcQUAHsik#Thq&LG*8es2DsZ$3bw^e";
pub const ENABLE_ARCHIVED_GAMES: bool = false;
pub const GAME_TITLE: &str = "Rustle";

// Localized strings
pub const WIN_MESSAGES: &[&str] = &["Great Job!", "Awesome", "Well done!"];
pub const GAME_COPIED_MESSAGE: &str = "Game copied to clipboard";
pub const NOT_ENOUGH_LETTERS_MESSAGE: &str = "Not enough letters";
pub const WORD_NOT_FOUND_MESSAGE: &str = "Word not found";
pub const HARD_MODE_ALERT_MESSAGE: &str = "Hard Mode can only be enabled at the start!";
pub const ENTER_TEXT: &str = "Enter";
pub const DELETE_TEXT: &str = "Delete";
pub const STATISTICS_TITLE: &str = "Statistics";
pub const GUESS_DISTRIBUTION_TEXT: &str = "Guess Distribution";
pub const NEW_WORD_TEXT: &str = "New word in";
pub const SHARE_TEXT: &str = "Share";
pub const SHARE_FAILURE_TEXT: &str = "Unable to share the results. This feature is available only in secure contexts (HTTPS), in some or all supporting browsers.";
pub const MIGRATE_BUTTON_TEXT: &str = "Transfer";
pub const MIGRATE_DESCRIPTION_TEXT: &str =
    "Click here to transfer your statistics to a new device.";
pub const TOTAL_TRIES_TEXT: &str = "Total tries";
pub const SUCCESS_RATE_TEXT: &str = "Success rate";
pub const CURRENT_STREAK_TEXT: &str = "Current streak";
pub const BEST_STREAK_TEXT: &str = "Best streak";
pub const DISCOURAGE_INAPP_BROWSER_TEXT: &str = "You are using an embedded browser and may experience problems sharing or saving your results. We encourage you rather to use your device's default browser.";
pub const DATEPICKER_TITLE: &str = "Choose a past date";
pub const DATEPICKER_CHOOSE_TEXT: &str = "Choose";

pub fn correct_word_message(solution: &str) -> String {
    format!("The word was {}", solution)
}

pub fn wrong_spot_message(guess: &str, position: usize) -> String {
    format!("Must use {} in position {}", guess, position)
}

pub fn not_contained_message(letter: &str) -> String {
    format!("Guess must contain {}", letter)
}
