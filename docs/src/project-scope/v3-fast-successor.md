---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# v3 Fast Successor Target

v3.0 keeps compatibility mode and introduces fast or modern simulation modes as
first-class public behavior. Fast modes may intentionally diverge from
EnergyPlus-exact numerical results only when the difference is engineering
validated and reported.

## Preparation Series

| Range | Focus |
|---|---|
| v2.1 | fast-mode benchmark design |
| v2.2 | execution plan optimizer |
| v2.3 | compiled model cache |
| v2.4 | parallel surface and zone execution |
| v2.5 | plant solver acceleration |
| v2.6 | adaptive timestep experiments |
| v2.7 | reduced-order fast envelope mode |
| v2.8 | batch simulation runtime |
| v2.9 | engineering validation report framework |

## v3.0 Requirements

v3.0 requires:

- compatibility mode remains available.
- fast or modern modes are declared separately.
- engineering validation cases.
- speedup report.
- deviation or error-bound report.
- user-facing mode selection for compatibility, diagnostic, fast, and
  experimental modes.

Fast-mode claims must not be mixed with compatibility-mode conformance claims.
