use std::fmt::Display;
use num::Float;

use super::Bernoulli;
use super::super::DiscreteDistrib;

impl<T> DiscreteDistrib<T> for Bernoulli<T> 
where 
    T: Float + Display
{
    fn df(&self, x: u64) -> T {
        match x {
            0 => T::one() - self.p,
            1 => self.p,
            _ => T::zero()
        }
    }

    fn cdf(&self, x: T) -> T {
        match x {
            x if x < T::zero() => T::zero(),
            x if x < T::one() => T::one() - self.p,
            _ => T::one()
        }
    }

    fn icdf(&self, p: T) -> u64 {
        assert_p!(p);
        match p {
            p if p < T::one() - self.p => 0,
            _ => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn df() {
        let bernoulli = Bernoulli::new(0.2);
        assert_eq!(bernoulli.df(0), 0.8);
        assert_eq!(bernoulli.df(1), 0.2);
    }

    #[test]
    fn cdf() {
        let bernoulli = Bernoulli::new(0.2);
        assert_eq!(bernoulli.cdf(-1.0), 0.0);
        assert_eq!(bernoulli.cdf(0.5), 0.8);
        assert_eq!(bernoulli.cdf(2.0), 1.0);
    }

    #[test]
    fn icdf() {
        let bernoulli = Bernoulli::new(0.2);
        assert_eq!(bernoulli.icdf(0.5), 0u64);
        assert_eq!(bernoulli.icdf(1.0), 1u64);
    }
}
