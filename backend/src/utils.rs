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

use std::path::Path;

pub fn get_holiday_for_date(date: chrono::NaiveDate) -> Option<(&'static str, &'static str)> {
    use chrono::Datelike;
    let year = date.year();
    let month = date.month();
    let day = date.day();

    if (month == 12 && day == 31) || (month == 1 && day == 1) {
        return Some(("newyear", "New Year's"));
    }
    if month == 2 && (12..=14).contains(&day) {
        return Some(("valentine", "Valentine's Day"));
    }
    if month == 3 && (15..=17).contains(&day) {
        return Some(("stpatrick", "St. Patrick's Day"));
    }
    let easter = get_easter_sunday(year);
    if let (Some(good_friday), Some(easter_monday)) = (
        easter.checked_sub_signed(chrono::Duration::days(2)),
        easter.checked_add_signed(chrono::Duration::days(1)),
    ) {
        if date >= good_friday && date <= easter_monday {
            return Some(("easter", "Easter"));
        }
    }
    if month == 7 && (3..=5).contains(&day) {
        return Some(("independence", "Independence Day"));
    }
    if month == 10 && (25..=31).contains(&day) {
        return Some(("halloween", "Halloween"));
    }
    let thanksgiving = get_thanksgiving_thursday(year);
    if let Some(thanksgiving_sunday) = thanksgiving.checked_add_signed(chrono::Duration::days(3)) {
        if date >= thanksgiving && date <= thanksgiving_sunday {
            return Some(("thanksgiving", "Thanksgiving"));
        }
    }
    if month == 12 && (20..=26).contains(&day) {
        return Some(("christmas", "Christmas"));
    }
    None
}

pub fn get_easter_sunday(year: i32) -> chrono::NaiveDate {
    // Meeus/Jones/Butcher Gregorian Easter algorithm.
    // Calculations determine the date of Easter Sunday based on lunar cycles and solar year alignment.
    let a = year % 19; // Cycle position in the 19-year Metonic cycle
    let b = year / 100; // Century index
    let c = year % 100; // Year within the century
    let d = b / 4; // Leap years in century cycle
    let e = b % 4;
    let f = (b + 8) / 25; // Proemptosis correction for lunar orbit shift
    let g = (b - f + 1) / 3; // Metonic cycle lunar alignment correction
    let h = (19 * a + b - d - g + 15) % 30; // Epact (age of the moon on Jan 1)
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7; // Day of the week offset
    let m = (a + 11 * h + 22 * l) / 451; // Correction for April double cycle
    let month = ((h + l - 7 * m + 114) / 31) as u32; // Determined month (3 for March, 4 for April)
    let day = (((h + l - 7 * m + 114) % 31) + 1) as u32; // Determined day of the month
    chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap_or_default()
}

pub fn get_thanksgiving_thursday(year: i32) -> chrono::NaiveDate {
    use chrono::Datelike;
    use chrono::Weekday;
    let first_of_nov = chrono::NaiveDate::from_ymd_opt(year, 11, 1).unwrap_or_default();
    let mut date = first_of_nov;
    while date.weekday() != Weekday::Thu {
        if let Some(next) = date.succ_opt() {
            date = next;
        } else {
            break;
        }
    }
    date + chrono::Duration::days(21)
}

pub fn get_files_recursive(dir: &Path, base: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_files_recursive(&path, base));
            } else if let Ok(rel) = path.strip_prefix(base) {
                if let Some(s) = rel.to_str() {
                    let url = format!("/{}", s.replace('\\', "/"));
                    files.push(url);
                }
            }
        }
    }
    files
}

pub fn build_asset_manifest() -> Vec<String> {
    let dist_path = Path::new("dist");
    let mut files = get_files_recursive(dist_path, dist_path);
    if !files.contains(&"/favicon.png".to_string()) {
        files.push("/favicon.png".to_string());
    }
    if !files.contains(&"/public/favicon.png".to_string()) {
        files.push("/public/favicon.png".to_string());
    }
    if !files.contains(&"/public/manifest.json".to_string()) {
        files.push("/public/manifest.json".to_string());
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_get_holiday_for_date_new_year() {
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
            Some(("newyear", "New Year's"))
        );
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
            Some(("newyear", "New Year's"))
        );
    }

    #[test]
    fn test_get_holiday_for_date_independence() {
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 7, 4).unwrap()),
            Some(("independence", "Independence Day"))
        );
    }

    #[test]
    fn test_get_holiday_for_date_christmas() {
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 12, 25).unwrap()),
            Some(("christmas", "Christmas"))
        );
    }

    #[test]
    fn test_get_holiday_for_date_easter_2026() {
        // Easter Sunday 2026 is April 5.
        // Good Friday is April 3, Easter Monday is April 6.
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 4, 3).unwrap()),
            Some(("easter", "Easter"))
        );
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 4, 5).unwrap()),
            Some(("easter", "Easter"))
        );
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 4, 6).unwrap()),
            Some(("easter", "Easter"))
        );
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 4, 2).unwrap()),
            None
        );
    }

    #[test]
    fn test_get_holiday_for_date_thanksgiving_2026() {
        // Thanksgiving 2026 is November 26, Sunday is November 29.
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 11, 26).unwrap()),
            Some(("thanksgiving", "Thanksgiving"))
        );
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 11, 29).unwrap()),
            Some(("thanksgiving", "Thanksgiving"))
        );
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 11, 25).unwrap()),
            None
        );
    }

    #[test]
    fn test_get_holiday_for_date_halloween() {
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 10, 31).unwrap()),
            Some(("halloween", "Halloween"))
        );
    }

    #[test]
    fn test_get_holiday_for_date_none() {
        assert_eq!(
            get_holiday_for_date(NaiveDate::from_ymd_opt(2026, 8, 15).unwrap()),
            None
        );
    }
}
