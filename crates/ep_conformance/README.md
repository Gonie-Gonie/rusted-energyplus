# ep_conformance

## Responsibility

Defines conformance case manifests, output requests, tolerance rules, report
contracts, gates, suites, schema v2 metadata, and validation behavior.

## Not responsible for

- executing EnergyPlus
- running Rust runtime algorithms
- parsing ESO/EIO/MTR numeric series
- broad compatibility wording outside declared evidence

## Current claim level

Policy and schema infrastructure. It can validate that a case is allowed to
claim conformance, but the claim still requires generated evidence artifacts.

## Main modules

- `conformance`
- `tests`
- planned: `case`, `output_request`, `tolerance`, `report_contract`, `gate`,
  `suite`, `validation`
