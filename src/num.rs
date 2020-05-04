pub trait One {
    fn one() -> Self;
}
impl One for f32 {
    fn one() -> Self {
        1.
    }
}

impl One for f64 {
    fn one() -> Self {
        1.
    }
}

macro_rules! impl_one {
    ($t:ty) => {
        impl One for $t {
            fn one() -> Self {
                1
            }
        }
    };
}

crate::impl_for_primitive!(impl_one: unsigned, signed);
