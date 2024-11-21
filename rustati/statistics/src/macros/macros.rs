#[macro_export]
macro_rules! assert_p {
  ($value:expr) => {
      if $value < T::zero() || $value > T::one() {
        panic!("Probability must be in [0,1] but got {}", $value);
      }
  };
}
