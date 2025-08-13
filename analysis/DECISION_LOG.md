# Decision Log (Sprint 0.20.0)

Chronological record of key architectural & FP compliance decisions. Each entry includes Context, Decision, Rationale, Alternatives, Consequences, and References.

## 2025-08-12: Replace Box<dyn ColorCollection> with Enum (ColorCollectionKind)
- Context: Dynamic dispatch incurred indirection & complicated exhaustive reasoning.
- Decision: Introduced `ColorCollectionKind` and aggregated `ColorCollections` ADT; removed trait-object manager.
- Rationale: Zero-cost dispatch, pattern match exhaustiveness, simpler CLI integration.
- Alternatives: Retain trait objects (rejected: complexity, less compile-time checking); generic parameterization (rejected: monomorphization bloat, less ergonomic for heterogeneous sets).
- Consequences: Easier future additions (add variant + match arms); minor refactor cost.
- References: analysis/ADR-ColorCollections-Enum.md, sprints/SPRINT-0.20.0.md (Milestone 2.0 notes).

## 2025-08-12: Smart Constructors & GradientConfigBuilder
- Context: Gradient parameter explosion & invariant drift risk (`start<end`, easing bounds, steps >=2).
- Decision: Added `EasingFactor`, `Position`, `Steps` newtypes and builder enforcing invariants; pure configuration struct `GradientCalculationConfig`.
- Rationale: Make illegal states unrepresentable; centralize validation; composable pure APIs.
- Alternatives: Traditional builder with interior mutation (rejected: hidden state & partial invalid states).
- Consequences: Clearer tests, reduced unwraps, improved clippy compliance.
- References: analysis/ADR-smart-constructors.md, migration plan.

## 2025-08-12: Logging Capability (FilteringLogger)
- Context: Need structured, level-filtered logs without polluting pure core.
- Decision: Introduced `Logger` trait + `StdoutLogger`, `NoOpLogger`, `FilteringLogger` with CLI `--log-level`.
- Rationale: Capability boundary for observability, zero-cost when disabled.
- Alternatives: env_logger/tracing dependency (deferred: keep dependency surface minimal first).
- Consequences: Pluggable future structured logger, clear effect isolation.
- References: src/logger.rs, analysis/ADR-logging-capability.md, cli flag implementation, sprint file milestone log.

## 2025-08-12: Performance Test Stabilization (Median-of-3)
- Context: Single-run performance test exhibited variance causing flaky gate.
- Decision: Use median of 3 batches, threshold unchanged (8000ms) for ral gradient test.
- Rationale: Median robust to outliers, minimal extra runtime.
- Alternatives: Mean of N (sensitive to spikes); p95 across >5 runs (higher cost).
- Consequences: Reduced flakiness, deterministic gating.
- References: tests/ral_gradient_tests.rs, sprint run log.

## 2025-08-12: Unwrap/Expect/Panic Audit & Remediation Policy
- Context: Governance gate requires zero non-test panic/unwrap/expect.
- Decision: Replace production unwrap/expect/panic with Result/Option propagation; allow test unwraps only when constructing known-valid test constants (prefer explicit pattern assertions otherwise). Bench validation code now returns Result.
- Rationale: Improves resilience & FP purity; documents invariants via types.
- Alternatives: Retain unwrap in benchmark-only code (rejected to keep uniform rule); custom assert macros (deferred until need arises).
- Consequences: Slight verbosity increase; clearer error surfaces; easier future error taxonomy unification.
- References: src/performance_validation.rs (refactored), command_execution tests (panic removal), collection tests (expect removal).

## 2025-08-12: Deprecated Compatibility Shim Scheduling
- Context: Residual `src/compat.rs` kept for transitional APIs.
- Decision: Marked deprecated with removal target 0.21.0; catalogued in sprint file.
- Rationale: Time-box legacy surface; explicit removal contract.
- Alternatives: Immediate removal (risk: unnoticed downstream usage); indefinite retention (risk: drift & tech debt).
- Consequences: Clear deadline; follow-up milestone item.
- References: sprint plan Milestone 3.0, src/compat.rs header.

---

Future entries append below this line in reverse chronological order (newest first) to aid quick scanning.
## 2025-08-12: Removal of Performance Validation & Benchmark Modules
- Context: `performance_validation.rs` and related examples provided ad-hoc micro-bench validations not integrated into core CLI flow; added maintenance surface and direct `Instant` usage outside capability boundary.
- Decision: Removed module export, source file, and example binaries (`performance_validation.rs`, `performance_benchmark.rs`). Retained only lightweight median-of-3 guard in `ral_gradient_tests`.
- Rationale: Eliminate non-essential code per FP-first minimal core; avoid fragile timing assertions and reduce surface area for drift; remove last direct time usage outside `Clock` capability.
- Alternatives: Keep and gate behind feature flag (added complexity for little value); Refactor into criterion benchmarks (deferred until genuine performance regression risk identified).
- Consequences: Smaller API surface; no public `performance_validation` module; sprint plan updated; users needing benchmarks can create external harnesses.
- References: sprints/SPRINT-0.20.0.md (Milestone 2.0 run log update), commit removing files.
