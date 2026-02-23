# Plan
- [ ] Align AGENTS.md with existing rules and Boris Cherny workflow orchestration guidance.
- [ ] Reorganize sections to embed workflow, task management, and verification expectations while reducing redundancy.
- [ ] Add tasks/lessons.md and related guidance so future work records lessons and plans as per the workflow.
- [ ] Review the updated AGENTS and task files for clarity, ensure instructions are consistent.

## Review
- [ ] Document what changed and why along with verification status.

## Current Task: Repository Skeleton + AGENTS Sync (2026-02-23)
- [x] Create `apps/desktop` entry crate and minimal startup wiring.
- [x] Create `crates/{red,logic,infra,database,memory,ui}` with `Cargo.toml`, `src/lib.rs`, and `tests/`.
- [x] Create root `tests/e2e` for cross-module end-to-end tests only.
- [x] Add module `README.md` files with dependency boundaries and test placement rules.
- [x] Update `AGENTS.md` paths (`creates/*` -> `crates/*`) and codify test layout policy.
- [x] Run basic checks (`cargo fmt --check`, `cargo test --workspace`) and log verification outcomes.

### Current Task Review (to be filled)
- [x] Summarize changes and why.
- [x] Record what was verified and any remaining gaps.

Verification:
- Added workspace skeleton and module READMEs with explicit dependency/test boundaries.
- Updated AGENTS.md module README paths and test placement policy.
- `cargo fmt --all -- --check` passed.
- `cargo test --workspace` passed (module tests + e2e smoke).
- Remaining gap: business logic is placeholder scaffold only; feature implementation is pending.


## Current Task: PRD Reframe + Module Responsibility Clarification (2026-02-24)
- [x] Re-clarify module positioning with `red` as autonomous pentest agent runtime.
- [x] Rewrite module README files to explain: responsibility, solved problem, UAV pentest lifecycle mapping, and dependency boundaries.
- [x] Update PRD.md with Trust Node generalization (not only IP assets), node/agent unified governance, and topology requirements.
- [x] Add requirements for execution-step live updates (LuaN1ao-style), checklist authoring, UTT&CK mapping, and risk scoring/exposure workflow.
- [x] Add explicit description of `infra` vs `database` vs `memory` relationship.
- [x] Run consistency checks on docs and record open design questions.

### Current Task Review (to be filled)
- [x] Summarize what changed and why.
- [x] Record what is verified and what remains ambiguous.

Current Task Review Notes:
- Module semantics were re-aligned with product intent: `red` now explicitly documented as autonomous pentest execution runtime.
- PRD gained a dedicated revision section (`18`) covering Trust Node generalization, step-update telemetry, checklist governance, and risk workflow.
- `infra/database/memory` relationship is now explicitly documented as infrastructure-layer siblings with different responsibilities.
- Verification: `cargo test --workspace` passed after documentation updates.
- Open ambiguity: `ui_designs/` currently has no files, so UI detail mapping in PRD remains at requirement-level only.

