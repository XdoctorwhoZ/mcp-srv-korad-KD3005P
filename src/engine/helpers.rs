//! Helper functions for device identification and port discovery.

use serde::Deserialize;
use serde::Serialize;

/// Device identity information for Korad power supplies.
///
/// Used to identify and locate power supply devices on available serial ports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIdentity {
    /// Product name
    pub product: String,
    /// Serial number of the device
    pub serial_number: String,
}

/// Find the serial port name by device serial number.
///
/// Searches all available USB ports and returns the port name of the device
/// matching the provided serial number.
///
/// # Arguments
///
/// * `serial_number` - The serial number to search for.
///
/// # Returns
///
/// The port name on success, or an error if no matching device is found.
pub fn find_port_name_by_serial_number(serial_number: &str) -> anyhow::Result<String> {
    let ports = serialport::available_ports()?;

    for port in ports {
        if let serialport::SerialPortType::UsbPort(usb_info) = &port.port_type {
            if let Some(port_serial_number) = &usb_info.serial_number {
                if port_serial_number == serial_number {
                    return Ok(port.port_name);
                }
            }
        }
    }

    Err(anyhow::anyhow!(
        "No serial port found with serial number: {}",
        serial_number
    ))
}
