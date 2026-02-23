# database

Durable persistence and audit data store.

## Position in the system
`database` owns long-lived state and historical records.

## What problem it solves
Persist trust nodes, assets, topology edges, execution logs, findings, and policy snapshots in an auditable way.

## Planned responsibilities
- Repository implementations and query interfaces.
- Schema and migration management.
- Execution/event persistence for replay and forensic audit.
- Risk/finding persistence and retrieval for reporting.

## In UAV pentest lifecycle
`Execution result (red) -> normalized by logic -> persisted by database -> visualized by ui`

## Dependency boundary
- Exposes persistence interfaces to upper layers.
- Must not include planning/execution strategy.
- Must not include UI logic.

## Testing
- `crates/database/tests/` for repository behavior, migration safety, and query correctness.
