# ADR: NCS Index 2050 Color Collection Feasibility

Status: Draft
Date: 2025-08-13
Decision: PENDING (Research Milestone 6.0)

## Context
A request was made to evaluate adding an NCS Index (~2050 colors) collection similar to existing CSS, RAL Classic, and RAL Design System+ sets. NCS (Natural Color System) is proprietary; licensing and redistribution rights for code lists and color values are uncertain. We must determine if inclusion (direct or indirect) is legally and technically feasible without violating IP or licensing constraints while maintaining FP core principles and repository hygiene.

## Problem Statement
Can the project safely (legally + technically) integrate an NCS Index color collection offering:
- Code (identifier) as canonical name (e.g., `NCS S 1050-Y90R`)
- Reliable approximate sRGB/HEX representations
- Distance algorithm compatibility (reuse existing DistanceAlgorithm enum)
without distributing restricted proprietary data or incurring unacceptable maintenance or fidelity risk?

## Forces / Constraints
- Legal/IP: Proprietary system; uncertain redistribution rights for full list.
- Accuracy: Need perceptual fidelity; absence of official open LAB dataset may force approximation.
- Performance: Adding ~2050 entries increases load/match overhead; must stay within acceptable performance envelope (baseline: RAL Design ~1600 colors).
- Maintainability: Data updates cadence unknown; must plan versioning or external data injection if frequently updated.
- User value: Expands professional design applicability; potential increased adoption.

## Options
1. Direct Inclusion (embed dataset in repo)
2. Feature-Gated Inclusion (compile with `--features ncs` gating dataset)
3. External Data Loader (user supplies CSV path; ship schema & loader only)
4. Plugin/Extension Crate (`color-rs-ncs` separate repository)
5. Defer (No-Go until licensing clarity / open dataset released)

## Evaluation Criteria
- Legal clarity (hard requirement)
- ΔE2000 average fidelity vs reference (target ≤4.0, max ≤7.0 across sample)
- Load & first 100 match latency comparable to RAL Design (±15%)
- Memory footprint growth acceptable (<25% over current largest collection)
- Maintenance overhead (update frequency + automation feasibility)

## Data Requirements (Proposed Schema)
`code,name,hex,l,a,b,source,delta_from_reference`
- `code`: canonical NCS identifier
- `name`: same as code (unless alternative label emerges)
- `hex`: approximated sRGB (uppercase #RRGGBB)
- `l,a,b`: LAB reference or derived values (if available)
- `source`: provenance tag (e.g., `derived_formula`, `third_party_open`, `user_supplied`)
- `delta_from_reference`: numeric ΔE vs authoritative LAB (if reference data available)

## Legal Research Checklist
- [ ] Identify official NCS licensing terms (URL, capture quote)
- [ ] Determine if code list (identifiers only) is copyrightable or protected
- [ ] Determine redistribution status of colorimetric values (LAB/XYZ/sRGB approximations)
- [ ] Assess fair use / transformative use boundaries (if approximation algorithm derived independently)
- [ ] Confirm whether third-party open datasets exist and their licenses (compatibility with MIT/Apache-like terms)
- [ ] Document risks & required attributions

## Technical Research Tasks
- [ ] Find or derive mapping algorithm NCS → CIE LAB (if formulaic relationships published)
- [ ] Prototype 25-sample conversion spanning hue/lightness/chroma
- [ ] Compute ΔE2000 vs any available reference for those samples
- [ ] Benchmark distance matching performance impact
- [ ] Memory footprint estimation (size per entry × 2050) vs RAL Design baseline

## Risk Analysis
| Risk | Category | Impact | Likelihood | Mitigation |
|------|----------|--------|------------|------------|
| Licensing forbids redistribution | Legal | High | Medium | External loader / plugin crate |
| Incomplete or low-fidelity data | Accuracy | Medium | Medium | Document ΔE stats; gate behind feature |
| Performance regression | Performance | Low-Med | Low | Precompute LAB; reuse existing structures |
| Maintenance churn | Ops | Low | Unknown | Externalize dataset sourcing |
| User confusion (approximate values not official) | UX | Medium | Medium | Clear labeling & disclaimers |

## Decision (TBD)
Will select one of: Go (Option 2 or 3 likely), Conditional Go (license pending), No-Go.

## Follow-Up (Conditional on Decision)
- If Go: implement feature-gated loader + schema validator; add CLI selector `ncs`.
- If External Loader: create `--ncs-file <path>` CLI flag, fallback errors with guidance.
- If Plugin: initialize separate crate with shared trait interface.
- If No-Go: record blockers + re-evaluation trigger (e.g., open dataset release).

## Timeline Alignment
Maps to Milestone 6.0 phases; completion required before any implementation tasks scheduled in a subsequent sprint.

## Open Questions
- Are NCS identifiers trademarked or only subject to database rights?
- Existence of reliable open-source approximations under permissive license?
- Acceptable ΔE threshold for “approximate but useful” designation?

## References
(Collect during research phase)

---
(End of ADR draft)
