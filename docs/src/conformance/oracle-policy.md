---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Oracle Policy

The initial oracle is EnergyPlus 26.1.0. The repository uses repo-local oracle
and source directories:

```text
.runtime/energyplus/26.1.0
.reference/energyplus-src/26.1.0
```

Conformance cases must state the oracle version. A case cannot silently switch
to a globally installed EnergyPlus executable.

Reference source is used for porting maps and algorithm interpretation. The
oracle executable is used for baseline outputs.

