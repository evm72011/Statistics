use num::Float;

use super::Bernoulli;
use super::super::super::Distrib;

impl<T> Distrib<T> for Bernoulli<T> where T: Float {
    fn mean(&self) -> T {
        self.p
    }

    fn variance(&self) -> T {
        self.p * (T::one() - self.p)
    }
}
