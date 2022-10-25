use std::error::Error;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target URL
    #[arg(short, long)]
    url: String,
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    println!("Target URL: {}", args.url);
    Ok(())
}