# desktop

Desktop entrypoint and composition root for UAVRed.

## Position in the system
`apps/desktop` is the executable shell. It starts the GPUI app and wires all crates together.

## What will be implemented here
- Startup sequence: config load, logging init, runtime boot.
- Dependency wiring: `ui + logic + red + infra + database + memory`.
- Process lifecycle: start/stop hooks, background task orchestration, graceful shutdown.
- Desktop-specific concerns: local integrations, platform-specific runtime behavior.

## What it should NOT do
- No core pentest strategy logic.
- No direct TTP reasoning implementation.
- No domain rule implementation.
