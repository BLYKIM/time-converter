use anyhow::{bail, Result};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub fn timestamp_to_utc(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_nanos(timestamp)
}

pub fn datetime_to_nanos(datetime: &str) -> Result<i64> {
    let Ok(datetime) = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S%.f%z") else {
        bail!("Invalid datetime format: %Y-%m-%dT%H:%M:%S%.f%z");
    };
    Ok(datetime.timestamp_nanos_opt().unwrap_or(i64::MAX))
}
