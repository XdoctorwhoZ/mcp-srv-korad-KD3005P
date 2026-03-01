mod cli;
mod constants;
mod engine;
mod runner;
mod service;
mod test;
mod trace;
mod types;

use rmcp::transport::stdio;
use rmcp::ServiceExt;
use tracing::Level;

use trace::TraceBootstrap;

use engine::Engine;

/// Package name from Cargo.toml at compile time
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

/// Main entry point for the Korad KD3005P MCP server.
///
/// Handles CLI argument parsing and starts either test mode or the MCP service.
#[tokio::main]
async fn main() {
    // Parse: extract CLI arguments
    let args = cli::parse();

    // Handle: CLI commands
    match args.command {
        Some(cli::Command::Test { scan }) => {
            if scan {
                println!("Scanning for available devices...");
                test::test_available_devices();
            } else {
                println!("You need to specify a subcommand. Use --help for more information.");
            }
        }
        None => {
            // Setup: initialize tracing logger for debugging
            TraceBootstrap::default()
                .with_level(if cfg!(debug_assertions) {
                    Level::TRACE
                } else {
                    Level::INFO
                })
                .filter_rmcp()
                .display_target(if cfg!(debug_assertions) { true } else { false })
                .on_mcp_file(PACKAGE_NAME)
                .build()
                .expect("failed to init logger");

            // Initialize: create the engine
            let engine = Engine::new();

            // Start: create and serve the MCP service
            let service = service::PowerSupplyEmulatorService::new(engine)
                .unwrap()
                .serve(stdio())
                .await
                .expect("Failed to serve the PowerSupplyEmulatorService");

            // Wait: for service to finish
            let _quit_reason = service.waiting().await.unwrap();
        }
    }
}
