# ADR: Logging Capability Abstraction

Status: Accepted (2025-08-12)

## Context
Initial logging used ad-hoc `eprintln!` calls scattered through orchestration code, inhibiting testability and future structured logging. FP-first goals require effect isolation with capability traits for IO concerns (Clock, Logger, etc.).

## Decision
Introduce a `Logger` trait with level-aware methods (`trace/debug/info/warn/error`) plus minimal implementations: `NoOpLogger`, `StdoutLogger`, and `FilteringLogger` (level threshold). Expose a CLI `--log-level` flag mapping to filtering logger instances.

## Rationale
- Effect Isolation: Pure core stays free of direct IO; logging occurs at shell boundaries.
- Configurability: Log level selection without rebuilding.
- Zero-cost Disabled Path: `NoOpLogger` allows elision in release builds.
- Extensibility: Future structured logger (JSON, spans) can implement trait without API churn.

## Alternatives Considered
1. External crate (tracing/log/env_logger): Deferred to keep dependency surface minimal until richer features justified.
2. Global static logger mutable behind `lazy_static`: Rejected – hidden dependency harms test isolation.
3. Inline enum-based logger (no trait): Less flexible for downstream customization; trait chosen for clearer capability boundary.

## Consequences
+ Simplifies future integration of structured logging.
+ Enables deterministic tests by injecting `NoOpLogger`.
- Slight abstraction overhead; negligible given rare logging hot paths.

## Migration Notes
- Replace `eprintln!` with capability calls in orchestration contexts.
- Pass logger through context structs (e.g., `ExecutionContext`).

## Future Evolution
- Add span/context support (correlation IDs) via wrapper implementing trait.
- Optional integration with `tracing` crate behind feature flag.
- Structured log formatter (JSON) for machine ingestion.

## References
- Sprint 0.20.0 Milestone 2 (Logger capability design)
- File: `src/logger.rs`
- Decision Log entry (2025-08-12)
