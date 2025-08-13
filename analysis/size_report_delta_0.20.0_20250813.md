# Size Report Delta (Serialization Extraction)

Date: 2025-08-13
Commit: 6d0d87c (serialization refactor)

## Affected Files
| File | LOC After | Notes |
|------|-----------|-------|
| output_formats.rs | 465 | Wrapper methods now delegate; LOC unchanged (extraction targeted duplication risk) |
| output_formats_gradient.rs | 106 | Delegation to central serializer |
| serialization.rs | 18 | New module with shared helpers |

## Observations
- Primary benefit is future-proofing and single-responsibility for serialization logic rather than immediate LOC reduction in main file (gradient extraction previously removed bulk).
- Next reductions should target large function bodies (see gate failures) and segmentation of `cli.rs` and `gradient/mod.rs`.

## Follow-up Targets (Gate Failures)
Largest offenders (LOC > 60): gradient/mod.rs (453), command_execution/commands.rs (249), cli.rs (162 in one fn), plus multiple 70–110 LOC functions across image_core.rs and others.

## Next Actions
1. ADR stubs to restore traceability.
2. Plan refactor slices for gradient/mod.rs (break into submodules: building, analysis, formatting).
3. Begin splitting command execution functions into smaller composable units.
