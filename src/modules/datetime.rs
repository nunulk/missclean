use chrono::{DateTime, FixedOffset};

pub fn str_to_datetime(str: &String) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(&format!("{}T00:00:00+09:00", str))
        .expect("Failed to parse string to datetime.")
}
