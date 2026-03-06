pub mod helpers;

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use lulu_logs_client::{lulu_publish, Data, LogLevel};
use tracing::info;
use tracing::instrument;

use crate::constants::*;
use crate::runner::Runner;
use crate::types::OnOffValue;

use helpers::*;

/// Format a decimal string to the specified number of decimal places.
///
/// - If fewer decimals than required, pads with zeros
/// - If more decimals than required, truncates excess decimals
fn format_decimal(value: &str, decimal_places: usize) -> String {
    let parts: Vec<&str> = value.split('.').collect();

    match parts.as_slice() {
        [integer_part] => {
            format!("{}.{}", integer_part, "0".repeat(decimal_places))
        }
        [integer_part, decimal_part] => {
            let current_decimals = decimal_part.len();
            if current_decimals < decimal_places {
                format!(
                    "{}.{}{}",
                    integer_part,
                    decimal_part,
                    "0".repeat(decimal_places - current_decimals)
                )
            } else if current_decimals > decimal_places {
                format!("{}.{}", integer_part, &decimal_part[..decimal_places])
            } else {
                value.to_string()
            }
        }
        _ => value.to_string(),
    }
}

/// Engine manages power supply runners.
///
/// Coordinates device connections and power supply control.
/// Supports multiple power supply instances via a HashMap of runners.
#[derive(Clone)]
pub struct Engine {
    /// Active runners by device name
    runners: Arc<tokio::sync::Mutex<HashMap<String, Arc<Mutex<Runner>>>>>,
}

// ================

impl Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine").finish()
    }
}

// ================

impl Engine {
    // ------------------------------------------------------------------------------

    /// Create a new engine instance.
    pub fn new() -> Self {
        Self {
            runners: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    // ------------------------------------------------------------------------------

    /// Connect a device by name and serial number.
    ///
    /// Creates a new runner for the device (connects to serial port).
    /// The connection happens in a blocking task to avoid blocking the async runtime.
    pub async fn connect_device(&self, name: &str, serial_number: &str) -> anyhow::Result<()> {
        let name_owned = name.to_string();
        let sn_owned = serial_number.to_string();

        let runner =
            tokio::task::spawn_blocking(move || Runner::new(name_owned, sn_owned)).await??;

        self.runners
            .lock()
            .await
            .insert(name.to_string(), Arc::new(Mutex::new(runner)));

        info!("Runner '{}' created and connected", name);
        let _ = lulu_publish(
            &format!("korad/kd3005p/{}", name),
            "logs",
            LogLevel::Info,
            Data::String("Runner created and connected".to_string()),
        );
        Ok(())
    }

    // ------------------------------------------------------------------------------

    /// List the names of all active power supply instances.
    pub async fn list_instance_names(&self) -> Vec<String> {
        let runners = self.runners.lock().await;
        runners.keys().cloned().collect()
    }

    // ------------------------------------------------------------------------------

    /// Set voltage and/or current parameters for a power supply.
    #[instrument(level = "trace")]
    pub async fn set_power_parameters(
        &self,
        name: &str,
        voltage: Option<String>,
        current: Option<String>,
    ) -> anyhow::Result<()> {
        let runner = self.get_runner(name).await?;

        if let Some(v) = voltage {
            let formatted = format_decimal(&v, VOLTAGE_DECIMAL_NB);
            let voltage_f32: f32 = formatted
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid voltage value: {}", v))?;
            let runner_clone = runner.clone();
            tokio::task::spawn_blocking(move || {
                runner_clone.lock().unwrap().set_voltage(voltage_f32)
            })
            .await??;
        }

        if let Some(c) = current {
            let formatted = format_decimal(&c, CURRENT_DECIMAL_NB);
            let current_f32: f32 = formatted
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid current value: {}", c))?;
            let runner_clone = runner.clone();
            tokio::task::spawn_blocking(move || {
                runner_clone.lock().unwrap().set_current(current_f32)
            })
            .await??;
        }

        Ok(())
    }

    // ------------------------------------------------------------------------------

    /// Get current voltage and current parameters for a power supply.
    pub async fn get_power_parameters(&self, name: &str) -> anyhow::Result<(String, String)> {
        let runner = self.get_runner(name).await?;

        let runner_clone = runner.clone();
        let (voltage, current) = tokio::task::spawn_blocking(move || {
            let mut r = runner_clone.lock().unwrap();
            let v = r.get_voltage()?;
            let c = r.get_current()?;
            Ok::<_, anyhow::Error>((v, c))
        })
        .await??;

        Ok((
            format!("{:.VOLTAGE_DECIMAL_NB$}", voltage),
            format!("{:.CURRENT_DECIMAL_NB$}", current),
        ))
    }

    // ------------------------------------------------------------------------------

    /// Set the power output state (on/off).
    pub async fn set_power_state(&self, name: &str, state: OnOffValue) -> anyhow::Result<()> {
        let runner = self.get_runner(name).await?;

        tokio::task::spawn_blocking(move || runner.lock().unwrap().set_state(state)).await??;

        Ok(())
    }

    // ------------------------------------------------------------------------------

    /// Get the current power output state.
    pub async fn get_power_state(&self, name: &str) -> anyhow::Result<OnOffValue> {
        let runner = self.get_runner(name).await?;

        let enabled =
            tokio::task::spawn_blocking(move || runner.lock().unwrap().get_state()).await??;

        Ok(if enabled {
            OnOffValue::On
        } else {
            OnOffValue::Off
        })
    }

    // ------------------------------------------------------------------------------

    /// Get the status of a power supply instance.
    pub async fn get_instance_status(&self, name: &str) -> anyhow::Result<String> {
        let runners = self.runners.lock().await;
        if runners.contains_key(name) {
            Ok("Instance is running normally.".to_string())
        } else {
            Err(anyhow::anyhow!(
                "Power supply instance '{}' not found",
                name
            ))
        }
    }

    // ------------------------------------------------------------------------------

    /// Get a runner by name, returning an error if not found.
    async fn get_runner(&self, name: &str) -> anyhow::Result<Arc<Mutex<Runner>>> {
        let runners = self.runners.lock().await;
        runners
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Power supply instance '{}' not found", name))
    }

    // ------------------------------------------------------------------------------

    /// List available Korad KD3005P serial devices.
    ///
    /// Scans all available serial ports and identifies Korad KD3005P power supplies
    /// by querying device identification.
    #[instrument(level = "trace")]
    pub fn available_devices() -> anyhow::Result<Vec<DeviceIdentity>> {
        let port_infos = serialport::available_ports()?;
        let filtered_ports: Vec<_> = port_infos
            .into_iter()
            .filter(|port_info| {
                #[cfg(target_os = "macos")]
                {
                    let name_lower = port_info.port_name.to_lowercase();
                    let keep = port_info.port_name.starts_with("/dev/cu.")
                        && !name_lower.contains("bluetooth")
                        && !name_lower.contains("debug");

                    if !keep {
                        info!(
                            "Filtering out port: {} (not matching criteria)",
                            port_info.port_name
                        );
                    }
                    keep
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = port_info;
                    true
                }
            })
            .collect();

        let mut devices = Vec::new();

        // Test: check each port with *IDN? command
        for port_info in &filtered_ports {
            info!(
                "Port: {} - Type: {:?}",
                port_info.port_name, port_info.port_type
            );
            if let Some(device_identity) = Self::is_korad_serial_port(port_info) {
                devices.push(device_identity);
            }
        }

        Ok(devices)
    }

    // ------------------------------------------------------------------------------

    /// Check if a serial port is a Korad KD3005P power supply.
    ///
    /// Sends the *IDN? command to the device and verifies the response matches
    /// the expected Korad KD3005P signature and USB VID/PID.
    fn is_korad_serial_port(port_info: &serialport::SerialPortInfo) -> Option<DeviceIdentity> {
        // Request: send *IDN? and retrieve response
        match Self::request_device_idn(&port_info.port_name) {
            Ok(response) => {
                info!("✓ Port {}: {}", port_info.port_name, response.trim());

                // Verify: check if response matches Korad KD3005P signature
                let response_str = response.trim();
                if !response_str.starts_with("KORAD KD3005P") {
                    return None;
                }

                // Validate: check USB VID/PID
                if let serialport::SerialPortType::UsbPort(usb_info) = &port_info.port_type {
                    if usb_info.vid == 0x0416 && usb_info.pid == 0x5011 {
                        // Create: build device identity
                        let device_identity = DeviceIdentity {
                            product: "KORAD KD3005P".to_string(),
                            serial_number: usb_info
                                .serial_number
                                .clone()
                                .unwrap_or("unknown".to_ascii_lowercase()),
                        };
                        return Some(device_identity);
                    }
                }
                None
            }
            Err(e) => {
                info!("✗ Port {}: Failed - {}", port_info.port_name, e);
                None
            }
        }
    }

    // ------------------------------------------------------------------------------

    /// Request device identification from a serial port.
    ///
    /// Sends the IEEE 488.2 *IDN? command and reads the device response.
    pub fn request_device_idn(port_name: &str) -> anyhow::Result<String> {
        // Open: establish serial port connection with timeout
        let mut port = serialport::new(port_name, 9600)
            .timeout(Duration::from_secs(5))
            .open()?;

        // Send: transmit IEEE 488.2 *IDN? command
        port.write_all(b"*IDN?\n")?;
        port.flush()?;

        // Read: receive device identification response
        let mut buffer = [0u8; 512];
        let bytes_read = port.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        // Log: trace device response for debugging
        info!(
            "Port {:?}: Response to *IDN?: {:?}",
            port_name,
            response.trim()
        );
        Ok(response)
    }

    // ------------------------------------------------------------------------------
}
