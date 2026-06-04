# Changelog

## v0.1.0 - 2026-06-04

Reproducible setup / oracle release.

### Added

- Rust workspace skeleton for the initial architecture boundary.
- Rust-only implementation policy.
- Windows setup scripts with `.cmd` wrappers.
- Rust `1.96.0-x86_64-pc-windows-gnu` toolchain pin.
- mdBook `0.5.3` docs tool setup.
- EnergyPlus 26.1.0 portable oracle lock and SHA256 verification.
- EnergyPlus 26.1.0 reference source download and bootstrap digest.
- Oracle smoke test using `1ZoneUncontrolled.idf`.
- IDF to epJSON conversion smoke through `ConvertInputFormat.exe`.
- Source smoke test for reference source, packaged schema, and early plant/HVAC reference files.
- v0.1 verification gate.
- Copied development plan v2 and initial docs book.

### Notes

- v0.1.0 does not implement simulation behavior in Rust yet.
- v0.2.0 starts RawModel and epJSON inspection.

