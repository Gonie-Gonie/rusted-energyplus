# Porting Map

Every implementation that follows EnergyPlus behavior should record:

- Rust file
- EnergyPlus source file or documentation section
- EnergyPlus version/tag
- observed behavior
- compatibility tests
- known tolerances

Example:

```text
Rust file:
  crates/ep_raw_model/src/lib.rs

Reference:
  .reference/energyplus-src/26.1.0/schema/Energy+.schema.epJSON

Notes:
  RawModel preserves original epJSON object/field names before typed
  normalization.
```

