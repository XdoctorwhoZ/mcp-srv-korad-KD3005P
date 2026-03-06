//! MCP service for power supply emulation and control.
//!
//! Implements the Model Context Protocol server for remote power supply management
//! via MCP tools and prompts.

use rmcp::handler::server::router::prompt::PromptRouter;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::prompt_handler;
use rmcp::prompt_router;
use rmcp::service::RequestContext;
use rmcp::tool;
use rmcp::tool_handler;
use rmcp::tool_router;
use rmcp::ErrorData as McpError;
use rmcp::RoleServer;
use rmcp::ServerHandler;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing::warn;

use crate::engine::Engine;
use crate::types::OnOffValue;

/// Parameters for connecting to a device.
#[derive(Serialize, Deserialize, JsonSchema)]
struct ConnectDeviceParams {
    /// Device name identifier
    name: String,
    /// Device serial number
    serial_number: String,
}

// ================

/// Parameters for setting power supply voltage and current.
#[derive(Serialize, Deserialize, JsonSchema)]
struct SetPowerParams {
    /// Device name identifier
    name: String,
    /// Output voltage setting
    voltage: Option<String>,
    /// Current limit setting
    current: Option<String>,
}

// ================

/// Parameters for getting power supply readings.
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetPowerParams {
    /// Device name identifier
    name: String,
}

// ================

/// Parameters for setting power supply state.
#[derive(Serialize, Deserialize, JsonSchema)]
struct SetPowerStateParams {
    /// Device name identifier
    name: String,
    /// Power state: "on" or "off"
    state: String,
}

// ================

/// Parameters for getting power supply state.
#[derive(Serialize, Deserialize, JsonSchema)]
struct GetPowerStateParams {
    /// Device name identifier
    name: String,
}

// ================

/// Parameters for deleting a power supply instance.
#[derive(Serialize, Deserialize, JsonSchema)]
struct DeleteInstanceParams {
    /// Device name identifier
    name: String,
}

// ================

/// Parameters for checking instance status.
#[derive(Serialize, Deserialize, JsonSchema)]
struct CheckInstanceParams {
    /// Device name identifier
    name: String,
}

// ================

/// MCP service for power supply emulation and control.
///
/// Implements the Model Context Protocol server, providing tools for discovering
/// power supplies, managing connections, and controlling power output.
#[derive(Clone)]
pub struct PowerSupplyEmulatorService {
    /// Engine for managing power supplies
    engine: Engine,
    /// MCP tool router
    tool_router: ToolRouter<PowerSupplyEmulatorService>,
    /// MCP prompt router
    prompt_router: PromptRouter<PowerSupplyEmulatorService>,
}

// ================

impl PowerSupplyEmulatorService {
    // --------------------------------------------------------------------------

    /// Create a new power supply emulator service.
    pub fn new(engine: Engine) -> anyhow::Result<Self> {
        Ok(Self {
            engine,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        })
    }
}

// ================

#[tool_router]
impl PowerSupplyEmulatorService {
    // --------------------------------------------------------------------------

    /// List all available Korad KD3005P devices connected to the system.
    #[tool(description = "List available devices plugged to the system")]
    async fn list_available_devices(&self) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: list_available_devices");

        //
        let devices = Engine::available_devices().map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to list available KORAD KD3005P devices: {}", e),
                None,
            )
        })?;
        if devices.is_empty() {
            Ok(CallToolResult::success(vec![Content::text(
                "No available serial devices found".to_string(),
            )]))
        } else {
            let json_output = serde_json::to_string_pretty(&devices).map_err(|e| {
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to serialize devices: {}", e),
                    None,
                )
            })?;
            info!("Found {} available device(s)", devices.len());
            Ok(CallToolResult::success(vec![Content::text(format!(
                "Available serial devices ({}):\n{}",
                devices.len(),
                json_output
            ))]))
        }
    }

    // --------------------------------------------------------------------------

    /// Create a new power supply instance.
    #[tool(description = "Create a new power supply instance")]
    async fn create_instance(
        &self,
        params: Parameters<ConnectDeviceParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: create_instance");

        // Parse parameters
        let name = &params.0.name;
        let serial_number = &params.0.serial_number;
        info!("name          : {}", name);
        info!("serial_number : {}", serial_number);

        // Try to create the instance
        self.engine
            .connect_device(name, serial_number)
            .await
            .map_err(|e| {
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to create power supply instance '{}': {}", name, e),
                    None,
                )
            })?;

        // Log success and return result
        info!("Success");
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Power supply instance '{}' created successfully",
            name
        ))]))
    }

    // --------------------------------------------------------------------------

    /// Check the status of a power supply instance.
    #[tool(description = "Check instance status (running/error)")]
    async fn check_instance(
        &self,
        params: Parameters<CheckInstanceParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: check_instance");

        //
        let name = &params.0.name;

        // Check if instance exists
        let instances = self.engine.list_instance_names().await;
        if !instances.contains(&name.to_string()) {
            return Err(McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("Instance '{}' does not exist", name),
                None,
            ));
        }

        // Get instance status
        let status = self.engine.get_instance_status(name).await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to get status for instance '{}': {}", name, e),
                None,
            )
        })?;

        info!("Instance '{}' status: {}", name, status);
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Instance '{}' status: {}",
            name, status
        ))]))
    }

    // --------------------------------------------------------------------------

    /// List all active power supply instances.
    #[tool(description = "List active instances")]
    async fn list_instances(&self) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: list_instances");

        //
        let instances = self.engine.list_instance_names().await;

        if instances.is_empty() {
            info!("No power supply instances found");
            Ok(CallToolResult::success(vec![Content::text(
                "No power supply instances have been created yet".to_string(),
            )]))
        } else {
            let instance_list = instances.join(", ");
            info!(
                "Found {} power supply instance(s): {}",
                instances.len(),
                instance_list
            );
            Ok(CallToolResult::success(vec![Content::text(format!(
                "Created power supply instances ({}): {}",
                instances.len(),
                instance_list
            ))]))
        }
    }

    // --------------------------------------------------------------------------

    /// Set voltage and/or current parameters for a power supply.
    #[tool(description = "Set voltage and/or current parameters")]
    async fn set_power_parameters(
        &self,
        params: Parameters<SetPowerParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: set_power_parameters");

        // Extract parameters
        let name = &params.0.name;
        let voltage = &params.0.voltage;
        let current = &params.0.current;
        info!("name    : {}", name);
        info!("voltage : {}", voltage.as_deref().unwrap_or("not provided"));
        info!("current : {}", current.as_deref().unwrap_or("not provided"));

        // Validate that at least one parameter is provided
        if voltage.is_none() && current.is_none() {
            warn!(
                "No voltage or current provided for '{}'. At least one must be set.",
                name
            );
            return Err(McpError::new(
                ErrorCode::INVALID_PARAMS,
                "At least one of voltage or current must be provided".to_string(),
                None,
            ));
        }

        // Set power parameters using the engine
        self.engine
            .set_power_parameters(name, voltage.clone(), current.clone())
            .await
            .map_err(|e| {
                warn!("Failed to set power parameters for '{}': {}", name, e);
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to set power parameters for '{}': {}", name, e),
                    None,
                )
            })?;

        // Construct success message
        let mut message_parts = vec![];
        if let Some(v) = voltage {
            message_parts.push(format!("voltage to {}", v));
        }
        if let Some(c) = current {
            message_parts.push(format!("current to {}", c));
        }
        let message = format!(
            "Successfully set {} for power supply '{}'",
            message_parts.join(" and "),
            name
        );

        // Log and return success
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    // --------------------------------------------------------------------------

    /// Get current voltage and current parameters.
    #[tool(description = "Get voltage and current parameters")]
    async fn get_power_parameters(
        &self,
        params: Parameters<GetPowerParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: get_power_parameters");

        //
        let name = &params.0.name;

        let (voltage, current) = self.engine.get_power_parameters(name).await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to get power parameters for '{}': {}", name, e),
                None,
            )
        })?;

        let message = format!(
            "Power supply '{}' parameters: voltage = {}, current = {}",
            name, voltage, current
        );

        info!("{}", message);
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    // --------------------------------------------------------------------------

    /// Set the power output state (on/off).
    #[tool(description = "Set power state (on/off)")]
    async fn set_power_state(
        &self,
        params: Parameters<SetPowerStateParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: set_power_state");

        //
        let name = &params.0.name;
        let state_str = &params.0.state;

        // Parse state string to OnOffValue
        let state = match state_str.to_lowercase().as_str() {
            "on" => OnOffValue::On,
            "off" => OnOffValue::Off,
            _ => {
                return Err(McpError::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid state '{}'. Must be 'on' or 'off'", state_str),
                    None,
                ));
            }
        };

        self.engine
            .set_power_state(name, state)
            .await
            .map_err(|e| {
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to set power state for '{}': {}", name, e),
                    None,
                )
            })?;

        let message = format!(
            "Successfully set power state to '{}' for power supply '{}'",
            state_str, name
        );

        info!("{}", message);
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    // --------------------------------------------------------------------------

    /// Get the current power output state.
    #[tool(description = "Get power state (on/off)")]
    async fn get_power_state(
        &self,
        params: Parameters<GetPowerStateParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: get_power_state");

        //
        let name = &params.0.name;

        let state = self.engine.get_power_state(name).await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to get power state for '{}': {}", name, e),
                None,
            )
        })?;

        let state_str = match state {
            OnOffValue::On => "on",
            OnOffValue::Off => "off",
        };

        let message = format!("Power supply '{}' state: {}", name, state_str);

        info!("{}", message);
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    // --------------------------------------------------------------------------

    /// Delete a power supply instance and close its connection.
    #[tool(description = "Delete a power supply instance")]
    async fn delete_instance(
        &self,
        params: Parameters<DeleteInstanceParams>,
    ) -> Result<CallToolResult, McpError> {
        // Logs
        info!("----------------------------------------------------------------");
        info!("Executing tool: delete_instance");

        //
        let name = &params.0.name;
        info!("name: {}", name);

        // Check if instance exists
        let instances = self.engine.list_instance_names().await;
        if !instances.contains(&name.to_string()) {
            return Err(McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("Instance '{}' does not exist", name),
                None,
            ));
        }

        // Delete the instance
        self.engine.delete_instance(name).await.map_err(|e| {
            McpError::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Failed to delete power supply instance '{}': {}", name, e),
                None,
            )
        })?;

        let message = format!("Power supply instance '{}' deleted successfully", name);

        info!("{}", message);
        Ok(CallToolResult::success(vec![Content::text(message)]))
    }

    // --------------------------------------------------------------------------
}

// ================

#[prompt_router]
impl PowerSupplyEmulatorService {
    // No prompts specified in requirements, but trait requires implementation
}

// ================

#[tool_handler]
#[prompt_handler]
impl ServerHandler for PowerSupplyEmulatorService {
    // --------------------------------------------------------------------------

    /// Get server information and capabilities.
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(include_str!("instructions.md").to_string()),
        }
    }
}
