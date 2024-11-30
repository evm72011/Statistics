use std::fmt::Display;
use num::Float;

use crate::utils::binomial_coeff;

use super::Binomial;
use super::super::DiscreteDistrib;

impl<T> DiscreteDistrib<T> for Binomial<T> 
where 
    T: Float + Display 
{
    fn df(&self, x: u64) -> T {
        match x {
            x if x > 0 => {
                let coefficient = T::from(binomial_coeff(self.n, x)).unwrap();
                coefficient * self.p.powi(x as i32) * (T::one() - self.p).powi((self.n - x) as i32)
            },
            _ => T::zero()
        }
    }

    fn cdf(&self, _: T) -> T {
        panic!("Not implemented")
    }

    fn icdf(&self, _: T) -> u64 {
        panic!("Not implemented")
    }
}
