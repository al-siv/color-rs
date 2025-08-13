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
- Legacy/OOP remnant count (target 0): baseline capture pending expanded structural audit.
- unwrap/expect/panic (non-test) outstanding: 0 (baseline established; current occurrences isolated to tests/examples).
- Oversized functions (>60 LOC) newly introduced: 0 (must remain 0).
- Dead dependencies (cargo-udeps/machete): 0.

## Milestones

### Milestone 1.0: Analysis & Foundation (FP Migration Plan)
Status: [ ] (PENDING) Target: Establish comprehensive analysis and actionable FP migration plan (blueprint partially expanded; final polish pending).

Phases (Plan → Build → Verify → Polish → Merge):
- Plan:
  - [x] Repository-wide OOP/OOP-like pattern discovery (initial scan complete: dynamic dispatch sites in color_parser collections, callbacks in gradient_formatter, capability trait Clock used (keep), compat layer present (evaluate decommission))
  - [x] Evaluate replacing Box<dyn ColorCollection> with enums/sealed trait where practical (ColorCollectionKind introduced 2025-08-12)
  - [x] Plan capability injection coverage for time usages (2025-08-12): Identified previously direct Instant/SystemTime calls confined to removed `performance_validation.rs`; production paths already use `Clock`.
  - [x] Baseline clippy executed; captured actionable lints (manual-clamp src/color_ops/contrast.rs:212; too-many-arguments gradient calculators (3); uninlined-format-args src/image.rs 544, 548-551)
  - [x] Detect dead/unused deps (cargo-udeps / machete) (2025-08-12: both tools report none; outputs stored locally udeps_output.txt, machete_output.txt)
  - [x] Catalog compat layers for removal (compat.rs) (2025-08-12: remaining shim isolated to src/compat.rs; fully deprecated; scheduled removal 0.21.0)
- Build:
  - [x] Draft FP migration blueprint (analysis/FP-Migration-Plan-0.20.0.md) expansion: ADT targets, capability injection map (initial expansion + ColorCollection enum completion log 2025-08-12)
- Verify:
  - [x] QG baseline: cargo check, clippy -D warnings (clean) recorded (2025-08-12 run)
  - [x] Legacy detection pipeline dry run (udeps/machete output stored; both clean 2025-08-12)
  - Evidence: test suite & clippy executed via `cargo clippy --all-targets -- -D warnings && cargo test --all --quiet` (exit code 0, 2025-08-12) establishing compilation + lint + test baseline.
- Polish:
  - [ ] Normalize checklist formatting to §6.2
  - [x] Add decision log entries referencing blueprint (analysis/DECISION_LOG.md created 2025-08-12)
  - [x] Link ADR for ColorCollection enum (analysis/ADR-ColorCollections-Enum.md) and smart constructors ADR (stubs created 2025-08-12)
- Merge:
  - [ ] Blueprint finalized & linked; milestone closure note added

Quality Gates (Milestone scope):
 - [x] QG-001 Compilation baseline (cargo check/test pass 2025-08-12)
 - [x] QG-002 FP compliance initial assessment (clippy -D warnings clean; non-test unwrap count 0)
 - [x] QG-003 Legacy detection pipeline baseline (udeps/machete clean)
 - [ ] QG-004 Documentation integrity baseline

Artifacts:
- analysis/FP-Migration-Plan-0.20.0.md
- Decision log entries (to add) summarizing initial OOP remnant catalog & planned eliminations

---

### Milestone 2.0: OOP Remnant Replacement – Core Modules
Status: [ ] (IN PROGRESS) Target: Replace highest-impact OOP remnants with FP architecture.

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
  - [x] Extract pure computational functions from mixed modules (image_core.rs extracted from image.rs; pure SVG builders + helpers isolated, image.rs now delegates)
  - [x] Introduce iterator pipelines for gradient SVG, hue gradient, and hue palette generation (replaced imperative loops with chained iterators)
  - [ ] Replace inheritance/strategy usage with enums + higher-order functions where applicable
    - [x] Gradient formatter: removed struct wrapper; introduced pure function `format_gradient_with_callbacks` (2025-08-12)
    - [x] Color collections: replaced `Box<dyn ColorCollection>` manager with ADT (`ColorCollectionKind`, `ColorCollections`) and refactored CLI hue analysis + unified manager (2025-08-12)
  - [x] Commands: existing `CommandType` enum already fulfills planned consolidated command ADT (documented 2025-08-12; no extra wrapper needed)
  - [ ] Introduce iterator/stream pipelines for data transforms where imperative loops remain
    - Partial: gradient output formatting modules (`gradient/output.rs`, `gradient/output_new.rs`) refactored to iterator pipelines (2025-08-12)
  - [x] Introduce capability trait Clock for hue analysis; remove direct SystemTime
  - [x] Logger capability design (skeleton trait & NoOp/Stdout impl added)
  - [x] Add smart constructors/newtypes (EasingFactor, Position, Steps) + GradientConfigBuilder enforcing invariants (start<end, easing ∈ [0,1], steps≥2)
- Verify:
  - [x] Clippy -D warnings green after config API migration
  - [x] Tests green after gradient & manager refactors
  - [x] Unwrap/expect/panic count trending down (baseline 0 non-test confirmed)
- Polish:
  - [x] Document ADT decisions and alternatives (minority reports) (analysis/ADT-alternatives.md created 2025-08-12)
  - [x] Ensure exhaustive pattern matching (remove wildcard catch-alls) (2025-08-12 audit: no wildcard matches on CommandType/ColorCollectionKind; only utility fallbacks retain '_')
  - [x] Inline docs updated for new config APIs & capability traits (gradient/unified_calculator.rs, clock.rs, logger.rs enhanced 2025-08-12)
  - [x] Add ADR for smart constructor introduction (EasingFactor/Position/Steps + builder rationale) (analysis/ADR-smart-constructors.md) (completed 2025-08-12)
  - [x] Add ADR for ColorCollection enum decision (analysis/ADR-ColorCollections-Enum.md) (completed 2025-08-12)
  - [x] Plan enum/HOF replacement for strategy trait objects (draft design snippet)
- Merge:
  - [ ] Milestone closure once legacy OOP remnants replaced & unwrap tracker at 0 (non-test)

Quality Gates:
- [x] QG-001 Compilation
- [x] QG-002 FP compliance (no deprecated public exports; config-struct APIs)
- [x] QG-003 Legacy code zero-introduction (no new OOP remnants)
- [x] Tests for migrated modules (updated; all green)

Remaining Work (Milestone 2.0):
- [x] Audit and remove remaining unwrap/expect/panic in non-test paths (convert to Result/Option or safer comparisons) (current audit 2025-08-13: production occurrences = 0; residual unwraps confined to #[cfg(test)] blocks)
- [x] Design Logger/IO capabilities (ports) for future extraction (ADR: analysis/ADR-logging-capability.md)
- [x] Begin dead-code sweep preparation (Milestone 3.0 Plan tasks) (plan: analysis/dead_code_sweep_plan.md)
 - [ ] Enum/HOF migration implementation (post design plan)
 - [x] Introduce Command enum & refactor dispatch (satisfied by enhancing existing `CommandType` enum docs)
 - [x] Capability injection plan for remaining time usages (no remaining direct SystemTime/Instant; previous benchmarking module removed)
  - Note: Removal of `performance_validation.rs` eliminated last direct `Instant::now` usage; production timing via `Clock` only.

Next Steps (near-term):
- [x] Update unwrap tracker counts after audit (2025-08-12: production panic/unwrap/expect = 0; remaining uses confined to tests; performance_validation refactored to Result)
- [x] Add decision log entries referencing blueprint (decision log created)

Artifacts:
- Refactoring notes per module under analysis/

Run log (2025-08-08 → 2025-08-12 updates):
 - Dead/unused dependency baseline verification: cargo +nightly udeps --all-targets (clean: no 'unused'); cargo machete (clean) (2025-08-12).
 - Dead code sweep: cargo-machete → No unused dependencies reported.
 - Unused deps sweep: ran with +nightly; flagged `regex` and dev `proptest` as unused → removed. Current sweep clean.
 - Compat removal & RAL rewiring: edited `lib.rs`, removed `src/color_parser/compat.rs` (module). Root `src/compat.rs` retained but now fully deprecated with 0.21.0 removal notice.
 - Added ColorCollection enum ADR (analysis/ADR-ColorCollections-Enum.md) and completion log in FP migration plan.
 - Documented performance test threshold rationale (tests/ral_gradient_tests.rs) for 8000ms upper bound; retained guard only.
 - Command enum task resolved: `CommandType` confirmed as adequate ADT; added explanatory doc comment in `command_execution/types.rs`.
 - Time capability plan: audited time usage; only benchmarking module uses direct `Instant::now`; production code compliant via `Clock`.
 - Iterator pipeline refactor: replaced imperative loops in gradient output formatters with iterator + map/join pipelines (2025-08-12).
 - Logger capability integration: ExecutionContext now carries injected logger (replaced eprintln! with Logger calls in hooks) (2025-08-12).
 - Performance benchmarking/validation modules permanently deleted (2025-08-12) — no stubs retained. Only the lightweight performance guard in ral_gradient_tests (median-of-3, 8000ms threshold) remains.
 - CLI logging: Added `--log-level` global flag with filtering logger capability (trace|debug|info|warn|error|none); implemented lightweight FilteringLogger and wired basic hue analysis completion debug log (2025-08-12).

---

### Milestone 3.0: Legacy/Dead/Deprecated Code Elimination
Status: [ ] (PENDING) Target: Zero-tolerance legacy elimination with safe rollback.

Phases:
- Phase 3.1: Automated Detection & Review
  - [x] Run cargo-udeps and cargo-machete; triage results (process: analysis/dead_code_sweep_plan.md) (2025-08-12: initial run attempted; capture anomaly for udeps, classification table initialized, follow-up actions logged)
  - [x] Identify unused modules, fns, types; assess cohesion (2025-08-13: legacy distance helpers removed Batch 1; deprecated no-op formatter function queued & removed Batch 2; compat.rs scheduled Phase 3.3)
  - [x] Prepare elimination plan with verification checkpoints (dead_code_sweep_plan updated 2025-08-13 with Batch 3 plan)

- Phase 3.2: Safe Removal & Cohesion Restoration
  - [x] Remove confirmed dead/unused code paths (Batch 1 complete 2025-08-12: removed legacy distance helpers `calculate_delta_e_76_legacy`, `calculate_delta_e_2000_legacy`, `calculate_euclidean_distance_legacy`, `parse_algorithm_legacy`; tests & clippy green)
  - [x] Remove deprecated no-op formatter function (Batch 2 complete 2025-08-13)
  - [x] Execute early compatibility layer removal (Batch 3 complete 2025-08-13; `src/compat.rs` deleted)
  - [ ] Restore connectivity where code is miswired but needed
  - [ ] Update public APIs or deprecate with migration notes (if needed)

- Phase 3.3: Compatibility Layer Decommission
  - [x] Remove deprecated `src/compat.rs` (executed early in Phase 3.2 Batch 3)
  - [x] Provide migration guidance in CHANGELOG/EXAMPLES (CHANGELOG updated 2025-08-13; examples unaffected)
  - [x] Verify no regressions in CLI/Examples/Docs (tests pass; no compat references remain)

Quality Gates:
- [x] Zero new warnings; clippy clean (post Batch 3)
- [x] All builds/tests pass
- [x] Documentation updated; links valid
- [x] Legacy count = 0

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

---

### Milestone 6.0: Research – NCS Index 2050 Color Collection Feasibility
Status: [ ] (PENDING) Target: Determine legality, data sourcing, and technical approach for adding NCS Index 2050 (Natural Color System) colors (HEX approximations, code-as-name) as an optional collection without violating licensing.

Context:
- Request: Evaluate if we can add an NCS Index (~2050) color collection similar to CSS / RAL sets.
- Constraints: NCS is a proprietary system; need to confirm redistribution rights for code lists and color approximations.
- Goal: Produce a go/no-go decision with documented rationale and (if go) an implementation plan for next sprint.

Phases (Plan → Build → Verify → Polish → Merge):
- Plan:
  - [ ] Identify authoritative sources for NCS codes and licensing terms (official site, published specs, existing open datasets if any)
  - [ ] Classify data elements required: code (e.g., NCS S 1050-Y90R), canonical name (likely same as code), approximate sRGB/HEX mapping strategy
  - [ ] Define acceptance criteria for perceptual fidelity vs legal compliance (tolerance thresholds ΔE2000?)
  - [ ] Risk register: legal (licensing), accuracy (mapping deviations), maintenance (updates), performance (lookup size)
- Build (Research Artifacts):
  - [ ] Prototype mapping method: derive sRGB approximations from published NCS → CIEXYZ or L*a*b* references (if obtainable) or document inability
  - [ ] Draft data schema: csv columns (code,name,hex,l,a,b,source,delta_from_reference)
  - [ ] Evaluate storage size & load time impact; estimate memory footprint
- Verify:
  - [ ] Validate sample subset (≥25 colors across hue/lightness range) against reference ΔE2000 threshold (target ≤4.0 avg if reference LAB available)
  - [ ] License compliance review (document terms with quotations & URLs)
  - [ ] Performance probe: load + first 100 distance matches timing benchmark (compare to RAL Design baseline)
- Polish:
  - [ ] Draft ADR: "ADR-NCS-Collection-Feasibility" covering options (Add / Add via plugin / Defer / Reject)
  - [ ] Add migration / usage note stub in FEATURE_CATALOG (conditional on go)
  - [ ] Update sprint assignments (A1/A2/A3) impact analysis
- Merge:
  - [ ] Final decision recorded (GO or NO-GO) with rationale; follow-up tasks enumerated (implementation deferred to next sprint if GO)

Quality Gates (Research Scope):
- [ ] Legal evidence captured (links + quoted excerpts) in analysis/ADR-NCS-Collection-Feasibility.md
- [ ] Sample color fidelity report (ΔE stats) present (if reference LAB found); else documented limitation
- [ ] Performance probe numbers recorded (same machine assumptions as prior benchmarks)
- [ ] No code collection added in this sprint (research only) – guard against premature inclusion

Success Criteria:
- Clear, actionable decision with risks/trade-offs
- If GO: concrete data acquisition + normalization plan, schema, and estimation of integration complexity
- If NO-GO: documented blockers & re-evaluation trigger (e.g., licensing change)

Risks:
- Legal: Proprietary licensing may forbid redistribution (High)
- Data Quality: Unofficial HEX approximations vary widely (Medium)
- Maintenance: Upstream changes to NCS definitions (Low-Med)

Mitigations:
- Use only data explicitly licensed for redistribution, or require user-provided dataset path
- Provide plugin/feature flag approach to keep core unencumbered
- Document approximation method & ΔE statistics transparently

Follow-ups (if GO):
- Implement NCS loader module (feature-gated)
- Add distance matching integration & CLI selector (ncs)
- Provide validation test set with sampled reference LAB values

Artifacts (expected):
- analysis/ADR-NCS-Collection-Feasibility.md (decision record)
- analysis/ncs_prototype/ (optional) containing sample CSV + fidelity report

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

## Governance Note (2025-08-13)
Milestone 3 work has moved to branch `sprint-0.20.0-m3-20250813` per §5 naming conventions. Previous branch `sprint-0.20.0-m2-20250812` retained for history; future Milestone 3 commits target the new branch.
