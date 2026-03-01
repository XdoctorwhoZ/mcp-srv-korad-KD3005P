//! Runner for power supply device management.
//!
//! Provides direct synchronous access to Korad KD3005P power supplies
//! via serial port. Each runner manages one physical device.

use crate::constants::*;
use crate::engine::helpers::find_port_name_by_serial_number;
use crate::types::OnOffValue;
use ka3005p::Command;
use ka3005p::Ka3005p;
use ka3005p::Switch;
use lulu_logs_client::{lulu_publish, Data, LogLevel};
use tracing::info;

/// Runner for managing a single Korad KD3005P power supply.
///
/// Holds the serial driver and provides synchronous methods
/// to control voltage, current, and output state.
pub struct Runner {
    /// Runner identifier name
    name: String,
    /// Device serial number
    #[allow(dead_code)]
    serial_number: String,
    /// Power supply serial driver
    driver: Ka3005p,
}

impl Runner {
    // ------------------------------------------------------------------------------

    /// Create a new runner and connect to the device.
    ///
    /// Finds the serial port by serial number, initializes the driver,
    /// and enables hardware protection features (OVP, OCP).
    ///
    /// # Arguments
    ///
    /// * `name` - Identifier for this runner instance
    /// * `serial_number` - USB serial number of the device
    pub fn new(name: String, serial_number: String) -> anyhow::Result<Self> {
        info!(
            "[{}] Connecting to device (serial: {})",
            name, serial_number
        );

        let port_name = find_port_name_by_serial_number(&serial_number)?;
        info!("[{}] Found device port: {}", name, port_name);

        let mut driver = Ka3005p::new(port_name.as_str())?;
        driver
            .execute(Command::Ovp(Switch::On))
            .map_err(|e| anyhow::anyhow!("Failed to enable OVP: {:?}", e))?;
        driver
            .execute(Command::Ocp(Switch::On))
            .map_err(|e| anyhow::anyhow!("Failed to enable OCP: {:?}", e))?;

        info!("[{}] Connection established", name);
        let _ = lulu_publish(
            &format!("korad/kd3005p/{}", name),
            "runner",
            LogLevel::Info,
            Data::String(format!("[{}] Connection established", name)),
        );

        Ok(Self {
            name,
            serial_number,
            driver,
        })
    }

    // ------------------------------------------------------------------------------

    /// Set the output voltage.
    ///
    /// Sends the voltage command, saves to device memory, waits for processing,
    /// and reads back the confirmed value.
    pub fn set_voltage(&mut self, voltage: f32) -> anyhow::Result<f32> {
        self.driver
            .execute(Command::Voltage(voltage))
            .map_err(|e| anyhow::anyhow!("Failed to set voltage: {:?}", e))?;

        self.driver
            .execute(Command::Save(1))
            .map_err(|e| anyhow::anyhow!("Failed to save: {:?}", e))?;

        std::thread::sleep(SAFE_CMD_DELAY_MS);

        let read_back = self.driver.read_set_voltage()?;
        info!("[{}] Voltage set to {}", self.name, read_back);
        let _ = lulu_publish(
            &format!("korad/kd3005p/{}", self.name),
            "runner",
            LogLevel::Info,
            Data::Float32(read_back),
        );
        Ok(read_back)
    }

    // ------------------------------------------------------------------------------

    /// Set the output current limit.
    ///
    /// Sends the current command, saves to device memory, waits for processing,
    /// and reads back the confirmed value.
    pub fn set_current(&mut self, current: f32) -> anyhow::Result<f32> {
        self.driver
            .execute(Command::Current(current))
            .map_err(|e| anyhow::anyhow!("Failed to set current: {:?}", e))?;

        self.driver
            .execute(Command::Save(1))
            .map_err(|e| anyhow::anyhow!("Failed to save: {:?}", e))?;

        std::thread::sleep(SAFE_CMD_DELAY_MS);

        let read_back = self.driver.read_set_current()?;
        info!("[{}] Current set to {}", self.name, read_back);
        let _ = lulu_publish(
            &format!("korad/kd3005p/{}", self.name),
            "runner",
            LogLevel::Info,
            Data::Float32(read_back),
        );
        Ok(read_back)
    }

    // ------------------------------------------------------------------------------

    /// Read the current voltage setting.
    pub fn get_voltage(&mut self) -> anyhow::Result<f32> {
        Ok(self.driver.read_set_voltage()?)
    }

    // ------------------------------------------------------------------------------

    /// Read the current current setting.
    pub fn get_current(&mut self) -> anyhow::Result<f32> {
        Ok(self.driver.read_set_current()?)
    }

    // ------------------------------------------------------------------------------

    /// Set the power output state (on/off).
    ///
    /// When turning off, saves settings to device memory.
    /// Returns the confirmed state after read-back.
    pub fn set_state(&mut self, state: OnOffValue) -> anyhow::Result<bool> {
        match state {
            OnOffValue::On => {
                self.driver.execute(Command::Power(Switch::On))?;
            }
            OnOffValue::Off => {
                self.driver.execute(Command::Power(Switch::Off))?;
                self.driver
                    .execute(Command::Save(1))
                    .map_err(|e| anyhow::anyhow!("Failed to save: {:?}", e))?;
            }
        }

        let read_back = self.driver.read_output_enable()?;
        info!("[{}] Output state: {}", self.name, read_back);
        let _ = lulu_publish(
            &format!("korad/kd3005p/{}", self.name),
            "runner",
            LogLevel::Info,
            Data::Bool(read_back),
        );
        Ok(read_back)
    }

    // ------------------------------------------------------------------------------

    /// Read the current power output state.
    pub fn get_state(&mut self) -> anyhow::Result<bool> {
        Ok(self.driver.read_output_enable()?)
    }

    // ------------------------------------------------------------------------------
}
