<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

# Milestone Map

Milestones are maintained in `specs/milestones.toml`.

| Version | Title | Status | Claim level | Required cases | Not claimed |
|---|---|---|---|---|---|
| 0.16 | Versioning and Evidence Cleanup | complete | planning-documentation |  | new numerical conformance, plant compatibility, HVAC compatibility |
| 0.17 | Case Manifest and Output Request Schema v2 | complete | infrastructure-only | heat_balance_nomass_001, surface_temperature_nomass_001 | new numerical conformance, ExampleFiles compatibility, meter conformance |
| 0.18 | Output Request Injection and Oracle Baseline Pipeline | planned | baseline-only |  | new numerical conformance unless promoted by report and gate, general heat-balance compatibility, HVAC compatibility, plant compatibility |
| 0.19 | Series Reader and Compare Engine v2 | planned | comparison-infrastructure |  | new numerical conformance unless a case is explicitly promoted, meter conformance |
| 0.20 | Conformance Report Generator | planned | reporting-infrastructure |  | new numerical conformance unless backed by generated evidence |
| 0.21 | Source Map and Algorithm Ledger v1 | planned | planning-guard |  | algorithm completion without source map |
| 0.22 | Time, Weather, and Schedule Conformance Expansion | planned | declared-variables-only |  | general runtime compatibility |
| 0.23 | Static Model Evidence Expansion | planned | static-evidence |  | dynamic heat-balance compatibility |
| 0.24 | Runtime State and Output Registry Hardening | planned | runtime-infrastructure |  | new numerical conformance |
| 0.25 | Opaque No-Mass Heat Balance Generalization | planned | limited-conformance | heat_balance_nomass_001, surface_temperature_nomass_001 | general heat-balance compatibility, HVAC compatibility, plant compatibility |

## Long-Term Targets

| Version | Title | Claim level |
|---|---|---|
| 1.0 | Substantial Compatibility Draft | declared-subset |
| 2.0 | EnergyPlus 26.1 Full Compatibility | full-compatibility-mode-with-evidence |
| 3.0 | Fast Modernized Successor | mode-specific |
