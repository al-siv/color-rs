# ADT Alternatives & Minority Reports (2025-08-12)

Status: Draft
Linked Milestones: 2.0 (OOP Remnant Replacement), 3.0 (Legacy Elimination)

## Scope
Covers alternative designs considered for:
- ColorCollectionKind / ColorCollections enum (replacing Box<dyn ColorCollection>)
- CommandType enum as central dispatch
- GradientCalculationConfig + builder + smart constructor newtypes (EasingFactor, Position, Steps)

## 1. ColorCollections Enum
Decision ref: ADR-ColorCollections-Enum.md

### Alternatives Considered
1. Trait objects (`Box<dyn ColorCollection>`) retained
   - Pros: Extensible without enum changes; dynamic loading possible.
   - Cons: Runtime cost (vtable), harder exhaustive reasoning, more heap allocations, obscures variant-specific data.
2. Enum + per-variant data (Chosen)
   - Pros: Exhaustive match ensures total handling; potential for niche optimization; clearer ownership and inlining.
   - Cons: Requires code changes on variant addition; may grow large if variants balloon.
3. Macro-generated registry
   - Pros: Pluggable variant addition via macro invocation.
   - Cons: Indirection obscures clarity; macro hygiene & maintenance overhead.

### Minority Report
A macro-based plugin model was proposed to allow external crates to inject new collections without modifying the enum. Rejected due to YAGNI: no current external extension requirement; added complexity harms maintainability now.

## 2. CommandType Enum
Decision ref: (Documented inline; no separate ADR - potential future ADR if surface expands)

### Alternatives Considered
1. Trait-based command objects (Strategy pattern)
   - Pros: Open for extension.
   - Cons: Boilerplate; dynamic dispatch; harder to analyze exhaustively.
2. CommandType enum with pure function dispatch (Chosen)
   - Pros: Exhaustive handling; simple pattern matching; no heap allocations.
   - Cons: Adding a command requires touching central match (acceptable trade-off).
3. Function pointer table / HashMap<String, Fn>
   - Pros: Dynamic registration; test injection friendly.
   - Cons: Indirection cost; stringly-typed risk; weaker compile-time guarantees.

### Minority Report
Dynamic registration via HashMap was argued for test seam flexibility. Rejected because tests can exercise individual pure handlers directly; compile-time enum keeps API discoverable.

## 3. GradientCalculationConfig & Smart Constructors
Decision refs: ADR-smart-constructors.md

### Alternatives Considered
1. Raw argument-heavy function (legacy)
   - Pros: Minimal ceremony.
   - Cons: Many parameters (readability issues), invariant drift risk, frequent misuse.
2. Builder without newtypes
   - Pros: Simpler types.
   - Cons: Weaker invariants; invalid states representable; more runtime checks.
3. Builder + newtypes + smart constructors (Chosen)
   - Pros: Enforced invariants (easing in [0,1], steps≥2, start<end) at construction time; illegal states unrepresentable.
   - Cons: Slightly more upfront code; user must convert primitives.
4. DSL macro for gradient spec
   - Pros: Potentially concise specification.
   - Cons: Higher cognitive load; macro maintenance; error messages less clear.

### Minority Report
A macro DSL was suggested to streamline CLI integration. Rejected pending evidence of repetitive config boilerplate; keep core explicit and type-safe first.

## Cross-Cutting Considerations
| Aspect | Enum Approach | Trait Objects | Macro Registry |
|--------|---------------|---------------|----------------|
| Exhaustiveness | Compile-time checked | Not guaranteed | Depends on macro design |
| Performance | Predictable dispatch | vtable indirection | Macro expansion overhead |
| Extensibility | Requires enum edit | Open | Macro driven |
| Clarity | High | Medium | Low-Medium |

## Future Re-evaluation Triggers
- External plugin requirement emerges (revisit macro/trait hybrid).
- Enum size or match boilerplate becomes unwieldy (> ~12 variants) → consider codegen aids.
- Significant pattern of similar newtypes suggests a generic wrapper pattern.

## Next Actions
- [ ] Link CommandType inline design commentary to this file.
- [ ] Add enumeration of any future ADT introductions here for continuity.

