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

//! Settings dialog component.
//! Manages game controls like Hard Mode, Dark Mode, and High Contrast Mode toggles.

use crate::components::modal_base::BaseModal;
use crate::constants::config::{HARD_MODE_DESCRIPTION, HIGH_CONTRAST_MODE_DESCRIPTION};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SettingsToggleProps {
    pub setting_name: String,
    pub flag: bool,
    pub handle_flag: Callback<bool>,
    #[prop_or_default]
    pub description: Option<String>,
}

#[function_component(SettingsToggle)]
pub fn settings_toggle(props: &SettingsToggleProps) -> Html {
    let flag = props.flag;
    let handle_flag = props.handle_flag.clone();

    let toggle_holder = classes!(
        "w-14",
        "h-8",
        "flex",
        "shrink-0",
        "items-center",
        "bg-gray-300",
        "rounded-full",
        "p-1",
        "duration-300",
        "ease-in-out",
        "cursor-pointer",
        if flag { "bg-green-400" } else { "" }
    );
    let toggle_button = classes!(
        "bg-white",
        "w-6",
        "h-6",
        "rounded-full",
        "shadow-md",
        "transform",
        "duration-300",
        "ease-in-out",
        "cursor-pointer",
        if flag { "translate-x-6" } else { "" }
    );

    html! {
        <div class="flex justify-between gap-4 py-3">
            <div class="mt-2 text-left text-gray-500 dark:text-gray-300">
                <p class="leading-none">{&props.setting_name}</p>
                {if let Some(ref desc) = props.description {
                    html! {
                        <p class="mt-1 text-xs text-gray-500 dark:text-gray-300">
                            {desc}
                        </p>
                    }
                } else {
                    html! {}
                }}
            </div>
            <div class={toggle_holder} onclick={move |_| handle_flag.emit(!flag)}>
                <div class={toggle_button} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SettingsModalProps {
    pub is_open: bool,
    pub handle_close: Callback<()>,
    pub is_hard_mode: bool,
    pub handle_hard_mode: Callback<bool>,
    pub is_dark_mode: bool,
    pub handle_dark_mode: Callback<bool>,
    pub is_high_contrast_mode: bool,
    pub handle_high_contrast_mode: Callback<bool>,
    pub is_military_theme: bool,
    pub handle_military_theme: Callback<bool>,
}

#[function_component(SettingsModal)]
pub fn settings_modal(props: &SettingsModalProps) -> Html {
    html! {
        <BaseModal title="Settings" is_open={props.is_open} handle_close={props.handle_close.clone()}>
            <div class="mt-2 flex flex-col divide-y">
                <SettingsToggle
                    setting_name="Hard Mode"
                    flag={props.is_hard_mode}
                    handle_flag={props.handle_hard_mode.clone()}
                    description={Some(HARD_MODE_DESCRIPTION.to_string())}
                />
                <SettingsToggle
                    setting_name="Dark Mode"
                    flag={props.is_dark_mode}
                    handle_flag={props.handle_dark_mode.clone()}
                />
                <SettingsToggle
                    setting_name="High Contrast Mode"
                    flag={props.is_high_contrast_mode}
                    handle_flag={props.handle_high_contrast_mode.clone()}
                    description={Some(HIGH_CONTRAST_MODE_DESCRIPTION.to_string())}
                />
                <SettingsToggle
                    setting_name="Sci-Fi Military Theme"
                    flag={props.is_military_theme}
                    handle_flag={props.handle_military_theme.clone()}
                    description={Some("Enable green-phosphor retro terminal colors".to_string())}
                />
            </div>
        </BaseModal>
    }
}
