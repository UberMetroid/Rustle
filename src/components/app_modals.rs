// Copyright (C) 2026 Jeryd
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
use crate::components::modal_settings::SettingsModal;
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

    let save_prefs = {
        let state = state.clone();
        move |dark: bool, contrast: bool, military: bool, hard: bool| {
            state.dispatch(Action::SetDarkMode(dark));
            state.dispatch(Action::SetHighContrast(contrast));
            state.dispatch(Action::SetMilitaryTheme(military));
            state.dispatch(Action::SetHardMode(hard));
            crate::helpers::local_storage::save_preferences_to_local_storage(
                &crate::helpers::local_storage::StoredPreferences {
                    is_dark_mode: dark,
                    is_high_contrast: contrast,
                    is_hard_mode: hard,
                    is_military_theme: military,
                },
            );
        }
    };

    let handle_hard_mode = {
        let state = state.clone();
        let show_alert = show_alert.clone();
        let save_prefs = save_prefs.clone();
        Callback::from(move |val| {
            if val {
                if state.guesses.is_empty() {
                    save_prefs(false, false, false, true);
                } else {
                    show_alert.emit((
                        HARD_MODE_ALERT_MESSAGE.to_string(),
                        "error".to_string(),
                        ALERT_TIME_MS,
                    ));
                }
            } else {
                save_prefs(false, false, false, false);
            }
        })
    };

    let handle_dark_mode = {
        let save_prefs = save_prefs.clone();
        Callback::from(move |val| {
            if val {
                save_prefs(true, false, false, false);
            } else {
                save_prefs(false, false, false, false);
            }
        })
    };

    let handle_high_contrast = {
        let save_prefs = save_prefs.clone();
        Callback::from(move |val| {
            if val {
                save_prefs(false, true, false, false);
            } else {
                save_prefs(false, false, false, false);
            }
        })
    };

    let handle_military_theme = {
        let save_prefs = save_prefs.clone();
        Callback::from(move |val| {
            if val {
                save_prefs(false, false, true, false);
            } else {
                save_prefs(false, false, false, false);
            }
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

            <SettingsModal
                is_open={state.is_settings_open}
                handle_close={ {
                    let state = state.clone();
                    Callback::from(move |_| state.dispatch(Action::SetSettingsOpen(false)))
                } }
                is_hard_mode={state.is_hard_mode}
                handle_hard_mode={handle_hard_mode}
                is_dark_mode={state.is_dark_mode}
                handle_dark_mode={handle_dark_mode}
                is_high_contrast_mode={state.is_high_contrast}
                handle_high_contrast_mode={handle_high_contrast}
                is_military_theme={state.is_military_theme}
                handle_military_theme={handle_military_theme}
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
                is_dark_mode={state.is_dark_mode}
                is_high_contrast_mode={state.is_high_contrast}
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
