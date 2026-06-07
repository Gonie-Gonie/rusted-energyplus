---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-07
---

# Developer Workflow

Use `scripts/dev.cmd` as the single entry point:

```powershell
.\scripts\dev.cmd list
.\scripts\dev.cmd test
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
.\scripts\dev.cmd file-size-check
.\scripts\dev.cmd check
```

When changing plans, claim boundaries, object coverage, variable coverage, or
algorithm status, update the matching file under `specs/` and regenerate the
generated docs.

When adding a conformance case, update `data/conformance_cases/<case>/case.toml`
and run:

```powershell
.\scripts\dev.cmd manifest-validate-all
.\scripts\dev.cmd strict-no-false-conformance
```

Do not add broad compatibility wording unless a generated report and blocking
gate support the exact case, variable or meter, and tolerance.
