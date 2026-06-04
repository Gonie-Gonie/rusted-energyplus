# Foundation Checkpoints

Foundation checkpoints are internal gates, not public semver releases.

## F0 - Reproducible Setup / Oracle

Purpose:

- pin Rust and docs tools
- initialize Cargo workspace
- download and verify the portable EnergyPlus 26.1.0 oracle
- download and verify reference source
- generate `config/local.toml`
- run oracle smoke
- build docs

Verification:

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
.\scripts\check.cmd
.\scripts\oracle-smoke.cmd
```

## Public Version Rule

Public `vX.Y.Z` tags start only when there is a built artifact and a user can
run a visible command from that artifact. Setup-only or docs-only checkpoints do
not receive public version tags.

