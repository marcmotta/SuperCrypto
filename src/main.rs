// src/main.rs
/*
 * Main executable for SuperCrypto
 */

use clap::Parser;
use supercrypto::{Result, run};

#[derive(Parser)]
#[command(version, about = "SuperCrypto - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
