use num::Float;

use super::Bernoulli;
use super::super::DiscreteDistrib;

impl<T> DiscreteDistrib<T> for Bernoulli<T> where T: Float {
    fn prob(&self, x: u64) -> T{
        match x {
            0 => self.p,
            1 => T::one() - self.p,
            _ => panic!("Invalid input: x must be 0 or 1")
        }
    }
}
