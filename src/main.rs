use clap::Parser;
use java2rs::process;

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
    process(args.input_dir);
}
