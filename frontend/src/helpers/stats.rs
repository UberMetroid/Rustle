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

use crate::constants::config::MAX_CHALLENGES;
use crate::helpers::local_storage::{
    load_stats_from_local_storage, save_stats_to_local_storage, GameStats,
};

pub fn default_stats() -> GameStats {
    GameStats {
        win_distribution: vec![0; MAX_CHALLENGES],
        games_failed: 0,
        current_streak: 0,
        best_streak: 0,
        total_games: 0,
        success_rate: 0,
    }
}

pub fn load_stats() -> GameStats {
    load_stats_from_local_storage().unwrap_or_else(default_stats)
}

pub fn add_stats_for_completed_game(mut stats: GameStats, count: usize) -> GameStats {
    stats.total_games = stats.total_games.saturating_add(1);

    if count >= MAX_CHALLENGES {
        // A fail situation
        stats.current_streak = 0;
        stats.games_failed = stats.games_failed.saturating_add(1);
    } else {
        if count < stats.win_distribution.len() {
            stats.win_distribution[count] = stats.win_distribution[count].saturating_add(1);
        }
        stats.current_streak = stats.current_streak.saturating_add(1);

        if stats.best_streak < stats.current_streak {
            stats.best_streak = stats.current_streak;
        }
    }

    stats.success_rate = get_success_rate(&stats);

    save_stats_to_local_storage(&stats);
    stats
}

fn get_success_rate(stats: &GameStats) -> i32 {
    let total_games = stats.total_games.max(0);
    let games_failed = stats.games_failed.max(0).min(total_games);

    if total_games == 0 {
        0
    } else {
        let successes = total_games - games_failed;
        ((successes as f64 * 100.0) / total_games as f64).round() as i32
    }
}
