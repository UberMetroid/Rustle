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

//! Statistics modal component.
//! Manages game statistics display including streaks, countdowns, and sharing.

use crate::components::modal_base::BaseModal;
use crate::components::stat_bar::StatBar;
use crate::components::stat_histogram::Histogram;
use crate::constants::config::{
    ENABLE_ARCHIVED_GAMES, ENABLE_MIGRATE_STATS, GUESS_DISTRIBUTION_TEXT, MIGRATE_BUTTON_TEXT,
    MIGRATE_DESCRIPTION_TEXT, NEW_WORD_TEXT, SHARE_TEXT, STATISTICS_TITLE,
};
use crate::helpers::local_storage::GameStats;
use crate::helpers::share::share_status;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MigrationIntroProps {
    pub handle_migrate_stats_button: Callback<()>,
}

#[function_component(MigrationIntro)]
pub fn migration_intro(props: &MigrationIntroProps) -> Html {
    let on_click = props.handle_migrate_stats_button.clone();
    html! {
        <div class="mt-5 columns-2 items-center items-stretch justify-center text-center dark:text-white sm:mt-6">
            <div class="mt-3 text-xs">{MIGRATE_DESCRIPTION_TEXT}</div>
            <button
                type="button"
                onclick={move |_| on_click.emit(())}
                class="mt-2 inline-flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-center text-base font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:text-sm"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mr-2 h-6 w-6 cursor-pointer dark:stroke-white">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75" />
                </svg>
                {MIGRATE_BUTTON_TEXT}
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CountdownProps {
    pub tomorrow: i64,
}

#[function_component(Countdown)]
fn countdown(props: &CountdownProps) -> Html {
    let tomorrow = props.tomorrow;
    let time_left = use_state(|| {
        let now = js_sys::Date::now() as i64;
        (tomorrow - now).max(0)
    });

    {
        let time_left = time_left.clone();
        use_effect_with(tomorrow, move |&tomorrow| {
            let interval = gloo_timers::callback::Interval::new(1000, move || {
                let now = js_sys::Date::now() as i64;
                time_left.set((tomorrow - now).max(0));
            });
            move || drop(interval)
        });
    }

    let seconds = (*time_left / 1000) % 60;
    let minutes = (*time_left / (1000 * 60)) % 60;
    let hours = *time_left / (1000 * 60 * 60);

    html! {
        <span class="text-lg font-medium text-gray-900 dark:text-gray-100">
            {format!("{:02}:{:02}:{:02}", hours, minutes, seconds)}
        </span>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct StatsModalProps {
    pub is_open: bool,
    pub handle_close: Callback<()>,
    pub solution: String,
    pub guesses: Vec<String>,
    pub game_stats: GameStats,
    pub is_latest_game: bool,
    pub is_game_lost: bool,
    pub is_game_won: bool,
    pub handle_share_to_clipboard: Callback<()>,
    pub handle_share_failure: Callback<()>,
    pub handle_migrate_stats_button: Callback<()>,
    pub is_hard_mode: bool,
    pub theme: String,
    pub solution_index: i32,
    pub tomorrow: i64,
}

#[function_component(StatsModal)]
pub fn stats_modal(props: &StatsModalProps) -> Html {
    let solution = props.solution.clone();
    let guesses = props.guesses.clone();
    let is_game_lost = props.is_game_lost;
    let is_hard_mode = props.is_hard_mode;
    let theme = props.theme.clone();
    let solution_index = props.solution_index;
    let handle_share_to_clipboard = props.handle_share_to_clipboard.clone();
    let handle_share_failure = props.handle_share_failure.clone();

    let share_click = Callback::from(move |_| {
        let success = handle_share_to_clipboard.clone();
        let fail = handle_share_failure.clone();
        share_status(
            &solution,
            &guesses,
            is_game_lost,
            is_hard_mode,
            &theme,
            solution_index,
            move || success.emit(()),
            move || fail.emit(()),
        );
    });

    if props.game_stats.total_games <= 0 {
        return html! {
            <BaseModal title={STATISTICS_TITLE.to_string()} is_open={props.is_open} handle_close={props.handle_close.clone()}>
                <StatBar game_stats={props.game_stats.clone()} />
                {if ENABLE_MIGRATE_STATS {
                    html! { <MigrationIntro handle_migrate_stats_button={props.handle_migrate_stats_button.clone()} /> }
                } else {
                    html! {}
                }}
            </BaseModal>
        };
    }

    html! {
        <BaseModal title={STATISTICS_TITLE.to_string()} is_open={props.is_open} handle_close={props.handle_close.clone()}>
            <StatBar game_stats={props.game_stats.clone()} />
            <h4 class="text-lg font-medium leading-6 text-gray-900 dark:text-gray-100">
                {GUESS_DISTRIBUTION_TEXT}
            </h4>
            <Histogram
                game_stats={props.game_stats.clone()}
                is_latest_game={props.is_latest_game}
                is_game_won={props.is_game_won}
                number_of_guesses_made={props.guesses.len()}
            />
            {if props.is_game_lost || props.is_game_won {
                html! {
                    <div class="mt-5 columns-2 items-center items-stretch justify-center text-center dark:text-white sm:mt-6">
                        <div class="inline-block w-full text-left">
                            {if !ENABLE_ARCHIVED_GAMES || props.is_latest_game {
                                html! {
                                    <div>
                                        <h5>{NEW_WORD_TEXT}</h5>
                                        <Countdown tomorrow={props.tomorrow} />
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                        <div>
                            <button
                                type="button"
                                onclick={share_click}
                                class="mt-2 inline-flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-center text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:text-base font-bold"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mr-2 h-6 w-6 cursor-pointer dark:stroke-white">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7.217 10.907a2.25 2.25 0 100 2.186m0-2.186l5.302 3.18m-5.302-1.006L12 9.3m6.783-1.15a2.25 2.25 0 100-2.186 2.25 2.25 0 000 2.186zm0 12.08a2.25 2.25 0 100-2.186 2.25 2.25 0 000 2.186z" />
                                </svg>
                                {SHARE_TEXT}
                            </button>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            {if ENABLE_MIGRATE_STATS {
                html! {
                    <div>
                        <hr class="mt-4 -mb-4 border-gray-500" />
                        <MigrationIntro handle_migrate_stats_button={props.handle_migrate_stats_button.clone()} />
                    </div>
                }
            } else {
                html! {}
            }}
        </BaseModal>
    }
}
