//! Main entry point for the color-rs CLI application
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use color_rs::{ColorRs, cli};

fn main() -> color_rs::Result<()> {
    // Parse command line arguments
    let cli_args = cli::Cli::parse();

    // Create color-rs instance and process command
    let color_rs = ColorRs::new();

    match cli_args.command {
        cli::Commands::Gradient(args) => color_rs.generate_gradient(args)?,
        cli::Commands::Color(args) => {
            // Validate arguments before processing
            args.validate()?;
            let result = color_rs.color_match(&args)?;
            println!("{result}");
        }
    }

    Ok(())
}
