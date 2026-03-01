use std::time::Duration;

/// Safe delay between commands to avoid overwhelming the device
pub const SAFE_CMD_DELAY_MS: Duration = Duration::from_millis(100);
/// Voltage decimal places for display
pub const VOLTAGE_DECIMAL_NB: usize = 2;
/// Current decimal places for display
pub const CURRENT_DECIMAL_NB: usize = 3;
