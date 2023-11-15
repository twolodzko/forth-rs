use std::{
    cmp::{PartialEq, PartialOrd},
    fmt::Display,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Int(pub i32);

pub const TRUE: i32 = -1;
pub const FALSE: i32 = 0;

impl Int {
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_true(&self) -> bool {
        self.0 != FALSE
    }
}

impl From<bool> for Int {
    #[inline]
    fn from(value: bool) -> Self {
        Int(if value { TRUE } else { FALSE })
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

impl From<char> for Int {
    #[inline]
    fn from(value: char) -> Self {
        Int(value as i32)
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

impl FromStr for Int {
    type Err = std::num::ParseIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Int(s.parse::<i32>()?))
    }
}

impl Display for Int {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
