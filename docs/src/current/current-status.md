---
status: active
claim_level: none
owner: core
last_reviewed: 2026-10-20
---

# Current Status

## Fixed Porting Scope

The [fixed CON-01 inputs](../../validation/contracts/scope.json) have completed
their scoped validation: A is an opaque unconditioned one-zone model; B adds
direct no-outdoor-air sensible IdealLoads with no limit or hard-sized flow,
capacity, or combined limits. The original work covered 54 applicable units for
A and 64 for B; their accepted adjudications remain in the validation evidence.

The fifteen cases cover 24 hours, 72 hours and a year for A and each of B's
four limit modes. They use 2013 weather-only simulation at four zone timesteps
per hour. B's hard-sized limits are 0.02 m3/s and 120 W; its availability is Off
for the first three hours each day and On for the remaining twenty-one. The
input contract excludes windows, active outdoor-air paths, infiltration/AFN,
Plant, EMS, equipment autosizing, and multiple zones/equipment.

The [final validation](../../validation/evidence/SRC-06-final-current-source-validation-60/summary.json)
records fifteen normal runs, 16,427,880 reported values with zero tolerance or
RMSE failures, real Surface/warmup handoffs, and Rust JSON/CSV output rereads.
The [SUR-02 supplement](../../validation/evidence/SUR-02-isolated-history-impulse-61/summary.json)
adds the previously missing executed impulse/current-row unit checks.
File-wide byte equality is not established; headers, elapsed times and some
inactive registry counters differ. Original abnormal afterstates remain
unavailable. These finite results do not establish general EnergyPlus parity.
The [validation assets](../../validation/README.md) retain the current inputs,
policies and accepted evidence. The completed task packet is historical material
at Git commit `b7907009cefd7372727672c6f1ecc211d897a383`.

## Running And Comparing

The current compatibility-mode arbitrary runtime covers the official
`1ZoneUncontrolled` heat-balance path and the declared direct-Zone no-OA
sensible PurchasedAir binding. The completed porting route is selected with
`--porting-scope A|B` and writes its own ESO/MTR and JSON/CSV results.

**Automatic oracle comparison is not connected to this Source route.**
Adding `--oracle-baseline` or `--compare-oracle` retains the Rust output but
returns `SourceReportedOracleComparisonUnsupported` (exit 4). Run the two
engines separately and use the fixed-case checker described in
[Verify the port](../guides/verify-porting.md).

Ad-hoc and launcher runs keep `conformance_claim=false`. Unsupported inputs
are blocked; explicitly allowed diagnostic partial runs remain diagnostic.
See [Run an IDF](../guides/run-arbitrary-idf.md) for artifacts and exit codes,
and [Roadmap](roadmap.md) for the remaining user-facing work.

## Current Conformance Claims

Current conformance evidence is limited to promoted manifests and declared
variables/meters. The exact case list is generated in
`docs/src/generated/conformance-case-index.md`. Algorithm and coverage boundaries
are generated from `specs/algorithm_ledger.toml`, `specs/object_coverage.toml`,
and `specs/variable_coverage.toml`.

- official `1ZoneUncontrolled` dynamic source-order compatibility rows are
  declared-variable compatibility only.
- IdealLoads no-OA/no-limit sensible, numeric capacity limit, numeric flow
  limit, flow-and-capacity limit, ConstantSensibleHeatRatio,
  ConstantSupplyHumidityRatio, selected Humidistat, selected outdoor-air,
  economizer, heat-recovery and reporting evidence remain case-scoped;
  they do not establish full IdealLoads compatibility.

{{#include ../generated/current-status-classification.md}}

{{#include ../generated/variable-coverage.md:current-status-variable-summary}}

Diagnostic-only evidence remains useful for source-order porting but does not
create compatibility claims. Historical diagnostic selectors and replay
wrappers are tracked in `specs/diagnostic_probe_ledger.toml`, outside the
default test suite. They are not additional finite-port production evidence.

## Not Claimed

General EnergyPlus replacement, broad heat-balance compatibility,
broad HVAC/node/meter compatibility, Plant, sizing, EMS, PythonPlugin, AFN,
fenestration, infiltration and broad ExampleFiles coverage are not claimed.
There is no broad IdealLoads branch coverage beyond the declared cases.
IdealLoads outdoor-air, economizer, heat-recovery and broader humidity branches
remain outside arbitrary-run compatibility even when individual release cases
have separate evidence.

Routine/domain readiness is owned by the algorithm ledger and project contract;
input support is owned by the capability registry. A finite unit pass, parser
support or successful demo does not promote those registries. The
[project contract](project-contract.md) defines the promotion requirements.
