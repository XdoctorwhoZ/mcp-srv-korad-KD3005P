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
  - `connected`: the 'boolean' connection status of the runner.
  - `logs`: a generic 'string' attribute for logs of the runner.
  - `serial_number`: the 'string' serial number of the power supply. (e.g. KD3005P-123456)
  - `serial_port`: the 'string' serial port of the power supply. (e.g. COM3, /dev/ttyUSB0)
- For runner logs it is useless to append the name into the log message, since the source name already contains the runner name. So the log message should only contain the log content without the runner name.
