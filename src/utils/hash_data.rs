use bytes::Bytes;
use std::ops::Deref;
use twox_hash::XxHash32;

const SEED_DATA: u32 = 2136;

type InnerHashData = u32;

/// This function uses XxHash32 algorithm
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HashData(InnerHashData);

impl Deref for HashData {
    type Target = InnerHashData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[u8]> for HashData {
    /// This function uses XxHash32 algorithm
    fn from(data: &[u8]) -> Self {
        let data = XxHash32::oneshot(SEED_DATA, data);
        Self(data)
    }
}

impl From<Vec<u8>> for HashData {
    fn from(data: Vec<u8>) -> Self {
        Self::from(data.as_slice())
    }
}

impl From<&[u8; 8]> for HashData {
    fn from(data: &[u8; 8]) -> Self {
        Self::from(data.as_slice())
    }
}

impl From<[u8; 8]> for HashData {
    fn from(data: [u8; 8]) -> Self {
        Self::from(&data)
    }
}

impl From<Bytes> for HashData {
    fn from(data: Bytes) -> Self {
        Self::from(&data)
    }
}

impl From<&Bytes> for HashData {
    /// This function converts from tokio-rs/bytes::Bytes to HashData
    fn from(data: &Bytes) -> Self {
        Self::from(data.as_ref())
    }
}

impl From<&str> for HashData {
    fn from(data: &str) -> Self {
        Self::from(data.as_bytes())
    }
}

impl From<String> for HashData {
    fn from(data: String) -> Self {
        Self::from(data.as_str())
    }
}

macro_rules! impl_hash_data_from_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for HashData {
                fn from(data: $t) -> Self {
                    let data_bytes = data.to_le_bytes();
                    Self::from(data_bytes.as_slice())
                }
            }

            impl From<&$t> for HashData {
                fn from(data: &$t) -> Self {
                    Self::from(*data)
                }
            }
        )*
    }
}

impl_hash_data_from_integer!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl PartialEq<InnerHashData> for HashData {
    /// compare the hash data with u32 directly
    fn eq(&self, other: &InnerHashData) -> bool {
        *(self.deref()) == *other
    }
}

impl PartialEq<HashData> for InnerHashData {
    fn eq(&self, other: &HashData) -> bool {
        other.eq(self)
    }
}

impl HashData {
    pub fn from_hash_data_directly(hash_data: &u32) -> Self {
        Self(*hash_data)
    }

    pub fn into_hash(&self) -> InnerHashData {
        **self
    }
}
