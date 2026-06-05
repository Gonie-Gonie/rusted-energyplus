# Scripts

Use one command wrapper from the repository root:

```powershell
.\scripts\dev.cmd <command> [args...]
```

Run `.\scripts\dev.cmd list` to see available commands.

The task implementations are grouped by area:

- `setup`: local toolchain, EnergyPlus oracle, and reference source setup
- `quality`: checks, docs, tests, performance, and wording guards
- `smoke`: foundation smoke and diagnostic plumbing checks
- `compare`: EnergyPlus oracle comparison smoke and regression commands
- `conformance`: manifest, baseline, report skeleton, and diagnostic report checks
- `release`: packaging and release verification
- `lib`: shared PowerShell helpers
