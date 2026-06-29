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
pub mod init;
pub mod callbacks;

use crate::app_effects::use_app_effects;
use crate::app_state::{Action, AppState};
use crate::components::app_modals::AppModals;
use crate::components::grid::Grid;
use crate::components::keyboard::Keyboard;
use crate::components::WeatherContainer;
use crate::constants::config::*;
use shared_frontend::{Header, Footer};
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
    let enable_translation = use_state(|| false);
    let enable_themes = use_state(|| true);
    let enable_print = use_state(|| true);
    let language_state = use_state(crate::i18n::get_saved_language);
 
    {
        let is_pin_required = is_pin_required.clone();
        let enable_translation = enable_translation.clone();
        let enable_themes = enable_themes.clone();
        let enable_print = enable_print.clone();
        let state = state.clone();
        use_effect_with((), move |_| {
            init::fetch_app_config(
                is_pin_required,
                enable_translation,
                enable_themes,
                enable_print,
                state,
            );
            || ()
        });
    }

    let on_logout = {
        let is_pin_required_val = *is_pin_required;
        Callback::from(move |_| {
            if is_pin_required_val {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(resp) = gloo_net::http::Request::post("/api/logout").send().await {
                        if resp.ok() {
                            if let Some(win) = web_sys::window() {
                                let _ = win.location().reload();
                            }
                        }
                    }
                });
            }
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

    let on_language_change = {
        let lang = language_state.clone();
        Callback::from(move |new_lang: crate::i18n::Language| {
            crate::i18n::save_language(new_lang);
            lang.set(new_lang);
        })
    };

    let i18n_context = crate::i18n::I18nContext {
        language: *language_state,
        translations: crate::i18n::get_translations(*language_state),
    };

    let on_enter = enter::build_on_enter(
        state.clone(),
        show_alert.clone(),
        solution,
        is_latest_game,
        i18n_context.clone(),
    );

    let on_theme_click = callbacks::build_on_theme_click(state.clone(), enable_themes.clone());
    let on_hard_mode_click = callbacks::build_on_hard_mode_click(state.clone(), show_alert.clone());

    html! {
        <ContextProvider<crate::i18n::I18nContext> context={i18n_context}>
            <div class="flex h-screen h-dvh flex-col app-container transition-colors duration-300">
                <WeatherContainer theme={state.theme.clone()} is_active={state.is_effects_active} />
                <div class="relative w-full">
                    <Header
                        site_title={GAME_TITLE.to_string()}
                        theme={state.theme.clone()}
                        language={shared_core::i18n::Language::from_code(language_state.code())}
                        toggle_theme={on_theme_click}
                        on_language_change={
                            let on_language_change = on_language_change.clone();
                            Callback::from(move |lang: shared_core::i18n::Language| on_language_change.emit(crate::i18n::Language::from_code(lang.code())))
                        }
                        is_authenticated={true}
                        pin_required={*is_pin_required}
                        on_logout={
                            let on_logout = on_logout.clone();
                            Callback::from(move |_| on_logout.emit(()))
                        }
                        enable_translation={*enable_translation}
                        enable_themes={*enable_themes}
                        enable_print={false}
                        print_disabled={true}
                        on_print={None}
                    />
                    <div class="header-center-toolbar">
                        <button class="icon-button" onclick={ { let s = state.clone(); Callback::from(move |_| s.dispatch(Action::SetInfoOpen(true))) } } aria-label="Info" title="How to Play">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
                            </svg>
                        </button>
                        <button class="icon-button" onclick={ { let s = state.clone(); Callback::from(move |_| s.dispatch(Action::SetStatsOpen(true))) } } aria-label="Stats" title="Statistics">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
                            </svg>
                        </button>
                        {if ENABLE_ARCHIVED_GAMES {
                            html! {
                                <button class="icon-button" onclick={ { let s = state.clone(); Callback::from(move |_| s.dispatch(Action::SetDatePickerOpen(true))) } } aria-label="DatePicker" title="Archive">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5m-9-6h.008v.008H12v-.008zM12 15h.008v.008H12V15zm0 2.25h.008v.008H12v-.008zM9.75 15h.008v.008H9.75V15zm0 2.25h.008v.008H9.75v-.008zM7.5 15h.008v.008H7.5V15zm0 2.25h.008v.008H7.5v-.008zm6.75-4.5h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V15zm0 2.25h.008v.008h-.008v-.008zm2.25-4.5h.008v.008H16.5v-.008zm0 2.25h.008v.008H16.5V15z" />
                                    </svg>
                                </button>
                            }
                        } else {
                            html! {}
                        }}
                        <button class={classes!("icon-button", state.is_hard_mode.then_some("active"))} onclick={on_hard_mode_click} aria-label="Hard Mode" title="Hard Mode">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill={if state.is_hard_mode { "currentColor" } else { "none" }} stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                                <path d="m2 4 3 12h14l3-12-6 7-4-7-4 7-6-7zm3 16h14" />
                            </svg>
                        </button>
                    </div>
                </div>
                <div class="mx-auto flex w-full max-w-7xl flex-grow flex-col px-1 py-1 sm:py-2 sm:px-6 lg:px-8">
                    <Grid solution={solution} guesses={state.guesses.clone()} current_guess={state.current_guess.clone()} is_revealing={state.is_revealing} current_row_class_name={state.jiggle_class.clone()} />
                    <div class="my-auto w-full">
                        <Keyboard on_char={on_char} on_delete={on_delete} on_enter={on_enter} solution={solution} guesses={state.guesses.clone()} is_revealing={state.is_revealing} />
                    </div>
                </div>
                <AppModals state={state.clone()} solution={solution} solution_index={solution_index} tomorrow={tomorrow} is_latest_game={is_latest_game} show_alert={show_alert} />
                <Footer
                    show_version={true}
                    version={env!("CARGO_PKG_VERSION").to_string()}
                    show_github={true}
                    github_url={Some("https://github.com/UberMetroid/rustle".to_string())}
                    version_url={Some(format!("https://github.com/UberMetroid/rustle/releases/tag/v{}", env!("CARGO_PKG_VERSION")))}
                >
                    {
                        if state.alert_visible && !state.alert_msg.is_empty() {
                            let cls = match state.alert_variant.as_str() {
                                "success" => "success",
                                "error" => "danger",
                                _ => "info",
                            };
                            html! { <div class={format!("footer-status-text {}", cls)}>{ &state.alert_msg }</div> }
                        } else {
                            html! { <div class="footer-status-text success">{"Ready"}</div> }
                        }
                    }
                </Footer>
            </div>
        </ContextProvider<crate::i18n::I18nContext>>
    }
}
