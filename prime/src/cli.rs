use clap::{Parser, builder::ValueParser};

const MIN_COUNT: usize = 2;
const MAX_COUNT: usize = 20_000_000_000;
const DEFAULT_COUNT: usize = 50;
const DEFAULT_SIEVE: usize = 3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Sieve version (1,2,3,4 only. Others => 3)
    #[arg(
        short,
        long,
        default_value_t = DEFAULT_SIEVE,
        value_parser = ValueParser::new(|s: &str| -> Result<usize, String> {
            let v: usize = s.parse().unwrap_or(DEFAULT_SIEVE);
            Ok(match v { 1..=4 => v, _ => 3 })
        })
    )]
    pub sieve: usize,

    /// Maximum number (clamped to 2..=20_000_000_000)
    #[arg(
        short,
        long,
        default_value_t = DEFAULT_COUNT,
        value_parser = ValueParser::new(|s: &str| -> Result<usize, String> {
            let v: usize = s.parse().unwrap_or(DEFAULT_COUNT);
            Ok(v.clamp(MIN_COUNT, MAX_COUNT))
        })
    )]
    pub count: usize,
}
