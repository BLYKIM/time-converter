use anyhow::{bail, Result};
use chrono::{DateTime, TimeZone, Utc};

pub fn timestamp_to_utc(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_nanos(timestamp)
}

pub fn datetime_to_nanos(datetime: &str) -> Result<i64> {
    let Ok(datetime) = DateTime::parse_from_rfc3339(datetime) else {
        bail!("Invalid rfc3339 datetime format");
    };
    Ok(datetime.timestamp_nanos_opt().unwrap_or(i64::MAX))
}
