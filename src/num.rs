use std::ops::Range;

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0 as $t
            }
        }
    };
}

crate::impl_for_primitive!(impl_zero: unsigned, signed, float);

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($t:ty) => {
        impl One for $t {
            fn one() -> Self {
                1 as $t
            }
        }
    };
}

crate::impl_for_primitive!(impl_one: unsigned, signed, float);

pub trait CheckedOp
where
    Self: Sized,
{
    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;
    fn checked_mul(self, other: Self) -> Option<Self>;
    fn checked_div(self, other: Self) -> Option<Self>;
}

macro_rules! impl_checked_op {
    ($t:ty) => {
        impl CheckedOp for $t {
            fn checked_add(self, other: Self) -> Option<Self> {
                self.checked_add(other)
            }
            fn checked_sub(self, other: Self) -> Option<Self> {
                self.checked_sub(other)
            }
            fn checked_mul(self, other: Self) -> Option<Self> {
                self.checked_mul(other)
            }
            fn checked_div(self, other: Self) -> Option<Self> {
                self.checked_div(other)
            }
        }
    };
}

crate::impl_for_primitive!(impl_checked_op: unsigned, signed);

pub trait Distance {
    /// return the distance between two numbers
    fn distance(self, other: Self) -> Self;
}

impl<Number> Distance for Number
where
    Number: std::cmp::Ord + std::ops::Sub<Number, Output = Number> + Copy,
{
    fn distance(self, other: Self) -> Self {
        self.max(other) - self.min(other)
    }
}

pub trait Pattern {
    type N;

    fn matches(&self, n: Self::N) -> bool;
}

impl Pattern for Range<isize> {
    type N = isize;

    fn matches(&self, n: Self::N) -> bool {
        self.contains(&n)
    }
}

impl Pattern for isize {
    type N = isize;

    fn matches(&self, n: Self::N) -> bool {
        *self == n
    }
}

impl Pattern for fn(isize) -> bool {
    type N = isize;

    fn matches(&self, n: Self::N) -> bool {
        self(n)
    }
}
