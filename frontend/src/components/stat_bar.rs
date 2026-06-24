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

//! Statistical dashboard components.
//! Displays overall play metrics like total games, success rates, and streak counters.

use crate::constants::config::{
    BEST_STREAK_TEXT, CURRENT_STREAK_TEXT, SUCCESS_RATE_TEXT, TOTAL_TRIES_TEXT,
};
use crate::helpers::local_storage::GameStats;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
struct StatItemProps {
    pub label: String,
    pub value: String,
}

#[function_component(StatItem)]
fn stat_item(props: &StatItemProps) -> Html {
    html! {
        <div class="m-1 w-1/4 text-center dark:text-white">
            <div class="text-2xl sm:text-3xl font-bold">{&props.value}</div>
            <div class="text-[9px] sm:text-xs leading-tight">{&props.label}</div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct StatBarProps {
    pub game_stats: GameStats,
}

#[function_component(StatBar)]
pub fn stat_bar(props: &StatBarProps) -> Html {
    html! {
        <div class="my-2 flex justify-center">
            <StatItem label={TOTAL_TRIES_TEXT.to_string()} value={props.game_stats.total_games.to_string()} />
            <StatItem label={SUCCESS_RATE_TEXT.to_string()} value={format!("{}%", props.game_stats.success_rate)} />
            <StatItem label={CURRENT_STREAK_TEXT.to_string()} value={props.game_stats.current_streak.to_string()} />
            <StatItem label={BEST_STREAK_TEXT.to_string()} value={props.game_stats.best_streak.to_string()} />
        </div>
    }
}
