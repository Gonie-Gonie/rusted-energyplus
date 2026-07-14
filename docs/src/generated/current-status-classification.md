<!-- DO NOT EDIT.
     Generated from specs/ and data/conformance_cases by tools/docs/generate_docs.py. -->

| Classification | Source of truth | Current boundary |
|---|---|---|
| conformance | Case manifests with `conformance_claim = true`, `specs/variable_coverage.toml`, `specs/algorithm_ledger.toml`, and generated compare reports | Only promoted declared cases, variables, and meters listed through `docs/src/generated/conformance-case-index.md`; README and current-status prose are mirrors, not claim sources. |
| diagnostic-only | Case manifests and diagnostic probes with `conformance_claim = false`, diagnostic output levels, and diagnostic reports | Useful source-order evidence, but never counted as a compatibility claim. |
| baseline-only | EnergyPlus oracle baseline artifacts and output levels marked `baseline` | Oracle artifact extraction only; Rust numerical parity is not claimed. |
| not claimed | `specs/project_contract.toml`, generated capability/coverage docs, and this document's Not Claimed section | Broad or full EnergyPlus, heat-balance, HVAC/node/meter, plant, sizing, EMS, PythonPlugin, AirflowNetwork, fenestration, infiltration, broad ExampleFiles, and broad IdealLoads compatibility remain outside current claims. |
