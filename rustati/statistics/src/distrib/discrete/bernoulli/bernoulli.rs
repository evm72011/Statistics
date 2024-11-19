use num::Float;


pub struct Bernoulli<T=f32> 
where 
    T: Float 
{
    pub p: T
}

impl<T> Bernoulli<T> 
where 
    T: Float 
{
    pub fn new(p: T) -> Bernoulli<T> {
        assert!(p >= T::zero() && p <= T::one(), "Probability must be in [0, 1]");
        Bernoulli::<T> {
            p
        }
    }

    pub fn estimate(samples: Vec<u64>) -> Bernoulli<T> {
        let count_total = samples.len();
        assert!(count_total > 0, "Sample size cannot be zero");
        let count_successes = samples.iter().filter(|&s| *s == 1).count();
        let p = T::from(count_successes as f64 / count_total as f64).unwrap();
        Bernoulli::new(p)
    }
}
