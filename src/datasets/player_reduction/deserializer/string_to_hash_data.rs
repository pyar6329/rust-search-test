use super::*;
use crate::utils::HashData;

pub fn string_to_hash_data<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let trimmed_s = s.trim();

    let hash_data = HashData::from(trimmed_s);
    let value = *hash_data;

    Ok(value)
}
