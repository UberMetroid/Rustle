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

//! Evaluates characters and guesses.
//! Provides mapping logic to determine if characters are correct, present, or absent.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CharStatus {
    Absent,
    Present,
    Correct,
}

/// Accumulates guess statuses across all turns to determine current keyboard keys styling.
pub fn get_statuses(solution: &str, guesses: &[String]) -> HashMap<char, CharStatus> {
    let mut char_obj = HashMap::new();
    let mut sol_chars = ['\0'; 10];
    let mut sol_len = 0;
    for (i, c) in solution.chars().enumerate() {
        if i < 10 {
            sol_chars[i] = c;
            sol_len += 1;
        }
    }

    for word in guesses {
        let mut word_chars = ['\0'; 10];
        let mut word_len = 0;
        for (i, c) in word.chars().enumerate() {
            if i < 10 {
                word_chars[i] = c;
                word_len += 1;
            }
        }

        for (i, &letter) in word_chars[..word_len].iter().enumerate() {
            if !sol_chars[..sol_len].contains(&letter) {
                char_obj.insert(letter, CharStatus::Absent);
                continue;
            }

            if i < sol_len && letter == sol_chars[i] {
                char_obj.insert(letter, CharStatus::Correct);
                continue;
            }

            if char_obj.get(&letter) != Some(&CharStatus::Correct) {
                char_obj.insert(letter, CharStatus::Present);
            }
        }
    }

    char_obj
}

/// Calculates character correct/present/absent evaluation states for a single guess.
pub fn get_guess_statuses(solution: &str, guess: &str) -> Vec<CharStatus> {
    let mut sol_chars = ['\0'; 10];
    let mut sol_len = 0;
    for (i, c) in solution.chars().enumerate() {
        if i < 10 {
            sol_chars[i] = c;
            sol_len += 1;
        }
    }

    let mut guess_chars = ['\0'; 10];
    let mut guess_len = 0;
    for (i, c) in guess.chars().enumerate() {
        if i < 10 {
            guess_chars[i] = c;
            guess_len += 1;
        }
    }

    let mut solution_chars_taken = [false; 10];
    let mut statuses = vec![CharStatus::Absent; guess_len];

    // Pass 1: handle all correct spots
    for i in 0..guess_len {
        if i < sol_len && guess_chars[i] == sol_chars[i] {
            statuses[i] = CharStatus::Correct;
            solution_chars_taken[i] = true;
        }
    }

    // Pass 2: handle present and absent spots
    for i in 0..guess_len {
        if statuses[i] == CharStatus::Correct {
            continue;
        }

        let letter = guess_chars[i];
        if !sol_chars[..sol_len].contains(&letter) {
            statuses[i] = CharStatus::Absent;
            continue;
        }

        let mut index_of_present_char = None;
        for idx in 0..sol_len {
            if sol_chars[idx] == letter && !solution_chars_taken[idx] {
                index_of_present_char = Some(idx);
                break;
            }
        }

        if let Some(idx) = index_of_present_char {
            statuses[i] = CharStatus::Present;
            solution_chars_taken[idx] = true;
        } else {
            statuses[i] = CharStatus::Absent;
        }
    }

    statuses
}
