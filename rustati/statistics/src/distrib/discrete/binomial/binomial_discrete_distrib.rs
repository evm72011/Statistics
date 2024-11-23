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
                let coeff = T::from(binomial_coeff(self.n, x)).unwrap();
                coeff * self.p.powi(x as i32) * (T::one() - self.p).powi((self.n - x) as i32)
            },
            _ => T::zero()
        }
    }

    fn cdf(&self, x: T) -> T {
        panic!("Not implemented")
    }

    fn icdf(&self, x: T) -> u64 {
        panic!("Not implemented")
    }
}
