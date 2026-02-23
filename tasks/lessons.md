# Lessons Learned
Record actionable lessons whenever a correction occurs: describe the mistake, the fix, and the rule change to prevent reruns.


## 2026-02-24 — Module semantics must follow product mental model
- Root cause: I defined module names from generic clean-architecture wording, which made `red/logic/infra` ambiguous against your actual product intent.
- Fix: Rebase module responsibilities on PRD semantics (`red` as autonomous pentest agent runtime), then update README/PRD to map each module to UAV pentest lifecycle responsibilities.
- Preventive rule: For naming/architecture decisions, first align with user's product vocabulary and historical intent, then apply architecture patterns.

