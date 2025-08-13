# Size Report 0.20.0 (Baseline after gradient extraction)

Date: 2025-08-13
Branch: sprint-0.20.0-m3-20250813

## Summary
This baseline captures current module sizes to guide further decomposition (serializer extraction, function length enforcement).

## Top-level Source File Line Counts
(Generated via `wc -l src/*.rs`)

| File | LOC |
|------|-----|
| cli.rs | 797 |
| color_matching.rs | 484 |
| output_formats.rs | 465 |
| lib.rs | 448 |
| image_core.rs | 405 |
| image.rs | 401 |
| parsing_chain.rs | 342 |
| color.rs | 305 |
| color_formatter.rs | 295 |
| file_output.rs | 264 |
| utils.rs | 240 |
| format_utils.rs | 219 |
| precision_utils.rs | 210 |
| config.rs | 185 |
| (others combined) | (remaining to 5853 total) |

Total (these files): 5853 LOC

## Notable Large Modules
- cli.rs exceeds soft cap; staged extractions (Range done, next: command handling segmentation planned).
- output_formats.rs slated for serializer method extraction into `serialization.rs`.

## Next Actions
1. Extract serializer logic (to_toml / to_yaml) → expect ~5-8% shrink in output_formats.rs.
2. Implement automated function length checker (<60 LOC gate).
3. Re-run size report post-extraction for delta tracking.

## Historical Context
Previous size reports were removed with `analysis/` directory cleanup; this re-establishes traceability.
