use std::fmt::Display;

use num::Float;

pub struct Bernoulli<T=f64> where T: Float 
{
    pub p: T
}

impl<T> Bernoulli<T> 
where 
    T: Float + Display
{
    pub fn new(p: T) -> Self {
        assert_p!(p);
        Bernoulli::<T> { p }
    }

    pub fn estimate(samples: Vec<u64>) -> Self {
        assert_sample_empty!(samples);
        let count_total = samples.len();
        let count_successes = samples.iter().filter(|&s| *s == 1).count();
        let p = T::from(count_successes as f64 / count_total as f64).unwrap();
        Bernoulli::<T>::new(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::distrib::Distrib;

    use super::*;

    #[test]
    fn estimate() {
        let samples = vec![0, 0, 1, 1, 0, 1, 1, 0, 1, 0];
        let bernoulli = Bernoulli::<f64>::estimate(samples);
        assert_eq!(bernoulli.mean(), 0.5);
        assert_eq!(bernoulli.variance(), 0.25);
    }
}
