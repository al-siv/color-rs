# ADR: ColorCollections Enum Consolidation

Status: Accepted
Date: 2025-08-13

## Context
Previous design used disparate structs and ad-hoc matching logic leading to branching complexity. An algebraic enum consolidates collection variants while ensuring exhaustive matching and compile-time safety.

## Decision
Adopt a single `ColorCollectionKind` (name representative) enum to represent all supported collection sources. Pattern matching replaces imperative branching. Re-export for compatibility where needed.

## Rationale
- Enforces exhaustive handling (F5 illegal states unrepresentable).
- Simplifies downstream code via single dispatch point (F4 composition).
- Reduces risk of drift between separate collection handling paths.

## Consequences
Positive:
- Clear addition point for new collection types.
- Simplified matching and validation logic.
Negative:
- Requires minor adaptation in modules referencing old structs.

## Alternatives Considered
- Trait object hierarchy (rejected: runtime cost, indirectness, less exhaustiveness).
- Separate modules with manual registry (rejected: duplication risk).

## Follow-up
Add property-based tests ensuring pattern match coverage remains exhaustive on evolution.
