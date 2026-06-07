---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# v2 Full Compatibility Target

v2.0 is the EnergyPlus 26.1 full compatibility target. The goal is broad/full
IDF and epJSON compatibility with the locked EnergyPlus 26.1.0 oracle.

Compatibility mode is the governing mode for v2.0. Structural changes are
allowed, but the compatibility contract still controls public claims.

## Expansion Series

| Range | Focus |
|---|---|
| v1.1 | major object-family parse, typed, default, and validation coverage |
| v1.2 | full envelope physics expansion |
| v1.3 | fenestration and daylighting expansion |
| v1.4 | airflow and zone interaction expansion |
| v1.5 | HVAC system broad coverage |
| v1.6 | plant broad coverage |
| v1.7 | controls, EMS, and PythonPlugin boundaries |
| v1.8 | output, meter, SQL, tabular report, and warning/report parity |
| v1.9 | official testfiles broad regression dashboard |

## v2.0 Requirements

v2.0 requires:

- major object families supported.
- major HVAC and plant systems supported.
- output, meter, SQL, and report systems supported at declared scope.
- large official testfile and ExampleFiles coverage dashboard.
- repeated-run stability.
- no global-state contamination.
- robust diagnostics.
- performance evidence for selected suites, or documented exceptions.

v2.0 is not a license to hide deviations behind performance changes. Any
deviation in compatibility mode still needs a case, variable list, tolerance,
report, gate, and known-gap entry.
