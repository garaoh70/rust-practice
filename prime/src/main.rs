mod cli;
mod domain;

use crate::domain::prime::{Prime, PrimeGenerator};
use crate::domain::sieve::{SieveGenerator, SieveV1, SieveV2, SieveV3, SieveV4};
use clap::Parser;
use cli::Arguments;

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let limit = args.number;

    let task = tokio::task::spawn_blocking(move || {
        let sieve: Box<dyn SieveGenerator> = match args.sieve {
            1 => Box::new(SieveV1::new(limit)),
            2 => Box::new(SieveV2::new(limit)),
            4 => Box::new(SieveV4::new(limit)),
            _ => Box::new(SieveV3::new(limit)),
        };
        let mut calculator = Prime::new(sieve);
        calculator.run()
    });
    let result = task.await.unwrap();

    let impl_name = result.sieve_name;
    let prime_length = result.primes.len();
    let max_prime = result.primes.last().copied().unwrap_or(0usize);
    let min_prime = result.primes.first().copied().unwrap_or(0usize);
    let elapsed_msec = result.elapsed;
    let throughput = if elapsed_msec != 0 {
        (limit * 1000usize) / elapsed_msec
    } else {
        limit
    };

    if args.show {
        println!("🦀 Rust Prime Sieve Report");
        println!("--------------------------------");
        println!("impl : {}", impl_name);
        println!("limit: {}", limit);
        println!("");
        println!("π(n)       = {}", prime_length);
        println!("min_prime  = {}", min_prime);
        println!("max_prime  = {}", max_prime);
        println!("elapsed    = {:.3}s", (elapsed_msec as f64) / 1000f64);
        println!("throughput = {} n/s", throughput);
        println!("--------------------------------");
    } else {
        println!("elapsed    = {:.3}s", (elapsed_msec as f64) / 1000f64);
    }
}
