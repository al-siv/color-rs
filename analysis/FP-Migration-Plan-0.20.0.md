# FP Migration Plan – Sprint 0.20.0

Date: 2025-08-08

## Scope
- Replace legacy OOP remnants with FP-first designs
- Eliminate unused/dead/deprecated/compat code
- Systematic refactoring for cohesion and clarity

## Findings (Phase 1.1/1.2)

### Dynamic Dispatch / OOP-like Constructs
- output_formats.rs: with_clock(clock: &dyn crate::clock::Clock, …)
- command_execution/*: multiple &dyn Clock usages (capability-based OK)
- color_parser/*: ColorCollection as Box<dyn ColorCollection> (REPLACED in 0.20.0 by `ColorCollectionKind` enum + `ColorCollections` manager – legacy trait object manager removed)
- gradient/gradient_formatter.rs: callbacks as Box<dyn Fn… + Send + Sync> (functional callbacks OK)
- compat.rs: create_parser returns Box<dyn ColorParserCompatTrait> (compat layer candidate for decommission)

Decision notes:
- Keep capability traits (Clock) for effect injection
- Evaluate replacing Box<dyn ColorCollection> with tagged enums or sealed trait + concrete types if performance and clarity benefit
- Assess compat.rs for deprecation/removal

### Hidden Effects / Time Access
- performance_validation.rs: Instant::now() (bench contexts OK)
- color_ops/analysis/hue.rs: SystemTime::now() in generated_at (consider injecting Clock)
- clock.rs: provides proper DI; prefer using Clock across modules

Action:
- Inventory sites using SystemTime/Instant directly and plan Clock injection where not test-only

### unwrap/expect/panic usage (non-test)
- Widespread in tests/examples; a few in non-test code:
  - command_execution/mod.rs: panic!("Wrong command type") – consider Result-based errors
  - file_output.rs: let content = result.unwrap(); – replace with proper error propagation
  - gradient/output.rs: unwrap for String write – safe but avoid unwrap in core modules
  - color_parser/unified_manager.rs: expect in constructors – assess conversion to Result
  - parsing_chain.rs: unwrap() on nth() – convert to safe parsing logic

Action:
- Replace non-test unwrap/expect/panic in business logic with Result/Option and ? propagation

### Globals / Singletons
- No direct global mut/static mut usage found

## Recommendations (Phase 1.3)

### Pure Core Extraction
- Extract pure transformation functions from mixed modules in color_ops, parsing_chain, file_output

### Effect Isolation
- Standardize Clock injection where time is needed
- Replace panic/unwrap with Result error types in command_execution, parsing_chain, file_output

### Domain Modeling
- Introduce/extend ADTs for parser results and command variants; exhaustive matches
  * Color collections enum migration COMPLETE (0.20.0) – Box<dyn ColorCollection> eliminated.

## Proposed Task Seeds (Updated)
- T1: Replace SystemTime::now() in hue analysis with Clock
- T2: Remove unwrap in file_output and gradient/output; error propagation
- T3: Replace panic! in command_execution with typed error
- T4: Decommission compat.rs (currently still present) OR clearly mark with #[deprecated(since = "0.20.0", note = "Will be removed in 0.21")] and update CHANGELOG
- T5: (DONE) Replace Box<dyn ColorCollection> – implemented via enum `ColorCollectionKind` & manager `ColorCollections`
- T6: Introduce Command enum to remove any residual dynamic dispatch in execution layer (planned)

## Risks
- Behavior changes from unwrap removal – covered by tests
- API adjustments when replacing dyn patterns – communicate in CHANGELOG

## Next
- Run cargo check/clippy baseline (DONE initial pass; keep nightly runs)
- Translate updated seeds into Milestone 2.0/3.0 checklist items
- Author ADR for ColorCollection enum decision (document rationale & alternatives)
- Decide compat.rs fate before closing Sprint 0.20.0 (remove vs deprecate)

---

## Completion Log (Addendum 2025-08-12)
### Color Collection Dynamic Dispatch Removal
- Implemented `ColorCollectionKind` enum encapsulating Css / RalClassic / RalDesign collections.
- Added `ColorCollections` manager providing aggregate search APIs.
- Removed legacy `ColorCollectionManager` and associated Box<dyn> usage.
- Updated CLI hue analysis and unified manager to construct and operate on enum variants.
- Added enum-level tests ensuring parity.
- Verified `grep -R "Box<dyn ColorCollection>"` now returns zero results.
- Performance test threshold (8000ms for 10 gradients) currently passes; monitor for regressions.

Pending clean-up: Remove or deprecate compatibility layer (compat.rs) and introduce Command enum refactor.
