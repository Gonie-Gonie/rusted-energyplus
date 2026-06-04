# Rust-Only Policy

Production implementation language is Rust.

EnergyPlus C++ source is a reference and oracle source only. It must not be
compiled into the Rust port, linked as a native kernel, or mechanically
translated line-by-line into production code.

## Allowed

- safe Rust implementation
- Rust data-structure improvements
- explicit caches and execution plans
- deterministic scalar compatibility mode
- tightly documented `unsafe Rust` only in future fast-mode hot paths, with
  benchmark evidence and scalar compatibility tests

## Not Allowed

- C, C++, Fortran, or assembly kernels in production simulation code
- native numerical kernels used to bypass Rust implementation work
- runtime object-name string lookup in timestep simulation paths
- silent fallback to an EnergyPlus executable for Rust-engine results

## Reference Source Use

Reference files live under:

```text
.reference/energyplus-src/26.1.0/
```

Use them to confirm algorithms, output variable names, test files, and issue
cases. Record every porting decision in `docs/src/porting-map/`.

