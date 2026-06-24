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

//! App side effects hook.
//! Manages game startup defaults, User-Agent browser checks, body styles, and animation classes.

use crate::app_state::{Action, AppState};
use crate::constants::config::{
    DISCOURAGE_INAPP_BROWSERS, DISCOURAGE_INAPP_BROWSER_TEXT, WELCOME_INFO_MODAL_MS,
};
use yew::prelude::*;

/// Custom hook to execute and coordinate all side-effects for the main game view.
#[hook]
pub fn use_app_effects(
    state: UseReducerHandle<AppState>,
    show_alert: Callback<(String, String, u32)>,
) {
    // Initialize Info modal if first time player
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let loaded = crate::helpers::local_storage::load_game_state_from_local_storage(true);
            if loaded.is_none() {
                let state = state.clone();
                let timeout =
                    gloo_timers::callback::Timeout::new(WELCOME_INFO_MODAL_MS, move || {
                        state.dispatch(Action::SetInfoOpen(true));
                    });
                timeout.forget();
            }
        });
    }

    // Discourage In-App browser warning alert
    {
        let show_alert = show_alert.clone();
        use_effect_with((), move |_| {
            if DISCOURAGE_INAPP_BROWSERS && crate::helpers::browser::is_in_app_browser() {
                show_alert.emit((
                    DISCOURAGE_INAPP_BROWSER_TEXT.to_string(),
                    "error".to_string(),
                    7000,
                ));
            }
        });
    }

    // Toggle document element classes and data-theme for cycle-based themes
    {
        let theme = state.theme.clone();
        use_effect_with(theme, move |theme| {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(el) = doc.document_element() {
                        let class_name = format!("{} dark", theme);
                        let _ = el.set_attribute("class", &class_name);
                        let _ = el.set_attribute("data-theme", theme);
                    }
                }
            }
        });
    }

    // Clear jiggle animation class after duration has run
    {
        let jiggle = state.jiggle_class.clone();
        let state = state.clone();
        use_effect_with(jiggle, move |jiggle| {
            if !jiggle.is_empty() {
                let state = state.clone();
                let timeout = gloo_timers::callback::Timeout::new(250, move || {
                    state.dispatch(Action::SetJiggle("".to_string()));
                });
                timeout.forget();
            }
        });
    }
}
