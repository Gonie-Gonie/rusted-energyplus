---
status: active
claim_level: reporting-infrastructure
owner: release
last_reviewed: 2026-06-08
---

# Release Evidence Manifest

The release evidence manifest is the GitHub Release asset manifest for a
published version. It answers a user-facing release question: which package,
coverage, and evidence files should exist beside the release notes?

Generate it after the release package and evidence reports:

```powershell
.\scripts\dev.cmd package -Version 0.31.0
.\scripts\dev.cmd conformance-evidence-report -Version 0.31.0
.\scripts\dev.cmd conformance-index-report -Version 0.31.0
.\scripts\dev.cmd support-coverage-report -Version 0.31.0
.\scripts\dev.cmd release-evidence-manifest -Version 0.31.0
```

Artifacts are written to:

```text
.runtime/release-evidence/v0.31.0/release-evidence-manifest.md
.runtime/release-evidence/v0.31.0/release-evidence-manifest.html
.runtime/release-evidence/v0.31.0/release-evidence-manifest.pdf
.runtime/release-evidence/v0.31.0/release-evidence-manifest.json
```

The PDF and HTML are generated with `oodocs`; the JSON records file paths,
SHA-256 hashes, content types, asset roles, and report summaries read from the
generated evidence JSON files.

## Asset Families

| Asset family | User-facing purpose |
|---|---|
| binary package | runnable CLI package with docs, specs, scripts, and test data |
| numeric evidence | promoted numerical conformance cases, variables, tolerances, and gate results |
| conformance index | tracked case, output, meter, domain, report, and gate coverage |
| support coverage | supported input objects, output variables, and algorithm families |
| release manifest | checklist proving the release package and evidence assets are present |

## Boundary

The manifest is release documentation infrastructure. It does not add new
numerical conformance, full EnergyPlus compatibility, HVAC numerical
conformance, plant numerical conformance, or meter conformance.

The release workflow uploads the binary zip and every generated file under
`.runtime/release-evidence/vX.Y.Z` as GitHub Release assets. The manifest makes
that asset set explicit and machine-checkable.
