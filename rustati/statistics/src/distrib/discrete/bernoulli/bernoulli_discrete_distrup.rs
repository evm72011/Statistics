use num::Float;

use super::Bernoulli;
use super::super::DiscreteDistrib;

impl<T> DiscreteDistrib<T> for Bernoulli<T> where T: Float {
    fn df(&self, x: u64) -> T {
        match x {
            0 => T::one() - self.p,
            1 => self.p,
            _ => panic!("Invalid input: x must be 0 or 1")
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
        assert!(p >= T::zero() && p <= T::one(), "Probability must be in [0, 1]");
        match p {
            p if p < self.p => 0,
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
        //TODO
        let bernoulli = Bernoulli::new(0.2);
        assert_eq!(bernoulli.cdf(-1.0), 0.0);
        assert_eq!(bernoulli.cdf(0.5), 0.8);
        assert_eq!(bernoulli.cdf(2.0), 1.0);
    }
}