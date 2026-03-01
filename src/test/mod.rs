//! Test utilities for device discovery and validation.

use super::engine;

/// Test available serial devices and display discovered power supplies.
///
/// Initializes tracing for debugging output and lists all connected
/// Korad KD3005P power supply devices.
pub fn test_available_devices() {
    crate::trace::init_tracing();

    match engine::Engine::available_devices() {
        Ok(devices) => {
            println!("Available serial devices:");
            match serde_json::to_string_pretty(&devices) {
                Ok(json) => println!("{}", json),
                Err(e) => eprintln!("Error serializing to JSON: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error listing serial devices: {}", e);
        }
    }
}
