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

use crate::constants::config::{MAX_CHALLENGES, REVEAL_TIME_MS};
use crate::helpers::statuses::{get_guess_statuses, CharStatus};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CellProps {
    #[prop_or_default]
    pub value: Option<char>,
    #[prop_or_default]
    pub status: Option<CharStatus>,
    #[prop_or_default]
    pub is_revealing: bool,
    #[prop_or_default]
    pub is_completed: bool,
    #[prop_or_default]
    pub position: usize,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let value = props.value;
    let status = props.status;
    let is_revealing = props.is_revealing;
    let is_completed = props.is_completed;
    let position = props.position;

    let is_filled = value.is_some() && !is_completed;
    let should_reveal = is_revealing && is_completed;
    let animation_delay = format!("{}ms", position * REVEAL_TIME_MS as usize);

    let state_class = match status {
        None => {
            if value.is_some() {
                "cell-filled"
            } else {
                "cell-empty"
            }
        }
        Some(CharStatus::Correct) => "correct shadowed",
        Some(CharStatus::Present) => "present shadowed",
        Some(CharStatus::Absent) => "absent shadowed",
    };

    let cell_classes = classes!(
        "xxshort:w-11",
        "xxshort:h-11",
        "short:text-2xl",
        "short:w-12",
        "short:h-12",
        "w-[17vw]",
        "h-[17vw]",
        "max-w-[66px]",
        "max-h-[66px]",
        "sm:w-14",
        "sm:h-14",
        "border-solid",
        "border-2",
        "flex",
        "items-center",
        "justify-center",
        "mx-0.5",
        "text-3xl",
        "sm:text-4xl",
        "font-bold",
        "rounded",
        "cell",
        state_class,
        if is_filled { "cell-fill-animation" } else { "" },
        if should_reveal { "cell-reveal" } else { "" }
    );

    let delay_style = format!(
        "animation-delay: {}; -webkit-animation-delay: {};",
        animation_delay, animation_delay
    );

    html! {
        <div class={cell_classes} style={delay_style.clone()}>
            <div class="letter-container" style={delay_style}>
                {if let Some(c) = value { html! { {c} } } else { html! {} }}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CompletedRowProps {
    pub solution: String,
    pub guess: String,
    pub is_revealing: bool,
}

#[function_component(CompletedRow)]
pub fn completed_row(props: &CompletedRowProps) -> Html {
    let statuses = get_guess_statuses(&props.solution, &props.guess);

    html! {
        <div class="flex justify-center mb-1">
            {for props.guess.chars().enumerate().map(|(i, val)| {
                html! {
                    <Cell
                        value={Some(val)}
                        status={Some(statuses[i])}
                        is_revealing={props.is_revealing}
                        is_completed=true
                        position={i}
                    />
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CurrentRowProps {
    pub guess: String,
    pub class_name: String,
}

#[function_component(CurrentRow)]
pub fn current_row(props: &CurrentRowProps) -> Html {
    let guess_len = props.guess.chars().count();
    let empties_count = 5 - guess_len.min(5);

    html! {
        <div class={classes!("flex", "justify-center", "mb-1", props.class_name.clone())}>
            {for props.guess.chars().map(|val| {
                html! { <Cell value={Some(val)} /> }
            })}
            {for (0..empties_count).map(|_| {
                html! { <Cell /> }
            })}
        </div>
    }
}

#[function_component(EmptyRow)]
pub fn empty_row() -> Html {
    html! {
        <div class="flex justify-center mb-1">
            {for (0..5).map(|_| html! { <Cell /> })}
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct GridProps {
    pub solution: String,
    pub guesses: Vec<String>,
    pub current_guess: String,
    pub is_revealing: bool,
    pub current_row_class_name: String,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let guesses_len = props.guesses.len();
    let empties_count = (MAX_CHALLENGES - 1).saturating_sub(guesses_len);

    html! {
        <div id="game-grid" class="flex flex-col justify-center mt-8 sm:mt-0 pb-1 sm:pb-2">
            {for props.guesses.iter().enumerate().map(|(i, g)| {
                html! {
                    <CompletedRow
                        solution={props.solution.clone()}
                        guess={g.clone()}
                        is_revealing={props.is_revealing && guesses_len - 1 == i}
                    />
                }
            })}
            {if guesses_len < MAX_CHALLENGES {
                html! {
                    <CurrentRow
                        guess={props.current_guess.clone()}
                        class_name={props.current_row_class_name.clone()}
                    />
                }
            } else {
                html! {}
            }}
            {for (0..empties_count).map(|_| {
                html! { <EmptyRow /> }
            })}
        </div>
    }
}
