# uavred

Autonomous red-team GUI platform for drone ecosystem security testing.

## Repository Layout
- `apps/desktop`: desktop entrypoint and composition root.
- `crates/red`: autonomous pentest agent runtime (execution layer).
- `crates/logic`: mission orchestration and policy layer.
- `crates/infra`: execution adapters (tool/protocol/hardware/sandbox connectors).
- `crates/database`: durable persistence and audit data store.
- `crates/memory`: cognitive memory strategy and retrieval layer.
- `crates/ui`: GPUI presentation layer.
- `tests/e2e`: cross-module end-to-end tests.

## Testing Strategy
- Module tests live in `crates/*/tests`.
- Cross-module integration flows live in `tests/e2e`.
