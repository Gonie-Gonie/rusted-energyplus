# Oracle Setup

The initial oracle is EnergyPlus 26.1.0.

On Windows, `rust-toolchain.toml` pins `1.96.0-x86_64-pc-windows-gnu` for the
early workspace. This avoids making Visual Studio Build Tools a prerequisite
for foundation setup checks. `scripts/setup.ps1 -InstallDocsTools` installs
`mdbook 0.5.3` so docs checks can build the book instead of only checking file
structure.

Recommended first run:

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
```

The setup script downloads and verifies the Windows x86_64 release zip using
the SHA256 published by the GitHub release. It extracts the runtime into:

```text
.runtime/energyplus/26.1.0/
```

The reference source archive is downloaded from the `v26.1.0` tag and extracted
into:

```text
.reference/energyplus-src/26.1.0/
```

GitHub generated source archives do not publish release-asset SHA256 digests.
For that archive, `scripts/setup.ps1` locks the tag commit and stores a local
bootstrap digest after first download.

## Smoke Test

Run:

```powershell
.\scripts\oracle-smoke.cmd
```

The smoke test should execute the portable `energyplus.exe` from `.runtime`,
not any EnergyPlus installation on PATH.

## Source Smoke

Run:

```powershell
.\scripts\source-smoke.cmd
```

This verifies that the reference source contains the expected 26.1.0 source
tree, source testfiles, epJSON schema generation tools, and plant/HVAC source
files used by early issue-driven planning. It also verifies that the packaged
runtime contains `Energy+.schema.epJSON`.
