//! Command-line argument parser using `clap`.
//!
//! Provides CLI argument parsing and command handling for the application.

use clap::Parser;
use clap::Subcommand;

/// Main CLI argument structure parsed from command line.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Optional MQTT connection string
    #[arg(long = "mqtt")]
    pub mqtt: Option<String>,
}

/// Supported subcommands for the CLI.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Device testing and scanning commands
    Test {
        /// Scan for available devices
        #[arg(long = "scan")]
        scan: bool,
    },
}

/// Parse command-line arguments from environment.
///
/// # Returns
///
/// Parsed command-line arguments as an `Args` struct.
pub fn parse() -> Args {
    Args::parse()
}
