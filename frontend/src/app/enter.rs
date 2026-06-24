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

use crate::app_state::{Action, AppState};
use crate::constants::config::{
    correct_word_message, ALERT_TIME_MS, LONG_ALERT_TIME_MS, MAX_CHALLENGES,
    NOT_ENOUGH_LETTERS_MESSAGE, REVEAL_TIME_MS, WIN_MESSAGES, WORD_NOT_FOUND_MESSAGE,
};
use yew::prelude::*;

pub fn build_on_enter(
    state: UseReducerHandle<AppState>,
    show_alert: Callback<(String, String, u32)>,
    solution: &'static str,
    is_latest_game: bool,
) -> Callback<()> {
    Callback::from(move |()| {
        if state.is_game_won || state.is_game_lost {
            return;
        }
        let guess_len = state.current_guess.chars().count();
        let sol_len = solution.chars().count();

        if guess_len < sol_len {
            show_alert.emit((
                NOT_ENOUGH_LETTERS_MESSAGE.to_string(),
                "error".to_string(),
                ALERT_TIME_MS,
            ));
            state.dispatch(Action::SetJiggle("jiggle".to_string()));
            return;
        }

        let word = state.current_guess.clone().to_uppercase();
        if !crate::helpers::words::is_word_in_word_list(&word) {
            show_alert.emit((
                WORD_NOT_FOUND_MESSAGE.to_string(),
                "error".to_string(),
                ALERT_TIME_MS,
            ));
            state.dispatch(Action::SetJiggle("jiggle".to_string()));
            return;
        }

        if state.is_hard_mode {
            if let Some(fail) =
                crate::helpers::words::find_first_unused_reveal(&word, &state.guesses, solution)
            {
                show_alert.emit((fail, "error".to_string(), ALERT_TIME_MS));
                state.dispatch(Action::SetJiggle("jiggle".to_string()));
                return;
            }
        }

        state.dispatch(Action::SetRevealing(true));
        let state_rev = state.clone();
        gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * sol_len as u32, move || {
            state_rev.dispatch(Action::SetRevealing(false))
        })
        .forget();

        let mut new_guesses = state.guesses.clone();
        new_guesses.push(word.clone());
        state.dispatch(Action::SetGuesses(new_guesses.clone()));

        crate::helpers::local_storage::save_game_state_to_local_storage(
            is_latest_game,
            &crate::helpers::local_storage::StoredGameState {
                guesses: new_guesses.clone(),
                solution: solution.to_string(),
            },
        );

        state.dispatch(Action::ClearGuess);

        if crate::helpers::words::is_winning_word(&word, solution) {
            state.dispatch(Action::SetWon(true));
            state.dispatch(Action::SetGameStats(
                crate::helpers::stats::add_stats_for_completed_game(
                    state.game_stats.clone(),
                    new_guesses.len() - 1,
                ),
            ));

            let win_message = WIN_MESSAGES
                [js_sys::Math::floor(js_sys::Math::random() * WIN_MESSAGES.len() as f64) as usize];
            let state_won = state.clone();
            let show_alert_clone = show_alert.clone();
            gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * sol_len as u32, move || {
                show_alert_clone.emit((
                    win_message.to_string(),
                    "success".to_string(),
                    ALERT_TIME_MS,
                ));
                state_won.dispatch(Action::SetStatsOpen(true));
                state_won.dispatch(Action::SetEffectsActive(true));
            })
            .forget();
        } else if new_guesses.len() >= MAX_CHALLENGES {
            state.dispatch(Action::SetLost(true));
            state.dispatch(Action::SetGameStats(
                crate::helpers::stats::add_stats_for_completed_game(
                    state.game_stats.clone(),
                    new_guesses.len(),
                ),
            ));

            let state_lost = state.clone();
            let show_alert_clone = show_alert.clone();
            gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * (sol_len as u32 + 1), move || {
                show_alert_clone.emit((
                    correct_word_message(solution),
                    "error".to_string(),
                    LONG_ALERT_TIME_MS,
                ));
                state_lost.dispatch(Action::SetStatsOpen(true));
            })
            .forget();
        }
    })
}
