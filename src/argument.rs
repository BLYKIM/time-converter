use anyhow::{bail, Result};
use chrono::{DateTime, Local, TimeZone, Utc};

pub fn timestamp_to_utc(timestamp: i64) -> String {
    let utc = Utc.timestamp_nanos(timestamp);

    format_datetime(utc)
}

pub fn datetime_to_nanos(datetime: &str) -> Result<i64> {
    let Ok(datetime) = DateTime::parse_from_rfc3339(datetime) else {
        bail!("Invalid rfc3339 datetime format");
    };
    Ok(datetime.timestamp_nanos_opt().unwrap_or(i64::MAX))
}

pub fn format_datetime(datetime: DateTime<Utc>) -> String {
    let utc = format!("{}", datetime.format("%Y-%m-%dT%H:%M:%S%.f%:z"));
    let local = format!(
        "{}",
        datetime
            .with_timezone(&Local)
            .format("%Y-%m-%dT%H:%M:%S%.f%:z")
    );
    format!("UTC: {utc}\nLOCAL: {local}")
}
