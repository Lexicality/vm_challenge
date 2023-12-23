use std::{
    fmt::Display,
    ops::{self, BitAnd},
};

const MATH_MOD: u32 = 32_768;
const MATH_MASK: u16 = !(MATH_MOD as u16);

#[derive(Debug)]
pub enum ValueState {
    Number(u16),
    Register(usize),
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(u16);

impl Value {
    pub const fn mew(value: u16) -> Self {
        Self(value)
    }

    pub fn get_value_state(self) -> ValueState {
        match self.0 {
            value if value <= 32767 => ValueState::Number(value),
            value if value <= 32775 => ValueState::Register((value - 32768) as usize),
            _ => ValueState::Invalid,
        }
    }

    pub fn to_register(self) -> usize {
        match self.get_value_state() {
            ValueState::Number(num) if num < 8 => panic!("TODO: Reckon this shouldn't be valid"),
            ValueState::Number(_) => panic!("Attempted to use a number as a register"),
            ValueState::Register(i) => i,
            ValueState::Invalid => panic!("Attempted to use invalid number {}", self.0),
        }
    }

    pub fn to_number(self) -> u16 {
        match self.get_value_state() {
            ValueState::Number(num) => num,
            ValueState::Register(i) => panic!("Attempted to use register {i} as a number!"),
            ValueState::Invalid => panic!("Attempted to use invalid number {}", self.0),
        }
    }

    pub fn to_ascii(self) -> char {
        char::from_u32(self.0.into()).expect("Value must be a valid ascii character")
    }

    fn mew_from_math(value: u32) -> Self {
        Self((value % MATH_MOD) as u16)
    }

    fn math_value(self) -> u32 {
        self.0 as u32
    }
}

impl ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::mew_from_math(self.math_value() + rhs.math_value())
    }
}

impl ops::Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::mew_from_math(self.math_value() * rhs.math_value())
    }
}

impl ops::Rem for Value {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::mew(self.0 % rhs.0)
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
