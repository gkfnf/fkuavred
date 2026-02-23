# ui

Operator-facing GPUI application layer.

## Position in the system
`ui` is where operators define intent, monitor execution, and review risks/findings.

## What problem it solves
Turn complex autonomous pentest behavior into controllable and explainable operator workflows.

## Planned responsibilities
- Intent authoring and scope definition UX.
- Trust-node/asset topology visualization.
- Live execution timeline (agent step updates, status, evidence drill-down).
- Risk and vulnerability dashboards with verification actions.
- Human override workflows (approve/reject/modify plan and checks).

## Dependency boundary
- Calls `logic` application interfaces.
- Must not directly implement orchestration rules or persistence logic.
- Keybindings must stay configurable.

## Testing
- `crates/ui/tests/` for interaction behavior and view-model state transitions.
