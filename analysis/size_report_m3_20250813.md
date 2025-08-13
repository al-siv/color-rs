# Module Size Report (Milestone 4 Phase 4.1)

Date: 2025-08-13
Branch: sprint-0.20.0-m3-20250813

## Line Counts (Top-Level src/*.rs)
| File | LOC |
|------|-----|
| cli.rs | 851 |
| output_formats.rs | 565 |
| image_core.rs | 405 |
| image.rs | 401 |
| lib.rs | 444 |
| color_matching.rs | 484 |
| parsing_chain.rs | 342 |
| color.rs | 305 |
| color_formatter.rs | 295 |
| file_output.rs | 264 |
| format_utils.rs | 219 |
| precision_utils.rs | 210 |
| utils.rs | 240 |
| config.rs | 185 |
| error.rs | 158 |
| clock.rs | 107 |
| logger.rs | 115 |
| main.rs | 44 |

Total top-level LOC counted: 5792 (excludes subdirectories and removed compat.rs).

## Oversized Module Thresholds
- Soft target: <400 LOC
- Hard cap (guidelines §8): 600 LOC (none exceed hard cap; cli.rs >600 so priority)

## Priority Modules for Decomposition
1. cli.rs (851) – candidate: split argument parsing, validation utilities, range helpers.
2. output_formats.rs (565) – candidate: separate format-specific renderers (e.g., table vs yaml).
3. color_matching.rs (484) – candidate: extract distance computation orchestration vs reporting.

## Preliminary Large Function Scan (cli.rs)
Sampling (line numbers approximate from grep output):
- validate methods (multiple impl blocks) – likely repetitive invariant checks.
- parse (Range::parse) – ensure concise; may be fine if <60 LOC; needs direct measurement.

Next step: measure function lengths precisely (rust-analyzer or simple awk script) – deferred until after first extraction candidate selected.

## Extraction Candidates (Initial)
- cli.rs: Range parsing & range validation logic – move to new `range.rs` or `cli_range.rs` module under `src/` (pure utilities).
- cli.rs: Output filename generation (svg_name/png_name) – consolidate into small pure helper file if cluster grows.
- output_formats.rs: Separate YAML vs tabular serialization pipelines into distinct modules (`output_yaml.rs`, `output_table.rs`).

## Plan (Phase 4.1 Slice 1)
1. Extract range-related types & functions from cli.rs into `src/cli_range.rs` (pure, unit-testable) without changing public CLI behavior.
2. Add focused unit tests (if not existing) for range parsing & membership logic.
3. Update `cli.rs` to re-export or use new module; ensure function sizes drop.
4. Re-run gates (clippy/tests) – must remain green.
5. Update sprint file with Phase 4.1 progress.

## Risks
- Minor: Public interface change if visibility altered (mitigate with re-export or `pub use`).
- Test coverage gaps for extracted logic (add tests where missing).

## Success Criteria
- cli.rs LOC reduction (target <700 in this slice; further slices later).
- No new warnings; behavior identical (CLI tests pass).

## Next (Deferred) Slices
- output_formats.rs format split.
- color_matching.rs orchestration extraction.

Prepared by autonomous agent per guidelines §13.
