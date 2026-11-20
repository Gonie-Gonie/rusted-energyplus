# Changelog

## Unreleased

No unreleased changes yet.

## v0.2.0 - 2026-11-20

Bounded-scope Windows prerelease, based on the published Source212 algorithm
implementation. Unfinished active-Window work is excluded.

- Documented explicit Source A/B, legacy, IDF/epJSON and object/value boundaries.
- Shortened current documentation and example-object summaries, retaining
  complete names and links to detailed registry constraints.
- Curated the Windows package around executable delivery, known examples,
  support documentation, checksums, a manifest and third-party notices.
- Added extracted-package verification without repeating porting experiments.

Retained numerical evidence and its declared tolerances are unchanged.

## v0.1.0 - 2026-06-18

Public limited-conformance pre-1.0 release.

### Added

- Windows release package with `eplus-rs.exe`, bundled EnergyPlus 26.1.0
  oracle runtime, and no-console `eplus-rs-launch.exe` EPLaunch-style UI.
- Arbitrary IDF/epJSON run pipeline with support assessment, typed diagnostics,
  claim-boundary reports, optional oracle baseline, and oracle comparison
  artifacts.
- Release evidence generation for numeric conformance evidence, conformance
  index, support coverage, user coverage handbook, and release evidence
  manifest.
- Official dynamic heat-balance diagnostic now executes a Rust run-period
  warmup loop before reporting samples.
- EnergyPlus EIO `Environment:WarmupDays` parsing and report fields for
  oracle run-period warmup day counts.
- Official dynamic heat-balance diagnostic now compares selected roof
  inside/outside face temperatures in addition to conduction rates.

### Conformance Scope

- Claims are limited to declared cases, declared output variables, declared
  tolerances, and blocking gates.
- Promoted scope includes no-mass heat-balance/surface-temperature cases,
  declared schedule and weather variables, internal convective gain, selected
  official `1ZoneUncontrolled` dynamic outputs, and the declared IdealLoads
  no-OA/outdoor-air/facility-meter candidate gates.
- Ad-hoc arbitrary runs and diagnostic projections keep
  `conformance_claim=false` even when oracle comparison artifacts are produced.

### Scope Boundary

- v0.1.0 does not claim full EnergyPlus compatibility, broad ExampleFiles
  compatibility, broad HVAC simulation, plant simulation, EMS/PythonPlugin
  support, AirflowNetwork support, or arbitrary-IDF numerical conformance.

Earlier internal development milestones remain in Git history.
