# Dead Code Sweep Plan (Phase 3.1)

Status: Draft (2025-08-12)
Target Milestone: 3.0 (Legacy/Dead/Deprecated Code Elimination)

## Objectives
Establish a repeatable, evidence-based process to identify and safely remove dead / unused / deprecated code before Milestone 3.2 removal actions.

## Detection Tools & Commands
1. Dependency level:
   - `cargo +nightly udeps --all-targets` (unused dependencies)
   - `cargo machete` (unused code paths)
2. Symbol/code path discovery:
   - `cargo rustdoc --document-private-items` (scan for orphan private items)
   - Grep scans for deprecated markers: `grep -R "deprecated" src/`
3. Test coverage heuristic (optional stretch): integrate `cargo tarpaulin` (deferred unless gaps suspected).

## Classification Criteria
Each candidate path gets labeled:
- Unused Dependency (UDEP)
- Unreferenced Function / Method (FUNC)
- Unreferenced Type / Enum Variant (TYPE)
- Duplicate Logic (DUP) – identical or near-identical implementation present elsewhere
- Deprecated (DEPR) – explicit annotation with removal target version ≤ 0.21.0
- Compatibility Shim (COMPAT) – transitional, scheduled removal

## Acceptance / Action Matrix
| Classification | Action | Preconditions |
|----------------|--------|---------------|
| UDEP | Remove from Cargo.toml | Tool confirms unused; not a transitive future dependency |
| FUNC/TYPE | Remove | No external/public API contract (or has deprecation window elapsed) |
| DUP | Consolidate | Replace call sites with canonical implementation; add test if logic subtle |
| DEPR | Remove | Current version ≥ removal target; CHANGELOG entry present |
| COMPAT | Remove / Defer | If downstream impact minimal, remove; else schedule final window |

## Safety Checks
- All removals gated by: clippy -D warnings green + tests green
- Public API change: add CHANGELOG note + migration guidance snippet
- Batch size: keep each removal PR ≤ ~300 LOC diff to ease review

## Workflow Steps
1. Run detection tools; capture raw outputs under `analysis/dead_code_reports/` (create folder).
2. Triage items into classification table in this doc (append section).
3. For each removal batch: create checklist in sprint file under Milestone 3.2.
4. Execute removal → run lint/tests → update sprint & CHANGELOG.
5. Final verification pass: re-run tools must return clean state.

## Metrics
- Unused deps count (target 0)
- Dead functions/types removed
- Deprecated shims removed (compat.rs by 0.21.0)

## Open Questions
- Introduce automated size report integration for code path sizing? (Deferred)
- Add tarpaulin-based risk heuristic? (Optional if time)

## References
- Sprint 0.20.0 Milestone 3.0 phases
- Governance GUIDELINES §8, §11 (dead code elimination, deny-list)

---

## Phase 3.1 Execution Log (2025-08-12)

Detection tool runs initiated. Initial attempt to capture `cargo udeps` and `cargo machete` outputs encountered redirection issue (empty directory listing). Re-ran commands; `cargo udeps` output still not captured (placeholder file present) indicating local environment/tool invocation anomaly to revisit. `cargo machete` run executed (output file expected, verify in subsequent pass; current listing issue suggests shell path nuance).

Action items:
- [ ] Re-verify tool availability (`cargo +nightly udeps --version`, `cargo machete --help`) before next run.
- [ ] If udeps capture continues failing, introduce fallback: `cargo metadata` + manual diff for dependency usage or upgrade cargo-udeps.
- [ ] Ensure Windows Git Bash redirection semantics not interfering (may switch to PowerShell wrapper for capture if persists).

## Preliminary Classification Table (Empty Baseline)

| Item | Path / Crate | Classification | Notes | Planned Action |
|------|--------------|----------------|-------|----------------|
| 1 | (workspace) | UDEP | `cargo machete`: no unused deps (clean). | Maintain |
| 2 | src/compat.rs | COMPAT | Deprecated shim; removal target 0.21.0 | Remove in Phase 3.3 or earlier |
| 3 | (workspace) | UDEP | `cargo udeps` capture incomplete (file lock wait) – rerun needed | Re-run with lock resolution |
| 4 | src/color_formatter.rs::ColorFormatter::format_comprehensive_report | DEPR | No-op deprecated placeholder; safe to remove (scheduled Phase 3.2 Batch 2) | Remove |
| 5 | src/color_distance_strategies/mod.rs legacy distance helpers | DEPR | Removed Batch 1 (2025-08-12) | Completed |

## Compat Layer Status

`src/compat.rs` present, fully deprecated (removal target 0.21.0). Multiple in-module legacy re-exports and compatibility trait/alias definitions remain. No new external references introduced (grep scan captured only internal doc/test lines and expected transitional comments). Scheduled for Phase 3.3 removal unless earlier elimination deemed low-risk.

Risk: Minimal (module isolated; no critical runtime logic). Benefit: Reduces surface complexity and maintenance overhead.

Next Sweep Iteration (Planned):
1. Fix output capture.
2. Add machete findings (if any) to classification table.
3. Begin manual scan for obvious dead helper functions (search patterns: `pub fn` unused, legacy comments).

---

## Phase 3.2 Updates (2025-08-13)

Completed Batches:
- Batch 1 (2025-08-12): Removed legacy distance helpers `calculate_delta_e_76_legacy`, `calculate_delta_e_2000_legacy`, `calculate_euclidean_distance_legacy`, `parse_algorithm_legacy`.
- Batch 2 (2025-08-13): Removed deprecated no-op formatter `format_comprehensive_report`; refactored call sites to concise summary output.

Audit Results (2025-08-13):
- Dependency audits: `cargo +nightly udeps --all-targets` (clean) & `cargo machete` (clean).
- Compat module: zero external references; safe for early removal.
- Production unwrap/expect/panic: 0 (all grep hits inside test modules / #[cfg(test)] blocks).

Updated Classification Table:
| Artifact | Kind | Status | Batch | Notes |
|----------|------|--------|-------|-------|
| calculate_delta_e_76_legacy | Func | Removed | 1 | Legacy distance helper |
| calculate_delta_e_2000_legacy | Func | Removed | 1 | Legacy distance helper |
| calculate_euclidean_distance_legacy | Func | Removed | 1 | Legacy distance helper |
| parse_algorithm_legacy | Func | Removed | 1 | Legacy parse helper |
| format_comprehensive_report | Func | Removed | 2 | No-op formatter |
| src/compat.rs | Module | Pending | 3 | Deprecated; no references |

## Batch 3 Plan (Early Compat Decommission)
Scope: Remove `src/compat.rs` (entire module) ahead of original 0.21.0 schedule.
Rationale: Zero usages; simplifies surface; reduces maintenance overhead.
Steps:
1. Delete `src/compat.rs`.
2. Remove any re-exports from `lib.rs` (if present) referencing compat.
3. Run clippy/tests; ensure green.
4. `grep -R "compat::" src/` must return 0 matches (already baseline; re-run post-removal).
5. Update CHANGELOG (Removed + Migration notes) and sprint (Milestone 3.0 Phase 3.2 progress, Phase 3.3 decommission advanced to completed for compat).
6. Update this plan: mark module removed, legacy count = 0.

Verification Checklist (Batch 3):
- [ ] Clippy -D warnings clean after removal.
- [ ] Test suite passes.
- [ ] No references to removed module.
- [ ] CHANGELOG updated with migration guidance (use direct parsing & execute_command APIs).
- [ ] Sprint file updated (Phase 3.2 progress + Phase 3.3 compat removal).

Rollback: Restore file from git history if regression discovered; add focused test reproducing issue; reattempt removal after fix.

Metrics Target Post-Batch 3:
- Legacy artifacts remaining: 0
- Unused deps: 0

---

## Forward Look
Potential automation (deferred): script to enumerate pub fns with single in-crate usage for candidate consolidation.

