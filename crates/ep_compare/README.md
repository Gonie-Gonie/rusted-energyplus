# ep_compare

## Responsibility

Reads EnergyPlus output artifacts, compares numeric series, applies tolerance
rules, and prepares comparison summaries used by reports.

## Not responsible for

- generating EnergyPlus oracle baselines
- running Rust simulations
- deciding project claim policy
- treating diagnostic output as conformance evidence

## Current claim level

Comparison infrastructure. A pass result supports a claim only when it is tied
to a manifest, tolerance, report, and blocking gate.

## Main modules

- `series`
- `tolerance`
- `eio`
- `eso`
- planned: `summary`, `mtr`, `selected_output`, `report`
