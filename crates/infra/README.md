# infra

External capability adapters (execution connectors).

## Position in the system
`infra` is for adapters that connect UAVRed runtime to external execution capabilities.

## What problem it solves
Provide concrete bridges from `red/logic` to tools, protocols, sandboxes, and hardware.

## Planned responsibilities
- Tool adapters: MCP tools, command wrappers, browser/traffic tools.
- Sandbox adapters: BoxLite/VM/container execution channels.
- Protocol adapters: MAVLink, RF command channels, cloud API connectors.
- Hardware adapters: USRP/HackRF/RFSoC control interfaces (when enabled).

## Clarification: infra vs database vs memory
All three are infrastructure-layer concerns, but split by concern:
- `infra`: external execution/connectivity adapters.
- `database`: durable persistence.
- `memory`: cognitive memory strategy and retrieval behavior.

## Dependency boundary
- Can depend on `logic`/`red` contracts.
- Must not own orchestration decisions.
- Must not contain domain policy rules.

## Testing
- `crates/infra/tests/` for adapter contract compliance and failure behavior.
