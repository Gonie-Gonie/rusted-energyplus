---
status: active
claim_level: none
owner: core
last_reviewed: 2026-10-28
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

The [example-object union](../generated/example-object-union.md) contains 111
object types; legal-value completion remains tracked per object. The retained
[input-domain evidence](../../validation/evidence/object-input-domains.json)
contains 143 paired physical cases and 3,727,446 reported values with zero
tolerance or RMSE failures. These cases span recorded Source builds; they do
not establish arbitrary legal object/value combinations.

Direct Source epJSON retains the first comparator-equivalent member spelling
and last whole subtree, checked by two Native convert-only observations and
existing loader assertions. `Schedule:File` comparisons cover hourly and
20-minute data on February 29, distinct leap-day values, per-file daylight-saving
controls and the next-day handoff. That controlled weather copies February 28
meteorology; full schedule-domain completion remains open.

Within the bounded one-Zone A/B route, `Zone.multiplier` accepts the Native
signed owner's positive range, `1..=2147483647`. The actual getter, aggregate
demand, air balance, meters and reports retain the live factor; per-Zone
intensive state keeps its original meaning. Group/list factors remain one.
Two multiplier-2 models compare their complete original profiles: 15,144 values
have zero tolerance or RMSE failures, with exact Zone Information records,
dictionaries and clocks. This finite evidence retains the other model limits.

The opaque Source route accepts all four `BuildingSurface:Detailed` class
choices: Wall, Floor, Roof and Ceiling. Raw/typed Ceiling remains visible while
the actual getter assigns its Roof class. Surface/full tracing now exports
the retained current Source geometry after successful completion. Two full-profile
Ceiling comparisons pass the original policies; their EIO bodies and DXF geometry
agree within the explicitly recorded version-header and line-ending boundaries.

Explicit Source A also accepts multiple closed opaque Zones, each with its
implicit Space. Direct OtherEquipment can target each Zone with all thirteen
non-Water fuels or thermal-only None and genuine built-in or authored schedules.
Direct People supports all three sizing
methods with live count, sensible/latent/radiant gains and humidity consumers.
Source typed OtherEquipment names preserve authored whitespace with ASCII case
folding. A direct-epJSON public-output witness distinguishes "A", " A ",
and "A " under unchanged numeric policies. Constant's missing-type-limit
warning uses the input format's field title.
HVAC and remaining gain families remain outside this increment. Reciprocal interzone Wall–Wall
and Roof–Floor pairs accept normal opaque stacks of one to ten layers, including
the getter's Ceiling alias. Exact reverse Material
identity retains the Native fast path; differing counts or layer identities use
an actual Source nominal-U difference of at most 0.001. Equal-count mismatches
retain Native's warning sequence. Infrared-transparent constructions retain
their sole-layer exact-reverse branch on Wall pairs. Pairs require equal Zone
factors and opposite coincident vertices. Fifteen 24-hour two-Zone
cases cover 216,792 derived values with zero tolerance or RMSE failures;
the maximum absolute difference is below 1.83e-11. Actual Zone/Space and enclosure links,
EIO, diagnostics and DXF agree within their recorded boundaries. This separate
class inherits no one-Zone capabilities or conformance promotion. Explicit
Spaces, groups, multi-Zone HVAC and broader interzone combinations remain open.
Source closure can complete collinear edge points in a temporary copy while
retaining authored Surface vertices and volume inputs. Open-zone fallbacks remain held.
Actual device and Zone W/J sums agree with all thirteen Facility Hourly meters;
None contributes heat without a resource report or meter.

The reversed two-layer and unequal two/three-layer cases also match all emitted
CTF records. The latter reads real nominal-U array slots by Construction number.
The reversed two-layer case's original 30-day warmup allowance failed in Native;
a separate input changes only that allowance to 100 days and converges after 32,
with unchanged convergence tolerances. The original failed reference remains
unaccepted.

Paired Walls accept the legal Sun/Wind exposure enums, omission and empty defaults.
IDF and direct epJSON comparisons preserve their distinct warning labels and
surface order; non-external Sun warnings and paired Wind clearing agree with
Native. Those two exposure cases finish with both flags false; raw authored fields
retain their presence and values. The existing input table covers all sixteen
presence/value combinations without adding test functions.

The two-pair construction-warning case also checks repeated warning suppression.
Both real Construction order-warning flags and the invocation-local hint follow
Native's record order. Reusing the warned constructions skips the later pair's
exposure clearing, so its actual WindExposed flags remain set. Internal Rust
path bookkeeping is separate from the observed scientific owner fields.

`Material:InfraredTransparent` now reaches the ordered Source A/B getter and
single-layer CTF on Ground or Adiabatic boundaries, preserving the Native
warnings and Invalid roughness. Ground and Adiabatic B comparisons plus proper
reciprocal interzone A pairs cover 49,824 values with zero tolerance or RMSE
failures. Adiabatic values are raw-exact; Ground's maximum absolute difference
is 4.66e-10. Exterior roughness lookups, Materials reports and used-IRT
Constructions reports remain excluded. Multi-Zone Ground/Adiabatic IRT uses,
CondFD and broader object combinations remain open.

Reporting key matching uses the [Rust RE2 port](../../../crates/ep_re2/README.md)
at pre-scan, request construction and live registration. Eager OnePass analysis,
a persistent anchored-longest DFA and the original-context NFA fallback use
independently checked official-binary accounting. Component comparisons retain
exact wire results, including resource failures and cache resets. Physical A/B
regex cases cover 160 series and 15,144 values under the original policy.
Two further 24-hour cases with a 100,000-byte Zone name cover 192 values and
reproduce the failed-DFA diagnostics before successful matching. Their complete
dictionaries and ordered timestamps agree; full-file equality is not claimed.

`MINREPORTFREQUENCY` now reaches the shared variable/meter frequency parser
through real reporting owners. It preserves first-call ordering, raw input
spelling, abbreviation warnings and EIO records. Two same-input 24-hour cases
with `Hourly` and `hour` compare 192 values across one variable and three meters;
the maximum absolute difference is 8.66e-15 with zero tolerance or RMSE failures.
Their dictionaries, clocks, override records and frequency diagnostics agree.
Other frequency overrides have source and existing-table coverage; this physical
comparison covers Hourly only. Unbound producers, unsupported output backends
and general legal-value completion remain open.

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
