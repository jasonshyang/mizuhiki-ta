use std::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
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
    + Neg<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
{
    fn abs(self) -> Self;

    fn one() -> Self;

    fn zero() -> Self {
        Self::default()
    }

    fn two() -> Self {
        Self::one() + Self::one()
    }

    fn fifty() -> Self {
        Self::hundred() / Self::two()
    }

    fn hundred() -> Self;
}

impl Numeric for f32 {
    fn abs(self) -> Self {
        self.abs()
    }

    fn one() -> Self {
        1.0
    }

    fn two() -> Self {
        2.0
    }

    fn fifty() -> Self {
        50.0
    }

    fn hundred() -> Self {
        100.0
    }
}

impl Numeric for f64 {
    fn abs(self) -> Self {
        self.abs()
    }

    fn one() -> Self {
        1.0
    }

    fn two() -> Self {
        2.0
    }

    fn fifty() -> Self {
        50.0
    }

    fn hundred() -> Self {
        100.0
    }
}

impl Numeric for i32 {
    fn abs(self) -> Self {
        self.abs()
    }

    fn one() -> Self {
        1
    }

    fn two() -> Self {
        2
    }

    fn fifty() -> Self {
        50
    }

    fn hundred() -> Self {
        100
    }
}

impl Numeric for i64 {
    fn abs(self) -> Self {
        self.abs()
    }

    fn one() -> Self {
        1
    }

    fn two() -> Self {
        2
    }

    fn fifty() -> Self {
        50
    }

    fn hundred() -> Self {
        100
    }
}

/// Trait for types that can be used as series indices.
pub trait Indexable: Copy + Default + PartialOrd + PartialEq + Ord + Eq + std::hash::Hash {}

/// Blanket implementation of Indexable for all suitable types.
impl<T> Indexable for T where T: Copy + Default + PartialOrd + PartialEq + Ord + Eq + std::hash::Hash
{}
