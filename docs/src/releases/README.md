# Releases

Current release notes record:

- eplus-rs version
- locked EnergyPlus oracle version
- toolchain versions
- verification commands
- supported scope
- known limitations

Only the latest active release note is retained in the working tree. Older
release notes remain available through git history and tags, which keeps the
repository lighter while preserving auditability.

Publishing:

- push an annotated `vX.Y.Z` tag
- `.github/workflows/release.yml` runs the matching current release gate
- the workflow builds `dist/eplus-rs-vX.Y.Z-windows-x64.zip`
- the workflow creates or updates the GitHub Release and uploads the zip asset

`scripts/dev.cmd github-release` remains available only as a local manual fallback
when a token is present.

Release notes:

- [v0.32.0](v0.32.0.md)
