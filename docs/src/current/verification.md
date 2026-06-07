---
status: active
claim_level: none
owner: qa
last_reviewed: 2026-06-07
---

# Verification

The standard local gate is:

```powershell
.\scripts\dev.cmd check
```

Documentation and spec references are maintained with:

```powershell
.\scripts\dev.cmd docs-generate
.\scripts\dev.cmd docs-check
```

Case manifest schema v2 is checked with:

```powershell
.\scripts\dev.cmd manifest-validate-all
```

The false-claim guard is:

```powershell
.\scripts\dev.cmd strict-no-false-conformance
```

Release evidence documents use the repo-local Python environment and oodocs:

```powershell
.\scripts\dev.cmd conformance-evidence-report -Version 0.17.0
```

Numerical conformance requires a generated report plus a blocking gate. Smoke
or diagnostic commands can support development, but they cannot support a
compatibility claim.

Frozen release evidence is published as GitHub Release assets. The local
`.runtime/release-evidence` directory is a staging area, not the long-term
evidence store.
