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

#![allow(unused_imports)]
//! UI component module declarations for the Rustle application.
//! Contains re-usable view components:
//! - Alerts: Toast style game notifications
//! - Grid: Wordle row and cell letter grid
//! - Keyboard: Interactive virtual letter keyboard
//! - Navbar: Navigation bar with action controls
//! - Modals: Collection of modular modal overlays for settings, stats, and info.
//! - Stat widgets: Isolated components for showing stats bars and histograms.
//! - AppModals: A consolidated container of all overlay dialogs.

pub mod alerts;
pub mod app_modals;
pub mod grid;
pub mod keyboard;
pub mod navbar;
pub mod stat_bar;
pub mod stat_histogram;
pub mod weather;
pub mod weather_engine;

// Modal components split into individual modular modules.
pub mod modal_base;
pub mod modal_date_picker;
pub mod modal_info;
pub mod modal_migrate;
pub mod modal_stats;

// Re-export major modal components for cleaner usage in other modules.
pub use app_modals::AppModals;
pub use modal_base::BaseModal;
pub use modal_date_picker::DatePickerModal;
pub use modal_info::InfoModal;
pub use modal_migrate::MigrateStatsModal;
pub use modal_stats::StatsModal;
pub use weather::WeatherContainer;
