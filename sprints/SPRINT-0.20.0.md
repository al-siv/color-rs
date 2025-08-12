# Sprint 0.20.0: FP Migration, Legacy Elimination, and Systematic Refactoring

<!-- MODE: LAISSEZ-FAIRE -->

Planning hierarchy: sprint → assignments → milestones → phases → checklist items (§6 GUIDELINES). Status markers strictly `[ ]`, `[x]`, `[-]` (§6.2). No emojis.

## Overview
This sprint establishes a governed, FP-first transformation to replace legacy OOP remnants, eliminate unused/legacy/deprecated code, and refactor the codebase while preserving and, where feasible, enhancing functionality.

## Version
- Target Version: 0.20.0
- Previous Version: 0.19.4

## Assignments
- [ ] A1: Systematically replace all legacy OOP remnants with FP-first designs while preserving behavior.
- [ ] A2: Remove all unused, legacy, dead, deprecated, or “kept for compatibility” code.
- [ ] A3: Refactor the codebase for cohesion, clarity, and FP compliance.

Assignment metrics trackers (updated continuously):
- Legacy/OOP remnant count (target 0): TBD baseline after audit.
- unwrap/expect/panic (non-test) outstanding: TBD (audit pending Milestone 2.0 Build phase task).
- Oversized functions (>60 LOC) newly introduced: 0 (must remain 0).
- Dead dependencies (cargo-udeps/machete): 0.

## Milestones

### Milestone 1.0: Analysis & Foundation (FP Migration Plan)
Status: [ ] (PENDING) Target: Establish comprehensive analysis and actionable FP migration plan.

Phases (Plan → Build → Verify → Polish → Merge):
- Plan:
  - [x] Repository-wide OOP/OOP-like pattern discovery (initial scan complete: dynamic dispatch sites in color_parser collections, callbacks in gradient_formatter, capability trait Clock used (keep), compat layer present (evaluate decommission))
  - [ ] Evaluate replacing Box<dyn ColorCollection> with enums/sealed trait where practical
  - [ ] Plan capability injection coverage for time usages
  - [x] Baseline clippy executed; captured actionable lints (manual-clamp src/color_ops/contrast.rs:212; too-many-arguments gradient calculators (3); uninlined-format-args src/image.rs 544, 548-551)
  - [ ] Detect dead/unused deps (cargo-udeps / machete)
  - [ ] Catalog compat layers for removal (compat.rs) (some removal later executed in Milestone 2.0 — ensure cross-reference)
- Build:
  - [ ] Draft FP migration blueprint (analysis/FP-Migration-Plan-0.20.0.md) expansion: ADT targets, capability injection map
- Verify:
  - [ ] QG baseline: cargo check, clippy -D warnings (clean) recorded
  - [ ] Legacy detection pipeline dry run (udeps/machete output stored)
- Polish:
  - [ ] Normalize checklist formatting to §6.2
  - [ ] Add decision log entries referencing blueprint
- Merge:
  - [ ] Blueprint finalized & linked; milestone closure note added

Quality Gates (Milestone scope):
- [ ] QG-001 Compilation baseline
- [ ] QG-002 FP compliance initial assessment
- [ ] QG-003 Legacy detection pipeline baseline
- [ ] QG-004 Documentation integrity baseline

Artifacts:
- analysis/FP-Migration-Plan-0.20.0.md
- Decision log entries (to add) summarizing initial OOP remnant catalog & planned eliminations

---

### Milestone 2.0: OOP Remnant Replacement – Core Modules
Status: [x] (IN PROGRESS) Target: Replace highest-impact OOP remnants with FP architecture.

Progress (2025-08-08):
  - parsing_chain: safe hex triplet expansion (no nth().unwrap())
  - image: handle pixmap.pixel Option with transparent fallback (no panics)
  - gradient/output: eliminate unwrap on fmt, propagate errors
  - command_execution/commands: avoid unwrap on writeln while assembling outputs
  - image.rs (SVG/PNG unified stops) now uses GradientCalculationConfig via GradientCalculator::calculate_unified_gradient_cfg
  - gradient/mod.rs (CLI path) now uses cfg-based unified calculation
  - tests: migrated gradient integration test to cfg-based API
  - unified_calculator.rs: added calculate_unified_gradient_cfg; preparing to drop legacy arg-heavy path
  - gradient/mod.rs: public re-export now exposes calculate_unified_gradient_cfg and GradientCalculationConfig; deprecated arg-heavy export removed
- Quality gates: clippy -D warnings clean; tests green across all suites.

Additional progress:
- Removed legacy arg-heavy unified gradient functions; `GradientCalculator` delegates to `calculate_unified_gradient_cfg`. Eliminated all `clippy::too_many_arguments` allows in gradient calculators; tests updated. Gates remain green.
- Safety: replaced float comparison unwraps with `total_cmp` in `color_parser/compat.rs`, `color_parser/collections.rs`, and `color_ops/distance.rs`. Made unified manager lazy init non-panicking with graceful fallback. All tests/lints remain green.
- Safety: removed unwrap in `UniversalColor::luminance()` (collections) and eliminated capture-group unwraps in `ral_matcher.rs` using `?` for graceful early return. Gates remain green.
- More safety: removed regex initialization unwraps by replacing `regex` usage in `ral_matcher` with manual parsing; switched sort comparisons in RAL collections to `total_cmp`; added empty() fallbacks for collections and made `UnifiedColorManager::default()` non-panicking. Clippy -D warnings clean; tests still green (231 passed, 2 ignored).
- Dependencies: removed unused `regex` and `proptest`; re-ran lint/tests—all green.
- Compatibility decommission: removed top-level `compat` module and `color_parser::compat`; rewired `ral_matcher` to use `UnifiedColorManager` directly with safe fallback and local conversion helpers; updated public API (no compat re-export). Lint/tests verified green.

Phases (Plan → Build → Verify → Polish → Merge):
- Plan:
  - [x] Identify high-impact OOP remnants & unwrap hotspots
  - [x] Define config-struct API strategy for unified gradient calculation
- Build:
  - [x] Add config-struct API for unified gradient calculation (callers migrated; legacy arg-heavy APIs removed)
  - [ ] Extract pure computational functions from mixed modules
  - [ ] Replace inheritance/strategy usage with enums + higher-order functions where applicable
  - [ ] Introduce iterator/stream pipelines for data transforms where imperative loops remain
  - [x] Introduce capability trait Clock for hue analysis; remove direct SystemTime
  - [ ] Logger/IO capability design (skeleton traits) (pending)
  - [ ] Add smart constructors/newtypes for key invariants
- Verify:
  - [x] Clippy -D warnings green after config API migration
  - [x] Tests green after gradient & manager refactors
  - [ ] Unwrap/expect/panic count trending down (tracker update needed)
- Polish:
  - [ ] Document ADT decisions and alternatives (minority reports)
  - [ ] Ensure exhaustive pattern matching (remove wildcard catch-alls)
  - [ ] Inline docs updated for new config APIs & capability traits
- Merge:
  - [ ] Milestone closure once legacy OOP remnants replaced & unwrap tracker at 0 (non-test)

Quality Gates:
- [x] QG-001 Compilation
- [x] QG-002 FP compliance (no deprecated public exports; config-struct APIs)
- [x] QG-003 Legacy code zero-introduction (no new OOP remnants)
- [x] Tests for migrated modules (updated; all green)

Remaining Work (Milestone 2.0):
- [ ] Audit and remove remaining unwrap/expect/panic in non-test paths (convert to Result/Option or safer comparisons)
- [ ] Design Logger/IO capabilities (ports) for future extraction (no direct effects in core)
- [ ] Begin dead-code sweep preparation (Milestone 3.0 Plan tasks)

Next Steps (near-term):
- [ ] Run cargo-udeps and cargo-machete; produce removal list and plan
- [ ] Update unwrap tracker counts after audit

Artifacts:
- Refactoring notes per module under analysis/

Run log (2025-08-08):
- Dead code sweep: cargo-machete → No unused dependencies reported.
- Unused deps sweep: ran with +nightly; flagged `regex` and dev `proptest` as unused → removed. Current sweep clean.
- Compat removal & RAL rewiring: edited `lib.rs`, removed `src/compat.rs`, `src/color_parser/compat.rs`, updated `src/color_parser/{mod.rs,ral_matcher.rs}`; clippy -D warnings PASS; cargo test --all PASS (227 passed; 2 ignored in latest run).

---

### Milestone 3.0: Legacy/Dead/Deprecated Code Elimination
Status: [ ] (PENDING) Target: Zero-tolerance legacy elimination with safe rollback.

Phases:
- Phase 3.1: Automated Detection & Review
  - [ ] Run cargo-udeps and cargo-machete; triage results
  - [ ] Identify unused modules, fns, types; assess cohesion
  - [ ] Prepare elimination plan with verification checkpoints

- Phase 3.2: Safe Removal & Cohesion Restoration
  - [ ] Remove confirmed dead/unused code paths
  - [ ] Restore connectivity where code is miswired but needed
  - [ ] Update public APIs or deprecate with migration notes (if needed)

- Phase 3.3: Compatibility Layer Decommission
  - [ ] Remove compatibility shims per plan
  - [ ] Provide migration guidance in CHANGELOG/EXAMPLES
  - [ ] Verify no regressions in CLI/Examples/Docs

Quality Gates:
- [ ] Zero new warnings; clippy clean
- [ ] All builds/tests pass
- [ ] Documentation updated; links valid
- [ ] Legacy count = 0

---

### Milestone 4.0: Systematic Refactoring & Modularization
Status: [ ] (PENDING) Target: Cohesion improvement, size thresholds, clear boundaries.

Phases:
- Phase 4.1: Function and Module Size Management
  - [ ] Reduce oversized functions (>50 lines) via extraction
  - [ ] Split large modules (>400 lines) along functional boundaries
  - [ ] Minimize cross-module dependencies; stabilize interfaces

- Phase 4.2: Composition & Pipelines
  - [ ] Replace imperative loops with iterator chains
  - [ ] Apply railway-oriented error composition
  - [ ] Ensure pipeline signatures are composition-friendly

- Phase 4.3: Documentation & Inline Docs
  - [ ] Update module-level docs, public APIs, invariants
  - [ ] Cross-reference methodology and patterns used
  - [ ] Add examples and property-based tests where valuable

Quality Gates:
- [ ] clippy pedantic nursery targets resolved or justified
- [ ] Property-based tests for pure cores
- [ ] Performance parity or improvement

---

### Milestone 5.0: QA, Documentation, and Finalization
Status: [ ] (PENDING) Target: Quality gates, reporting, and readiness.

Phases:
- Phase 5.1: Quality Gates & CI
  - [ ] cargo fmt, clippy -D warnings, test --all-features
  - [ ] Verify documentation integrity and truthful reporting
  - [ ] Ensure zero legacy code and FP compliance = 100%

- Phase 5.2: Examples, CLI, and Docs Sync
  - [ ] Validate examples, CLI reference, API docs
  - [ ] Update CHANGELOG with migration/refactor notes
  - [ ] Prepare milestone/phase completion reports

- Phase 5.3: Release Prep (if applicable)
  - [ ] Tag milestone branches after verification
  - [ ] Ensure reproducible builds; capture performance snapshot

Quality Gates:
- [ ] Compilation/test/linting all green
- [ ] Docs complete; links valid
- [ ] Reports comply with templates

## Definition of Done (Sprint / PR Alignment)
- Branch naming: `sprint-0.20.0-m2-YYYYMMDD-*` (example pattern) matching §5 guidelines.
- Gates: cargo fmt, clippy -D warnings, cargo test all green; size: 0 new oversized functions (>60 LOC) or modules (>600 LOC) unless justified.
- Legacy/OOP remnant count = 0 at sprint completion (tracked each milestone).
- unwrap/expect/panic count (non-test) = 0 prior to final merge.
- Updated sprint file reflects scope/status before merging related PRs.
- Decision log entries for major architectural refactors added (analysis/ folder) where changes affect public API or capability boundaries.

## Quality Gate Mapping
| Gate | Description | Verification Command / Evidence |
|------|-------------|---------------------------------|
| QG-001 | Compilation | `cargo check` / implicit via tests build |
| QG-002 | FP Compliance | Absence of unwrap/expect/panic (non-test), capability traits at effect boundaries, clippy -D warnings clean |
| QG-003 | Legacy Detection | cargo-udeps, cargo-machete reports clean |
| QG-004 | Documentation Integrity | Links valid; sprint & docs updated; no emojis; version alignment |
| QG-005 | Size Constraints | Function <60 LOC, Module <600 LOC (manual scan / future automated) |
| QG-006 | Test Coverage (baseline) | All existing tests pass; new pure logic has unit tests |

## FP Checklist Reference
Refer to GUIDELINES §10 (FP-first checklist). Not duplicated here to avoid divergence.

## MCP Tooling Plan (Governed Usage)

- Sequential Thinking (`sequential-thinking`):
  - Complex decomposition for FP extraction, effect isolation, domain modeling
  - Use for milestone planning and risk analysis; store thought artifacts in analysis/

- Context7 (`Context7`):
  - Retrieve library docs (docs.rs, upstream crates) for FP-friendly APIs
  - Verify patterns against official guides; cite versions

- Memory (`memory`):
  - Persist FP decisions, pattern usage, and architecture maps (entities/relations)
  - Track refactoring experiences and outcomes for future reuse

- Time (`time`):
  - Timestamp reports and cadence; coordinate timezones if needed
  - Immutable timestamps for audit trails

- Filesystem (`filesystem`):
  - Directory tree scans for module sizing and structural analysis
  - Bulk file ops when reorganizing modules (with checkpoints)

- Fetch (`fetch`):
  - Pull current references (Rust book, RFCs) to inform decisions
  - Supplement Context7 with external authoritative docs

- Deepwiki (`deepwiki`):
  - Ask targeted questions for methodology and pattern clarification
  - Review related repository practices and design notes

## Success Criteria
- Behavior preserved/improved with FP-first architecture
- Legacy/OOP remnants replaced; legacy count = 0
- clippy clean with -D warnings; zero unwrap/expect in core
- Comprehensive tests; property-based where applicable
- Documentation and examples updated and consistent

## Risk Assessment
- Medium: Broad refactoring scope; mitigated by milestone sequencing and quality gates
- Medium: Potential compatibility impact; mitigated with migration notes and tests
- Low: Tooling friction; mitigated by MCP integration and stepwise changes
- Emerging: Potential regression from unwrap removals; mitigation: incremental tests + snapshot of failing scenario reproduction steps before change

## Sprint Duration
- Estimated: 1–2 weeks (iterative, milestone-based)
- Dependencies: Existing modules and docs; MCP servers available as configured

## Quality Gates
- QG-001 Compilation, QG-002 FP Compliance, QG-003 Legacy Detection, QG-004 Documentation Integrity
- Milestone-specific gates as listed per milestone

Status: [ ] IN PROGRESS (continuous autonomous progression per GUIDELINES §13 Laissez-faire)
