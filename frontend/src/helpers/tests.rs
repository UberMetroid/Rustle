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

use crate::helpers::encryption::{decrypt, encrypt};
use crate::helpers::local_storage::{
    load_game_state_from_local_storage, load_preferences_from_local_storage,
    save_game_state_to_local_storage, save_preferences_to_local_storage, StoredGameState,
    StoredPreferences,
};
use crate::helpers::share::generate_emoji_grid;
use crate::helpers::stats::{add_stats_for_completed_game, default_stats};
use crate::helpers::statuses::{get_guess_statuses, get_statuses, CharStatus};
use crate::helpers::words::{
    find_first_unused_reveal, get_index, get_solution, get_word_of_day, is_winning_word,
};
use chrono::NaiveDate;

#[test]
fn test_guess_statuses_correct_and_present() {
    let statuses = get_guess_statuses("APPLE", "PEARL");
    assert_eq!(statuses[0], CharStatus::Present);
    assert_eq!(statuses[1], CharStatus::Present);
    assert_eq!(statuses[2], CharStatus::Present);
    assert_eq!(statuses[3], CharStatus::Absent);
    assert_eq!(statuses[4], CharStatus::Present);
}

#[test]
fn test_guess_statuses_exact_match() {
    let statuses = get_guess_statuses("APPLE", "APPLE");
    assert_eq!(statuses, vec![CharStatus::Correct; 5]);
}

#[test]
fn test_get_statuses_keyboard() {
    let guesses = vec!["PEARL".to_string(), "BOARD".to_string()];
    let char_statuses = get_statuses("APPLE", &guesses);

    assert_eq!(char_statuses.get(&'P'), Some(&CharStatus::Present));
    assert_eq!(char_statuses.get(&'E'), Some(&CharStatus::Present));
    assert_eq!(char_statuses.get(&'A'), Some(&CharStatus::Present));
    assert_eq!(char_statuses.get(&'R'), Some(&CharStatus::Absent));
    assert_eq!(char_statuses.get(&'B'), Some(&CharStatus::Absent));
}

#[test]
fn test_winning_word_case_insensitive() {
    assert!(is_winning_word("ApPlE", "aPpLe"));
    assert!(!is_winning_word("APPLE", "PEARL"));
}

#[test]
fn test_is_word_in_word_list() {
    assert!(crate::helpers::words::is_word_in_word_list("APPLE"));
    assert!(crate::helpers::words::is_word_in_word_list("apple"));
    assert!(!crate::helpers::words::is_word_in_word_list("RAT"));
}

#[test]
fn test_date_indices_and_solutions() {
    // 2026-07-10 is a non-holiday (Independence Day spread is Jul 3 - Jul 5).
    let test_date = NaiveDate::from_ymd_opt(2026, 7, 10).unwrap();
    let index = get_index(test_date);
    assert_eq!(index, 9);

    let word = get_word_of_day(9);
    assert_eq!(word.len(), 5);

    let sol = get_solution(test_date);
    assert_eq!(sol.solution, word);
    assert_eq!(sol.solution_index, 9);
}

#[test]
fn test_beta_indices_and_solutions() {
    // 2026-06-25 is a beta date (before 2026-07-01), index should be negative.
    let date_a = NaiveDate::from_ymd_opt(2026, 6, 25).unwrap();
    let date_b = NaiveDate::from_ymd_opt(2026, 6, 26).unwrap();

    let index_a = get_index(date_a);
    let index_b = get_index(date_b);

    assert!(index_a < 0);
    assert!(index_b < 0);
    assert_ne!(index_a, index_b);

    let word_a = get_word_of_day(index_a);
    let word_b = get_word_of_day(index_b);

    assert_eq!(word_a.len(), 5);
    assert_eq!(word_b.len(), 5);
    // Verify we get distinct words for distinct negative indices
    assert_ne!(word_a, word_b);
}

#[test]
fn test_hard_mode_guess_validation() {
    let guesses = vec!["WATER".to_string()];
    let unused = find_first_unused_reveal("WATES", &guesses, "WATER");
    assert!(unused.is_some());
    assert!(unused.unwrap().contains('R'));
}

#[test]
fn test_persistence_game_state_and_preferences() {
    let state = StoredGameState {
        guesses: vec!["APPLE".to_string(), "PEARL".to_string()],
        solution: "WATER".to_string(),
    };
    save_game_state_to_local_storage(true, &state);

    let loaded = load_game_state_from_local_storage(true);
    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap(), state);

    let prefs = StoredPreferences {
        theme: "crateria".to_string(),
        is_hard_mode: true,
    };
    save_preferences_to_local_storage(&prefs);

    let loaded_prefs = load_preferences_from_local_storage(false);
    assert_eq!(loaded_prefs, prefs);
}

#[test]
fn test_encryption_decryption() {
    let plain = "Hello Blowfish Encryption!";
    let enc = encrypt(plain);
    assert!(enc.is_ok());
    let dec = decrypt(&enc.unwrap());
    assert_eq!(dec, Some(plain.to_string()));
}

#[test]
fn test_game_stats_streaks() {
    let mut stats = default_stats();
    assert_eq!(stats.total_games, 0);
    assert_eq!(stats.current_streak, 0);

    stats = add_stats_for_completed_game(stats, 2);
    assert_eq!(stats.total_games, 1);
    assert_eq!(stats.current_streak, 1);
    assert_eq!(stats.best_streak, 1);
    assert_eq!(stats.win_distribution[2], 1);

    stats = add_stats_for_completed_game(stats, 6);
    assert_eq!(stats.total_games, 2);
    assert_eq!(stats.current_streak, 0);
    assert_eq!(stats.best_streak, 1);
}

#[test]
fn test_generate_emoji_grid() {
    let guesses = vec!["APPLE".to_string(), "WATER".to_string()];
    let tiles = ["🟩", "🟨", "⬛"];
    let grid = generate_emoji_grid("WATER", &guesses, &tiles);
    assert!(grid.contains("🟨⬛⬛⬛🟨"));
    assert!(grid.contains("🟩🟩🟩🟩🟩"));
}

#[test]
fn test_holiday_words_validity() {
    let prefixes = vec![
        "newyear",
        "valentine",
        "stpatrick",
        "easter",
        "independence",
        "halloween",
        "thanksgiving",
        "christmas",
    ];

    for prefix in prefixes {
        for idx in 0..100 {
            let word = crate::helpers::holidays::get_holiday_word(prefix, idx);
            assert!(
                crate::helpers::words::is_word_in_word_list(word),
                "Holiday word '{}' for prefix '{}' is not present in dictionary files",
                word,
                prefix
            );
        }
    }
}
