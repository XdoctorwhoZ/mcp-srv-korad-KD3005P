mod cli;
mod constants;
mod engine;
mod runner;
mod service;
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
/// Handles CLI argument parsing and starts the MCP service.
#[tokio::main]
async fn main() {
    // Setup: initialize tracing logger for debugging (debug builds only)
    trace::init_tracing();

    // Parse: extract CLI arguments
    let args = cli::parse();

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

    info!("Starting Korad KD3005P MCP server with args: {:?}", args);
    lulu_start_pulse("korad/KD3005P", Some(env!("BUILD_VERSION")))
        .expect("Failed to start lulu pulse");

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
