//! Command-line argument parser using `clap`.
//!
//! Provides CLI argument parsing for the application.

use clap::Parser;

/// Main CLI argument structure parsed from command line.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable lulu-logs and connect to the given MQTT broker (default: 127.0.0.1:1883)
    #[arg(long = "lulu", value_name = "HOST:PORT")]
    pub lulu: Option<String>,
}

/// Parse a "host:port" string into separate host and port values.
///
/// Falls back to `127.0.0.1` for host and `1883` for port when parts are
/// missing or invalid.
pub fn parse_lulu_address(s: &str) -> (String, u16) {
    let default_host = "127.0.0.1";
    let default_port: u16 = 1883;

    if let Some((host, port_str)) = s.rsplit_once(':') {
        let host = if host.is_empty() { default_host } else { host };
        let port = port_str.parse::<u16>().unwrap_or(default_port);
        (host.to_string(), port)
    } else {
        // No colon found — treat the whole string as host
        (s.to_string(), default_port)
    }
}

/// Parse command-line arguments from environment.
///
/// # Returns
///
/// Parsed command-line arguments as an `Args` struct.
pub fn parse() -> Args {
    Args::parse()
}
