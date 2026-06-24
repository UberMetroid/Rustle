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

use crate::constants::config::{not_contained_message, wrong_spot_message};
use crate::constants::word_db;
use crate::helpers::statuses::get_guess_statuses;
use chrono::{Duration, Local, NaiveDate};

pub fn first_game_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 7, 1).unwrap_or_default()
}

pub fn get_today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn is_word_in_word_list(word: &str) -> bool {
    word_db::is_word_in_word_list(word)
}

pub fn is_winning_word(word: &str, solution: &str) -> bool {
    solution.eq_ignore_ascii_case(word)
}

pub fn get_index(game_date: NaiveDate) -> i32 {
    let duration = game_date.signed_duration_since(first_game_date());
    duration.num_days() as i32
}

pub fn get_word_of_day(index: i32) -> &'static str {
    word_db::get_word_of_day(index)
}

pub struct SolutionInfo {
    pub solution: &'static str,
    pub solution_index: i32,
    pub tomorrow: i64, // timestamp in ms
}

pub fn get_solution(game_date: NaiveDate) -> SolutionInfo {
    let index = get_index(game_date);
    let solution =
        if let Some((prefix, _)) = crate::helpers::holidays::get_holiday_for_date(game_date) {
            crate::helpers::holidays::get_holiday_word(prefix, index)
        } else {
            get_word_of_day(index)
        };
    let tomorrow_date = game_date + Duration::days(1);
    let tomorrow_dt = tomorrow_date.and_hms_opt(0, 0, 0).unwrap_or_default();
    let tomorrow_ms = tomorrow_dt.and_utc().timestamp_millis();

    SolutionInfo {
        solution,
        solution_index: index,
        tomorrow: tomorrow_ms,
    }
}

pub fn find_first_unused_reveal(word: &str, guesses: &[String], solution: &str) -> Option<String> {
    if guesses.is_empty() {
        return None;
    }

    let guess = &guesses[guesses.len() - 1];
    let statuses = get_guess_statuses(solution, guess);
    let split_word: Vec<char> = word.chars().collect();
    let split_guess: Vec<char> = guess.chars().collect();

    let mut letters_left_array = Vec::new();

    for i in 0..split_guess.len() {
        if statuses[i] == crate::helpers::statuses::CharStatus::Correct
            || statuses[i] == crate::helpers::statuses::CharStatus::Present
        {
            letters_left_array.push(split_guess[i]);
        }
        if statuses[i] == crate::helpers::statuses::CharStatus::Correct
            && split_word[i] != split_guess[i]
        {
            return Some(wrong_spot_message(&split_guess[i].to_string(), i + 1));
        }
    }

    for letter in &split_word {
        if let Some(pos) = letters_left_array.iter().position(|l| l == letter) {
            letters_left_array.remove(pos);
        }
    }

    if !letters_left_array.is_empty() {
        return Some(not_contained_message(&letters_left_array[0].to_string()));
    }

    None
}
pub fn get_game_date() -> NaiveDate {
    // If we have ENABLE_ARCHIVED_GAMES, we can read '?d=' query parameter.
    // However, since it is false by default, we just return today.
    // Let's implement query param parsing in case.
    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        if let Ok(search) = win.location().search() {
            if let Some(date_str) = search.strip_prefix("?d=") {
                if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    let today = get_today();
                    if parsed_date <= today && parsed_date >= first_game_date() {
                        return parsed_date;
                    }
                }
            }
        }
    }
    get_today()
}

#[allow(unused_variables)]
pub fn set_game_date(d: NaiveDate) {
    #[cfg(target_arch = "wasm32")]
    if let Some(win) = web_sys::window() {
        let today = get_today();
        if d < today {
            let redirect_url = format!("/?d={}", d.format("%Y-%m-%d"));
            let _ = win.location().set_href(&redirect_url);
        } else {
            let _ = win.location().set_href("/");
        }
    }
}
