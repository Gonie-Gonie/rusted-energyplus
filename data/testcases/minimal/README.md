# Minimal Testcase Area

This directory is reserved for the first repo-owned smoke and regression cases.

For v0.1.0, `scripts/oracle-smoke.ps1` runs an EnergyPlus bundled example from
the portable oracle package and converts a copied IDF to epJSON. The testcase
manifest is tracked in `case.toml`; generated outputs stay under `.runtime`.

Repo-owned IDF/epJSON cases will be added in v0.2.0 once RawModel parsing
starts.
