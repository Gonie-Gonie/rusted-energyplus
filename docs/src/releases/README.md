# Releases

Current release notes record:

- eplus-rs version
- locked EnergyPlus oracle version
- toolchain versions
- verification commands
- supported scope
- known limitations

The public release note is retained in the working tree. Internal evidence
milestone notes may also remain when release guards still reference their
claim-boundary text.

Publishing:

- push an annotated `vX.Y.Z` tag
- `.github/workflows/release.yml` runs the matching current release gate
- the workflow builds `dist/eplus-rs-vX.Y.Z-windows-x64.zip`
- the workflow creates or updates the GitHub Release and uploads the zip asset

`scripts/dev.cmd github-release` remains available only as a local manual fallback
when a token is present.

Release notes:

- [v0.1.0](v0.1.0.md)
- [v0.32.0 internal evidence milestone](v0.32.0.md)
