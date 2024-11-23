#[macro_export]
macro_rules! assert_p {
    ($value:expr) => {
        if $value < T::zero() || $value > T::one() {
            panic!("Probability must be in [0,1] but got {}", $value);
        }
    };
}

#[macro_export]
macro_rules! assert_sample_empty {
    ($value:expr) => {
        assert!($value.len() > 0, "Sample array can not be empty")
    };
}
