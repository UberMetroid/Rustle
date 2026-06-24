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

#![allow(unused_imports)]
//! Core helper libraries and utilities for the Rustle application.
//! Contains logic for web browser features, statistics, local storage, cryptology, and sharing.
//!
//! Submodules:
//! - Browser: User agent checks for restricted embedded web views.
//! - Encryption: Symmetric Blowfish cryptology for stats backup/transfer hashes.
//! - Local Storage: Local web storage persistence of statistics and game sessions.
//! - Share: Share status creation (grid emojis) and clipboard exports.
//! - Stats: Win distributions, streaks, and load/save helper methods.
//! - Statuses: Cell evaluation states (Correct, Present, Absent).
//! - Words: Puzzle date epochs, solution indexes, and word list helpers.
//!
//! Re-exports common types to simplify module interfaces throughout the application.

pub mod browser;
pub mod encryption;
pub mod holidays;
pub mod local_storage;
pub mod share;
pub mod stats;
pub mod statuses;
pub mod words;

// Re-export common types for internal convenience
pub use browser::is_in_app_browser;
pub use local_storage::{GameStats, StoredGameState};
pub use statuses::CharStatus;

#[cfg(test)]
mod tests;
