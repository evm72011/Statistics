use num::Float;

pub struct Binomial<T=f32> where T: Float {
    pub n: u64,
    pub p: T
}

impl<T> Binomial<T> where T: Float {
    pub fn new(n: u64, p: T) -> Binomial<T> {
        Binomial::<T> { n, p }
    }
}
