use statistics::distrib::{Bernoulli, DiscreteDistrib, Distrib}; // Binomial,

fn main() {
    let bernoulli = Bernoulli::new(0.99);
    let samples = bernoulli.samples(10);
    println!("Samples: {:?}", samples);
    println!("Bernoulli mean: {}", bernoulli.mean());
    println!("Bernoulli std dev:{}", bernoulli.std_dev());

    let bernoulli = Bernoulli::<f32>::estimate(samples);
    println!("Bernoulli mean: {}", bernoulli.mean());
    println!("Bernoulli std dev:{}", bernoulli.std_dev());

    //let binomial = Binomial::<f32>::new(5, 0.2);
    //println!("{}", binomial.mean());
    //println!("{}", binomial.std_dev());
}
