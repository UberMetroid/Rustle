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

//! Base modal component overlay wrapper.
//! Manages modal overlays, animations, accessibility focus, and close action triggers.

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BaseModalProps {
    pub title: String,
    pub children: Html,
    pub is_open: bool,
    pub handle_close: Callback<()>,
}

#[function_component(BaseModal)]
pub fn base_modal(props: &BaseModalProps) -> Html {
    let handle_close_backdrop = props.handle_close.clone();
    let handle_close_button = props.handle_close.clone();
    let title = props.title.clone();
    let children = props.children.clone();
    let is_open = props.is_open;

    let classes = classes!(
        "fixed",
        "inset-0",
        "z-10",
        "overflow-y-auto",
        if is_open {
            "block"
        } else {
            "hidden pointer-events-none"
        }
    );

    let backdrop_classes = classes!(
        "fixed",
        "inset-0",
        "min-h-screen",
        "bg-gray-500",
        "bg-opacity-75",
        "transition-opacity",
        "duration-300"
    );

    html! {
        <div class={classes}>
            <div class="flex min-h-full items-center justify-center py-10 px-4 text-center sm:p-0">
                <div class={backdrop_classes} onclick={move |_| handle_close_backdrop.emit(())} />
                <div class="inline-block transform overflow-hidden rounded-lg bg-white px-4 pt-5 pb-4 text-left align-bottom shadow-xl transition-all dark:bg-gray-800 sm:my-8 sm:w-full sm:max-w-sm sm:p-6 sm:align-middle z-20 relative">
                    <button
                        onclick={move |_| handle_close_button.emit(())}
                        tabindex=0
                        aria-pressed="false"
                        class="absolute right-4 top-4 focus:outline-none"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 cursor-pointer dark:stroke-white">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </button>
                    <div>
                        <div class="text-center">
                            <h3 class="text-lg font-medium leading-6 text-gray-900 dark:text-gray-100">
                                {title}
                            </h3>
                            <div class="mt-2">{children}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
