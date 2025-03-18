use super::*;
use crate::utils::default_naive_date;

const DATE_FORMATS: [&str; 4] = [
    "%Y-%m-%d",  // 2025-03-06
    "%Y-%m-%-d", // 2025-03-6
    "%Y-%-m-%d", // 2025-3-06
    "%Y-%-m-%-d", // 2025-3-6
                 // TODO: 0-0-0が渡ってくる可能性があるのでなんとかする
];

pub fn date_string_to_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed_s = s.trim();

    if trimmed_s.is_empty() {
        return Ok(default_naive_date());
    }

    // let value = parse_date(trimmed_s).map_err(serde::de::Error::custom)?;
    let value = parse_date(trimmed_s).unwrap_or(default_naive_date());
    Ok(value)
}

pub(super) fn parse_date(s: &str) -> Result<NaiveDate, Error> {
    for format in DATE_FORMATS.iter() {
        if let Ok(date) = NaiveDate::parse_from_str(s, format) {
            return Ok(date);
        }
    }

    Err(anyhow!("Unsupported date format: {}", s))
}
