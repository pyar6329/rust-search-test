use rust_decimal::{
    Decimal,
    prelude::{FromPrimitive, RoundingStrategy, ToPrimitive},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, de::Visitor};
use std::{
    fmt,
    marker::PhantomData,
    ops::{Add, Deref, Div, Mul, Rem, Sub},
    str::FromStr,
};

type InternalValue = Decimal;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DecimalValue<T> {
    value: InternalValue,
    _phantom: PhantomData<T>,
}

impl<T: Copy> DecimalValue<T> {
    /// Decimal multiply by 100 times and convert to i64.
    pub fn to_scaled_i64(&self) -> i64 {
        let scaled_value = *self * DecimalValue::from(100_u64);
        let decimal_scaled_value: Decimal = *scaled_value;
        decimal_scaled_value.to_i64().unwrap_or(0)
    }

    /// i64 devide by 100 times and convert to DecimalValue.
    pub fn from_scaled_i64(i: &i64) -> Self {
        let scaled_value = DecimalValue::from(i);
        scaled_value / DecimalValue::from(100_u64)
    }
}

impl<T> DecimalValue<T> {
    /// Returns the largest integer less than or equal to `self`.
    pub fn floor(&self) -> Self {
        self.floor_with_scale(0)
    }

    /// Returns the smallest integer greater than or equal to a number.
    pub fn ceil(&self) -> Self {
        self.ceil_with_scale(0)
    }

    /// Returns a new DecimalValue number with no fractional portion (i.e. an integer).
    /// Rounding currently follows “Bankers Rounding” rules. e.g. 6.5 -> 6, 7.5 -> 8
    pub fn round(&self) -> Self {
        self.round_with_scale(0)
    }

    /// Returns a new DecimalValue with the fractional portion delimited by scale.
    /// This is a true truncation whereby no rounding is performed.
    pub fn floor_with_scale(&self, scale: u8) -> Self {
        let v = (**self).trunc_with_scale(scale as u32);
        Self::from(v)
    }

    /// Returns a new Decimal number with the specified number of decimal points for fractional portion.
    /// The number is always rounded away from zero. e.g. -6.8 -> -7, 6.8 -> 7
    pub fn ceil_with_scale(&self, scale: u8) -> Self {
        let v = (**self).round_dp_with_strategy(scale as u32, RoundingStrategy::AwayFromZero);
        Self::from(v)
    }

    /// Returns a new Decimal number with the specified number of decimal points for fractional portion.
    /// Rounding currently follows “Bankers Rounding” rules. e.g. 6.5 -> 6, 7.5 -> 8
    pub fn round_with_scale(&self, scale: u8) -> Self {
        let v = (**self).round_dp(scale as u32);
        Self::from(v)
    }
}

impl<T> Default for DecimalValue<T> {
    fn default() -> Self {
        Self::from(0.0)
    }
}

impl<T> From<&InternalValue> for DecimalValue<T> {
    fn from(value: &InternalValue) -> Self {
        Self {
            value: *value,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<InternalValue> for DecimalValue<T> {
    fn from(value: InternalValue) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&i64> for DecimalValue<T> {
    fn from(value: &i64) -> Self {
        let decimal = Decimal::from_i64(*value).unwrap_or(Decimal::ZERO);
        Self::from(decimal)
    }
}

impl<T> From<i64> for DecimalValue<T> {
    fn from(value: i64) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&u64> for DecimalValue<T> {
    fn from(value: &u64) -> Self {
        let decimal = Decimal::from_u64(*value).unwrap_or(Decimal::ZERO);
        Self::from(decimal)
    }
}

impl<T> From<u64> for DecimalValue<T> {
    fn from(value: u64) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&f64> for DecimalValue<T> {
    fn from(value: &f64) -> Self {
        let decimal_min = Decimal::MIN.to_f64().unwrap_or_default();
        let decimal_max = Decimal::MAX.to_f64().unwrap_or_default();
        if *value < decimal_min {
            return Self::from(Decimal::MIN);
        }
        if *value > decimal_max {
            return Self::from(Decimal::MAX);
        }
        let decimal = Decimal::from_f64(*value).unwrap_or(Decimal::ZERO);
        Self::from(decimal)
    }
}

impl<T> From<f64> for DecimalValue<T> {
    fn from(value: f64) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&f32> for DecimalValue<T> {
    fn from(value: &f32) -> Self {
        let decimal_min = Decimal::MIN.to_f32().unwrap_or_default();
        let decimal_max = Decimal::MAX.to_f32().unwrap_or_default();
        if *value < decimal_min {
            return Self::from(Decimal::MIN);
        }
        if *value > decimal_max {
            return Self::from(Decimal::MAX);
        }
        let decimal = Decimal::from_f32(*value).unwrap_or(Decimal::ZERO);
        Self::from(decimal)
    }
}

impl<T> From<f32> for DecimalValue<T> {
    fn from(value: f32) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&DecimalValue<T>> for f64 {
    fn from(value: &DecimalValue<T>) -> Self {
        let max_decimal_value = Decimal::MAX;
        let min_decimal_value = Decimal::MIN;

        let decimal_value: Decimal = **value;
        if decimal_value > max_decimal_value {
            return Decimal::MAX.to_f64().unwrap_or_default();
        }

        if decimal_value < min_decimal_value {
            return Decimal::MIN.to_f64().unwrap_or_default();
        }

        decimal_value.to_f64().unwrap_or(0.0)
    }
}

impl<T> From<DecimalValue<T>> for f64 {
    fn from(value: DecimalValue<T>) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&DecimalValue<T>> for f32 {
    fn from(value: &DecimalValue<T>) -> Self {
        let max_decimal_value = Decimal::MAX;
        let min_decimal_value = Decimal::MIN;

        let decimal_value: Decimal = **value;
        if decimal_value > max_decimal_value {
            return Decimal::MAX.to_f32().unwrap_or_default();
        }

        if decimal_value < min_decimal_value {
            return Decimal::MIN.to_f32().unwrap_or_default();
        }

        decimal_value.to_f32().unwrap_or(0.0)
    }
}

impl<T> From<DecimalValue<T>> for f32 {
    fn from(value: DecimalValue<T>) -> Self {
        Self::from(&value)
    }
}

impl<T> From<&str> for DecimalValue<T> {
    fn from(s: &str) -> Self {
        let decimal_value = Decimal::from_str(s).unwrap_or(Decimal::ZERO);
        Self::from(decimal_value)
    }
}

impl<T> From<String> for DecimalValue<T> {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl<T> Deref for DecimalValue<T> {
    type Target = InternalValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> fmt::Display for DecimalValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = self.deref().to_string();
        f.write_str(&id)?;
        Ok(())
    }
}

impl<T> fmt::Debug for DecimalValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<T> Add for DecimalValue<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let a: InternalValue = *self;
        let b: InternalValue = *other;

        Self::from(a + b)
    }
}

impl<T> Sub for DecimalValue<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a: InternalValue = *self;
        let b: InternalValue = *other;

        Self::from(a - b)
    }
}

impl<T> Mul for DecimalValue<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a: InternalValue = *self;
        let b: InternalValue = *other;
        Self::from(a * b)
    }
}

impl<T> Div for DecimalValue<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let a: InternalValue = *self;
        let b: InternalValue = *other;

        if b == Decimal::ZERO {
            panic!("Cannot divide by zero-valued `DecimalValue`");
        }

        Self::from(a / b)
    }
}

impl<T> Rem for DecimalValue<T> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let a: InternalValue = *self;
        let b: InternalValue = *other;

        Self::from(a % b)
    }
}

impl<T> Serialize for DecimalValue<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val_f64: f64 = self.into();
        serializer.serialize_f64(val_f64)
    }
}

struct DecimalValueVisitor<T> {
    _phantom: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for DecimalValueVisitor<T> {
    type Value = DecimalValue<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an decimal between -79,228,162,514,264,337,593,543,950,335 and 79,228,162,514,264,337,593,543,950,335")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value as u64);
        Ok(decimal_value)
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value as u64);
        Ok(decimal_value)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value as u64);
        Ok(decimal_value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value as i64);
        Ok(decimal_value)
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value as i64);
        Ok(decimal_value)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value as i64);
        Ok(decimal_value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decimal_value = DecimalValue::<T>::from(value);
        Ok(decimal_value)
    }
}

impl<'de, T> Deserialize<'de> for DecimalValue<T> {
    fn deserialize<D>(deserializer: D) -> Result<DecimalValue<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DecimalValueVisitor::<T> {
            _phantom: PhantomData,
        })
    }
}
