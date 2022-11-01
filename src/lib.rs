use clap::Parser;
use std::io::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target URL
    #[arg(short, long)]
    pub url: String,
}

pub fn run() -> Result<Args> {
    let args = Args::parse();
    println!("Target URL: {}", args.url);
    Ok(args)
}