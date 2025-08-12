# ADR: Replace ColorCollection Trait Objects with Enum (ColorCollectionKind)

Status: Accepted  
Date: 2025-08-12  
Decision Drivers: Performance clarity, reduction of dynamic dispatch, exhaustive reasoning, functional core goals.

## Context
Historically, color collections were managed via a `ColorCollection` trait with a `ColorCollectionManager` holding `Vec<Box<dyn ColorCollection>>`. Each method invocation incurred dynamic dispatch and an extra heap indirection. As part of the FP migration (Sprint 0.20.0), we aim to eliminate avoidable trait-object polymorphism in favor of Algebraic Data Types to improve transparency, enable exhaustive matching, and simplify inlining opportunities.

Collections in scope:
- CssColorCollection
- RalClassicCollection
- RalDesignCollection

These are closed-world: new variants are not user-extended at runtime. This makes an enum an ideal representation.

## Decision
Introduce an enum:
```rust
pub enum ColorCollectionKind {
    Css(CssColorCollection),
    RalClassic(RalClassicCollection),
    RalDesign(RalDesignCollection),
}
```
Add a manager struct `ColorCollections` (aggregate over `Vec<ColorCollectionKind>`) with APIs mirroring the previous manager. Remove the legacy `ColorCollectionManager` and all `Box<dyn ColorCollection>` usages.

## Alternatives Considered
1. Keep Trait Objects (Status Quo)
   - Pros: No refactor, flexible for plugins.
   - Cons: Unnecessary indirection, harder to reason about exhaustiveness, slight runtime overhead.
2. Sealed Trait + Generics
   - Pros: Still enables static dispatch.
   - Cons: More boilerplate; still resembles trait pattern; less explicit closed set.
3. Enum of References / Arc<[Entry]> Storage
   - Pros: Potential sharing.
   - Cons: Added complexity; premature until profiling indicates memory duplication issues.
4. Macro-generated match dispatch
   - Pros: Could reduce repetitive code.
   - Cons: Adds metaprogramming complexity without clear benefit now.

Enum chosen for simplicity and explicitness.

## Consequences
Positive:
- Removes all `Box<dyn ColorCollection>` allocations and vtable lookups.
- Enables exhaustive matching (no `_` wildcards) improving future refactor safety.
- Simplifies debugging—variant is explicit in logs.

Neutral/Trade-offs:
- Slight enum size overhead (largest variant defines size); acceptable given small number of variants.
- Reduced immediate extensibility for third-party runtime additions (not currently a goal).

Negative / Mitigations:
- If future plugin architecture needed, may need to reintroduce a thin trait boundary. Mitigation: keep enum internal or provide conversion path.

## Implementation Notes
- All previous manager APIs ported: `names`, `find_closest_all`, `find_closest_all_with_algorithm`, search helpers.
- Helper logic (filtering, algorithm selection) moved into inherent impl / free helpers.
- Tests added to assert basic functionality via enum path only.
- Documentation updated: `FP-Migration-Plan-0.20.0.md` completion log; sprint file notes.

## Acceptance Criteria Met
- `grep -R "Box<dyn ColorCollection>"` yields no results.
- Clippy passes with `-D warnings`.
- Existing RAL gradient and collection-related tests pass.

## Follow-ups
- Decide fate of `compat.rs` (deprecate/remove in 0.21.0).
- Possible future Command enum replacing dynamic dispatch in execution layer.
- Optional micro-benchmark for `find_closest` to measure post-refactor improvements.

## Decision Log
Accepted in Sprint 0.20.0; will revisit only if plugin extensibility requirements emerge.
