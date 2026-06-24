// Copyright (C) 2026 Jeryd
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

use crate::constants::config::{ENABLE_ARCHIVED_GAMES, GAME_TITLE};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NavbarProps {
    pub on_info_click: Callback<()>,
    pub on_stats_click: Callback<()>,
    pub on_date_click: Callback<()>,
    pub is_hard_mode: bool,
    pub on_hard_mode_click: Callback<()>,
    pub theme: String,
    pub on_theme_click: Callback<()>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let info_click = props.on_info_click.clone();
    let stats_click = props.on_stats_click.clone();
    let date_click = props.on_date_click.clone();
    let is_hard_mode = props.is_hard_mode;
    let hard_mode_click = props.on_hard_mode_click.clone();
    let theme = props.theme.clone();
    let theme_click = props.on_theme_click.clone();

    let date = crate::helpers::words::get_game_date();
    let holiday_info = crate::helpers::holidays::get_holiday_for_date(date);

    let theme_toggle_icon = if let Some((prefix, name)) = holiday_info {
        let emoji = match prefix {
            "newyear" => "🎉",
            "valentine" => "💖",
            "stpatrick" => "🍀",
            "easter" => "🥚",
            "independence" => "💥",
            "halloween" => "🎃",
            "thanksgiving" => "🦃",
            "christmas" => "🎄",
            _ => "🎁",
        };
        html! {
            <span class="text-xl h-6 w-6 inline-flex items-center justify-center cursor-pointer select-none" title={format!("{} - Toggle Light/Dark", name)}>
                {emoji}
            </span>
        }
    } else {
        match theme.as_str() {
            "brinstar" => html! {
                <svg id="leaf-icon" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Brinstar"><path d="M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 3.5 1 9.8a7 7 0 0 1-9 8.2Z" /><path d="M19 2 9.8 11.5" /></svg>
            },
            "norfair" => html! {
                <svg id="flame-icon" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Norfair"><path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" /></svg>
            },
            "wrecked_ship" => html! {
                <svg id="ghost-icon" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Wrecked Ship"><path d="M9 10h.01"/><path d="M15 10h.01"/><path d="M12 2a8 8 0 0 0-8 8v12l3-3 2.5 2.5L12 19l2.5 2.5L17 19l3 3V10a8 8 0 0 0-8-8z"/></svg>
            },
            "maridia" => html! {
                <svg id="waves-icon" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Maridia"><path d="M2 6c.6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1 .6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1 .6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1" /><path d="M2 12c.6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1 .6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1 .6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1" /><path d="M2 18c.6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1 .6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1 .6 0 1.2-.4 1.8-1 1.2-1.2 2.4-1.2 3.6 0 .6.6 1.2 1 1.8 1" /></svg>
            },
            "tourian" => html! {
                <svg id="target-icon" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Tourian"><circle cx="12" cy="12" r="10" /><circle cx="12" cy="12" r="6" /><circle cx="12" cy="12" r="2" /></svg>
            },
            _ => html! {
                <svg id="cloud-rain-icon" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Crateria"><path d="M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25" /><path d="M8 20v2" /><path d="M12 20v2" /><path d="M16 20v2" /></svg>
            },
        }
    };

    html! {
        <div class="navbar">
            <div class="navbar-content relative px-5 short:h-auto">
                <div class="flex">
                    <button class="focus:outline-none" onclick={move |_| info_click.emit(())} aria-label="Info">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 cursor-pointer dark:stroke-white">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
                        </svg>
                    </button>
                    {if ENABLE_ARCHIVED_GAMES {
                        html! {
                            <button class="ml-3 focus:outline-none" onclick={move |_| date_click.emit(())} aria-label="DatePicker">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 cursor-pointer dark:stroke-white">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5m-9-6h.008v.008H12v-.008zM12 15h.008v.008H12V15zm0 2.25h.008v.008H12v-.008zM9.75 15h.008v.008H9.75V15zm0 2.25h.008v.008H9.75v-.008zM7.5 15h.008v.008H7.5V15zm0 2.25h.008v.008H7.5v-.008zm6.75-4.5h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V15zm0 2.25h.008v.008h-.008v-.008zm2.25-4.5h.008v.008H16.5v-.008zm0 2.25h.008v.008H16.5V15z" />
                                </svg>
                            </button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
                <p class="absolute left-1/2 -translate-x-1/2 text-xl font-bold dark:text-white">{GAME_TITLE}</p>
                <div class="right-icons">
                    <button class="mr-3 focus:outline-none" onclick={move |_| stats_click.emit(())} aria-label="Stats">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="h-6 w-6 cursor-pointer dark:stroke-white">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
                        </svg>
                    </button>
                    <button class="mr-3 focus:outline-none" onclick={move |_| theme_click.emit(())} aria-label="Toggle theme">
                        {theme_toggle_icon}
                    </button>
                    <button class="focus:outline-none" onclick={move |_| hard_mode_click.emit(())} aria-label="Hard Mode">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 cursor-pointer" width="24" height="24" viewBox="0 0 24 24" fill={if is_hard_mode { "currentColor" } else { "none" }} stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" title="Hard Mode">
                            <path d="m2 4 3 12h14l3-12-6 7-4-7-4 7-6-7zm3 16h14" />
                        </svg>
                    </button>
                </div>
            </div>
            <hr />
        </div>
    }
}
