# Development Rules

## 1) First Message
This file is for **how to build and evolve** this project.
If the user did not give you a concrete task in their first message,
read README.md, then ask which module(s) to work on. 
Based on the answer, read the relevant README.md files in parallel.
- apps/desktop/README.md
- crates/infra/README.md
- crates/red/README.md
- crates/ui/README.md
- crates/database/README.md
- crates/memory/README.md
- crates/logic/README.md

## 2) Workflow Orchestration
- **Plan node default**: Enter plan mode for any non-trivial task (three or more steps, architectural decisions, or verification that might cross files) and write a detailed specification before coding. Record that plan as checkable items inside `tasks/todo.md`, include verification steps, and stop to re-plan the moment something goes sideways instead of pushing ahead.
- **Write specs upfront**: Treat plan mode as the place to sketch the intent, scope, and acceptance criteria so there is no ambiguity when implementation starts. Bring the plan back into view before touching code.
- **Plan verification steps**: Use plan mode for verification and testing nodes, not just development. Verify that the behavior matches the plan, document the diff relative to the existing surface, and keep notes in the plan.
- **Subagent strategy**: Keep the main context clean—offload research, exploration, or parallel analysis into focused subagents. One tack per subagent and throw compute at complex problems by spinning up as many focused agents as needed.
- **Self-improvement loop**: After any correction or new constraint arrives from the user, immediately update `tasks/lessons.md` with the root cause, the fix, and a preventative rule for future work. Review lessons at session start when they are relevant to the current project.
- **Verification before done**: Never mark a task complete without proving it works. Run the needed tests or capture logs, verify the diff between previous and new behavior when relevant, and mentally ask, “Would a staff engineer approve this?” before signing off.
- **Demand elegance (balanced)**: Pause on non-trivial changes and ask if a more elegant solution exists. If a fix feels hacky, implement the elegant alternative knowing everything you know now. Skip the over-engineering audit when the change is simple and obvious.
- **Autonomous bug fixing**: Treat bug reports as instructions to fix without asking the user how. Point at logs, failing tests, or errors, resolve them, and keep the user from context-switching into debugging.

## 3) Task Management
1. **Plan first**: Record every plan in `tasks/todo.md` before touching implementation. Use checkboxes so progress is visible and the plan can be referenced later.
2. **Verify plan**: Before implementation begins, ensure the plan is correct and accepted. Revisit it whenever requirements shift.
3. **Track progress**: Mark plan items complete as you go so the plan reflects the current state.
4. **Explain changes**: Include a high-level summary of what changed and why at each major step, either within the plan or at the end of the session.
5. **Document results**: Populate the review section of `tasks/todo.md` with the summary of outcomes, verification steps, and any outstanding questions.
6. **Capture lessons**: After corrections, add a new entry to `tasks/lessons.md` describing the lesson, the fix, and the rule that prevents repetition.

## 4) Code Quality & Quality Gate
- **Simplicity and explicit intent**: Prefer readable, maintainable code over cleverness.
- **Small, reviewable diffs**: Make focused changes that are easy to reason about in code review and interactive iteration.
- **Pattern alignment**: Match existing repository patterns before introducing new abstractions.
- **Preserve intentional behavior**: Do not remove functionality without explicit instruction; ask if unclear.
- **Static structure**: Avoid dynamic imports or conditional includes; keep module structure statically analyzable.
- **Dependency integrity**: Avoid unnecessary crates; never downgrade or remove code to mask type errors from outdated dependencies—upgrade instead if needed.
- **Documentation hygiene**: Keep docs under 300 lines; split if needed. Keep directory fan-out controlled; avoid more than 20 files in one directory.
- **Completion checklist**: Before marking work complete, verify requirements are addressed, code style expectations are satisfied (formatted, clippy clean), relevant validation ran (or gaps documented), documentation updates land in `docs/` (except this root `AGENTS.md`), and the handoff note includes risks plus recommended follow-up.

## 5) Definition of Done
A task is done only when:
- Implementation is complete and coherent.
- Validation is complete, or missing checks are transparently declared.
- Changes are minimal, scoped, and reviewable.
- Required documentation updates are completed in the right place.

## 6) Engineering Principles
- **Simplicity first**: Make every change as simple as possible without sacrificing clarity.
- **No laziness**: Find the root cause; avoid temporary fixes.
- **Minimal impact**: Touch only what is necessary and prevent ripple effects.

## 7) Rust + GPUI Development Rules
- Follow Rust best practices and strict typing.
- Align structure and naming with `gpui-component` style patterns.
- Keep component responsibilities focused and composable.
- Keep state boundaries explicit and predictable.
- Include explicit error handling on critical paths.
- **Keybinding rule**: All keybindings must be configurable. Never hardcode (e.g., avoid `matches_key(key, "ctrl+x")`). Define defaults in `DEFAULT_KEYBINDINGS` or equivalent.

## 8) Default Task Workflow
1. **Clarify** goal, constraints, and acceptance criteria.
2. **Propose** a minimal implementation plan.
3. **Implement** in small, reviewable increments.
4. **Validate** changed surface (format, clippy, tests as appropriate).
5. **Handoff** with concise summary, risks, and next steps.

## 9) Testing Rules
- Validate the changed surface first, then critical adjacent paths.
- Test placement policy:
  - Module-level tests live in `crates/*/tests`.
  - Cross-module end-to-end tests live in `tests/e2e` only.
- Prioritize logic, state transitions, and error-path coverage.
- Do not bypass failures by weakening assertions or skipping checks.
- If tests are not run, explicitly state what is unverified and why.
- **Testing TUI with tmux**: For interactive UI testing, use scripted tmux sessions:
  ```bash
  tmux new-session -d -s test -x 80 -y 24
  tmux send-keys -t test "cargo run" Enter
  sleep 3 && tmux capture-pane -t test -p
  tmux kill-session -t test
  ```

## 10) Git Safety Rules (Mandatory)
- Always inspect `git status` before staging and before commit.
- **Stage explicitly**: `git add <file1> <file2>` only. Never use `git add .` or `git add -A`.
- **Destructive commands forbidden**:
  - `git reset --hard`
  - `git checkout .`
  - `git clean -fd`
  - `git stash` (may stash others' work in parallel agent scenarios)
- **Never commit unless user explicitly asks**.
- Commit messages must explain **what changed and why**.
- **Parallel work isolation**: when multiple agents work simultaneously:
  - Only stage files you modified in your session.
  - Before commit, verify `git status` shows only your changes.
  - If rebase conflicts occur in files you didn't touch, abort and ask.
  - Never force push.

## 11) GitHub Workflow
### Issues
- **Reading**: Get complete context in one call:
  ```bash
  gh issue view <number> --json title,body,comments,labels,state
  ```
- **Creating**: Add component labels (e.g., `comp:ui`, `comp:core`). Add all relevant labels for cross-component issues.
- **Closing via commit**: Include `fixes #<number>` or `closes #<number>` in commit message.

### Pull Requests
- **Analysis first**: analyze PRs without pulling locally when possible.
- **Approval workflow** (user approves):
  1. Create feature branch
  2. Pull PR changes
  3. Rebase on main
  4. Apply adjustments if needed
  5. Commit and merge to main
  6. Push and close PR
- **No self-PRs**: work in feature branches until requirements are met, then merge.

## 12) Changelog Convention
- **Location**: `CHANGELOG.md` or `docs/CHANGELOG.md`.
- **Format**: Under `## [Unreleased]`, use these subsections:
  - `### Breaking Changes` - API changes requiring migration
  - `### Added` - New features
  - `### Changed` - Changes to existing functionality
  - `### Fixed` - Bug fixes
  - `### Removed` - Removed features
- **Rules**:
  - Read existing `[Unreleased]` section before adding entries.
  - Append to existing subsections; never create duplicates.
  - Never modify already-released version sections.
  - Reference format: `Fixed foo bar ([#123](https://github.com/user/repo/issues/123))`.

## 13) Communication Style
- Keep answers short and concise.
- No emojis in commits, issues, PR comments, or code.
- No fluff or cheerful filler text.
- Technical prose only; be kind but direct (e.g., "Thanks @user" not "Thanks so much @user!").

## 14) Security and Compliance
- Work only within explicitly authorized security-testing scope.
- Do not provide or execute instructions targeting unauthorized systems.
- Minimize sensitive data exposure in code, logs, and artifacts.
- Keep development actions auditable where practical.
