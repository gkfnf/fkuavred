# logic

Mission control and orchestration layer.

## Position in the system
`logic` is the control-plane brain. It decides **what to run, in what order, and under what policy constraints**.

## What problem it solves
Translate user security intent into executable workflows and coordinate agent execution with governance.

## Planned responsibilities
- Intent parsing into structured security objectives.
- TTP/UTT&CK workflow planning (DAG and dependency resolution).
- Agent assignment and scheduling policy.
- Rules of engagement enforcement (approval gates, denied techniques, budget/time caps).
- Finding normalization and risk synthesis pipeline.

## In UAV pentest lifecycle
`User Intent -> Scope & Policy -> TTP DAG -> Assign to red agents -> Aggregate execution results`

## Dependency boundary
- Depends on domain model contracts.
- Depends on abstraction ports, not concrete infra/database implementations.
- No UI rendering responsibilities.

## Testing
- `crates/logic/tests/` for planning, policy checks, orchestration transitions.
