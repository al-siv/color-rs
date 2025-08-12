# ADR: Smart Constructors & GradientConfigBuilder Invariants

Status: Accepted 2025-08-12
Context: Unified gradient calculation previously accepted raw primitives (positions, easing factors, steps) leading to scattered validation and potential invalid states (start >= end, easing outside [0,1], steps < 2). We introduced domain newtypes (EasingFactor, Position, Steps) plus GradientConfigBuilder enforcing invariants before constructing GradientCalculationConfig.

Decision:
- Use newtypes wrapping primitive numeric types to encode domain constraints early.
- Centralize validation in builder methods returning Result<Self> for chaining.
- Prohibit constructing GradientCalculationConfig directly with invalid parameters (make illegal states unrepresentable at construction boundary).
- Keep builder in gradient::unified_calculator; pure logic consumes validated GradientCalculationConfig.

Alternatives Considered:
1. Public struct fields + ad hoc validation (rejected: scattered, error-prone).
2. Single validate() free function post-construction (rejected: allows temporary invalid states; harder reasoning in future refactors).
3. Typestate multi-stage builder (rejected: added generic complexity without clear ROI for current parameter count).

Consequences:
+ Earlier failure for invalid configs; simpler downstream code (no re-checks).
+ Facilitates future API surface reduction (enum-based strategy selection) by constraining inputs.
+ Improves testability (unit tests target builder invariants in isolation).
- Slightly more verbose construction in callers (builder method chaining).
- Need to maintain Default/constructor parity to satisfy clippy (added impl Default).

Migration Notes:
- Legacy arg-heavy functions removed; callers updated to construct GradientCalculationConfig via builder.
- Future enum/HOF migration will reuse validated config without change.

Open Questions / Follow-ups:
- Evaluate extracting DistanceAlgorithm selection into an enum variant of a higher-level GradientMode ADT.
- Consider property-based tests for builder invariants (currently example-based).

References:
- Sprint 0.20.0 Milestone 2.0 Build phase (smart constructors newtypes + builder).
- Clippy lint new-without-default triggered addition of Default impl.
