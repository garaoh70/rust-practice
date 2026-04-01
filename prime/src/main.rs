mod cli;
mod domain;
mod entity;
mod repository;

use self::domain::prime::{Prime, PrimeGenerator, PrimeResult};
use self::domain::sieve::{SieveGenerator, SieveV1, SieveV2, SieveV3, SieveV4};
use self::repository::*;

use clap::Parser;
use cli::Arguments;

#[tokio::main]
async fn main() {
    // 引数パース
    let args = Arguments::parse();

    // 素数計算
    let result = calculate(&args).await;

    // 結果表示
    display_result(&args, &result);

    // 結果記録
    record_result(&args, &result).await;
}

async fn calculate(args: &Arguments) -> PrimeResult {
    let limit = args.number;

    if cfg!(feature = "dual_sync") && args.is_async {
        let sieve_version = args.sieve;
        let task = tokio::task::spawn_blocking(move || {
            let sieve: Box<dyn SieveGenerator> = match sieve_version {
                1 => Box::new(SieveV1::new(limit)),
                2 => Box::new(SieveV2::new(limit)),
                4 => Box::new(SieveV4::new(limit)),
                _ => Box::new(SieveV3::new(limit)),
            };
            let mut calculator = Prime::new(sieve);
            calculator.run()
        });
        task.await.unwrap()
    } else {
        let sieve: Box<dyn SieveGenerator> = match args.sieve {
            1 => Box::new(SieveV1::new(limit)),
            2 => Box::new(SieveV2::new(limit)),
            4 => Box::new(SieveV4::new(limit)),
            _ => Box::new(SieveV3::new(limit)),
        };
        let mut calculator = Prime::new(sieve);
        calculator.run()
    }
}

fn display_result(args: &Arguments, result: &PrimeResult) {
    let impl_name = result.sieve_name;
    let limit = args.number;
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

async fn record_result(args: &Arguments, result: &PrimeResult) {
    let repository: Box<dyn RepositoryGenerator> = Box::new(SQLite::new());
    repository.append(&args, &result).await;
    let repository: Box<dyn RepositoryGenerator> = Box::new(BinaryU64::new());
    repository.append(&args, &result).await;
}
