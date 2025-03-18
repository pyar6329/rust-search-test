use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DecimalConverter;
type DecimalConverterValue = DecimalValue<DecimalConverter>;

pub fn string_height_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
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
        return Ok(centimeter.to_scaled_i64() as u8);
    }

    Ok(meter.to_scaled_i64() as u8)
}
