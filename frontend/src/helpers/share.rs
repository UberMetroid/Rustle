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

//! Share status formatter utilities.
//! Prepares emoji grids representing game performance for export or system clipboard copy.

use crate::constants::config::{GAME_TITLE, MAX_CHALLENGES};
use crate::helpers::statuses::get_guess_statuses;
use web_sys::window;

/// Formats the final game performance as a text block and exports it via the navigator share API or clipboard.
#[allow(clippy::too_many_arguments)]
pub fn share_status(
    solution: &str,
    guesses: &[String],
    lost: bool,
    is_hard_mode: bool,
    theme: &str,
    solution_index: i32,
    handle_share_to_clipboard: impl FnOnce(),
    handle_share_failure: impl FnOnce(),
) {
    let guess_count = if lost {
        "X".to_string()
    } else {
        guesses.len().to_string()
    };
    let hard_indicator = if is_hard_mode { "*" } else { "" };

    let emoji_grid = generate_emoji_grid(solution, guesses, &get_emoji_tiles(theme));

    let puzzle_num_str = if solution_index < 0 {
        format!("Beta #{}", solution_index.abs())
    } else {
        format!("#{}", solution_index + 1)
    };

    let text_to_share = format!(
        "{} {} {}/{}{}\n\n{}",
        GAME_TITLE, puzzle_num_str, guess_count, MAX_CHALLENGES, hard_indicator, emoji_grid
    );

    let mut shared = false;
    if let Some(win) = window() {
        let nav = win.navigator();
        let share_key = wasm_bindgen::JsValue::from_str("share");
        if js_sys::Reflect::has(&nav, &share_key).unwrap_or(false) {
            if let Ok(share_fn) = js_sys::Reflect::get(&nav, &share_key) {
                if share_fn.is_function() {
                    let share_data = js_sys::Object::new();
                    let _ = js_sys::Reflect::set(
                        &share_data,
                        &"text".into(),
                        &wasm_bindgen::JsValue::from_str(&text_to_share),
                    );

                    let args = js_sys::Array::of1(&share_data);
                    if js_sys::Reflect::apply(&share_fn.into(), &nav, &args).is_ok() {
                        shared = true;
                    }
                }
            }
        }
    }

    if !shared {
        if let Some(win) = window() {
            let clipboard = win.navigator().clipboard();
            let _ = clipboard.write_text(&text_to_share);
            handle_share_to_clipboard();
            return;
        }
        handle_share_failure();
    }
}

/// Creates a horizontal/vertical grid of emojis reflecting character accuracy.
pub fn generate_emoji_grid(solution: &str, guesses: &[String], tiles: &[&str; 3]) -> String {
    let mut grid = String::with_capacity(guesses.len() * 25);
    for (i, guess) in guesses.iter().enumerate() {
        if i > 0 {
            grid.push('\n');
        }
        let statuses = get_guess_statuses(solution, guess);
        for status in statuses {
            let tile = match status {
                crate::helpers::statuses::CharStatus::Correct => tiles[0],
                crate::helpers::statuses::CharStatus::Present => tiles[1],
                crate::helpers::statuses::CharStatus::Absent => tiles[2],
            };
            grid.push_str(tile);
        }
    }
    grid
}

/// Helper returning the target emoji tiles depending on selected theme display parameters.
fn get_emoji_tiles(theme: &str) -> [&'static str; 3] {
    match theme {
        "brinstar" => ["🟩", "🟨", "⬛"],
        "norfair" => ["🟥", "🟧", "⬛"],
        "wrecked_ship" => ["🟦", "🟪", "⬛"],
        "maridia" => ["🟦", "🟨", "⬛"],
        "tourian" => ["🟩", "🟥", "⬛"],
        _ => ["🟦", "🟪", "⬛"], // crateria
    }
}
