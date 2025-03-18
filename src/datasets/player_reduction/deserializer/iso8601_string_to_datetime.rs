use super::*;

const DATETIME_FORMAT1: &str = "%Y-%m-%dT%H:%M:%S%.6f";

pub fn iso8601_string_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed_s = s.trim();

    if trimmed_s.is_empty() {
        return Ok(default_utc_datetime());
    }

    // let parsed_datetime = DateTime::parse_from_rfc3339(trimmed_s)
    //     .map_err(serde::de::Error::custom)
    //     .inspect_err(|e| {
    //         eprintln!(
    //             "deseralize iso8601_string_to_datetime error: {:?}, value: {}",
    //             e, trimmed_s
    //         );
    //     })?;

    let value = parse_iso8601(trimmed_s).map_err(serde::de::Error::custom)?;
    Ok(value)
}

pub(super) fn parse_iso8601(s: &str) -> Result<DateTime<Utc>, Error> {
    // it returns immediately when string is RFC3339 format
    // Such as "2025-03-06T09:06:28.739021Z"
    if let Ok(value) = DateTime::parse_from_rfc3339(s) {
        return Ok(value.with_timezone(&Utc));
    }

    // It converts NaiveDateTime when string don't have timezone
    // Such as "2025-03-06T12:08:32.110710"
    let parsed_datetime_without_timezone = NaiveDateTime::parse_from_str(s, DATETIME_FORMAT1)
        .inspect_err(|e| {
            eprintln!(
                "deseralize iso8601_string_to_datetime error: {:?}, value: {}",
                e, s
            );
        })?;

    let value = Utc
        .from_local_datetime(&parsed_datetime_without_timezone)
        .single()
        .ok_or(anyhow!("Failed to convert from local_datetime to Utc"))?;

    Ok(value)
}
