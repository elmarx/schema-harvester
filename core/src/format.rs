//! "stolen" from https://github.com/Stranger6667/jsonschema/blob/f94b36d4f55ec12a0d77c2797626a8539e4197ed/crates/jsonschema/src/keywords/format.rs

use uuid_simd::{Out, parse_hyphenated};

pub fn is_valid_uuid(uuid: &str) -> bool {
    let mut out = [0; 16];
    parse_hyphenated(uuid.as_bytes(), Out::from_mut(&mut out)).is_ok()
}

#[inline]
fn parse_two_digits(bytes: &[u8]) -> Option<u8> {
    let value = u16::from_ne_bytes([bytes[0], bytes[1]]);

    // Check if both bytes are ASCII digits
    if value.wrapping_sub(0x3030) & 0xF0F0 == 0 {
        Some(((value & 0x0F0F).wrapping_mul(2561) >> 8) as u8)
    } else {
        None
    }
}

#[inline]
fn parse_four_digits(bytes: &[u8]) -> Option<u16> {
    let value = u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    // Check if all bytes are ASCII digits
    if value.wrapping_sub(0x30303030) & 0xF0F0F0F0 == 0 {
        let val = (value & 0x0F0F_0F0F).wrapping_mul(2561) >> 8;
        Some(((val & 0x00FF_00FF).wrapping_mul(6_553_601) >> 16) as u16)
    } else {
        None
    }
}

#[inline]
fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub(crate) fn is_valid_date(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }

    let bytes = date.as_bytes();

    // Check format: YYYY-MM-DD
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }

    // Parse year (YYYY)
    let Some(year) = parse_four_digits(&bytes[0..4]) else {
        return false;
    };

    // Parse month (MM)
    let Some(month) = parse_two_digits(&bytes[5..7]) else {
        return false;
    };
    if !(1..=12).contains(&month) {
        return false;
    }

    // Parse day (DD)
    let Some(day) = parse_two_digits(&bytes[8..10]) else {
        return false;
    };
    if day == 0 {
        return false;
    }

    // Check day validity
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => {
            if is_leap_year(year) {
                day <= 29
            } else {
                day <= 28
            }
        }
        _ => unreachable!("Month value is checked above"),
    }
}

macro_rules! handle_offset {
    ($sign:tt, $i:ident, $bytes:expr, $hour:expr, $minute:expr, $second:expr) => {{
        if $bytes.len() - $i != 6 {
            return false;
        }
        $i += 1;
        if $bytes[$i + 2] != b':' {
            return false;
        }
        let Some(offset_hh) = parse_two_digits(&$bytes[$i..$i + 2]) else {
            return false;
        };
        let Some(offset_mm) = parse_two_digits(&$bytes[$i + 3..$i + 5]) else {
            return false;
        };
        if offset_hh > 23 || offset_mm > 59 {
            return false;
        }

        if $second == 60 {
            let mut utc_hh = $hour as i8;
            let mut utc_mm = $minute as i8;

            // Apply offset based on the sign (+ or -)
            utc_hh $sign offset_hh as i8;
            utc_mm $sign offset_mm as i8;

            // Adjust for minute overflow/underflow
            utc_hh += utc_mm / 60;
            utc_mm %= 60;
            if utc_mm < 0 {
                utc_mm += 60;
                utc_hh -= 1;
            }

            // Adjust for hour overflow/underflow
            utc_hh = (utc_hh + 24) % 24;
            utc_hh == 23 && utc_mm == 59
        } else {
            true
        }
    }};
}

pub fn is_valid_time(time: &str) -> bool {
    let bytes = time.as_bytes();
    let len = bytes.len();

    if len < 9 {
        // Minimum valid time is "HH:MM:SSZ"
        return false;
    }

    // Check HH:MM:SS format
    if bytes[2] != b':' || bytes[5] != b':' {
        return false;
    }

    // Parse hour (HH)
    let Some(hour) = parse_two_digits(&bytes[..2]) else {
        return false;
    };
    // Parse minute (MM)
    let Some(minute) = parse_two_digits(&bytes[3..5]) else {
        return false;
    };
    // Parse second (SS)
    let Some(second) = parse_two_digits(&bytes[6..8]) else {
        return false;
    };

    if hour > 23 || minute > 59 || second > 60 {
        return false;
    }

    let mut i = 8;

    // Check fractional seconds
    if i < len && bytes[i] == b'.' {
        i += 1;
        let mut has_digit = false;
        while i < len && bytes[i].is_ascii_digit() {
            has_digit = true;
            i += 1;
        }
        if !has_digit {
            return false;
        }
    }

    // Check offset
    if i == len {
        return false;
    }

    match bytes[i] {
        b'Z' | b'z' => i == len - 1 && (second != 60 || (hour == 23 && minute == 59)),
        b'+' => handle_offset!(-=, i, bytes, hour, minute, second),
        b'-' => handle_offset!(+=, i, bytes, hour, minute, second),
        _ => false,
    }
}

pub(crate) fn is_valid_datetime(datetime: &str) -> bool {
    // Find the position of 'T' or 't' separator
    let t_pos = match datetime.bytes().position(|b| b == b'T' || b == b't') {
        Some(pos) => pos,
        None => return false, // 'T' separator not found
    };

    // Split the string into date and time parts
    let (date_part, time_part) = datetime.split_at(t_pos);

    is_valid_date(date_part) && is_valid_time(&time_part[1..])
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("2023-01-01", true; "valid regular date")]
    #[test_case("2020-02-29", true; "valid leap year date")]
    #[test_case("2021-02-28", true; "valid non-leap year date")]
    #[test_case("1900-02-28", true; "valid century non-leap year")]
    #[test_case("2000-02-29", true; "valid leap century year")]
    #[test_case("1999-12-31", true; "valid end of year date")]
    #[test_case("202-12-01", false; "invalid short year")]
    #[test_case("2023-1-01", false; "invalid short month")]
    #[test_case("2023-12-1", false; "invalid short day")]
    #[test_case("2023/12/01", false; "invalid separators")]
    #[test_case("2023-13-01", false; "invalid month too high")]
    #[test_case("2023-00-01", false; "invalid month too low")]
    #[test_case("2023-12-32", false; "invalid day too high")]
    #[test_case("2023-11-31", false; "invalid day for 30-day month")]
    #[test_case("2023-02-30", false; "invalid day for February in non-leap year")]
    #[test_case("2021-02-29", false; "invalid day for non-leap year")]
    #[test_case("2023-12-00", false; "invalid day too low")]
    #[test_case("99999-12-01", false; "year too long")]
    #[test_case("1900-02-29", false; "invalid leap century non-leap year")]
    #[test_case("2000-02-30", false; "invalid day for leap century year")]
    #[test_case("2400-02-29", true; "valid leap year in distant future")]
    #[test_case("0000-01-01", true; "valid boundary start date")]
    #[test_case("9999-12-31", true; "valid boundary end date")]
    #[test_case("aaaa-01-12", false; "Malformed (letters in year)")]
    #[test_case("2000-bb-12", false; "Malformed (letters in month)")]
    #[test_case("2000-01-cc", false; "Malformed (letters in day)")]
    fn test_is_valid_date(input: &str, expected: bool) {
        assert_eq!(is_valid_date(input), expected);
    }

    #[test_case("23:59:59Z", true; "valid time with Z")]
    #[test_case("00:00:00Z", true; "valid midnight time with Z")]
    #[test_case("12:30:45.123Z", true; "valid time with fractional seconds and Z")]
    #[test_case("23:59:60Z", true; "valid leap second UTC time")]
    #[test_case("12:30:45+01:00", true; "valid time with positive offset")]
    #[test_case("12:30:45-01:00", true; "valid time with negative offset")]
    #[test_case("23:59:60+00:00", true; "valid leap second with offset UTC 00:00")]
    #[test_case("23:59:59+01:00", true; "valid time with +01:00 offset")]
    #[test_case("23:59:59A", false; "invalid time with non-Z/non-offset letter")]
    #[test_case("12:3:45Z", false; "invalid time with missing digit in minute")]
    #[test_case("12:30:4Z", false; "invalid time with missing digit in second")]
    #[test_case("12-30-45Z", false; "invalid time with wrong separator")]
    #[test_case("12:30:45Z+01:00", false; "invalid time with Z and offset together")]
    #[test_case("12:30:45A01:00", false; "invalid time with wrong separator between time and offset")]
    #[test_case("12:30:45++01:00", false; "invalid double plus in offset")]
    #[test_case("12:30:45+01:60", false; "invalid minute in offset")]
    #[test_case("12:30:45+24:00", false; "invalid hour in offset")]
    #[test_case("12:30:45.", false; "invalid time with incomplete fractional second")]
    #[test_case("24:00:00Z", false; "invalid hour > 23")]
    #[test_case("12:60:00Z", false; "invalid minute > 59")]
    #[test_case("12:30:61Z", false; "invalid second > 60")]
    #[test_case("12:30:60+01:00", false; "invalid leap second with non-UTC offset")]
    #[test_case("23:59:60Z+01:00", false; "invalid leap second with non-zero offset")]
    #[test_case("23:59:60+00:30", false; "invalid leap second with non-zero minute offset")]
    #[test_case("23:59:60Z", true; "valid leap second at the end of day")]
    #[test_case("23:59:60+00:00", true; "valid leap second with zero offset")]
    #[test_case("ab:59:59Z", false; "invalid time with letters in hour")]
    #[test_case("23:ab:59Z", false; "invalid time with letters in minute")]
    #[test_case("23:59:abZ", false; "invalid time with letters in second")]
    #[test_case("23:59:59aZ", false; "invalid time with letter after seconds")]
    #[test_case("12:30:45+ab:00", false; "invalid offset hour with letters")]
    #[test_case("12:30:45+01:ab", false; "invalid offset minute with letters")]
    #[test_case("12:30:45.abcZ", false; "invalid fractional seconds with letters")]
    fn test_is_valid_time(input: &str, expected: bool) {
        assert_eq!(is_valid_time(input), expected);
    }

    #[test]
    fn test_is_valid_datetime() {
        assert!(!is_valid_datetime(""));
    }
}
