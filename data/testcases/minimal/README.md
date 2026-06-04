# Minimal Testcase Area

This directory is reserved for the first repo-owned smoke and regression cases.

For v0.1.0, `scripts/oracle-smoke.ps1` runs an EnergyPlus bundled example from
the portable oracle package and converts a copied IDF to epJSON. The testcase
manifest is tracked in `case.toml`; generated outputs stay under `.runtime`.

Repo-owned IDF/epJSON cases will be added in v0.2.0 once RawModel parsing
starts.

For v0.2.0, `minimal.epJSON` is a repo-owned RawModel inspection fixture. It
intentionally includes `Unknown:Diagnostic` so raw parsing proves that unknown
object types are preserved instead of silently dropped.

Manifests:

- `case.toml`: v0.1 oracle smoke
- `raw-model.case.toml`: v0.2 RawModel inspection
