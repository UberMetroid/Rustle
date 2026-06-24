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

//! Browser helper utility functions.
//! Detects custom WebView runtime environments and in-app browsers.

use web_sys::window;

/// Detects if the page is running inside a known social media or in-app web view.
/// This includes platforms like Facebook, Instagram, Twitter/X, Messenger, and Line.
/// Many in-app browsers have restricted API access, limiting storage or sharing features.
///
/// # Returns
/// `true` if running inside a restricted in-app browser view, `false` otherwise.
pub fn is_in_app_browser() -> bool {
    if let Some(win) = window() {
        if let Ok(ua) = win.navigator().user_agent() {
            let ua = ua.to_uppercase();

            // Check for common in-app browser signatures in User-Agent header
            let is_fb = ua.contains("FBAN") || ua.contains("FBAV");
            let is_ig = ua.contains("INSTAGRAM");
            let is_messenger = ua.contains("MESSENGER");
            let is_twitter = ua.contains("TWITTER");
            let is_line = ua.contains("LINE");

            return is_fb || is_ig || is_messenger || is_twitter || is_line;
        }
    }
    false
}
