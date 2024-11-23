use num::Float;

pub trait Distrib<T=f64> where T: Float {
    fn mean(&self) -> T;
    
    fn variance(&self) -> T;
        
    fn std_dev(&self) -> T {
        self.variance().sqrt()
    }
}
