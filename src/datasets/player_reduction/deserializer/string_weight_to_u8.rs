use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DecimalConverter;
type DecimalConverterValue = DecimalValue<DecimalConverter>;

pub fn string_weight_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(0);
    }
    let weight_kilogram = DecimalConverterValue::from(s.as_str());
    let min_weight_kilogram = DecimalConverterValue::from("30.0");
    let max_weight_kilogram = DecimalConverterValue::from("120.0");

    if weight_kilogram < min_weight_kilogram || weight_kilogram > max_weight_kilogram {
        return Ok(0);
    }

    let value = weight_kilogram.to_scaled_i64() as u8;

    Ok(value)
}
