# Enum / Higher-Order Function Replacement Plan

Status: Draft (2025-08-12)
Objective: Replace remaining strategy trait object usages and dynamic dispatch with Algebraic Data Types (enums) and higher-order pure functions to improve performance transparency, enable exhaustive matching, and simplify reasoning.

## 1. Inventory of Dynamic Dispatch Sites

| Area | Current Pattern | Candidate Enum | Notes |
|------|-----------------|----------------|-------|
| gradient::gradient_formatter | Trait objects for formatting strategies | GradientFormatStrategy enum { Table, Css, Json, Composite(Vec<GradientFormatStrategy>) } | Composite may remain recursive enum; replace Box<dyn Formatter>. |
| gradient::gradient_stops | Strategy pattern for stop calculation (simple vs smart) | GradientStopMode enum { Simple, Smart } | Already partially unified by config flag use_simple_mode; convert to enum with data payload if needed. |
| command_execution::commands | Trait-like command variants | Command enum { Gradient(GradientArgs), Hue(HueArgs), ... } with execute(&self, ctx) -> Result<Output> | Enables exhaustive matching at dispatcher. |
| color_parser::collections | Box<dyn ColorCollection> for multiple collections | ColorCollection enum { Css(CssColorCollection), RalClassic(...), RalDesign(...)} | Must evaluate memory size impact; may wrap in Arc if cloning heavy. |

## 2. Design Principles
- Keep enum variants thin; store only necessary data.
- Provide pure functions: format_gradient(values, strategy: GradientFormatStrategy) -> String.
- Use match statements; forbid wildcard `_` to ensure exhaustiveness.
- Consider feature flags if some variants are optional to avoid bloat.

## 3. Migration Steps (Incremental)
1. Introduce new enums alongside existing traits (no breaking changes yet) behind internal module gates.
2. Implement conversion From<OldType> for new enums where trivial.
3. Refactor internal call sites to use enum-based pure functions; keep public API stable initially.
4. Deprecate trait-based constructors (add #[deprecated] notices) once enum path validated.
5. Remove trait objects and Box allocations; update documentation & sprint plan.

## 4. Risk & Mitigation
- Size increase of monomorphic enum dispatch tables: measure via size check before/after.
- Potential compile time increase: limit macro complexity, incremental PRs.
- API churn: hide enums in non-public modules until stable.

## 5. Open Questions
- Should color collections remain heap allocated for large datasets? Potential Option: enum holds &'static [Entry] or Arc<[Entry]>.
- Introduce smallvec for composite formatting strategies? (Defer until profiling indicates need.)

## 6. Acceptance Criteria
- No Box<dyn ...> in targeted modules (grep check).
- All strategy executions via exhaustive match; no wildcard arms.
- Tests updated; no behavior regressions (CLI examples produce identical outputs).
- Documentation updated (ARCHITECTURE.md & sprint file notes).

## 7. Follow-ups
- Potential derive macro to auto-implement Display for enums (optional polish).
- Property-based tests for gradient stop generation across enum modes.

---

## 8. ColorCollection Enum Migration (Detailed Design)

### 8.1 Current State
- Manager: `ColorCollectionManager { collections: Vec<Box<dyn ColorCollection>> }`.
- Implementors: `CssColorCollection`, `RalClassicCollection`, `RalDesignCollection` (each holds `Vec<ColorEntry>`).
- Dynamic dispatch sites: iteration in manager methods (`find_closest_across_all*`, `search_by_name`, CLI hue analysis command constructing `Box<dyn ColorCollection>` via `match`).

### 8.2 Target ADT
```rust
pub enum ColorCollectionKind {
	Css(CssColorCollection),
	RalClassic(RalClassicCollection),
	RalDesign(RalDesignCollection),
}
```
Supporting sealed-like module privacy: keep enum in `color_parser::collections` module (non-public initially) or expose with `pub(crate)` until stable.

### 8.3 Enum Methods (mirroring trait)
Essential interface implemented via inherent impl:
```rust
impl ColorCollectionKind {
	pub fn name(&self) -> &'static str { /* exhaustive match */ }
	pub fn colors(&self) -> &[ColorEntry] { /* match */ }
	pub fn find_closest(&self, target: &UniversalColor, max: usize, filter: Option<&SearchFilter>) -> Vec<ColorMatch> { /* delegate to variant specific logic, reusing existing impl bodies */ }
	pub fn find_closest_with_algorithm(&self, target: &UniversalColor, max: usize, filter: Option<&SearchFilter>, algo: DistanceAlgorithm) -> Vec<ColorMatch> { /* shared helper */ }
	pub fn find_by_name(&self, name: &str) -> Option<ColorEntry> { /* match */ }
	pub fn find_by_code(&self, code: &str) -> Option<ColorEntry> { /* variant specific where optimized, else generic */ }
	pub fn find_by_name_pattern(&self, pattern: &str) -> Vec<ColorEntry> { /* generic over colors() */ }
	pub fn find_by_luminance_range(&self, min: f64, max: f64) -> Vec<ColorEntry> { /* generic */ }
	fn matches_filter(entry: &ColorEntry, filter: Option<&SearchFilter>) -> bool { /* lift from trait default */ }
}
```

### 8.4 Manager Replacement
Introduce parallel manager during transition:
```rust
pub struct ColorCollections { items: Vec<ColorCollectionKind> }
impl ColorCollections {
	pub fn add(&mut self, item: ColorCollectionKind);
	pub fn names(&self) -> Vec<&'static str>;
	pub fn find_closest_all(&self, target: &UniversalColor, per: usize, filter: Option<&SearchFilter>) -> Vec<(String, Vec<ColorMatch>)>;
	// ... mirrors existing API
}
```
Stage 1: Add new manager (no removals). Stage 2: Migrate CLI & internal callers. Stage 3: Remove old trait object manager and `ColorCollection` trait (convert existing default method logic into free/helper functions).

### 8.5 Memory & Clone Strategy
- Collections own `Vec<ColorEntry>`; moving into enum variant moves the vector (no clone cost at construction).
- Accessors return `&[ColorEntry]` (slice) to avoid copying.
- If future sharing needed, switch inner storage to `Arc<[ColorEntry]>` (deferred; not premature).

### 8.6 Performance Considerations
- Removes one level of indirection and vtable call per method invocation.
- Exhaustive match enables compiler inlining on hot paths (e.g., distance calculations loops remain identical).
- Benchmark post-migration: reuse existing performance test (RAL gradient) to ensure no regression; consider adding micro-benchmark for `find_closest` across collections.

### 8.7 Migration Steps (Detailed)
1. Add `ColorCollectionKind` enum + inherent impl methods (use existing trait default bodies). Keep trait & manager unchanged.
2. Add new `ColorCollections` manager with mirrored APIs.
3. Adjust CLI hue analysis command to construct enum variants instead of `Box<dyn ...>` (behind feature flag or conditional compile if needed, else direct switch).
4. Duplicate a subset of tests exercising manager behavior for both old and new paths (temporarily) to validate parity.
5. Grep for `Box<dyn ColorCollection>`; migrate remaining call sites to new manager.
6. Remove legacy trait + old manager; inline or move helper logic (e.g., `matches_filter`).
7. Update documentation (`SPRINT-0.20.0.md`, `ARCHITECTURE.md` if necessary) and mark sprint checklist items.
8. Run clippy/tests/perf; capture before/after metrics (distance search runtime) for decision log.

### 8.8 Acceptance Criteria (Specific)
- `grep -R "Box<dyn ColorCollection>"` returns 0.
- All previous collection-related tests pass using enum path only.
- No wildcard `_` in `match` over `ColorCollectionKind`.
- Sprint plan updated with sub-bullet under enum/HOF migration listing completion.
- Performance delta within ±5% of baseline (document; minor improvements acceptable).

### 8.9 Risks & Mitigations
| Risk | Mitigation |
|------|------------|
| Hidden external user reliance on `ColorCollection` trait (if public) | Maintain trait temporarily; mark deprecated; provide `From` impls to enum; final removal in next minor if needed. |
| Code duplication between trait default methods and enum impl | Extract shared free functions in private module to reuse. |
| Test brittleness due to duplicated test logic | Introduce helper asserting parity across both managers before removing old path. |

### 8.10 Deferred Considerations
- Potential further enum variant addition (user-defined collections) may justify a small trait retained for plugin boundary; currently out of scope (YAGNI).
- Wrapper newtype to store metadata (load time, source path) if needed later.

Status: Planned (implementation next sequence after doc update) (2025-08-12)

