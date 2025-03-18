use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DecimalConverter;
type DecimalConverterValue = DecimalValue<DecimalConverter>;

pub fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed_s = s.trim();

    if trimmed_s.is_empty() {
        return Ok(0);
    }

    let decimal_value = DecimalConverterValue::from(trimmed_s);

    let floor_value = decimal_value.floor();
    let value = floor_value.to_scaled_i64() as u64;

    Ok(value)
}
