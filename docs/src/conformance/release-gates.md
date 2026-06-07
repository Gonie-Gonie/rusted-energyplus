---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Release Gates

Before a compatibility claim appears in a release note:

- release note states oracle version
- release note states evidence level
- supported case manifests exist
- requested output variables are declared
- tolerance policy is declared
- EnergyPlus baseline artifacts exist
- Rust artifacts exist
- `compare-summary.json` and `compare-report.md` exist
- release PDF/HTML/JSON evidence exists for promoted numerical cases
- blocking gate runs in script or CI
- unsupported scope is listed
- false-conformance guard passes

If any item is missing, the release may still publish engineering progress, but
it must not claim numerical compatibility for that behavior.
