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
use tracing::error;
use tracing::info;

use lulu_logs_client::{lulu_init, lulu_shutdown, lulu_start_pulse, LuluClientConfig};

use engine::Engine;

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
            // Lulu-logs: init if --lulu is provided
            let lulu_enabled = args.lulu.is_some();
            if let Some(ref addr) = args.lulu {
                let (host, port) = cli::parse_lulu_address(addr);
                lulu_init(LuluClientConfig {
                    broker_host: host,
                    broker_port: port,
                    ..LuluClientConfig::default()
                })
                .expect("Failed to initialize lulu-logs");
            }

            // Setup: initialize tracing logger for debugging (debug builds only)
            trace::init_tracing();

            info!("Starting Korad KD3005P MCP server with args: {:?}", args);
            lulu_start_pulse("mcp/korad/KD3005P").expect("Failed to start lulu pulse");

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

            error!("MCP service has stopped. Reason: {:?}", _quit_reason);

            // Lulu-logs: drain and shutdown
            if lulu_enabled {
                lulu_shutdown();
            }
        }
    }
}
