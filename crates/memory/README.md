# memory

Cognitive memory system for agent and mission reasoning.

## Position in the system
`memory` handles how knowledge is retained and recalled across steps/sessions.

## What problem it solves
Improve agent quality over time without hardcoding all rules into prompts.

## Planned responsibilities
- Four-layer memory model: episodic, semantic, procedural, contextual.
- Memory indexing/retrieval APIs for planner and agents.
- Memory write policies (what to keep, summarize, or discard).
- Relevance scoring and retrieval constraints by scope/policy.

## Relationship with database
- `memory` defines memory behavior and retrieval logic.
- `database` provides durable storage backends used by memory.
- `memory` is not a replacement for database; it is a reasoning-oriented layer on top.

## Testing
- `crates/memory/tests/` for retrieval quality rules, write policies, and determinism checks.
