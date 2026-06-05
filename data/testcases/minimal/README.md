# Minimal Testcase Area

This directory is reserved for the first repo-owned smoke and regression cases.

For F0 foundation setup, `scripts/dev.cmd oracle-smoke` runs an EnergyPlus bundled example from
the portable oracle package and converts a copied IDF to epJSON. The testcase
manifest is tracked in `case.toml`; generated outputs stay under `.runtime`.

For v0.1.0, `minimal.epJSON` is a repo-owned RawModel inspection fixture. It
intentionally includes `Unknown:Diagnostic` so raw parsing proves that unknown
object types are preserved instead of silently dropped.

For v0.1.0, `typed-model.epJSON` is a repo-owned TypedModel preview fixture
with deliberately omitted defaultable fields. `missing-reference.epJSON` is the
negative preview fixture for missing name reference diagnostics.

For v0.2.0 hardening, `invalid-enum.epJSON` verifies that typed compilation
reports stable enum diagnostics as part of the coverage contract.

Manifests:

- `case.toml`: F0 oracle smoke
- `raw-model.case.toml`: v0.1.0 RawModel inspection
- `typed-model.case.toml`: v0.1.0 preview and v0.2.0 TypedModel contract
