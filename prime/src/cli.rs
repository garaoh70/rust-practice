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

    /// Maximum number
    #[arg(
        short,
        long,
        default_value_t = DEFAULT_COUNT,
        value_parser = parse_maximum_number
    )]
    pub number: usize,

    /// Show result
    #[arg(long, default_value_t = false)]
    pub show: bool,
}

fn parse_maximum_number(input: &str) -> Result<usize, String> {
    let numeric_pos = input
        .find(|c: char| !c.is_numeric() && c != '.')
        .unwrap_or(input.len());
    let (num_part, unit_part) = input.split_at(numeric_pos);

    let value: f64 = num_part.parse().expect("parse error");
    let multiplier = match unit_part.to_ascii_uppercase().as_str() {
        "K" => 1_000_f64,
        "M" => 1_000_000_f64,
        "G" => 1_000_000_000_f64,
        "T" => 1_000_000_000_000_f64,
        "" => 1_f64,
        _ => return Err("unknown unit".to_string()),
    };

    let maximum_number = (value * multiplier) as usize;
    if (MIN_COUNT..=MAX_COUNT).contains(&maximum_number) {
        Ok(maximum_number)
    } else {
        Err("out of range".to_string())
    }
}
