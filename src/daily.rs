//! Fetches the official daily Wordle solution from the public NYT endpoint.
//!
//! To keep the dependency tree minimal (a supply-chain consideration), this
//! shells out to the system `curl` rather than pulling in an HTTP/TLS crate,
//! and parses the small JSON response by hand. If the network call fails (e.g.
//! offline), callers fall back to a locally chosen word.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// The puzzle the player is solving, with provenance for display.
pub struct Daily {
    pub solution: String,
    /// Human-readable puzzle date, e.g. "2026-06-14".
    pub date: String,
    /// Puzzle number (days since Wordle launch), if known.
    pub number: Option<u32>,
    /// True when the answer came from the live NYT endpoint.
    pub official: bool,
}

/// Attempts to fetch today's official Wordle. Returns `None` on any failure so
/// the caller can fall back to an offline word.
pub fn fetch_today() -> Option<Daily> {
    let (y, m, d) = today_utc();
    let date = format!("{y:04}-{m:02}-{d:02}");
    let url = format!("https://www.nytimes.com/svc/wordle/v2/{date}.json");

    let output = Command::new("curl")
        .arg("-s")
        .arg("--max-time")
        .arg("8")
        .arg(&url)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }
    let body = String::from_utf8(output.stdout).ok()?;

    let solution = extract_json_string(&body, "solution")?;
    if solution.len() != 5 || !solution.bytes().all(|b| b.is_ascii_alphabetic()) {
        return None;
    }
    let number = extract_json_number(&body, "days_since_launch");
    let date = extract_json_string(&body, "print_date").unwrap_or(date);

    Some(Daily {
        solution: solution.to_ascii_lowercase(),
        date,
        number,
        official: true,
    })
}

/// Extracts a string value for `key` from a flat JSON object via simple
/// scanning. Sufficient for the known, well-formed NYT payload.
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let start = json.find(&needle)? + needle.len();
    let rest = &json[start..];
    let colon = rest.find(':')?;
    let after = &rest[colon + 1..];
    let q1 = after.find('"')? + 1;
    let q2 = after[q1..].find('"')? + q1;
    Some(after[q1..q2].to_string())
}

/// Extracts an integer value for `key` from a flat JSON object.
fn extract_json_number(json: &str, key: &str) -> Option<u32> {
    let needle = format!("\"{key}\"");
    let start = json.find(&needle)? + needle.len();
    let rest = &json[start..];
    let colon = rest.find(':')?;
    let after = rest[colon + 1..].trim_start();
    let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// Today's UTC date formatted as "YYYY-MM-DD".
pub fn today_date_string() -> String {
    let (y, m, d) = today_utc();
    format!("{y:04}-{m:02}-{d:02}")
}

/// Current UTC date as (year, month, day) using the proleptic Gregorian
/// calendar (Howard Hinnant's civil-from-days algorithm).
fn today_utc() -> (i64, u32, u32) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86_400);
    civil_from_days(days)
}

/// Converts a count of days since the Unix epoch (1970-01-01) to a calendar
/// date. See https://howardhinnant.github.io/date_algorithms.html
#[allow(clippy::let_and_return)]
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{"id":999,"solution":"sepia","print_date":"2026-06-14","days_since_launch":1821,"editor":"Tracy Bennett"}"#;

    #[test]
    fn parses_string_field() {
        assert_eq!(
            extract_json_string(SAMPLE, "solution").as_deref(),
            Some("sepia")
        );
        assert_eq!(
            extract_json_string(SAMPLE, "print_date").as_deref(),
            Some("2026-06-14")
        );
        assert_eq!(
            extract_json_string(SAMPLE, "editor").as_deref(),
            Some("Tracy Bennett")
        );
    }

    #[test]
    fn parses_number_field() {
        assert_eq!(extract_json_number(SAMPLE, "days_since_launch"), Some(1821));
        assert_eq!(extract_json_number(SAMPLE, "id"), Some(999));
    }

    #[test]
    fn missing_field_returns_none() {
        assert_eq!(extract_json_string(SAMPLE, "nope"), None);
        assert_eq!(extract_json_number(SAMPLE, "nope"), None);
    }

    #[test]
    fn handles_whitespace_around_values() {
        let json = r#"{ "solution" : "abcde" , "days_since_launch" :  42 }"#;
        assert_eq!(
            extract_json_string(json, "solution").as_deref(),
            Some("abcde")
        );
        assert_eq!(extract_json_number(json, "days_since_launch"), Some(42));
    }

    #[test]
    fn civil_from_days_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1)); // Unix epoch
        assert_eq!(civil_from_days(-1), (1969, 12, 31)); // day before epoch
        assert_eq!(civil_from_days(365), (1971, 1, 1)); // 1970 is not a leap year
        assert_eq!(civil_from_days(10_957), (2000, 1, 1)); // 30 years later
        assert_eq!(civil_from_days(18_321), (2020, 2, 29)); // leap day
    }

    #[test]
    fn civil_from_days_roundtrips_a_range() {
        // Walk a span of days and ensure month/day stay in valid bounds.
        for z in 0..(40 * 365) {
            let (_y, m, d) = civil_from_days(z);
            assert!((1..=12).contains(&m), "bad month at {z}: {m}");
            assert!((1..=31).contains(&d), "bad day at {z}: {d}");
        }
    }
}
