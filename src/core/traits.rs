use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

/// Trait for numeric types that can be used in technical analysis calculations.
pub trait Numeric:
    Copy
    + Debug
    + Default
    + PartialOrd
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
{
    const ZERO: Self;
    const ONE: Self;

    fn two() -> Self;
    fn fifty() -> Self;
    fn hundred() -> Self;
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn is_positive(self) -> bool {
        self > Self::ZERO
    }
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
}

impl Numeric for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn two() -> Self {
        2.0
    }
    fn fifty() -> Self {
        50.0
    }
    fn hundred() -> Self {
        100.0
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}

impl Numeric for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn two() -> Self {
        2.0
    }
    fn fifty() -> Self {
        50.0
    }
    fn hundred() -> Self {
        100.0
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}
