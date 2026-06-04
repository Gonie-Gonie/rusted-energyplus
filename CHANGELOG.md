# Changelog

## v0.2.0 - 2026-06-04

RawModel / epJSON inspection.

### Added

- epJSON loader in `ep_raw_model`.
- Raw value preservation for strings, booleans, nulls, numbers, arrays, and nested objects.
- RawModel summary with version, object type count, object count, and per-type counts.
- `eplus-rs model inspect <input.epJSON>` CLI command.
- Seed tracked/untracked object reporting in model inspection.
- v0.2 verification gate.
- v0.2 readiness and release documentation.

### Notes

- v0.2.0 does not perform schema validation or typed conversion yet.
- v0.3.0 starts TypedModel / Reference Resolution.

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
