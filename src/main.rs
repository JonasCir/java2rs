use clap::Parser;
use java2rs::process;
use std::fs;

/// java2rs command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input directory
    #[clap(short, long)]
    input_dir: String,
}

fn main() {
    let args = Args::parse();
    let input = fs::read_to_string(args.input_dir).unwrap();
    process(&input);
}
