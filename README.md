# MCP Server for *korad-KD3005P*

## Run tests

```bash
# into agent
/test-hw
# agent will test the connected power supply
```


## Add to VS Code Copilot

This server communicates over stdio and can be used directly as an MCP server in VS Code Copilot.

### 1. Install the server

```bash
cargo install mcp-srv-korad-KD3005P
```

Or if you have cloned the repo

```bash
cargo install --path .
```

### 2. Register the server in VS Code

Run the following command in your terminal to add the server to your VS Code user profile (available across all workspaces):

```bash
code --add-mcp '{"name":"korad-KD3005P","command":"mcp-srv-korad-KD3005P"}'
```
