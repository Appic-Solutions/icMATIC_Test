use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::ops::Rem;

use candid::CandidType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// `CheckedAmountOf<Unit>` provides a type-safe way to keep an amount of some `Unit`.
/// In contrast to `AmountOf<Unit>`, all operations are checked and do not overflow.

pub struct CheckedAmountOf<Unit>(ethnum::u256, PhantomData<Unit>);

impl<Unit> CheckedAmountOf<Unit> {
    pub const ZERO: Self = Self(ethnum::u256::ZERO, PhantomData);
    pub const ONE: Self = Self(ethnum::u256::ONE, PhantomData);
    pub const TWO: Self = Self(ethnum::u256::new(2), PhantomData);
    pub const MAX: Self = Self(ethnum::u256::MAX, PhantomData);

    /// `new` is a synonym for `from` that can be evaluated in
    /// compile time. The main use-case of this functions is defining
    /// constants.
    #[inline]
    pub const fn new(value: u128) -> CheckedAmountOf<Unit> {
        Self(ethnum::u256::new(value), PhantomData)
    }

    #[inline]
    const fn from_inner(value: ethnum::u256) -> Self {
        Self(value, PhantomData)
    }

    pub const fn into_inner(self) -> ethnum::u256 {
        self.0
    }

    #[inline]
    pub const fn from_words(hi: u128, lo: u128) -> Self {
        Self::from_inner(ethnum::u256::from_words(hi, lo))
    }

    pub fn from_str_hex(src: &str) -> Result<Self, ParseIntError> {
        ethnum::u256::from_str_hex(src).map(Self::from_inner)
    }

    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Self::from_inner(ethnum::u256::from_be_bytes(bytes))
    }

    pub fn to_be_bytes(self) -> [u8; 32] {
        self.0.to_be_bytes()
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self::from_inner)
    }

    pub fn checked_increment(&self) -> Option<Self> {
        self.checked_add(Self::ONE)
    }

    pub fn checked_decrement(&self) -> Option<Self> {
        self.checked_sub(Self::ONE)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self::from_inner)
    }

    pub fn change_units<NewUnits>(self) -> CheckedAmountOf<NewUnits> {
        CheckedAmountOf::<NewUnits>::from_inner(self.0)
    }

    pub fn checked_mul<T: Into<ethnum::u256>>(self, factor: T) -> Option<Self> {
        self.0.checked_mul(factor.into()).map(Self::from_inner)
    }

    pub fn checked_div_ceil<T: Into<ethnum::u256>>(self, rhs: T) -> Option<Self> {
        let rhs = rhs.into();
        if rhs == ethnum::u256::ZERO {
            return None;
        }
        let (quotient, remainder) = (self.0.div_euclid(rhs), self.0.rem(&rhs));
        if remainder == ethnum::u256::ZERO {
            Some(Self::from_inner(quotient))
        } else {
            Self::from_inner(quotient).checked_increment()
        }
    }

    pub fn div_by_two(self) -> Self {
        Self::from_inner(self.0 >> 1)
    }

    pub fn as_f64(&self) -> f64 {
        self.0.as_f64()
    }

    /// Returns the display implementation of the inner value.
    /// Useful to avoid thousands separators if value is used for example in URLs.
    /// ```
    /// use ic_cketh_minter::checked_amount::CheckedAmountOf;
    ///
    /// enum MetricApple{}
    /// type Apples = CheckedAmountOf<MetricApple>;
    /// let many_apples = Apples::from(4_332_415_u32);
    ///
    /// assert_eq!(many_apples.to_string_inner(), "4332415".to_string());
    /// ```
    pub fn to_string_inner(&self) -> String {
        self.0.to_string()
    }
}

macro_rules! impl_from {
    ($($t:ty),* $(,)?) => {$(
        impl<Unit> From<$t> for CheckedAmountOf<Unit> {
            #[inline]
            fn from(value: $t) -> Self {
                Self(ethnum::u256::from(value), PhantomData)
            }
        }
    )*};
}

impl_from! { u8, u16, u32, u64, u128 }

impl<Unit> TryFrom<candid::Nat> for CheckedAmountOf<Unit> {
    type Error = String;

    fn try_from(value: candid::Nat) -> Result<Self, Self::Error> {
        let value_bytes = value.0.to_bytes_be();
        let mut value_u256 = [0u8; 32];
        if value_bytes.len() <= 32 {
            value_u256[32 - value_bytes.len()..].copy_from_slice(&value_bytes);
        } else {
            return Err(format!("Nat does not fit in a U256: {}", value));
        }
        Ok(Self::from_inner(ethnum::u256::from_be_bytes(value_u256)))
    }
}

impl<Unit> From<CheckedAmountOf<Unit>> for candid::Nat {
    fn from(value: CheckedAmountOf<Unit>) -> Self {
        use num_bigint::BigUint;
        candid::Nat::from(BigUint::from_bytes_be(&value.0.to_be_bytes()))
    }
}

impl<Unit> fmt::Debug for CheckedAmountOf<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use thousands::Separable;
        write!(f, "{}", self.0.separate_with_underscores())
    }
}

impl<Unit> fmt::Display for CheckedAmountOf<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use thousands::Separable;
        write!(f, "{}", self.0.separate_with_underscores())
    }
}

impl<Unit> fmt::LowerHex for CheckedAmountOf<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl<Unit> fmt::UpperHex for CheckedAmountOf<Unit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl<Unit> Clone for CheckedAmountOf<Unit> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Unit> Copy for CheckedAmountOf<Unit> {}

impl<Unit> PartialEq for CheckedAmountOf<Unit> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

impl<Unit> Eq for CheckedAmountOf<Unit> {}

impl<Unit> PartialOrd for CheckedAmountOf<Unit> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<Unit> Ord for CheckedAmountOf<Unit> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

// Derived serde `impl Serialize` produces an extra `unit` value for
// phantom data, e.g. `AmountOf::<Meters>::from(10)` is serialized
// into json as `[10, null]` by default.
//
// We want serialization format of `Repr` and the `AmountOf` to match
// exactly, that's why we have to provide custom instances.

impl<Unit> Serialize for CheckedAmountOf<Unit> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u128(self.0.as_u128())
    }
}

impl<'de, Unit> Deserialize<'de> for CheckedAmountOf<Unit> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u128::deserialize(deserializer)?;
        Ok(Self(ethnum::u256::from(value), PhantomData))
    }
}
