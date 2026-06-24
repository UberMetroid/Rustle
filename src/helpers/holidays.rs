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

//! Holiday detection and theme mapping logic.

use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// Checks if a date falls within a holiday spread, returning its prefix and user-facing name.
pub fn get_holiday_for_date(date: NaiveDate) -> Option<(&'static str, &'static str)> {
    let year = date.year();
    let month = date.month();
    let day = date.day();

    // 1. New Year's (Dec 31 - Jan 1)
    if (month == 12 && day == 31) || (month == 1 && day == 1) {
        return Some(("newyear", "New Year's"));
    }

    // 2. Valentine's Day (Feb 12 - Feb 14)
    if month == 2 && (12..=14).contains(&day) {
        return Some(("valentine", "Valentine's Day"));
    }

    // 3. St. Patrick's Day (Mar 15 - Mar 17)
    if month == 3 && (15..=17).contains(&day) {
        return Some(("stpatrick", "St. Patrick's Day"));
    }

    // 4. Easter (Good Friday to Easter Monday)
    let easter = get_easter_sunday(year);
    if let (Some(good_friday), Some(easter_monday)) = (
        easter.checked_sub_signed(Duration::days(2)),
        easter.checked_add_signed(Duration::days(1)),
    ) {
        if date >= good_friday && date <= easter_monday {
            return Some(("easter", "Easter"));
        }
    }

    // 5. Independence Day / Summer (Jul 3 - Jul 5)
    if month == 7 && (3..=5).contains(&day) {
        return Some(("independence", "Independence Day"));
    }

    // 6. Halloween (Oct 25 - Oct 31)
    if month == 10 && (25..=31).contains(&day) {
        return Some(("halloween", "Halloween"));
    }

    // 7. Thanksgiving (US: 4th Thursday in Nov to Sunday)
    let thanksgiving = get_thanksgiving_thursday(year);
    if let Some(thanksgiving_sunday) = thanksgiving.checked_add_signed(Duration::days(3)) {
        if date >= thanksgiving && date <= thanksgiving_sunday {
            return Some(("thanksgiving", "Thanksgiving"));
        }
    }

    // 8. Christmas (Dec 20 - Dec 26)
    if month == 12 && (20..=26).contains(&day) {
        return Some(("christmas", "Christmas"));
    }

    None
}

/// Helper to check if a theme string belongs to a holiday.
pub fn is_holiday_theme(theme: &str) -> bool {
    theme.starts_with("newyear-")
        || theme.starts_with("valentine-")
        || theme.starts_with("stpatrick-")
        || theme.starts_with("easter-")
        || theme.starts_with("independence-")
        || theme.starts_with("halloween-")
        || theme.starts_with("thanksgiving-")
        || theme.starts_with("christmas-")
}

/// Meeus/Jones/Butcher algorithm for Easter Sunday (Gregorian calendar).
fn get_easter_sunday(year: i32) -> NaiveDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = ((h + l - 7 * m + 114) / 31) as u32;
    let day = (((h + l - 7 * m + 114) % 31) + 1) as u32;
    NaiveDate::from_ymd_opt(year, month, day).unwrap_or_default()
}

/// Calculates US Thanksgiving (4th Thursday of November).
fn get_thanksgiving_thursday(year: i32) -> NaiveDate {
    let first_of_nov = NaiveDate::from_ymd_opt(year, 11, 1).unwrap_or_default();
    let mut date = first_of_nov;
    while date.weekday() != Weekday::Thu {
        if let Some(next) = date.succ_opt() {
            date = next;
        } else {
            break;
        }
    }
    date + Duration::days(21)
}
