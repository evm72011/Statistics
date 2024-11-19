use num::Float;

use super::Bernoulli;
use super::super::super::Distrib;

impl<P> Distrib<P> for Bernoulli<P> where P: Float {
    fn mean(&self) -> P {
        self.p
    }

    fn std_dev(&self) -> P {
        self.p * (P::one() - self.p)
    }
}
