# rusted-energyplus

Rust-only EnergyPlus-compatible porting project.

## Compatibility Contract

The target is compatibility with the locked EnergyPlus 26.1.0 oracle. The
project does not replace EnergyPlus engineering or physical algorithms.
Optimization work is limited to Rust data structures, numerical implementation,
execution planning, tracing, caching, diagnostics, and code organization.

Any numerical difference from EnergyPlus must be measured against the oracle
and documented with declared variables, tolerance policy, reports, and a
blocking gate before it can support a compatibility claim.

## Current Public Scope

The public scope is evidence-gated compatibility work for the locked
EnergyPlus 26.1.0 oracle:

- pinned Rust toolchain
- repo-local EnergyPlus 26.1.0 oracle setup
- repo-local EnergyPlus 26.1.0 reference source setup
- repo-local portable Python and pinned report-generation virtual environment
- epJSON RawModel inspection
- TypedModel compile preview for declared seed object families
- missing-reference diagnostics preview
- package and local release scripts
- conformance harness, baseline generation, and report skeletons
- release PDF/HTML/JSON evidence pack for promoted numerical conformance cases
- static geometry, construction/material, and internal-gain smoke evidence
- tolerance-gated conformance only for declared v0.8/v0.9 no-mass cases
- baseline-only thermostat and IdealLoads evidence, plus diagnostic-only
  air-side node baseline/projection evidence
- smoke-level PlantLoop typed graph skeleton evidence

This public scope does not claim:

- general EnergyPlus heat-balance compatibility
- HVAC or plant simulation compatibility
- general zone-temperature conformance
- node or IdealLoads numerical conformance
- plant numerical conformance
- meter conformance
- full runtime simulation compatibility

## Development Evidence

The repository also contains smoke gates, diagnostic comparisons, conformance
schema work, and porting maps used to build toward future compatibility
milestones. These are documented in the mdBook under `docs/src`.

Development-only diagnostics include:

- `run first-zone`
- `compare zone-temperature`

Those commands are useful for runtime plumbing and report generation, but they
must not be used as conformance evidence. Their output uses
`conformance_claim: false`.

## Quick Start

Public setup and current release checks:

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd v0.1-verify
```

The setup script keeps external oracle assets inside repo-local directories:

- `.runtime/energyplus/26.1.0`
- `.reference/energyplus-src/26.1.0`
- `.runtime/python/3.11.9`
- `.runtime/python-venvs/report`

It does not use a globally installed EnergyPlus as the oracle, and the release
evidence generator uses the repo-local Python environment rather than ambient
Python packages.

Current development evidence checks:

```powershell
.\scripts\dev.cmd v0.2-verify
.\scripts\dev.cmd v0.3-verify
.\scripts\dev.cmd v0.4-verify
.\scripts\dev.cmd v0.5-verify
.\scripts\dev.cmd v0.6-verify
.\scripts\dev.cmd v0.7-verify
.\scripts\dev.cmd v0.8-verify
.\scripts\dev.cmd v0.9-verify
.\scripts\dev.cmd v0.10-verify
.\scripts\dev.cmd v0.11-verify
.\scripts\dev.cmd v0.12-verify
.\scripts\dev.cmd v0.13-verify
.\scripts\dev.cmd conformance-evidence-report -Version 0.13.0
```

Only v0.8 and v0.9 contain tolerance-gated conformance claims, and only for
their declared variables. v0.10 is a baseline-only typed-graph gate for
thermostat and IdealLoads intake. v0.11 is diagnostic-only air-side node
baseline evidence plus `NodeStateStore`-backed Rust projection plumbing, with
no node numerical conformance claim. v0.12 is a node source-mapping and
evidence-policy release; it also packages the promoted v0.8/v0.9 numerical
evidence as PDF/HTML/JSON under `evidence/v0.12.0`. v0.13 is a PlantLoop
typed graph skeleton smoke gate and still makes no plant numerical conformance
claim; it packages the same promoted numerical evidence set under
`evidence/v0.13.0`.

Developer-only diagnostics:

```powershell
.\scripts\dev.cmd first-zone-smoke
.\scripts\dev.cmd compare-zone-smoke
```

Those diagnostics are not public compatibility evidence.

Full local quality check:

```powershell
.\scripts\dev.cmd check
```

## Documentation

Start here:

- `docs/src/project-scope/compatibility-contract.md`
- `docs/src/project-scope/evidence-levels.md`
- `docs/src/project-scope/milestone-map.md`
- `docs/src/operations/milestone-reset-plan.md`
- `docs/src/conformance/overview.md`
- `docs/src/conformance/examplefiles-coverage-plan.md`
- `docs/src/operations/script-index.md`
- `docs/src/porting-map/heat-balance.md`

Build the book with:

```powershell
.\scripts\dev.cmd docs-check
```
