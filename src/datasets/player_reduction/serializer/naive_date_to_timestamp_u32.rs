use super::*;
use crate::utils::default_naive_date;

/// u32's range between 0 and 2^32 - 1 (=4,294,967,295)
/// In the other words, 2106/Feb/07 06:28:15 is the maximum value of u32.
pub fn naive_date_to_timestamp_u32<S: Serializer>(
    date: &NaiveDate,
    s: S,
) -> Result<S::Ok, S::Error> {
    let value = naive_date_to_u32(date);
    s.serialize_u32(value as u32)
}

pub(super) fn naive_date_to_u32(date: &NaiveDate) -> u32 {
    let epoc_date = default_naive_date();
    if *date < epoc_date {
        return 0;
    }

    (*date - epoc_date).num_days() as u32
}
