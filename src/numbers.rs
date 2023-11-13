use std::{
    cmp::{PartialEq, PartialOrd},
    fmt::Display,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Int(pub i32);

impl Int {
    #[inline]
    pub fn parse(string: &str) -> Option<Self> {
        match string.parse::<i32>() {
            Err(_) => None,
            Ok(val) => Some(Int(val)),
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_true(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Int(self.0.abs())
    }
}

impl Add for Int {
    type Output = Int;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Int(self.0.saturating_add(rhs.0))
    }
}

impl Sub for Int {
    type Output = Int;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Int(self.0.saturating_sub(rhs.0))
    }
}

impl Mul for Int {
    type Output = Int;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Int(self.0.saturating_mul(rhs.0))
    }
}

impl Div for Int {
    type Output = Int;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Int(self.0.saturating_div(rhs.0))
    }
}

impl Rem for Int {
    type Output = Int;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Int(self.0 % rhs.0)
    }
}

impl Neg for Int {
    type Output = Int;

    #[inline]
    fn neg(self) -> Self::Output {
        Int(-self.0)
    }
}

impl BitAnd for Int {
    type Output = Int;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        if self.is_true() {
            rhs
        } else {
            self
        }
    }
}

impl BitOr for Int {
    type Output = Int;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        if self.is_true() {
            self
        } else {
            rhs
        }
    }
}

impl BitXor for Int {
    type Output = Int;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self.is_true() ^ rhs.is_true()).into()
    }
}

impl Not for Int {
    type Output = Int;

    #[inline]
    fn not(self) -> Self::Output {
        Int(if self.is_true() { 0 } else { -1 })
    }
}

impl PartialEq for Int {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for Int {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<bool> for Int {
    #[inline]
    fn from(value: bool) -> Self {
        Int(if value { -1 } else { 0 })
    }
}

impl From<i64> for Int {
    #[inline]
    fn from(value: i64) -> Self {
        Int(saturating_i64_to_i32(value))
    }
}

impl From<Int> for i64 {
    #[inline]
    fn from(value: Int) -> Self {
        value.0 as i64
    }
}

impl From<usize> for Int {
    #[inline]
    fn from(value: usize) -> Self {
        // this is a lossy operation and may result in overflows
        Int(value as i32)
    }
}

impl From<Int> for usize {
    #[inline]
    fn from(value: Int) -> Self {
        value.0 as usize
    }
}

impl From<Int> for char {
    #[inline]
    fn from(value: Int) -> Self {
        if let Ok(u) = value.0.try_into() {
            if let Some(c) = char::from_u32(u) {
                return c;
            }
        }
        '�'
    }
}

impl Display for Int {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Int {
    pub fn from_addr(value: usize) -> Self {
        #[cfg(target_pointer_width = "64")]
        let value = saturating_u64_to_u32(value as u64);
        #[cfg(target_pointer_width = "32")]
        let value = value as u32;
        Int(unsigned_to_signed(value))
    }

    pub fn to_addr(self) -> usize {
        signed_to_unsigned(self.0) as usize
    }
}

/// Transform i64 to i32 rounding the i64 values to the limits of i32.
#[inline]
fn saturating_i64_to_i32(value: i64) -> i32 {
    if value < i32::MIN as i64 {
        i32::MIN
    } else if value > i32::MAX as i64 {
        i32::MAX
    } else {
        value as i32
    }
}

/// Transform i64 to i32 rounding the i64 values to the limits of i32.
#[inline]
fn saturating_u64_to_u32(value: u64) -> u32 {
    if value < u32::MIN as u64 {
        u32::MIN
    } else if value > u32::MAX as u64 {
        u32::MAX
    } else {
        value as u32
    }
}

/// Treat signed integer bytes as representation of a unsigned integer.
#[inline]
fn signed_to_unsigned(value: i32) -> u32 {
    let bytes = value.to_ne_bytes();
    u32::from_be_bytes(bytes)
}

/// Treat unsigned integer bytes as representation of a signed integer.
#[inline]
fn unsigned_to_signed(value: u32) -> i32 {
    let bytes = value.to_ne_bytes();
    i32::from_be_bytes(bytes)
}
