# Fixed Porting Validation Assets

The current scope and results are in [Current Status](../src/current/current-status.md),
state ownership is in [Architecture](../src/current/architecture-overview.md),
and the [verification guide](../src/guides/verify-porting.md) explains how to run
both engines and inspect the comparison files.

This directory retains authenticated inputs rather than a second work plan:

- `cases/`: fixed IDFs, metadata and unit input/weather fixtures.
- `contracts/`: frozen scope, state, source and tolerance contracts.
- `evidence/`: the final fifteen-case validation, SUR-02 supplement and selected
  input/baseline dependencies used by the retained comparison tools.
- `STATE_CONTRACTS.md`: the original source bytes authenticated by
  `contracts/state-contracts.json`; current explanatory prose lives in Architecture.

The JSON, ZIP and fixture bytes are unchanged. Their original
`energyplus_porting_plan/...` references identify historical objects. Tool readers
map that prefix to `docs/validation/` for retained files, while historical policy
checks still read the original Git path. Do not rewrite frozen references to
make a document link look current.

The completed plan, cards, HTML checklist and intermediate evidence remain at
Git commit `b7907009cefd7372727672c6f1ecc211d897a383`. The current evidence packs
also refer to earlier proofs; they do not embed every historical dependency.
Read a historical text object with, for example:

```powershell
git show b7907009cefd7372727672c6f1ecc211d897a383:energyplus_porting_plan/plan.json
```

For an exact historical tree, use a separate checkout of that commit. Original
source-pinned observers and receipts apply to their tested checkout; moving
assets does not issue new scientific receipts. Ignored Native build artifacts
and local execution outputs are separate from the tracked evidence.
