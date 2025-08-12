# PR Plan: Sprint 0.20.0 Milestone 2.0 Alignment (2025-08-12)

Scope: Consolidate unwrap removals, functional core extraction, iterator pipelines, smart constructors, and planning governance updates accomplished on `milestone-2.1-unwrap-removal-20250808` into new compliant branch `sprint-0.20.0-m2-20250812`.

## Steps
1. Ensure `main` is up to date locally: fetch + rebase (no history rewrite).
2. Cherry-pick or merge prior branch commits if divergence arises (currently branch created at HEAD of old branch so no action required).
3. Run verification: cargo fmt, cargo clippy -D warnings, cargo test --all.
4. Generate size report (TODO script placeholder) and verification JSON.
5. Update sprint file statuses before opening PR (done for ADR and governance note).
6. Open PR with title: `Sprint 0.20.0 M2: FP core & invariants hardening`.
7. Populate PR template sections (summary, scope, changes, verification, risks, follow-ups, Autonomous Decisions).
8. After review & green gates, merge via squash (retain semantic conventional commit prefixes) or merge commit (no rebase onto main post-approval unless conflicts).
9. Tag if release-worthy (not yet; later milestones will handle tag).
10. Delete intermediate branch `milestone-2.1-unwrap-removal-20250808` after merge.

## Risks
- Stale clippy phantom warnings (mitigate by single clean run on fresh clone if persists).

## Follow-ups
- Implement first enum/HOF migration (GradientFormatStrategy) on a dated branch: `sprint-0.20.0-m2-20250812-format-enum`.
- Add CI script for Box<dyn> strategy detection.
