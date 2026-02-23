# red

Autonomous offensive agent runtime (execution-side brain).

## Position in the system
`red` is the runtime for **task-executing pentest agents**. This crate is responsible for how an agent actually runs a test step.

## What problem it solves
Given a task assigned by orchestration, `red` executes it through an AI-driven loop and produces verifiable execution artifacts.

## Planned responsibilities
- Agent execution loop (observe → think → act).
- Step-level execution state machine (queued/running/succeeded/failed/retry).
- Tool invocation contract integration (via adapters exposed by `infra`).
- Execution telemetry/events for live UI updates.
- Evidence packaging: command traces, outputs, artifacts, confidence metadata.

## In UAV pentest lifecycle
`Intent -> TTP Plan (logic) -> Agent Execute (red) -> Findings/Risk (logic + ui)`

## Dependency boundary
- Can depend on `logic` contracts and `domain` concepts.
- Must not call UI directly.
- Must not own persistence implementation.

## Testing
- `crates/red/tests/` for execution-loop behavior and state transitions.
