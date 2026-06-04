# Contributing

## Working Rule

Work in meaningful, reviewable units.

For this repository, that means:

- keep each change aligned to one setup, architecture, feature, test, or docs goal
- run the relevant local script before committing
- commit after each meaningful unit is complete
- push after committing when the remote accepts the update
- do not mix unrelated refactors into compatibility or oracle changes
- update docs, porting maps, and supported object coverage when behavior changes

## Local Gates

Use the same scripts locally and in CI:

```powershell
.\scripts\setup.cmd -InstallRust -InstallDocsTools
.\scripts\check.cmd
.\scripts\oracle-smoke.cmd
```

## Commit Message Shape

Prefer concise imperative messages:

```text
Initialize reproducible setup skeleton
Add oracle smoke script
Document Rust-only architecture policy
```
