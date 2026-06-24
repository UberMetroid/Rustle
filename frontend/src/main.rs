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

//! Rustle Game Application
//!
//! An open-source clone of Wordle built using Rust, Yew, and WebAssembly.
//!
//! # Architecture Overview
//! - `constants`: Houses the zero-allocation solution database, guesses list, and UI strings.
//! - `components`: Interactive UI widgets (Keyboard, Grid, Modals, Navbar, Alerts).
//! - `lib`: Business logic engines (Local Storage state persistence, Share emoji builder).
//! - `app_state`: Game state reducer and action definitions.
//! - `app_effects`: Side effects hook containing page styles and settings setups.
//! - `app`: Main layout router and coordinator that renders Yew views.
//!
//! # Compilation & Bootstrap
//! The application compiles to WebAssembly (`wasm32-unknown-unknown`) using Trunk.
//! The `main` function bootstraps the client application onto the page root DOM.

mod app;
mod app_effects;
mod app_state;
mod components;
mod constants;
mod helpers;

use app::App;

/// Main entry point of the Rustle client WASM application.
/// Instantiates the root `App` component and binds it to the browser's document body.
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);

    yew::Renderer::<App>::new().render();
}
