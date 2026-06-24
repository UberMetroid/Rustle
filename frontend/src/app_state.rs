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

use crate::helpers::local_storage::GameStats;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub guesses: Vec<String>,
    pub current_guess: String,
    pub is_game_won: bool,
    pub is_game_lost: bool,
    pub is_revealing: bool,
    pub jiggle_class: String,
    pub alert_msg: String,
    pub alert_visible: bool,
    pub alert_variant: String,
    pub is_info_open: bool,
    pub is_stats_open: bool,
    pub is_datepicker_open: bool,
    pub is_migrate_open: bool,
    pub game_stats: GameStats,
    pub theme: String,
    pub is_hard_mode: bool,
    pub is_effects_active: bool,
}

pub enum Action {
    AddChar(char),
    DeleteChar,
    ClearGuess,
    SetGuesses(Vec<String>),
    SetWon(bool),
    SetLost(bool),
    SetRevealing(bool),
    SetJiggle(String),
    ShowAlert(String, String),
    HideAlert,
    SetInfoOpen(bool),
    SetStatsOpen(bool),
    SetDatePickerOpen(bool),
    SetMigrateOpen(bool),
    SetGameStats(GameStats),
    SetHardMode(bool),
    SetTheme(String),
    SetEffectsActive(bool),
}

impl Reducible for AppState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        match action {
            Action::AddChar(c) => {
                state.current_guess.push(c);
            }
            Action::DeleteChar => {
                let _ = state.current_guess.pop();
            }
            Action::ClearGuess => {
                state.current_guess.clear();
            }
            Action::SetGuesses(g) => {
                state.guesses = g;
            }
            Action::SetWon(w) => {
                state.is_game_won = w;
            }
            Action::SetLost(l) => {
                state.is_game_lost = l;
            }
            Action::SetRevealing(r) => {
                state.is_revealing = r;
            }
            Action::SetJiggle(j) => {
                state.jiggle_class = j;
            }
            Action::ShowAlert(msg, var) => {
                state.alert_msg = msg;
                state.alert_variant = var;
                state.alert_visible = true;
            }
            Action::HideAlert => {
                state.alert_visible = false;
            }
            Action::SetInfoOpen(o) => {
                state.is_info_open = o;
            }
            Action::SetStatsOpen(o) => {
                state.is_stats_open = o;
            }
            Action::SetDatePickerOpen(o) => {
                state.is_datepicker_open = o;
            }
            Action::SetMigrateOpen(o) => {
                state.is_migrate_open = o;
            }
            Action::SetGameStats(s) => {
                state.game_stats = s;
            }
            Action::SetHardMode(val) => {
                state.is_hard_mode = val;
            }
            Action::SetTheme(val) => {
                state.theme = val;
            }
            Action::SetEffectsActive(val) => {
                state.is_effects_active = val;
            }
        }
        Rc::new(state)
    }
}

impl AppState {
    pub fn new(solution: &str, is_latest_game: bool, prefers_dark: bool) -> Self {
        let guesses = if let Some(state) =
            crate::helpers::local_storage::load_game_state_from_local_storage(is_latest_game)
        {
            if state.solution == solution {
                state.guesses
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let is_game_won = guesses.contains(&solution.to_string());
        let is_game_lost =
            guesses.len() >= crate::constants::config::MAX_CHALLENGES && !is_game_won;

        let prefs =
            crate::helpers::local_storage::load_preferences_from_local_storage(prefers_dark);
        let theme = prefs.theme;
        let is_hard_mode = prefs.is_hard_mode;

        let game_stats = crate::helpers::stats::load_stats();
        let is_effects_active = is_game_won;

        Self {
            guesses,
            current_guess: "".to_string(),
            is_game_won,
            is_game_lost,
            is_revealing: false,
            jiggle_class: "".to_string(),
            alert_msg: "".to_string(),
            alert_visible: false,
            alert_variant: "error".to_string(),
            is_info_open: false,
            is_stats_open: false,
            is_datepicker_open: false,
            is_migrate_open: false,
            game_stats,
            theme,
            is_hard_mode,
            is_effects_active,
        }
    }
}
