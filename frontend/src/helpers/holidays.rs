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
    theme == "newyear"
        || theme == "valentine"
        || theme == "stpatrick"
        || theme == "easter"
        || theme == "independence"
        || theme == "halloween"
        || theme == "thanksgiving"
        || theme == "christmas"
}

/// Returns a holiday-specific 5-letter word for the given puzzle index.
/// Scrambles the index using a seeded Linear Congruential Generator step to make word choices
/// non-sequential and unpredictable, while remaining completely deterministic.
pub fn get_holiday_word(prefix: &str, index: i32) -> &'static str {
    let list: &[&str] = match prefix {
        "newyear" => &[
            "BEERS", "BEGIN", "BELLS", "BOOZE", "BOOZY", "BURST", "CHAMP", "CHEER", "CHIME",
            "CLASS", "CLEAN", "CLOCK", "COUNT", "CROWD", "CROWN", "CUPPY", "DANCE", "DREAM",
            "DRINK", "EVENT", "FAITH", "FESTS", "FESTY", "FIRST", "FLASH", "FLUTE", "FRESH",
            "FROST", "GLASS", "GLORY", "GLOWS", "GOALS", "GRACE", "GREET", "HAPPY", "LAGER",
            "LIGHT", "LUCKY", "MIDST", "MUSIC", "NIGHT", "NOISE", "PARTY", "PEACE", "PEALS",
            "PROUD", "SHARE", "SHINE", "SMILE", "SMOKE", "SPARK", "STAGE", "START", "STEEL",
            "SWEET", "TIMES", "TOAST", "VALOR", "WATCH", "WINDY", "WINES", "YEARS", "YOUTH",
        ],
        "valentine" => &[
            "ADORE", "AMOUR", "ANGEL", "BEAUT", "BLUSH", "BOOST", "BRIDE", "CANDY", "CARDS",
            "CHARM", "CHEEK", "CUPID", "DATES", "DEARY", "DREAM", "FAITH", "FANCY", "FEELS",
            "FEVER", "FLAME", "FLESH", "FLIRT", "GIFTS", "GLIDE", "GLOWS", "GRACE", "GREAT",
            "GROOM", "HEART", "HONEY", "HUGGY", "KISSY", "LOVED", "LOVER", "LOVES", "LUCKY",
            "MARRY", "MERRY", "PEACE", "PEACH", "PEARL", "PINKS", "POEMS", "PRIDE", "PULSE",
            "REDDY", "ROSES", "SHINE", "SMILE", "SWEET", "TIARA", "TOUCH", "TRUST", "UNION",
            "VALES", "VERSE", "VOICE", "WARMS", "WRITE",
        ],
        "stpatrick" => &[
            "BEERS", "BOOZE", "BOOZY", "CELTS", "CHARM", "CHEER", "CIDER", "CLANS", "CLOUD",
            "CLOVE", "COINS", "CRAFT", "CROSS", "CROWN", "CUPPY", "DANCE", "DRINK", "FAIRY",
            "FEAST", "FIELD", "FLUTE", "FOLKS", "GLORY", "GOLDS", "GOLDY", "GRASS", "GREEN",
            "HAPPY", "HARPS", "HEART", "HILLY", "LAGER", "LEAFY", "LUCKS", "LUCKY", "MAGIC",
            "MARCH", "MERRY", "PATTY", "PEACE", "PINTS", "PIPER", "PIPES", "POTTY", "RAINY",
            "RIVER", "ROADS", "SALTS", "SHAME", "SHEEN", "SHIRE", "SHORE", "SNAKE", "SONGS",
            "STEEL", "STONE", "STORY", "STOUT", "SWEET", "TOADS", "TOAST", "TUNES", "VALOR",
        ],
        "easter" => &[
            "ALIVE", "ALTAR", "ANGEL", "ARISE", "BIBLE", "BIRTH", "BLOOM", "BUNNY", "CANDY",
            "CHICK", "CHIRP", "CHOIR", "CLOUD", "CLOVE", "COCOA", "COLOR", "CROSS", "DAISY",
            "DYING", "EARTH", "FAITH", "FIELD", "FLORA", "FRESH", "GIFTS", "GLORY", "GLOWS",
            "GRACE", "GRASS", "GREEN", "HAPPY", "HARES", "HONOR", "HOPPY", "HYMNS", "JELLY",
            "JUMPS", "LAMBS", "LEAPS", "LIGHT", "LILAC", "MERCY", "NESTS", "PAINT", "PEACE",
            "PEACH", "PETAL", "PLANT", "PRAYS", "PRIME", "RABBI", "RISEN", "RISES", "ROOTS",
            "ROSES", "SEEDS", "SHEEP", "SHINE", "SPRIG", "START", "SWEET", "TULIP", "WARMS",
            "WHITE", "YOUTH",
        ],
        "independence" => &[
            "BANDS", "BEACH", "BEERS", "BLAST", "BOOMS", "BOOMY", "BRASS", "BRAVE", "BURST",
            "CHIPS", "CIVIC", "CROWD", "DANCE", "DRINK", "DRUMS", "EAGLE", "FESTS", "FIRES",
            "FLAGS", "FLAME", "FLASH", "FLUTE", "FREED", "FREER", "FREES", "GLORY", "GLOWS",
            "GREAT", "GRILL", "HAPPY", "HONOR", "LAKES", "LANDS", "LIGHT", "MARCH", "MEATS",
            "MUSIC", "NIGHT", "OCEAN", "PARTY", "PEACE", "PIPES", "PRIDE", "PROUD", "RIVER",
            "SALAD", "SHINE", "SHORE", "SMOKE", "SPARK", "STARS", "STATE", "STEAK", "STRIP",
            "SUNNS", "SUNNY", "SWIMS", "TOAST", "UNION", "UNITE", "VALOR", "WATER", "WHITE",
            "WINES",
        ],
        "halloween" => &[
            "BATTY", "BEAST", "BLACK", "BLOOD", "BONES", "BONEY", "CANDY", "CLAWS", "CLOWN",
            "CORPS", "CREEP", "CROWS", "CRYPT", "CURSE", "DARKS", "DEATH", "DEMON", "DEVIL",
            "DREAD", "FANGS", "FOGGY", "GHOST", "GHOUL", "GLOOM", "GRAVE", "HAUNT", "HOWLS",
            "MAGIC", "MASKS", "MISTS", "MISTY", "MOANS", "MUMMY", "NIGHT", "OOZES", "SCARE",
            "SCARY", "SHADE", "SHADY", "SHARK", "SHOCK", "SKULL", "SLIME", "SLIMY", "SPELL",
            "SPIDE", "SPOOK", "STORM", "SWEET", "TOMBS", "TREAT", "WINDY", "WITCH", "WOLFS",
        ],
        "thanksgiving" => &[
            "AMBER", "APPLE", "BAKES", "BERRY", "BLESS", "BREAD", "BROWN", "CARVE", "CHEER",
            "CHILL", "CIDER", "COLDS", "COOKS", "CROWD", "CRUST", "DINER", "FAITH", "FALLS",
            "FEAST", "FLOUR", "FROST", "GIFTS", "GOLDS", "GOLDY", "GRACE", "GRAIN", "GRAVY",
            "GROUP", "GUEST", "HAPPY", "HEART", "HOMEY", "HONEY", "LEAFY", "LOVES", "MAIZE",
            "PEACE", "PEARS", "PLATE", "PLUMS", "PRAYS", "PRIDE", "ROAST", "SAUCE", "SHARE",
            "SMELL", "SPICE", "SUGAR", "SWEET", "TABLE", "TASTE", "THANK", "WARMS", "WHEAT",
            "WINES",
        ],
        "christmas" => &[
            "ANGEL", "BEAMS", "BELLS", "BIRTH", "CABIN", "CAKES", "CANDY", "CAROL", "CHILL",
            "CHIME", "CHOIR", "CHUTE", "COLDS", "ELVES", "FAITH", "FIRES", "FROST", "GIFTS",
            "GIVER", "GIVES", "GLORY", "GLOWS", "GOLDS", "GOLDY", "GRACE", "GREEN", "GUEST",
            "HAPPY", "HOLLY", "HOMES", "HOMEY", "HYMNS", "ICIER", "ICING", "JESUS", "JOLLY",
            "LIGHT", "MERRY", "NORTH", "PEACE", "PEALS", "PINES", "POLES", "SHARE", "SHINE",
            "SINGS", "SLEET", "SNOWS", "SNOWY", "STARS", "SWEET", "TREES", "WHITE",
        ],
        _ => &[],
    };

    if list.is_empty() {
        return "";
    }

    let seed = index as i64 as u64;
    let multiplier: u64 = 6364136223846793005;
    let increment: u64 = 1442695040888963407;
    let scrambled = seed.wrapping_mul(multiplier).wrapping_add(increment);
    let target_idx = scrambled as usize % list.len();

    list[target_idx]
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
