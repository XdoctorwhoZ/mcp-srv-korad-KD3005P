# Project Purpose

The purpose of this project is to provide an MCP server for serial communication operations.

## Documentations

You can find coding rules for Rust in the [Rust Coding Rules](../docs/rust-coding-rules.md) document.

## Lulu logs rules

Runners of this project must follow the following rules when using lulu-logs:
- Each runner is a connection to a power supply and must have its own source name.
- Each runner have the following attributes:
  - `voltage`: the 'float' voltage of the power supply.
  - `current`: the 'float' current of the power supply.
  - `output_enabled`: the 'boolean' output status of the power supply.
  - `logs`: a generic 'string' attribute for logs of the runner.
  - `serial_number`: the 'string' serial number of the power supply. (e.g. KD3005P-123456)
  - `serial_port`: the 'string' serial port of the power supply. (e.g. COM3, /dev/ttyUSB0)
- For runner logs it is useless to append the name into the log message, since the source name already contains the runner name. So the log message should only contain the log content without the runner name.

## Tools exposed by this mcp server

### `list_available_devices`
Scans the system for available Korad KD3005P serial devices and returns their identifiers.
- No parameters.

### `create_instance`
Opens a connection to the power supply identified by its serial number and registers it under the given name.
- `name` _(string)_: identifier to assign to the instance.
- `serial_number` _(string)_: serial number of the target device (e.g. `KD3005P-123456`).

### `check_instance`
Returns the current status (`running` / `error`) of a previously created instance.
- `name` _(string)_: identifier of the instance to check.

### `list_instances`
Lists the names of all currently active power supply instances.
- No parameters.

### `set_power_parameters`
Sets the voltage and/or current limit on the named instance. At least one of `voltage` or `current` must be provided.
- `name` _(string)_: identifier of the instance.
- `voltage` _(string, optional)_: desired output voltage (e.g. `"3.3"`).
- `current` _(string, optional)_: desired current limit (e.g. `"0.5"`).

### `get_power_parameters`
Reads and returns the current voltage and current-limit settings of the named instance.
- `name` _(string)_: identifier of the instance.

### `set_power_state`
Enables or disables the power output of the named instance.
- `name` _(string)_: identifier of the instance.
- `state` _(string)_: `"on"` or `"off"`.

### `get_power_state`
Returns the current output state (`on` or `off`) of the named instance.
- `name` _(string)_: identifier of the instance.

