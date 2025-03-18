use chrono::{DateTime, NaiveDate, TimeZone, Utc};

/// It returns DateTime<Utc> of timestamp is 0
pub fn default_utc_datetime() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).single().unwrap()
}

/// It returns NaiveDate of timestamp is 0
pub fn default_naive_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
}

/// It generates DateTime<Utc> from year, month, day, hour, minutes, seconds, milliseconds and microseconds.
pub fn generate_utc(
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
    microseconds: u16,
) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
        .and_then(|date| {
            date.and_hms_micro_opt(
                hour as u32,
                minutes as u32,
                seconds as u32,
                ((milliseconds as u32) * 1000) + (microseconds as u32),
            )
        })
        .map(|naive_datetime| Utc.from_local_datetime(&naive_datetime))
        .and_then(|local_datetime| local_datetime.single())
        .unwrap_or(default_utc_datetime())
}

/// It generates NaiveDate from year, month and day.
pub fn generate_naive_date(year: u16, month: u8, day: u8) -> NaiveDate {
    NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap_or(default_naive_date())
}
