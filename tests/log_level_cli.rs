//! Tests for --log-level CLI flag parsing
use clap::Parser;
use color_rs::cli::{Cli, Commands, LogLevelCli};

#[test]
fn parse_log_level_debug() {
    let args = Cli::parse_from([
        "color-rs", "--log-level", "debug", "hue", "css"
    ]);
    assert!(matches!(args.log_level, Some(LogLevelCli::Debug)));
    match args.command { Commands::Hue(h)=> assert_eq!(h.collection, "css"), _=> panic!("wrong command") }
}

#[test]
fn parse_log_level_none_default() {
    let args = Cli::parse_from(["color-rs", "hue", "css"]);
    assert!(args.log_level.is_none());
}

#[test]
fn parse_log_level_error() {
    let args = Cli::parse_from([
        "color-rs", "--log-level", "error", "hue", "css"
    ]);
    assert!(matches!(args.log_level, Some(LogLevelCli::Error)));
}
