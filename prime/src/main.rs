mod domain;
use crate::domain::prime::{PrimeGenerator, Prime};
#[allow(unused_imports)]
use crate::domain::sieve::{SieveV1, SieveV2, SieveV3};

fn main() {
    let sieve = SieveV3::new(50usize);
    let mut calculator = Prime::new(sieve);

    calculator.run();

    let elapsed = calculator.elapsed();
    for prime in calculator.values() {
        println!("{}", prime);
    }
    println!("Elapsed: {} msec", elapsed);
}
