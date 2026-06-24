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

//! Guess distribution histogram components.
//! Renders horizontal progress bars showing the number of guesses taken to win across games.

use crate::helpers::local_storage::GameStats;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
struct ProgressProps {
    pub index: usize,
    pub size: f64,
    pub label: String,
    pub is_current_day_stat_row: bool,
}

#[function_component(Progress)]
fn progress(props: &ProgressProps) -> Html {
    let current_row_class = classes!(
        "text-xs",
        "font-medium",
        "text-blue-100",
        "text-center",
        "p-0.5",
        if props.is_current_day_stat_row {
            "bg-blue-600"
        } else {
            "bg-gray-600"
        }
    );
    let bar_style = format!("width: {}%;", 8.0 + props.size);
    html! {
        <div class="justify-left m-1 flex">
            <div class="w-2 items-center justify-center">{props.index + 1}</div>
            <div class="ml-2 w-full">
                <div style={bar_style} class={current_row_class}>
                    {&props.label}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HistogramProps {
    pub game_stats: GameStats,
    pub is_latest_game: bool,
    pub is_game_won: bool,
    pub number_of_guesses_made: usize,
}

#[function_component(Histogram)]
pub fn histogram(props: &HistogramProps) -> Html {
    let max_value = props
        .game_stats
        .win_distribution
        .iter()
        .max()
        .copied()
        .unwrap_or(0)
        .max(1);
    let is_latest_game = props.is_latest_game;
    let is_game_won = props.is_game_won;
    let number_of_guesses_made = props.number_of_guesses_made;

    html! {
        <div class="justify-left m-2 columns-1 text-sm dark:text-white">
            {for props.game_stats.win_distribution.iter().enumerate().map(|(i, &value)| {
                let is_current = is_latest_game && is_game_won && number_of_guesses_made == i + 1;
                let size = 90.0 * (value as f64 / max_value as f64);
                html! {
                    <Progress
                        index={i}
                        is_current_day_stat_row={is_current}
                        size={size}
                        label={value.to_string()}
                    />
                }
            })}
        </div>
    }
}
