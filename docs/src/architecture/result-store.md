---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-07
---

# Result Store

`ResultStore` is the runtime-native container for output series.

The current `first-zone` command writes a diagnostic `ResultStore`. That path
exercises runtime plumbing only and is not heat-balance conformance evidence.

v0.6 keeps this command as developer diagnostic infrastructure. The release
evidence is that native output series can be written and inspected; it is not a
public claim that zone air temperature matches EnergyPlus.

Future conformance cases should write Rust result artifacts from `ResultStore`
or successor output stores and compare them through declared output requests.
