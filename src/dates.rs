//! Calendar and timestamp readings, mirroring the Python date and
//! time behaviour the ingest and rulings modules stand on.
//!
//! A timestamp is read into an instant in microseconds so two
//! timestamps compare the way the Python datetimes compare, and the
//! UTC calendar date of an instant is read off that instant, so an
//! offset timestamp is judged on the day it names in UTC. A calendar
//! date is the strict hyphenated ISO form alone.
//!
//! The timestamp reading carries the grammar of the Python
//! fromisoformat reading as the interpreter on this system reads it,
//! found by probing that reading shape by shape. The date stands in
//! the extended form or the basic form, as a calendar date or as an
//! ISO week date, the separator between the date and the time is the
//! one character standing there whatever it is, and the time stands
//! in the extended form or the basic form with a fraction allowed
//! after the last component present.

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub fn valid_calendar_date(year: i64, month: i64, day: i64) -> bool {
    (1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month)
}

/// Days from the civil epoch for a calendar date, by the standard
/// civil calendar arithmetic.
pub fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = if shifted >= 0 { shifted } else { shifted - 399 } / 400;
    let year_of_era = shifted - era * 400;
    let month_prime = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * month_prime + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146097 + day_of_era - 719468
}

/// The calendar date standing at a day number, as year, month, day.
pub fn civil_from_days(day_number: i64) -> (i64, i64, i64) {
    let shifted = day_number + 719468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146096
    } / 146097;
    let day_of_era = shifted - era * 146097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = if month_prime < 10 {
        month_prime + 3
    } else {
        month_prime - 9
    };
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// Floor division for possibly negative values.
fn floor_div(value: i128, divisor: i128) -> i128 {
    let quotient = value / divisor;
    if value % divisor != 0 && ((value < 0) != (divisor < 0)) {
        quotient - 1
    } else {
        quotient
    }
}

/// The UTC day number an instant in microseconds stands on.
pub fn utc_day_of_instant(microseconds: i128) -> i64 {
    floor_div(microseconds, 86_400_000_000) as i64
}

/// The ISO weekday of a day number, Monday standing at one and
/// Sunday at seven.
fn iso_weekday(day_number: i64) -> i64 {
    (day_number + 3).rem_euclid(7) + 1
}

/// The day number of the Monday of ISO week one of a year, the
/// Monday of the week holding the fourth of January of that year.
fn iso_week_one_monday(year: i64) -> i64 {
    let jan_fourth = days_from_civil(year, 1, 4);
    jan_fourth - (iso_weekday(jan_fourth) - 1)
}

/// Read an ISO week date into a day number, refusing a week the
/// year does not hold. A week date stands in its year when the
/// Thursday of its week falls in that year.
fn iso_week_day_number(year: i64, week: i64, weekday: i64) -> Option<i64> {
    if !(1..=53).contains(&week) || !(1..=7).contains(&weekday) {
        return None;
    }
    let candidate = iso_week_one_monday(year) + (week - 1) * 7 + (weekday - 1);
    let thursday = candidate + (4 - weekday);
    if civil_from_days(thursday).0 == year {
        Some(candidate)
    } else {
        None
    }
}

/// Read a run of exactly the given count of ASCII digits.
fn take_ndigits(chars: &[char], pos: usize, count: usize) -> Option<i64> {
    let mut value: i64 = 0;
    for index in 0..count {
        let ch = *chars.get(pos + index)?;
        if !ch.is_ascii_digit() {
            return None;
        }
        value = value * 10 + (ch as u32 - '0' as u32) as i64;
    }
    Some(value)
}

/// Find where the date ends and the separator stands in a timestamp
/// text, as the Python reading finds it. The date the finding names
/// is seven or eight or ten characters long, and a finding equal to
/// the length of the text means the text is a date alone.
fn find_datetime_separator(chars: &[char]) -> Option<usize> {
    let len = chars.len();
    if len == 7 {
        return Some(7);
    }
    if chars[4] == '-' {
        if chars[5] == 'W' {
            if len > 8 && chars[8] == '-' {
                if len == 9 {
                    return None;
                }
                if len > 10 && chars[10].is_ascii_digit() {
                    return Some(8);
                }
                return Some(10);
            }
            return Some(8);
        }
        return Some(10);
    }
    if chars[4] == 'W' {
        let mut idx = 7;
        while idx < len && chars[idx].is_ascii_digit() {
            idx += 1;
        }
        if idx < 9 {
            return Some(idx);
        }
        return Some(if idx % 2 == 0 { 7 } else { 8 });
    }
    Some(8)
}

/// Read the date opening a timestamp into a day number. The date
/// text is seven or eight or ten characters, the finding above
/// having bounded it, and it stands as a calendar date or as an ISO
/// week date in the extended form or the basic form, and in no
/// reduced precision form.
fn parse_iso_date(chars: &[char]) -> Option<i64> {
    if !matches!(chars.len(), 7 | 8 | 10) {
        return None;
    }
    let year = take_ndigits(chars, 0, 4)?;
    if !(1..=9999).contains(&year) {
        return None;
    }
    let has_sep = chars[4] == '-';
    let mut pos = 4 + has_sep as usize;
    if chars.get(pos) == Some(&'W') {
        let week = take_ndigits(chars, pos + 1, 2)?;
        pos += 3;
        let mut weekday = 1;
        if chars.len() > pos {
            if (chars[pos] == '-') != has_sep {
                return None;
            }
            pos += has_sep as usize;
            weekday = take_ndigits(chars, pos, 1)?;
        }
        iso_week_day_number(year, week, weekday)
    } else {
        let month = take_ndigits(chars, pos, 2)?;
        pos += 2;
        if (chars.get(pos) == Some(&'-')) != has_sep {
            return None;
        }
        pos += has_sep as usize;
        let day = take_ndigits(chars, pos, 2)?;
        if valid_calendar_date(year, month, day) {
            Some(days_from_civil(year, month, day))
        } else {
            None
        }
    }
}

/// Read a fraction standing after a time component, the separator
/// at the given position being a point or a comma or a colon and
/// the fraction attaching to the second as the Python reading
/// attaches it.
/// Digits past the sixth are discarded, as the Python reading
/// discards them. A separator standing at the very end of the text
/// reads as a fraction of no digits where an offset follows the
/// time, the forgiving reading, and refuses where none does. The
/// reading hands back the microseconds and the position after the
/// fraction and the count of fraction digits it read.
fn parse_fraction(chars: &[char], pos: usize, forgiving: bool) -> Option<(i64, usize, usize)> {
    let mut cursor = pos + 1;
    if cursor == chars.len() {
        if forgiving {
            return Some((0, cursor, 0));
        }
        return None;
    }
    let mut micros: i64 = 0;
    let mut places = 0;
    let mut digits = 0;
    while cursor < chars.len() && chars[cursor].is_ascii_digit() {
        if places < 6 {
            micros = micros * 10 + (chars[cursor] as u32 - '0' as u32) as i64;
            places += 1;
        }
        digits += 1;
        cursor += 1;
    }
    while places < 6 {
        micros *= 10;
        places += 1;
    }
    Some((micros, cursor, digits))
}

/// Read the components of a time text as hour and minute and second
/// and microseconds, in the extended form or the basic form. The
/// forgiving reading, the one the time of a timestamp gets where an
/// offset follows it, drops the one dangling digit a basic form can
/// end its pairs on, reads a fraction separator standing at the end
/// of the text as a fraction of no digits, reads a component
/// separator standing at the end of the text as the close of the
/// components read so far, skips the one byte standing between the
/// finished components and the offset marker, and past six fraction
/// digits stops reading the text at all, as the Python reading does
/// each of these. A colon standing after the seconds in the
/// extended form opens the fraction as a point or a comma does. The
/// offset text gets the strict reading.
fn parse_time_components(chars: &[char], forgiving: bool) -> Option<(i64, i64, i64, i64)> {
    let len = chars.len();
    if len < 2 {
        return None;
    }
    let hour = take_ndigits(chars, 0, 2)?;
    if len == 2 {
        return Some((hour, 0, 0, 0));
    }
    if chars[2] == ':' {
        let mut minute = 0;
        let mut second = 0;
        let mut pos = 2;
        for stage in 0..2 {
            if stage == 1 && chars[pos] != ':' {
                break;
            }
            // The separator stands at pos. A separator standing at
            // the very end of the text closes the components read
            // so far under the forgiving reading.
            if pos == len - 1 {
                if forgiving {
                    return Some((hour, minute, second, 0));
                }
                return None;
            }
            pos += 1;
            let component = take_ndigits(chars, pos, 2)?;
            pos += 2;
            if stage == 0 {
                minute = component;
            } else {
                second = component;
            }
            if pos == len {
                return Some((hour, minute, second, 0));
            }
        }
        let mut micros = 0;
        if pos < len {
            if matches!(chars[pos], '.' | ',' | ':') {
                let (value, next, digits) = parse_fraction(chars, pos, forgiving)?;
                micros = value;
                pos = next;
                if digits >= 6 && forgiving {
                    pos = len;
                }
            } else if pos == len - 1 && forgiving && chars[pos].is_ascii() {
                // One byte stands between the finished components
                // and the offset marker, and the forgiving reading
                // skips it, a digit included.
                pos = len;
            } else {
                return None;
            }
        }
        if pos != len {
            return None;
        }
        return Some((hour, minute, second, micros));
    }
    // The basic form, its components standing in digit pairs over
    // the digit run opening the text.
    let mut run = 0;
    while run < len && chars[run].is_ascii_digit() {
        run += 1;
    }
    let minute = if run >= 4 {
        take_ndigits(chars, 2, 2)?
    } else {
        0
    };
    let second = if run >= 6 {
        take_ndigits(chars, 4, 2)?
    } else {
        0
    };
    let mut micros = 0;
    let mut pos = run;
    if run >= 8 {
        // Fraction digits stand in the run itself past the seconds
        // pair, up to six of them, and any further digits of the run
        // are dropped. Past six of them the forgiving reading stops
        // reading the text at all.
        let mut value: i64 = 0;
        let mut places = 0;
        for ch in chars.iter().take(run.min(12)).skip(6) {
            value = value * 10 + (*ch as u32 - '0' as u32) as i64;
            places += 1;
        }
        while places < 6 {
            value *= 10;
            places += 1;
        }
        micros = value;
        if run >= 12 && forgiving {
            pos = len;
        }
    } else if run % 2 == 1 && !forgiving {
        // One dangling digit stands at the end of the pairs, and the
        // strict reading refuses it where the forgiving reading
        // drops it.
        return None;
    }
    if pos < len {
        if matches!(chars[pos], '.' | ',') && run <= 6 && run % 2 == 0 {
            let (value, next, digits) = parse_fraction(chars, pos, forgiving)?;
            micros = value;
            pos = next;
            if digits >= 6 && forgiving {
                pos = len;
            }
        } else if pos == len - 1 && run <= 6 && run % 2 == 0 && forgiving && chars[pos].is_ascii() {
            // One byte stands between the finished pairs and the
            // offset marker, and the forgiving reading skips it.
            pos = len;
        } else {
            return None;
        }
    }
    if pos != len {
        return None;
    }
    Some((hour, minute, second, micros))
}

/// Read the time standing after the separator of a timestamp into
/// microseconds past the start of its day on the UTC clock, with
/// whether an offset stood at all. The offset marker the reading
/// finds is the first dash or plus or capital Z standing in the
/// text, as the Python reading finds it, and the capital Z stands
/// only as the last character.
/// The offset is refused when its whole size reaches a full day,
/// and the fraction of an offset whose whole part stands at zero is
/// dropped, both as the Python reading does them.
fn parse_iso_time(chars: &[char]) -> Option<(i128, bool)> {
    if chars.len() < 2 {
        return None;
    }
    let marker = chars.iter().position(|ch| matches!(ch, '-' | '+' | 'Z'));
    let (time_text, offset) = match marker {
        Some(index) => (&chars[..index], Some((chars[index], &chars[index + 1..]))),
        None => (chars, None),
    };
    let (hour, minute, second, micros) = parse_time_components(time_text, offset.is_some())?;
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let local = (hour * 3600 + minute * 60 + second) as i128 * 1_000_000 + micros as i128;
    match offset {
        None => Some((local, false)),
        Some(('Z', rest)) => {
            if rest.is_empty() {
                Some((local, true))
            } else {
                None
            }
        }
        Some((sign @ ('+' | '-'), offset_text)) => {
            let (offset_hour, offset_minute, offset_second, offset_micros) =
                parse_time_components(offset_text, false)?;
            let whole_seconds = offset_hour * 3600 + offset_minute * 60 + offset_second;
            let fraction = if whole_seconds == 0 { 0 } else { offset_micros };
            let total = whole_seconds as i128 * 1_000_000 + fraction as i128;
            if total >= 86_400_000_000 {
                return None;
            }
            let signed = if sign == '-' { -total } else { total };
            Some((local - signed, true))
        }
        _ => None,
    }
}

/// Read an ISO 8601 timestamp the way the Python fromisoformat
/// reader reads one, returning the UTC instant in microseconds and
/// whether the timestamp carried a timezone offset at all. A bare
/// date reads as midnight with no offset carried, and a separator
/// standing with no time after it refuses.
pub fn parse_timestamp_instant(text: &str) -> Option<(i128, bool)> {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() < 7 {
        return None;
    }
    let separator = find_datetime_separator(&chars)?;
    // The finding can stand past the end of a short text, and the
    // date reading then refuses the text on its length, as the
    // Python reading refuses it.
    let day = parse_iso_date(&chars[..separator.min(chars.len())])?;
    let local_day = day as i128 * 86_400_000_000;
    if separator == chars.len() {
        return Some((local_day, false));
    }
    let time_text = &chars[separator + 1..];
    if time_text.is_empty() {
        return None;
    }
    let (time, carried) = parse_iso_time(time_text)?;
    Some((local_day + time, carried))
}

/// Read the strict hyphenated ISO calendar date into a day number.
/// The valid years stand at one through 9999, as the Python date
/// reading bounds them, so the year zero refuses.
pub fn parse_calendar_date(text: &str) -> Option<i64> {
    let bytes = text.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    let year: i64 = text.get(0..4)?.parse().ok()?;
    let month: i64 = text.get(5..7)?.parse().ok()?;
    let day: i64 = text.get(8..10)?.parse().ok()?;
    if !text
        .chars()
        .enumerate()
        .all(|(index, ch)| index == 4 || index == 7 || ch.is_ascii_digit())
    {
        return None;
    }
    if !(1..=9999).contains(&year) {
        return None;
    }
    if valid_calendar_date(year, month, day) {
        Some(days_from_civil(year, month, day))
    } else {
        None
    }
}

/// The current UTC calendar date as a day number, from the system clock.
pub fn today_day_number() -> i64 {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i128)
        .unwrap_or(0);
    floor_div(seconds, 86_400) as i64
}
