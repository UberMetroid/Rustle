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

//! Migration stats modal component.
//! Handles exporting and importing user statistics and game state between devices.

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::modal_base::BaseModal;
use crate::helpers::encryption::{decrypt, encrypt};
use crate::helpers::local_storage::{
    save_game_state_to_local_storage, save_stats_to_local_storage, GameStats, StoredGameState,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MigrationStats {
    pub statistics: GameStats,
    #[serde(rename = "gameState")]
    pub game_state: Option<StoredGameState>,
}

#[function_component(EmigratePanel)]
pub fn emigrate_panel() -> Html {
    let is_copy_button_enabled = use_state(|| true);
    let copy_button_text = use_state(|| "Copy".to_string());

    let stats = crate::helpers::stats::load_stats();
    let game_state = crate::helpers::local_storage::load_game_state_from_local_storage(true);

    let migration_stats = MigrationStats {
        statistics: stats,
        game_state,
    };
    let serialized = serde_json::to_string(&migration_stats).unwrap_or_default();
    let emigration_code = encrypt(&serialized).unwrap_or_default();

    let copy_click = {
        let is_copy_button_enabled = is_copy_button_enabled.clone();
        let copy_button_text = copy_button_text.clone();
        let code = emigration_code.clone();
        Callback::from(move |_| {
            if let Some(win) = web_sys::window() {
                let _ = win.navigator().clipboard().write_text(&code);
                copy_button_text.set("Copied!".to_string());
                is_copy_button_enabled.set(false);
            }
        })
    };

    html! {
        <div class="text-sm text-gray-500 dark:text-gray-300">
            <label class="mb-2 block text-left text-sm font-medium text-gray-900 dark:text-gray-400">{"Copy your migration code:"}</label>
            <textarea id="emigration-code" readonly=true rows=8 class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500" value={emigration_code} />
            <button disabled={!*is_copy_button_enabled} onclick={copy_click} type="button" class="mt-2 inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-left text-base font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:border-gray-200 disabled:bg-white disabled:text-gray-900 disabled:focus:outline-none disabled:dark:border-gray-600 disabled:dark:bg-gray-800 disabled:dark:text-gray-400 sm:text-sm font-bold">
                {if *is_copy_button_enabled {
                    html! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mr-2 h-6 w-6 cursor-pointer dark:stroke-white">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376A8.965 8.965 0 0012 12.75c-.497 0-.982.04-1.455.12l-.04.008a8.966 8.966 0 00-1.685.583m10.02 4.042l1.802-1.802a1.125 1.125 0 000-1.59l-1.802-1.802m-1.802 5.194l-5.194-5.194M6.75 10.5h.008v.008H6.75V10.5zm0 3h.008v.008H6.75V13.5zm0 3h.008v.008H6.75V16.5z" />
                        </svg>
                    }
                } else { html! {} }}
                {(*copy_button_text).clone()}
            </button>
        </div>
    }
}

#[function_component(ImmigratePanel)]
pub fn immigrate_panel() -> Html {
    let is_save_enabled = use_state(|| false);
    let textarea_ref = use_node_ref();

    let on_change = {
        let is_save_enabled = is_save_enabled.clone();
        let textarea_ref = textarea_ref.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let text = textarea.value();
                if let Some(element) = textarea.dyn_ref::<web_sys::Element>() {
                    let class_list = element.class_list();
                    for c in &[
                        "bg-gray-100",
                        "bg-red-400",
                        "dark:bg-gray-700",
                        "dark:bg-red-900",
                    ] {
                        let _ = class_list.remove_1(c);
                    }
                    is_save_enabled.set(false);
                    if let Some(decrypted) = decrypt(&text) {
                        if serde_json::from_str::<MigrationStats>(&decrypted).is_ok() {
                            let _ = class_list.add_1("bg-gray-100");
                            let _ = class_list.add_1("dark:bg-gray-700");
                            is_save_enabled.set(true);
                            return;
                        }
                    }
                    if !text.is_empty() {
                        let _ = class_list.add_1("bg-red-400");
                        let _ = class_list.add_1("dark:bg-red-900");
                    }
                }
            }
        })
    };

    let on_save = {
        let textarea_ref = textarea_ref.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let text = textarea.value();
                if let Some(win) = web_sys::window() {
                    let confirmed = win.confirm_with_message("Are you sure you want to override the statistics on this device? This action is not reversible.").unwrap_or(false);
                    if confirmed {
                        if let Some(decrypted) = decrypt(&text) {
                            if let Ok(migration_stats) =
                                serde_json::from_str::<MigrationStats>(&decrypted)
                            {
                                if let Some(gs) = migration_stats.game_state {
                                    save_game_state_to_local_storage(true, &gs);
                                }
                                save_stats_to_local_storage(&migration_stats.statistics);
                                let _ = win.alert_with_message("The site will now reload.");
                                let _ = win.location().reload();
                            }
                        }
                    }
                }
            }
        })
    };

    html! {
        <div class="text-sm text-gray-500 dark:text-gray-300">
            <label class="mb-2 block text-left text-sm font-medium text-gray-900 dark:text-gray-400">{"Paste your migration code:"}</label>
            <textarea ref={textarea_ref} oninput={on_change} id="immigration-code" rows=8 class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500" />
            <button disabled={!*is_save_enabled} onclick={on_save} type="button" class="mt-2 inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-left text-base font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:border-gray-200 disabled:bg-white disabled:text-gray-900 disabled:focus:outline-none disabled:dark:border-gray-600 disabled:dark:bg-gray-800 disabled:dark:text-gray-400 sm:text-sm font-bold">
                {if *is_save_enabled {
                    html! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="mr-2 h-6 w-6 cursor-pointer dark:stroke-white">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 13.5l3 3m0 0l3-3m-3 3v-6m1.06-4.19l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                        </svg>
                    }
                } else { html! {} }}
                {"Save"}
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MigrateStatsModalProps {
    pub is_open: bool,
    pub handle_close: Callback<()>,
}

#[function_component(MigrateStatsModal)]
pub fn migrate_stats_modal(props: &MigrateStatsModalProps) -> Html {
    let is_emigrate_visible = use_state(|| true);
    let set_emigrate = {
        let is_emigrate_visible = is_emigrate_visible.clone();
        Callback::from(move |_| is_emigrate_visible.set(true))
    };
    let set_immigrate = {
        let is_emigrate_visible = is_emigrate_visible.clone();
        Callback::from(move |_| is_emigrate_visible.set(false))
    };

    html! {
        <BaseModal title="Transfer your statistics" is_open={props.is_open} handle_close={props.handle_close.clone()}>
            <p class="mt-4 mb-4 text-sm text-gray-500 dark:text-gray-300">{"Copy the migration code on your old device and paste into the input on the new device."}</p>
            <div class="w-full columns-3 gap-0 flex justify-between">
                <div class="mb-4 flex items-center">
                    <p class="mb-0 flex text-sm font-medium text-gray-900 dark:text-gray-300">{"This is my:"}</p>
                </div>
                <div class="mb-4 flex items-center">
                    <input checked={*is_emigrate_visible} onchange={set_emigrate} id="emigrate-radio-button" type="radio" name="migrate-radio-buttons" class="h-4 w-4 border-gray-300 bg-gray-100 text-blue-600 focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600" />
                    <label htmlFor="emigrate-radio-button" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">{"old device"}</label>
                </div>
                <div class="flex items-center">
                    <input checked={!*is_emigrate_visible} onchange={set_immigrate} id="immigrate-radio-button" type="radio" name="migrate-radio-buttons" class="h-4 w-4 border-gray-300 bg-gray-100 text-blue-600 focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600" />
                    <label htmlFor="immigrate-radio-button" class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">{"new device"}</label>
                </div>
            </div>
            {if *is_emigrate_visible { html! { <EmigratePanel /> } } else { html! { <ImmigratePanel /> } }}
        </BaseModal>
    }
}
