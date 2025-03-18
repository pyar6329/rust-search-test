use super::*;

/// u32's range between 0 and 2^32 - 1 (=4,294,967,295)
/// In the other words, 2106/Feb/07 06:28:15 is the maximum value of u32.
pub fn datetime_to_timestamp_u32<S: Serializer>(
    datetime: &DateTime<Utc>,
    s: S,
) -> Result<S::Ok, S::Error> {
    let timestamp: i64 = datetime.timestamp();
    s.serialize_u32(timestamp as u32)
}
