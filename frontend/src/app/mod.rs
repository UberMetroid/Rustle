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

//! Rustle App view coordinator.
//! Coordinates layout structure, handles user key input, and manages game status alerts.

pub mod enter;

use crate::app_effects::use_app_effects;
use crate::app_state::{Action, AppState};
use crate::components::alerts::Alert;
use crate::components::app_modals::AppModals;
use crate::components::grid::Grid;
use crate::components::keyboard::Keyboard;
use crate::components::navbar::Navbar;
use crate::components::WeatherContainer;
use crate::constants::config::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let game_date = crate::helpers::words::get_game_date();
    let solution_info = crate::helpers::words::get_solution(game_date);
    let solution = solution_info.solution;
    let solution_index = solution_info.solution_index;
    let tomorrow = solution_info.tomorrow;
    let is_latest_game = crate::helpers::words::get_today() == game_date;

    let prefers_dark = use_state(|| {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(m)) = win.match_media("(prefers-color-scheme: dark)") {
                return m.matches();
            }
        }
        false
    });

    let state = use_reducer(move || AppState::new(solution, is_latest_game, *prefers_dark));

    let is_pin_required = use_state(|| false);

    {
        let is_pin_required = is_pin_required.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = gloo_net::http::Request::get("/api/pin-required").send().await {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(req) = json.get("required").and_then(|v| v.as_bool()) {
                            is_pin_required.set(req);
                        }
                    }
                }
            });
            || ()
        });
    }

    let on_logout = {
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = gloo_net::http::Request::post("/api/logout").send().await {
                    if resp.ok() {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().reload();
                        }
                    }
                }
            });
        })
    };

    let show_alert = {
        let state = state.clone();
        Callback::from(move |(msg, variant, duration_ms): (String, String, u32)| {
            state.dispatch(Action::ShowAlert(msg, variant));
            let state_clone = state.clone();
            gloo_timers::callback::Timeout::new(duration_ms, move || {
                state_clone.dispatch(Action::HideAlert);
            })
            .forget();
        })
    };

    use_app_effects(state.clone(), show_alert.clone());

    let on_char = {
        let state = state.clone();
        let sol_len = solution.chars().count();
        Callback::from(move |value: char| {
            if !state.is_game_won
                && state.current_guess.chars().count() < sol_len
                && state.guesses.len() < MAX_CHALLENGES
            {
                state.dispatch(Action::AddChar(value));
            }
        })
    };

    let on_delete = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::DeleteChar))
    };

    let on_enter =
        enter::build_on_enter(state.clone(), show_alert.clone(), solution, is_latest_game);

    let on_theme_click = {
        let state = state.clone();
        Callback::from(move |_| {
            let date = crate::helpers::words::get_game_date();
            if crate::helpers::holidays::get_holiday_for_date(date).is_some() {
                return;
            }
            let next_theme = match state.theme.as_str() {
                "crateria" => "brinstar".to_string(),
                "brinstar" => "norfair".to_string(),
                "norfair" => "wrecked_ship".to_string(),
                "wrecked_ship" => "maridia".to_string(),
                "maridia" => "tourian".to_string(),
                _ => "crateria".to_string(),
            };
            state.dispatch(Action::SetTheme(next_theme.clone()));
            crate::helpers::local_storage::save_preferences_to_local_storage(
                &crate::helpers::local_storage::StoredPreferences {
                    theme: next_theme,
                    is_hard_mode: state.is_hard_mode,
                },
            );
        })
    };

    let on_hard_mode_click = {
        let state = state.clone();
        let show_alert = show_alert.clone();
        Callback::from(move |_| {
            let next_val = !state.is_hard_mode;
            if next_val {
                if state.guesses.is_empty() {
                    state.dispatch(Action::SetHardMode(true));
                    crate::helpers::local_storage::save_preferences_to_local_storage(
                        &crate::helpers::local_storage::StoredPreferences {
                            theme: state.theme.clone(),
                            is_hard_mode: true,
                        },
                    );
                } else {
                    show_alert.emit((
                        HARD_MODE_ALERT_MESSAGE.to_string(),
                        "error".to_string(),
                        ALERT_TIME_MS,
                    ));
                }
            } else {
                state.dispatch(Action::SetHardMode(false));
                crate::helpers::local_storage::save_preferences_to_local_storage(
                    &crate::helpers::local_storage::StoredPreferences {
                        theme: state.theme.clone(),
                        is_hard_mode: false,
                    },
                );
            }
        })
    };

    html! {
        <div class="flex h-screen h-dvh flex-col app-container transition-colors duration-300">
            <WeatherContainer theme={state.theme.clone()} is_active={state.is_effects_active} />
            <Navbar
                on_info_click={ { let s = state.clone(); Callback::from(move |_| s.dispatch(Action::SetInfoOpen(true))) } }
                on_stats_click={ { let s = state.clone(); Callback::from(move |_| s.dispatch(Action::SetStatsOpen(true))) } }
                on_date_click={ { let s = state.clone(); Callback::from(move |_| s.dispatch(Action::SetDatePickerOpen(true))) } }
                is_hard_mode={state.is_hard_mode}
                on_hard_mode_click={on_hard_mode_click}
                theme={state.theme.clone()}
                on_theme_click={on_theme_click}
                is_pin_required={*is_pin_required}
                on_logout={on_logout}
            />
            <Alert message={state.alert_msg.clone()} is_visible={state.alert_visible} variant={state.alert_variant.clone()} />
            <div class="mx-auto flex w-full max-w-7xl flex-grow flex-col px-1 py-1 sm:py-2 sm:px-6 lg:px-8">
                <Grid solution={solution} guesses={state.guesses.clone()} current_guess={state.current_guess.clone()} is_revealing={state.is_revealing} current_row_class_name={state.jiggle_class.clone()} />
                <div class="my-auto w-full">
                    <Keyboard on_char={on_char} on_delete={on_delete} on_enter={on_enter} solution={solution} guesses={state.guesses.clone()} is_revealing={state.is_revealing} />
                </div>
            </div>
            <AppModals state={state.clone()} solution={solution} solution_index={solution_index} tomorrow={tomorrow} is_latest_game={is_latest_game} show_alert={show_alert} />
        </div>
    }
}
