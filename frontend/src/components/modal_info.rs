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

//! Instructions dialog component showing Wordle game instructions.
//! Explains cell color semantics (correct letter position, wrong spot, absent).

use crate::components::grid::Cell;
use crate::components::modal_base::BaseModal;
use crate::helpers::statuses::CharStatus;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InfoModalProps {
    pub is_open: bool,
    pub handle_close: Callback<()>,
}

#[function_component(InfoModal)]
pub fn info_modal(props: &InfoModalProps) -> Html {
    html! {
        <BaseModal title="How to play" is_open={props.is_open} handle_close={props.handle_close.clone()}>
            <p class="text-sm text-gray-500 dark:text-gray-300">
                {"Guess the 5-letter word in 6 tries. After each guess, the color of the tiles will change to show how close your guess was to the word."}
            </p>

            <div class="mb-1 mt-4 flex justify-center">
                <Cell is_revealing=true is_completed=true value={Some('W')} status={Some(CharStatus::Correct)} />
                <Cell value={Some('E')} is_completed=true />
                <Cell value={Some('A')} is_completed=true />
                <Cell value={Some('R')} is_completed=true />
                <Cell value={Some('Y')} is_completed=true />
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-300">
                {"The letter W is in the word and in the correct spot."}
            </p>

            <div class="mb-1 mt-4 flex justify-center">
                <Cell value={Some('P')} is_completed=true />
                <Cell value={Some('I')} is_completed=true />
                <Cell is_revealing=true is_completed=true value={Some('L')} status={Some(CharStatus::Present)} />
                <Cell value={Some('O')} is_completed=true />
                <Cell value={Some('T')} is_completed=true />
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-300">
                {"The letter L is in the word but in the wrong spot."}
            </p>

            <div class="mb-1 mt-4 flex justify-center">
                <Cell value={Some('V')} is_completed=true />
                <Cell value={Some('A')} is_completed=true />
                <Cell value={Some('G')} is_completed=true />
                <Cell is_revealing=true is_completed=true value={Some('U')} status={Some(CharStatus::Absent)} />
                <Cell value={Some('E')} is_completed=true />
            </div>
            <p class="text-sm text-gray-500 dark:text-gray-300">
                {"The letter U is not in the word in any spot."}
            </p>

            <p class="mt-6 text-sm italic text-gray-500 dark:text-gray-300">
                {"This is an open source version of the word guessing game we all know and love - "}
                <a href="https://github.com/UberMetroid/rustle" class="font-bold underline" target="_blank" rel="noopener noreferrer">
                    {"check out the code here"}
                </a>
            </p>
        </BaseModal>
    }
}
