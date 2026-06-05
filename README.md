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

The public scope is foundation and model intake:

- pinned Rust toolchain
- repo-local EnergyPlus 26.1.0 oracle setup
- repo-local EnergyPlus 26.1.0 reference source setup
- epJSON RawModel inspection
- TypedModel compile preview for declared seed object families
- missing-reference diagnostics preview
- package and local release scripts

This public scope does not claim:

- EnergyPlus heat-balance compatibility
- HVAC or plant simulation compatibility
- zone-temperature conformance
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

```powershell
.\scripts\dev.cmd setup -InstallRust -InstallDocsTools
.\scripts\dev.cmd check
.\scripts\dev.cmd v0.1-verify
```

The setup script keeps external oracle assets inside repo-local directories:

- `.runtime/energyplus/26.1.0`
- `.reference/energyplus-src/26.1.0`

It does not use a globally installed EnergyPlus as the oracle.

## Documentation

Start here:

- `docs/src/project-scope/compatibility-contract.md`
- `docs/src/project-scope/evidence-levels.md`
- `docs/src/project-scope/milestone-map.md`
- `docs/src/conformance/overview.md`
- `docs/src/operations/script-index.md`
- `docs/src/porting-map/heat-balance.md`

Build the book with:

```powershell
.\scripts\dev.cmd docs-check
```
