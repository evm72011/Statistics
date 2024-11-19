use std::marker::PhantomData;

use num::{Float, Integer};
use rand::Rng;

pub struct Bernoulli<P=f32, V=u32> 
where 
    P: Float, 
    V: Integer 
{
    pub p: P,
    _marker: PhantomData<V>, 
}

impl<P,V> Bernoulli<P,V> 
where 
    P: Float, 
    V: Integer 
{
    pub fn new(p: P) -> Bernoulli<P,V> {
        assert!(p >= P::zero() && p <= P::one(), "Probability must be in [0, 1]");
        Bernoulli::<P,V> {
            p,
            _marker: PhantomData
        }
    }

    pub fn estimate(samples: Vec<V>) -> Bernoulli<P,V> {
        let count_total = samples.len();
        assert!(count_total > 0, "Sample size cannot be zero");
        let count_successes = samples.iter().filter(|&s| *s == V::one()).count();
        let p = P::from(count_successes as f64 / count_total as f64).unwrap();
        Bernoulli::new(p)
    }

    pub fn sample(&self) -> V {
        let mut rng = rand::thread_rng();
        let random = P::from(rng.gen::<f32>()).unwrap();
        match random < self.p {
            true => V::one(),
            false => V::zero(),
        }
    }

    pub fn samples(&self, count: u64) -> Vec<V> {
        (0..count).map(|_| self.sample()).collect()
    }
}
