//! Main entry point for the color-rs CLI application

use clap::Parser;
use color_rs::{cli, ColorRs};

fn main() -> color_rs::Result<()> {
    // Print application information
    cli::print_app_info();

    // Parse command line arguments
    let cli_args = cli::Cli::parse();

    // Create color-rs instance and process command
    let color_rs = ColorRs::new();
    
    match cli_args.command {
        cli::Commands::Gradient(args) => color_rs.generate_gradient(args)?,
        cli::Commands::ColorMatch(args) => {
            let result = color_rs.color_match(args)?;
            println!("{}", result);
        },
    }

    Ok(())
}
