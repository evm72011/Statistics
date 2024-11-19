use num::Float;
use rand::Rng;

pub trait DiscreteDistrib<T=f64> 
where 
    T: Float 
{

    /// Distribution function
    fn df(&self, x: u64) -> T;
    
    /// Cumulative distribution function
    fn cdf(&self, x: T) -> T;
    
    /// Inverse cumulative distribution function
    fn icdf(&self, p: T) -> u64;

    fn sample(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let p = T::from(rng.gen::<f32>()).unwrap();
        self.icdf(p)
    }

    fn samples(&self, count: u64) -> Vec<u64> {
        (0..count).map(|_| self.sample()).collect()
    }
}
