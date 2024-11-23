use num::Float;

use super::Binomial;
use super::super::super::Distrib;

impl<T> Distrib<T> for Binomial<T> where T: Float {
    fn mean(&self) -> T {
        T::from(self.n).unwrap() * self.p
    }

    fn variance(&self) -> T {
        T::from(self.n).unwrap() * self.p * (T::one() - self.p)
    }

}
