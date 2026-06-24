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

//! Constants module configuration.
//! Consolidates application rules, timing configuration, localized strings,
//! and compile-time word lists.
//!
//! # Architecture
//! - `config`: Game timing parameters, localized strings, blowfish cryptographic key.
//! - `word_db`: Zero-allocation binary search, text file embeddings.
//!
//! This module acts as the central hub for static resources required by the game engine.

pub mod config;
pub mod word_db;

#[cfg(test)]
mod tests {
    use super::word_db::{VALID_GUESSES, WORDS};

    /// Validates that embedded word lists are not empty during build testing.
    #[test]
    fn test_word_list_embeddings() {
        assert!(!WORDS.is_empty(), "Solutions list cannot be empty");
        assert!(
            !VALID_GUESSES.is_empty(),
            "Valid guesses list cannot be empty"
        );
        assert_eq!(
            WORDS.len() % 5,
            0,
            "Solutions list size must be a multiple of 5"
        );
        assert_eq!(
            VALID_GUESSES.len() % 5,
            0,
            "Guesses list size must be a multiple of 5"
        );
    }
}
