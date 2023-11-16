pub type Int = i32;
pub const TRUE: i32 = -1;
pub const FALSE: i32 = 0;

/// Is the value true-ish.
#[inline]
pub fn is_true(value: Int) -> bool {
    value != FALSE
}

/// Transform `bool` to `Int`.
#[inline]
pub fn from_bool(value: bool) -> Int {
    if value {
        TRUE
    } else {
        FALSE
    }
}

/// Return character for the numerical code, if not possible show the replacement character.
#[inline]
pub fn to_char(value: Int) -> char {
    if let Ok(u) = value.try_into() {
        if let Some(c) = char::from_u32(u) {
            return c;
        }
    }
    '�'
}

/// Transform i64 to i32 rounding the i64 values to the limits of i32.
#[inline]
pub fn saturating_i64_to_i32(value: i64) -> i32 {
    if value < i32::MIN as i64 {
        i32::MIN
    } else if value > i32::MAX as i64 {
        i32::MAX
    } else {
        value as i32
    }
}
