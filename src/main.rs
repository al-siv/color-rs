//! Main entry point for the color-rs CLI application
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use color_rs::{ColorRs, cli};
use color_rs::logger::{DEFAULT_LOGGER, LOGGER_TRACE, LOGGER_DEBUG, LOGGER_INFO, LOGGER_WARN, LOGGER_ERROR};

fn main() -> color_rs::Result<()> {
    // Parse command line arguments
    let cli_args = cli::Cli::parse();

    // Select logger based on global flag
    let logger: &'static dyn color_rs::logger::Logger = match cli_args.log_level {
        None => &DEFAULT_LOGGER,
        Some(cli::LogLevelCli::None) => &DEFAULT_LOGGER,
        Some(cli::LogLevelCli::Trace) => &LOGGER_TRACE,
        Some(cli::LogLevelCli::Debug) => &LOGGER_DEBUG,
        Some(cli::LogLevelCli::Info) => &LOGGER_INFO,
        Some(cli::LogLevelCli::Warn) => &LOGGER_WARN,
        Some(cli::LogLevelCli::Error) => &LOGGER_ERROR,
    };

    // Create color-rs instance and process command (logger currently used in command execution paths)
    let color_rs = ColorRs::new();

    match cli_args.command {
        cli::Commands::Gradient(args) => color_rs.generate_gradient(args)?,
        cli::Commands::Color(args) => {
            // Validate arguments before processing
            args.validate()?;
            let result = color_rs.color_match(&args)?;
            println!("{result}");
        }
        cli::Commands::Hue(args) => {
            // Validate arguments before processing
            args.validate()?;
            // Hue analysis already routes through command execution; ensure future wiring uses logger
            color_rs.analyze_hue(&args)?;
            logger.debug("Hue analysis completed");
        }
    }

    Ok(())
}
