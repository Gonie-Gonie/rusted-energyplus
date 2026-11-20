# Third-party notices

The project's Rust code is distributed under the BSD-3-Clause terms in
[LICENSE](LICENSE). Third-party code retains its original notices.

## EnergyPlus version 26.1.0

Ported algorithms are adapted from EnergyPlus version 26.1.0, source commit
`6f2e40d10250a105b49966baa24d843711e61048`. The applicable copyright, government
notice, redistribution conditions and disclaimer are retained in
[licenses/EnergyPlus-LICENSE.txt](licenses/EnergyPlus-LICENSE.txt).

The Windows package also includes an unmodified EnergyPlus version 26.1.0
engine and input converter under `oracle/energyplus/26.1.0`. Its `LICENSE.txt`
applies to those files. They provide IDF conversion and an optional independent
reference engine; Rust simulation does not substitute reference-engine results.
This project is independently developed and is not the official distribution.

## RE2

The RE2-derived implementation retains the RE2 Authors' BSD notice in
[licenses/RE2-LICENSE.txt](licenses/RE2-LICENSE.txt), copied unchanged from
`crates/ep_re2/LICENSE.RE2`.

## Rust dependencies and Windows runtimes

Dependency versions and checksums are pinned in `Cargo.lock`. Their respective
license notices accompany the Windows distribution under `licenses/`.
Any bundled native runtime DLLs retain their original licenses and notices.
