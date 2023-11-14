use std::{
    cmp::{PartialEq, PartialOrd},
    fmt::Display,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
pub struct Int(pub i32);

impl Int {
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

/// Generate operation trait (`std::ops`) implementation.
macro_rules! impl_op {
    ( $trait:ty, $op:tt ) => {
        impl_op!($trait, $op, $op);
    };
    ( $trait:ty, $op_name:tt, $op_used:tt ) => {
        impl $trait for Int {
            type Output = Int;

            #[inline]
            fn $op_name(self, rhs: Self) -> Self::Output {
                Int(self.0.$op_used(rhs.0))
            }
        }
    };
}

impl_op!(Add, add, saturating_add);
impl_op!(Sub, sub, saturating_sub);
impl_op!(Mul, mul, saturating_mul);
impl_op!(Div, div, saturating_div);
impl_op!(Rem, rem);
impl_op!(BitAnd, bitand);
impl_op!(BitOr, bitor);
impl_op!(BitXor, bitxor);

impl Neg for Int {
    type Output = Int;

    #[inline]
    fn neg(self) -> Self::Output {
        Int(-self.0)
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
