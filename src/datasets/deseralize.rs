use super::*;

use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DecimalConverter;
type DecimalConverterValue = DecimalValue<DecimalConverter>;

pub(super) fn default_if_empty<'de, D, T>(deserialize: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Option::<T>::deserialize(deserialize).map(|x| x.unwrap_or_else(|| T::default()))
}

pub fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(0);
    }
    u64::from_str(&s)
        .map_err(serde::de::Error::custom)
        .inspect_err(|e| {
            eprintln!("deseralize string_to_u64 error: {:?}, value: {}", e, s);
        })
}

pub fn string_height_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(0);
    }
    let meter = DecimalConverterValue::from(s.as_str());
    let min_height_meter = DecimalConverterValue::from("1.0");
    let max_height_meter = DecimalConverterValue::from("3.0");

    if meter > min_height_meter && meter < max_height_meter {
        let centimeter = meter * DecimalConverterValue::from("100.0");
        return Ok(centimeter.to_scaled_i64() as u64);
    }

    u64::from_str(&s)
        .map_err(serde::de::Error::custom)
        .inspect_err(|e| {
            eprintln!("deseralize string_to_u64 error: {:?}, value: {}", e, s);
        })
}

pub fn string_cost_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed_s = s.replace(",", "");

    if trimmed_s.is_empty() {
        return Ok(0);
    }

    let decimal_value = DecimalConverterValue::from(trimmed_s.as_str());

    let floor_value = decimal_value.floor();
    let value = floor_value.to_scaled_i64() as u64;

    Ok(value)
}
