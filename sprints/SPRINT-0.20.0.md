# Sprint 0.20.0: FP Migration, Legacy Elimination, and Systematic Refactoring

## Overview
This sprint establishes a governed, FP-first transformation to replace legacy OOP remnants, eliminate unused/legacy/deprecated code, and refactor the codebase while preserving and, where feasible, enhancing functionality.

## Version
- Target Version: 0.20.0
- Previous Version: 0.19.4

## Assignments
1. Systematically replace all legacy OOP remnants with FP-first designs while preserving behavior.
2. Remove all unused, legacy, dead, deprecated, or “kept for compatibility” code.
3. Refactor the codebase for cohesion, clarity, and FP compliance.

## Milestones

### Milestone 1.0: Analysis & Foundation (FP Migration Plan)
Status: [ ] PENDING | Target: Establish comprehensive analysis and actionable FP migration plan.

Phases:
- Phase 1.1: Repository-wide OOP/OOP-like pattern discovery
  - [x] Initial scan complete: dynamic dispatch sites in color_parser collections, callbacks in gradient_formatter, capability trait Clock used (keep), compat layer present (evaluate decommission)
  - [ ] Evaluate replacing Box<dyn ColorCollection> with enums/sealed trait where practical
  - [ ] Plan capability injection coverage for time usages

- Phase 1.2: Legacy/Dead Code Detection & Catalog
  - [x] Baseline clippy executed; key actionables captured
    - manual-clamp: src/color_ops/contrast.rs:212
    - too-many-arguments: gradient calculator functions (3 occurrences)
    - uninlined-format-args: src/image.rs lines 544, 548-551
  - [ ] Detect dead/unused deps (udeps/machete)
  - [ ] Catalog compat layers for removal (compat.rs)

- Phase 1.3: FP Refactoring Blueprint
  - [ ] Replace panic/unwrap in business logic with Result/Option and ?
  - [ ] Inject Clock where SystemTime/Instant used outside tests/bench
  - [ ] Plan ADT upgrades for command/parse results

Quality Gates:
- [ ] QG-001 Compilation check (cargo check)
- [ ] QG-002 FP compliance initial assessment
- [ ] QG-003 Legacy detection pipeline baseline
- [ ] QG-004 Documentation integrity baseline

Artifacts:
- analysis/FP-Migration-Plan-0.20.0.md (created)
- Updated sprint checklist with discovered tasks (this section)

---

### Milestone 2.0: OOP Remnant Replacement – Core Modules
Status: [ ] PENDING | Target: Replace highest-impact OOP remnants with FP architecture.

Phases:
- Phase 2.1: Pure Core extraction
  - [ ] Extract pure computational functions from mixed modules
  - [ ] Replace inheritance/strategy with enums + HOFs where applicable
  - [ ] Introduce iterator/stream pipelines for data transforms

- Phase 2.2: Effect Isolation
  - [ ] Introduce capability traits (Clock, Logger, IO ports) at boundaries
  - [ ] Remove hidden time/random/env access in business logic
  - [ ] Standardize Result/Option usage; eliminate unwrap/expect in core

- Phase 2.3: Domain Modeling Upgrades
  - [ ] Add smart constructors and newtypes for key invariants
  - [ ] Ensure exhaustive pattern matching without wildcard catch-alls
  - [ ] Document ADT decisions and alternatives (minority reports)

Quality Gates:
- [ ] QG-001 Compilation
- [ ] QG-002 FP compliance (100%)
- [ ] QG-003 Legacy code zero-introduction
- [ ] Tests for migrated modules (property + example)

Artifacts:
- Refactoring notes per module under analysis/

---

### Milestone 3.0: Legacy/Dead/Deprecated Code Elimination
Status: [ ] PENDING | Target: Zero-tolerance legacy elimination with safe rollback.

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
Status: [ ] PENDING | Target: Cohesion improvement, size thresholds, clear boundaries.

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
Status: [ ] PENDING | Target: Quality gates, reporting, and readiness.

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

## Sprint Duration
- Estimated: 1–2 weeks (iterative, milestone-based)
- Dependencies: Existing modules and docs; MCP servers available as configured

## Quality Gates
- QG-001 Compilation, QG-002 FP Compliance, QG-003 Legacy Detection, QG-004 Documentation Integrity
- Milestone-specific gates as listed per milestone

Status: [ ] IN PROGRESS
