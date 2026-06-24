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

//! Date picker modal component for selecting archived games.
//! Allows players to choose a past date to replay archived puzzles.

use crate::components::modal_base::BaseModal;
use crate::constants::config::{DATEPICKER_CHOOSE_TEXT, DATEPICKER_TITLE};
use crate::helpers::words::set_game_date;
use chrono::NaiveDate;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DatePickerModalProps {
    pub is_open: bool,
    pub handle_close: Callback<()>,
}

#[function_component(DatePickerModal)]
pub fn date_picker_modal(props: &DatePickerModalProps) -> Html {
    let date_ref = use_node_ref();
    let on_choose = {
        let date_ref = date_ref.clone();
        let handle_close = props.handle_close.clone();
        Callback::from(move |_| {
            if let Some(input) = date_ref.cast::<web_sys::HtmlInputElement>() {
                let val = input.value();
                if !val.is_empty() {
                    if let Ok(d) = NaiveDate::parse_from_str(&val, "%Y-%m-%d") {
                        set_game_date(d);
                        handle_close.emit(());
                    }
                }
            }
        })
    };

    html! {
        <BaseModal title={DATEPICKER_TITLE.to_string()} is_open={props.is_open} handle_close={props.handle_close.clone()}>
            <div class="mt-4 flex flex-col items-center">
                <input
                    ref={date_ref}
                    type="date"
                    class="block w-full rounded border border-gray-300 bg-white p-2 text-base dark:bg-gray-700 dark:text-white"
                />
                <button
                    onclick={on_choose}
                    class="mt-4 inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-center text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none sm:text-base font-bold w-full"
                >
                    {DATEPICKER_CHOOSE_TEXT}
                </button>
            </div>
        </BaseModal>
    }
}
