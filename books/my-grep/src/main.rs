use anyhow;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let cli = Cli::parse();
    if let Err(e) = start(cli) {
        println!("Error: {e}");
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long)]
    file: String,
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let file = File::open(&cli.file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&cli.pattern) {
            println!("{line}");
        }
    }

    Ok(())
}
