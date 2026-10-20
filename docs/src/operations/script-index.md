---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-10-20
---

# Commands

Run commands from the repository root through the Windows wrapper:

```powershell
.\scripts\dev.cmd list
.\scripts\dev.cmd <command> [args...]
```

PowerShell can also invoke `.\scripts\dev.ps1 <command> [args...]`.
The live `list` output reads the [public command catalog](../../../scripts/dev/commands.json).
For every script's entry point, callers, classification, and artifact hints, use
[Script Inventory](../generated/script-index.md). Artifact hints describe paths;
they do not establish that a run succeeded.

| Work | Choose |
|---|---|
| Prepare this checkout | `setup`; see [Setup](../guides/setup.md). |
| Implement a change | `check -Package ep_runtime -Filter <test_name> -Lib`; select the affected package and test. Use `-List` to find matches or `-Plan` to preview commands. |
| Run tests without lint | `test -Package <package> -Filter <test_name> -Lib`. |
| Integrate a milestone | `check -Full` runs workspace tests and broader guards. See [Developer Workflow](../guides/developer-workflow.md). |
| Edit docs or specs | `docs-generate`, then `docs-check`; update the source registry rather than a generated page. |
| Change script registration | Edit `scripts/dev/commands.json`, regenerate docs, then run `script-inventory-check`. |
| Package or publish evidence | Follow [Release Process](../guides/release-process.md) for the required commands and artifacts. |

Use focused checks during implementation. `check -Full` is the integration gate;
`test` without selectors runs workspace tests once. A filtered run must execute
at least one non-ignored test.

Select comparison commands from the catalog for the relevant case. A conformance
claim requires a case manifest with `comparison_class = "conformance"`,
`conformance_claim = true`, declared variables, tolerances, reports, and a blocking
gate. Smoke and diagnostic commands can help investigate behavior; their results
do not establish compatibility. Release report generators package existing
evidence and do not promote additional variables.

[Documentation Framework](documentation-framework.md) explains the report toolchain.
The generated [machine registry](../../../specs/script_inventory.toml) records script
reachability and inventory metadata; it is maintained from the executable scripts
and public catalog.
