#[macro_export]
macro_rules! impl_for_primitive {
    ($macro:tt: unsigned) => {
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(u128);
        $macro!(usize);
    };

    ($macro:tt: signed) => {
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(i128);
        $macro!(isize);
    };

    ($macro:tt: float) => {
        $macro!(f32);
        $macro!(f64);
    };

    ($macro:tt: $( $type:tt ),*) => {
        $(
            $crate::impl_for_primitive!($macro: $type);
        )*
    };
}
