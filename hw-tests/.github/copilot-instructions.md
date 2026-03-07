# Copilot Instructions

## Project Context
This project is a hardware testing toolkit using MCP servers in the context of GitHub Copilot CLI.

## Project Structure
Each test scenario MUST be a Markdown file located in `tests`.

```
project-root
├── tests/                        # Main directory containing all test scenarios organized by feature
│   ├── feature_01/               # Feature folder grouping related test scenarios
│   │   ├── .index.md             # Overview and description of the feature being tested
│   │   ├── 01_scenario.md        # Individual test scenario (numbered sequentially)
│   │   └── 02_scenario.md        # Multiple scenarios per feature folder
│   └── feature_02/
│       ├── .index.md
│       ├── 01_scenario.md
│       └── 02_scenario.md
├── benches/                      # Bench definitions describing connected hardware
│   ├── .constitution.md          # Minimum hardware requirements for the project
│   ├── bench_lab.md              # Concrete bench: maps requirements to real devices
│   └── bench_exemple.md          # Example bench (prefixed with 'bench_')
├── mcp-config.json               # MCP server configuration for tool integration
├── README.md                     # Project overview and general documentation
└── LICENSE                       # Project license
```

### Directory Details

- **tests/**: Container for all automated test scenarios, organized by feature/functionality being tested. Each feature gets its own subdirectory with numbered scenario files.
  - **feature_XX/**: Groups related test scenarios for a specific hardware feature or functionality
  - **.index.md**: Feature-level documentation explaining what is being tested and the overall scope
  - **XX_scenario.md**: Individual test scenarios following the naming convention `01_`, `02_`, etc.

- **benches/**: Contains bench definitions that describe the physical hardware setup used to run the tests. **The user must select a bench file before executing any test scenario.** The selected bench file declares the concrete equipment connected to the server and maps each logical device name to a real physical device.
  - **.constitution.md**: Defines the minimum hardware requirements for the project — the logical device names and their required capabilities (e.g., a power supply named `test_power_supply` with at least 20V output). This file is hardware-agnostic.
  - **bench_XX.md**: Concrete implementation of the constitution — maps each logical device to a real physical device (e.g., brand, model, serial number). Use the `bench_` prefix for all bench files.

- **mcp-config.json**: Configuration file for integrating MCP (Model Context Protocol) servers with the testing toolkit.

- **README.md**: Main project documentation with setup instructions and usage guidelines.

## Bench Workflow

Before executing any test scenario, the user must choose a bench file from `benches/`. The bench file provides the mapping between the logical device names defined in `.constitution.md` and the actual hardware connected to the test server.

### `.constitution.md` Convention

This file lists the hardware requirements as logical device declarations:

```markdown
# Constitution

## Required Equipment

### test_power_supply
- Type: Power Supply
- Minimum voltage: 20V
- Minimum current: 3A
```

### Bench File Convention

Each bench file implements the constitution by binding each logical device name to a real device:

```markdown
---
name: Lab bench A
description: Bench using Korad KD3005P on workstation USB port
---

## Equipment

### test_power_supply
- Driver: korad-kd3005p
- Serial number: 12345678
- Max voltage: 30V
- Max current: 5A
```

## Scenario File Convention
When creating a new scenario, always use this Markdown template:

```markdown
---
name: Scenario name
description: Short description of the hardware test
---

## Steps

1. Step 1
2. Step 2

## Expected Results

- Result 1
- Result 2
```

## Rules
- Each scenario must have a frontmatter header (`name`, `description`)
- A bench file MUST be selected by the user before running any test scenario
- Bench files MUST implement all logical devices declared in `.constitution.md`
- The `.constitution.md` file is hardware-agnostic — it defines requirements, not implementations