use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Database
    #[arg(short, long)]
    pub database: String,

    /// Bind
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    pub bind: String,
}
