use num::{Float, Integer};

use crate::utils::binomial_coeff;

use super::Binomial;
use super::super::DiscreteDistrib;

impl<P,V> DiscreteDistrib<P,V> for Binomial<P,V> 
where 
    P: Float,
    V: Integer
{
    fn prob(&self, x: V) -> P {
        match x {
            x if x > V::zero() => {
                let coeff = P::from(binomial_coeff(self.n, x)).unwrap();
                coeff * self.p.powi(x as i32) * (P::one() - self.p).powi((self.n - x) as i32)
            },
            _ => P::zero()
        }
    }
}
