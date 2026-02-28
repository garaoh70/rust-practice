mod domain;
use crate::domain::prime::{Prime, PrimeGenerator};
#[allow(unused_imports)]
use crate::domain::sieve::{SieveV1, SieveV2, SieveV3};

fn main() {
    let limit = 50usize;

    let sieve = SieveV3::new(limit);
    let implement = type_of(&sieve).rsplit("::").next().unwrap();

    let mut calculator = Prime::new(sieve);
    calculator.run();

    let prime_length = calculator.values().len();
    let max_prime = calculator.values().last().copied().unwrap_or(0usize);
    let min_prime = calculator.values().first().copied().unwrap_or(0usize);
    let elapsed_msec = calculator.elapsed() as usize;
    let elapsed_sec = elapsed_msec / 1000;
    let throughput = if elapsed_sec != 0 {
        limit / elapsed_sec
    } else {
        limit
    };

    println!("🦀 Rust Prime Sieve Report");
    println!("--------------------------------");
    println!("impl : {}", implement);
    println!("limit: {}", limit);
    println!("");
    println!("π(n)       = {}", prime_length);
    println!("min_prime  = {}", min_prime);
    println!("max_prime  = {}", max_prime);
    println!("elapsed    = {:.3}s", (elapsed_msec as f64) / 1000f64);
    println!("throughput = {} n/s", throughput);
    println!("--------------------------------");
}

#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
