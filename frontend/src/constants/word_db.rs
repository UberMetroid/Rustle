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

//! Word database containing the solution list and valid guesses list.
//! Words are stored in a flat string literal to maximize compilation efficiency.

/// Concatenated string of all five-letter solution words.
pub const WORDS: &str = include_str!("wordlist.txt");

/// Concatenated string of all five-letter valid guess words.
pub const VALID_GUESSES: &str = include_str!("valid_guesses.txt");

/// Check if the uppercase word exists in either the solution list or valid guesses list.
pub fn is_word_in_word_list(word: &str) -> bool {
    if word.len() != 5 {
        return false;
    }
    let mut upper_bytes = [0u8; 5];
    for (i, &byte) in word.as_bytes().iter().take(5).enumerate() {
        upper_bytes[i] = byte.to_ascii_uppercase();
    }
    let upper = match std::str::from_utf8(&upper_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };
    contains_word(WORDS, upper) || contains_word(VALID_GUESSES, upper)
}

/// Retrieve the uppercase solution word of the day based on the game index.
/// Utilizes a seeded scrambling step to prevent players from reading the list sequentially.
pub fn get_word_of_day(index: i32) -> &'static str {
    let num_words = WORDS.len() / 5;

    // Deterministic seeded scrambling step (Linear Congruential Generator step)
    let seed = index as i64 as u64;
    let multiplier: u64 = 6364136223846793005;
    let increment: u64 = 1442695040888963407;
    let scrambled = seed.wrapping_mul(multiplier).wrapping_add(increment);

    let target_idx = (scrambled as usize % num_words) * 5;
    &WORDS[target_idx..target_idx + 5]
}

/// Perform a zero-allocation binary search on a flat string slice of sorted 5-letter ASCII words.
fn contains_word(flat_list: &str, word: &str) -> bool {
    let word_bytes = word.as_bytes();
    let num_words = flat_list.len() / 5;

    let mut low = 0;
    let mut high = num_words;
    while low < high {
        let mid = low + (high - low) / 2;
        let mid_start = mid * 5;
        let mid_word = &flat_list.as_bytes()[mid_start..mid_start + 5];
        match word_bytes.cmp(mid_word) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }
    false
}
