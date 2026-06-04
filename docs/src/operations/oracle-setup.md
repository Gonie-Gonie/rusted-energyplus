# Oracle Setup

The initial oracle is EnergyPlus 26.1.0.

On Windows, `rust-toolchain.toml` pins `1.96.0-x86_64-pc-windows-gnu` for the
early workspace. This avoids making Visual Studio Build Tools a prerequisite
for v0.1.0 setup checks.

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
