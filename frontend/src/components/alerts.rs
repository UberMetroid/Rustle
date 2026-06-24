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

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AlertProps {
    pub message: String,
    pub is_visible: bool,
    pub variant: String, // "success" or "error"
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    let classes = classes!(
        "fixed",
        "z-50",
        "top-14",
        "left-1/2",
        "transform",
        "-translate-x-1/2",
        "max-w-sm",
        "shadow-lg",
        "rounded-lg",
        "pointer-events-auto",
        "ring-1",
        "ring-black",
        "ring-opacity-5",
        "overflow-hidden",
        "transition-opacity",
        "duration-300",
        if props.is_visible {
            "opacity-100"
        } else {
            "opacity-0 pointer-events-none"
        },
        if props.variant == "success" {
            "bg-blue-500 text-white"
        } else {
            "bg-rose-500 text-white"
        }
    );

    html! {
        <div class={classes}>
            <div class="p-2">
                <p class="text-center text-sm font-medium">{props.message.clone()}</p>
            </div>
        </div>
    }
}
