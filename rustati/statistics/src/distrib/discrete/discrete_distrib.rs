use num::Float;

pub trait DiscreteDistrib<T=f64> 
where 
    T: Float 
{
    fn prob(&self, x: u64) -> T;
}
