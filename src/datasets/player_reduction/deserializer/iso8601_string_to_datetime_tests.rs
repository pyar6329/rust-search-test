use super::parse_iso8601;
use crate::utils::generate_utc;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

#[test]
fn it_should_be_return_succeed_when_timezone_is_not_set() {
    let s = "2025-03-06T12:08:32.110710";

    let result = parse_iso8601(s);
    let expected_datetime = generate_utc(2025, 3, 6, 12, 8, 32, 110, 710);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_datetime);
}

#[test]
fn it_should_be_return_succeed_data_when_timezone_is_set() {
    let s = "2025-03-06T09:06:28.739021Z";
    let result = parse_iso8601(s);
    let expected_datetime = generate_utc(2025, 3, 6, 9, 6, 28, 739, 21);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_datetime);
}
