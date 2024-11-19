use num::Float;

pub trait Distrib<T=f64> where T: Float {
    fn mean(&self) -> T;
    
    fn std_dev(&self) -> T;

    fn variance(&self) -> T {
        let std_dev = self.std_dev();
        std_dev * std_dev
    }
}
