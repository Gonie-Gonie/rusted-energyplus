---
status: active
claim_level: none
owner: release
last_reviewed: 2026-11-20
---

# Current status

Version **0.2.0** packages the verified, bounded Rust Source scope against the
locked **EnergyPlus 26.1.0** reference. This release concentrates on explicit
support boundaries, usable documentation and a checked Windows package.
Further object/parameter expansion is deferred.

The algorithm baseline is the published implementation through **Source212**.
Unfinished active-Window porting and its unpublished validation tools are
excluded. Release/version, documentation and packaging changes do not create
new numerical conformance evidence.

## What can run

| Select | Boundary |
| --- | --- |
| `--porting-scope A` | Bounded opaque unconditioned heat balance, with separately reviewed geometry, multizone, Space/list and gains extensions. |
| `--porting-scope B` | Bounded direct-Zone IdealLoads, with selected hard limits, humidity, outdoor-air, economizer and heat-recovery combinations. |
| No scope flag | Older generic/diagnostic execution with separate capability and comparison behavior. |

The Windows launcher uses the older generic route. Use the CLI for Source A/B.
The [supported-input guide](../guides/supported-inputs.md) and
[detailed object registry](../generated/object-coverage.md) define the actual
field, value, reference, topology, algorithm and consumer limits. Run
`--dry-run` and inspect the support report for a complete new input.

Active windows, general HVAC/Plant, sizing, EMS, PythonPlugin and AirflowNetwork
remain outside ordinary Source release support. A recognized/typed object or
an unused material definition does not establish an active simulation consumer.
No arbitrary-valid-value or full-object completion is claimed.

## Retained evidence

The exact case list is generated in `docs/src/generated/conformance-case-index.md`.
Algorithm, object and coverage boundaries are generated from
`specs/algorithm_ledger.toml`, `specs/object_coverage.toml`, and
`specs/variable_coverage.toml`.

| Archive | Accepted cases | Reported values | Result |
| --- | ---: | ---: | --- |
| Fixed CON-01 | 15 | 16,427,880 | Zero declared tolerance/RMSE failures. |
| Published input-domain expansion through Source212 | 284 | 13,276,216 | Zero declared tolerance/RMSE failures. |

These archives have different purposes and are not added into a unique total.
Whole-file byte identity, all valid parameter combinations and general
EnergyPlus replacement behavior are separate, unestablished claims.

The [fixed final validation](../../validation/evidence/SRC-06-final-current-source-validation-60/summary.json),
[SUR-02 supplement](../../validation/evidence/SUR-02-isolated-history-impulse-61/summary.json)
and [input-domain archive](../../validation/evidence/object-input-domains.json)
retain their policies, results and limitations. Historical local `.runtime/`
receipts and executables are not present in a fresh clone. Their recorded
paths are provenance, not a promise that every historical run can be replayed
without obtaining the corresponding retained assets.

## Complete object inventory

The [generated list](../generated/example-object-union.md) covers **188 input
files**, **110 occurring object types** and **9,118 instances**. Corpus labels
are 47 bounded A/B, 27 bounded B-only and 36 blocked. Adding separately
reviewed published types yields 123 names; the full registry has 147 entries.
The historical 116-name checkpoint in the evidence remains historical.
None of these inventories is a count of fully supported objects.

## Output claim classification

{{#include ../generated/current-status-classification.md}}

{{#include ../generated/variable-coverage.md:current-status-variable-summary}}

## Run, inspect and compare

Follow [quick start](../../../QUICKSTART.md), [run an input](../guides/run-arbitrary-idf.md)
and [verify results](../guides/verify-porting.md). Source runs expose ESO/MTR,
JSON/CSV reported results, support reports, diagnostics and run summaries.
Ad-hoc runs keep `conformance_claim=false`; unsupported inputs do not silently
substitute oracle results.

Source A/B automatic `--oracle-baseline`/`--compare-oracle` integration remains
unconnected. Use separate same-input engine runs as described in the guide.
The older generic route's comparison options do not certify the Source engine.

The [release process](../guides/release-process.md) verifies the extracted
Windows package, including an existing accepted example and a blocked input.
That delivery check does not expand numerical support.

## Older generic route

The retained legacy capability registry records official `1ZoneUncontrolled`
dynamic source-order compatibility rows as declared-variable compatibility only.
The current compatibility-mode arbitrary runtime covers the official
`1ZoneUncontrolled` heat-balance path and selected no-OA IdealLoads limit and
humidity branches. In that older generic route, IdealLoads outdoor-air,
economizer, heat-recovery and broader missing combinations remain outside
arbitrary-run compatibility. The separately reviewed Source B extensions do
not change the legacy selector.
