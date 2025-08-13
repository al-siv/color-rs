# NCS Collection Research Plan

Status: Draft
Date: 2025-08-13

## Objective
Assess feasibility of integrating (or approximating) NCS Index 2050 color set into the system while respecting licensing and ensuring accurate colorimetric mapping.

## Key Questions
1. Licensing: Are the canonical NCS sRGB/LAB values redistributable? What attribution required?
2. Source Data: Identify authoritative dataset (CSV, JSON, PDF) with color codes and reference values.
3. Color Space: Provided values in sRGB, LAB, or proprietary? Need conversion approach (D65 alignment, adaptation?).
4. Granularity: Full 2050 set or curated subset for performance/usability?
5. Storage: On-demand lazy load vs compiled static table (size impact)?
6. Matching Strategy: Distance algorithm suitability; need weighting adjustments vs existing collections?

## Tasks
- [ ] Inventory publicly available NCS datasets (record URLs, license notes)
- [ ] Contact/license review summary (if needed)
- [ ] Prototype parsing script (not committed if license unclear) to evaluate structure
- [ ] Color difference validation vs sample references (ΔE thresholds)
- [ ] Decide storage format (compressed CSV vs generated Rust module)
- [ ] Performance impact estimate (lookup time, memory footprint)
- [ ] Risk assessment & go/no-go decision

## Success Criteria
- Documented license path OR decision not to proceed
- Technical plan for ingestion (parsing → normalization → indexing)
- Estimated impact on binary size and matching performance

## Risks
- Licensing restrictions blocking redistribution
- Inconsistent or approximate public data leading to inaccurate matches

## Follow-up
If approved: implement ingestion pipeline with feature flag (ncs) and property tests ensuring ΔE tolerances.
