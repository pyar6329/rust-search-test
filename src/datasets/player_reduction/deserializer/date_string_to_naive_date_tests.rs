use super::parse_date;
use crate::utils::generate_naive_date;

#[test]
fn it_should_be_return_succeed_when_month_and_day_include_zero() {
    let s = "2025-03-06";

    let result = parse_date(s);
    let expected_date = generate_naive_date(2025, 3, 6);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_date);
}

#[test]
fn it_should_be_return_succeed_when_month_include_zero() {
    let s = "2025-3-06";
    let result = parse_date(s);
    let expected_date = generate_naive_date(2025, 3, 6);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_date);
}

#[test]
fn it_should_be_return_succeed_when_day_include_zero() {
    let s = "2025-03-6";
    let result = parse_date(s);
    let expected_date = generate_naive_date(2025, 3, 6);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_date);
}

#[test]
fn it_should_be_return_succeed_leap_year_when_month_and_day_do_not_include_zero() {
    let s = "2012-2-29";
    let result = parse_date(s);
    let expected_date = generate_naive_date(2012, 2, 29);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_date);
}
