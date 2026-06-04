# Releases

Release notes are written before tagging a version and should record:

- eplus-rs version
- locked EnergyPlus oracle version
- toolchain versions
- verification commands
- supported scope
- known limitations

Public version tags start only when the repository can build a distributable
artifact with at least one user-visible runnable command. Foundation setup
checkpoints are documented separately and do not receive semver tags.

Publishing:

- push an annotated `vX.Y.Z` tag
- `.github/workflows/release.yml` runs the matching `scripts\vX.Y-verify.ps1`
- the workflow builds `dist/eplus-rs-vX.Y.Z-windows-x64.zip`
- the workflow creates or updates the GitHub Release and uploads the zip asset

`scripts\github-release.cmd` remains available only as a local manual fallback
when a token is present.

Release notes:

- [v0.1.0](v0.1.0.md)
