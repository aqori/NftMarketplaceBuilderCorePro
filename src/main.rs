// src/main.rs
/*
 * Main executable for NftMarketplaceBuilderCorePro
 */

use clap::Parser;
use nftmarketplacebuildercorepro::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceBuilderCorePro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
