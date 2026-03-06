# Project Purpose

The purpose of this project is to provide an MCP server for serial communication operations.

## Documentations

You can find coding rules for Rust in the [Rust Coding Rules](../docs/rust-coding-rules.md) document.

## Lulu-logs rules

Runners of this project must follow the following rules when using lulu-logs:
- Each runner is a serial connection and must have its own source name.
- Each runner have the following attributes:
  - `tx_data`: the 'byte' data sent to the device.
  - `rx_data`: the 'byte' data received from the device.
  - `connected`: the 'boolean' connection status of the runner.
  - `logs`: the 'string' logs of the runner.
- For runner logs it is useless to append the name into the log message, since the source name already contains the runner name. So the log message should only contain the log content without the runner name.
