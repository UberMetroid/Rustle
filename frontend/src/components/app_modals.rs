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

//! Modals container component.
//! Renders and manages all game overlay dialogs (Info, Settings, Stats, DatePicker, Migration).

use crate::app_state::{Action, AppState};
use crate::components::modal_date_picker::DatePickerModal;
use crate::components::modal_info::InfoModal;
use crate::components::modal_migrate::MigrateStatsModal;
use crate::components::modal_stats::StatsModal;
use crate::constants::config::{
    ALERT_TIME_MS, GAME_COPIED_MESSAGE, HARD_MODE_ALERT_MESSAGE, SHARE_FAILURE_TEXT,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppModalsProps {
    pub state: UseReducerHandle<AppState>,
    pub solution: String,
    pub solution_index: i32,
    pub tomorrow: i64,
    pub is_latest_game: bool,
    pub show_alert: Callback<(String, String, u32)>,
}

#[function_component(AppModals)]
pub fn app_modals(props: &AppModalsProps) -> Html {
    let state = props.state.clone();
    let solution = props.solution.clone();
    let solution_index = props.solution_index;
    let tomorrow = props.tomorrow;
    let is_latest_game = props.is_latest_game;
    let show_alert = props.show_alert.clone();

    let share_success = {
        let show_alert = show_alert.clone();
        Callback::from(move |_| {
            show_alert.emit((
                GAME_COPIED_MESSAGE.to_string(),
                "success".to_string(),
                ALERT_TIME_MS,
            ))
        })
    };

    let share_fail = {
        let show_alert = show_alert.clone();
        Callback::from(move |_| {
            show_alert.emit((
                SHARE_FAILURE_TEXT.to_string(),
                "error".to_string(),
                ALERT_TIME_MS,
            ))
        })
    };

    html! {
        <>
            <InfoModal
                is_open={state.is_info_open}
                handle_close={ {
                    let state = state.clone();
                    Callback::from(move |_| state.dispatch(Action::SetInfoOpen(false)))
                } }
            />



            <StatsModal
                is_open={state.is_stats_open}
                handle_close={ {
                    let state = state.clone();
                    Callback::from(move |_| state.dispatch(Action::SetStatsOpen(false)))
                } }
                solution={solution}
                guesses={state.guesses.clone()}
                game_stats={state.game_stats.clone()}
                is_latest_game={is_latest_game}
                is_game_lost={state.is_game_lost}
                is_game_won={state.is_game_won}
                handle_share_to_clipboard={share_success}
                handle_share_failure={share_fail}
                handle_migrate_stats_button={ {
                    let state = state.clone();
                    Callback::from(move |_| {
                        state.dispatch(Action::SetStatsOpen(false));
                        state.dispatch(Action::SetMigrateOpen(true));
                    })
                } }
                is_hard_mode={state.is_hard_mode}
                theme={state.theme.clone()}
                solution_index={solution_index}
                tomorrow={tomorrow}
            />

            <DatePickerModal
                is_open={state.is_datepicker_open}
                handle_close={ {
                    let state = state.clone();
                    Callback::from(move |_| state.dispatch(Action::SetDatePickerOpen(false)))
                } }
            />

            <MigrateStatsModal
                is_open={state.is_migrate_open}
                handle_close={ {
                    let state = state.clone();
                    Callback::from(move |_| state.dispatch(Action::SetMigrateOpen(false)))
                } }
            />
        </>
    }
}
