use std::{
    fmt::Display,
    ops::{self, BitAnd},
};

const MATH_MOD: u32 = 32_768;
const MATH_MASK: u16 = !(MATH_MOD as u16);

#[derive(Debug)]
pub enum ValueState {
    Number(u16),
    Register(u16),
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(u16);

impl Value {
    pub fn mew(value: u16) -> Self {
        Self(value)
    }

    pub fn get_value(self) -> ValueState {
        match self.0 {
            value if value <= 32767 => ValueState::Number(value),
            value if value <= 32775 => ValueState::Register(value - 32768),
            _ => ValueState::Invalid,
        }
    }

    pub fn to_ascii(self) -> char {
        char::from_u32(self.0.into()).expect("Value must be a valid ascii character")
    }

    fn new_from_math(value: u32) -> Self {
        Self((value % MATH_MOD) as u16)
    }

    fn math_value(self) -> u32 {
        self.0 as u32
    }
}

impl ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_from_math(self.math_value() + rhs.math_value())
    }
}

impl ops::Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_from_math(self.math_value() * rhs.math_value())
    }
}

impl ops::BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
    }
}

impl ops::BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}

impl ops::Not for Value {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0.not().bitand(MATH_MASK))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
