use num::Float;

pub struct Binomial<T=f64> 
where 
    T: Float 
{
    pub n: u64,
    pub p: T
}

impl<T> Binomial<T> 
where 
    T: Float 
{
    pub fn new(n: u64, p: T) -> Self {
        Binomial::<T> { n, p }
    }

    pub fn estimate(samples: Vec<u64>) -> Self {
        assert_sample_empty!(samples);
        let n = samples.len() as u64;
        let count_successes = samples.iter().filter(|&s| *s == 1).count();
        let p = T::from(count_successes as f64 / n as f64).unwrap();
        Binomial::<T>::new(n, p)
    }
}
