mod domain;
#[allow(unused_imports)]
use crate::domain::prime::{PrimeGenerator, PrimeV1, PrimeV2, PrimeV3};

fn main() {
    let mut sieve = PrimeV3::new();

    sieve.run(50usize);

    let elapsed = sieve.elapsed();
    for prime in sieve.values() {
        println!("{}", prime);
    }
    println!("Elapsed: {} msec", elapsed);
}
