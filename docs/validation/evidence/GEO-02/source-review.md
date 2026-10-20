# GEO-02 bounded original producer and state review

The selected original source is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`. The original detailed opaque
surface producer is `SurfaceGeometry::GetVertices`, followed by
`CalcSurfaceCentroid`. `ProcessSurfaceVertices` consumes their existing
geometry and produces shape/shading coordinates. The card's producer-name
correction does not close GEO-03, SRC-04, SRC-05, or whole simulation physics.

The frozen input-only contracts are
[source](../../contracts/GEO-02-source.json),
[cases](../../contracts/GEO-02-cases.json), and
[tolerances](../../contracts/GEO-02-tolerances.json).
Their hashes are respectively
`68af6b2b164832f110d1fa308688463790ab899d9fae77565661cf3f56bbeb09`,
`452d910b6afb81a1dcbf23b43d75d50e74c59da5d7e372ff45424511dc6cf435`, and
`3876a34a85f84dc459d1fec406106c0166ef0ff1dd6942beb0d3a7fbcb19a0e1`.
The [independent pre-run review](independent-contract-review.json) verifies
16 original files, 46 exact source ranges, the actual input patches, and
the absence of a dynamic `Site:VariableLocation` orientation producer.
These pre-run checks establish source/input identity. The committed unit and
bounded physical evidence described below supplies the separate execution proof.

## Producers and order

`SetupZoneGeometry:273–289` prepares building/zone trigonometric state.
`GetSurfaceData` calls `GetVertices`, then calls `CalcSurfaceCentroid:2591`.
After `GetSurfaceData` returns, `SetupZoneGeometry` deallocates zone
trigonometric arrays and calls `ProcessSurfaceVertices`. A passive native
observer reads first initialized, first physical, final weather, and final
retained state through the unchanged same-GCC API DLL. It adds no geometry
calls, model actuation, or state reset.

`GetVertices:9383–9457` stores cyclic Newell normal, ordered fan Newell area
vector, gross/net area, local coordinate system, azimuth/tilt, trigonometric
coefficients, and independently component-snapped `OutNormVec`. The raw
Newell normal and snapped consumer normal are distinct fields. Angles use
the raw normal; current and period solar incidence read their existing
normal operands. No observer recomputes geometry or solar answers.

`CalcSurfaceCentroid:14544–14597` weights triangle centroids against its own
stored gross area. The alternate split is selected only when the original
area fractions sum above 1.05. Nonpositive gross area warns and continues
without storing a new centroid. `ProcessSurfaceVertices` consumes these
fields, allocates scratch on its first call, stores shape/shading
coordinates, and subsequently returns early when `VerticesProcessed` is
already true.

## Input and state boundaries

There are 11 successful native IDF lifetimes: seven unchanged CON
cases plus four valid diagnostic vertex patches. Two separately frozen
ordinary negative IDFs contain a collinear or coincident first wall. Their
actual original nonzero exit, error logs, and zero physical callbacks must
be observed; unavailable initialization snapshots remain null. The
negative inputs do not expand CON admission or imply equal error messages.

The prepared-state helper request has 15 valid quad rows and two source-only
degenerate rows. Valid rows prepare real original owners and declared
one-zone/base-surface inputs, include the byte-exact original setup fragment,
then call unchanged original `GetVertices`, copy its own temporary surface
to its own retained surface, call `CalcSurfaceCentroid`, and call
`ProcessSurfaceVertices` twice. This is not whole-parser admission or the
complete `SetupZoneGeometry` lifecycle. Globals, scratch, shape, and warning
aggregation without corresponding Rust owners remain unpaired.

Valid raw World inputs contain positive geometric zeros only. Original
post-`GetVertices` ordered coordinate bits must equal the frozen raw request
bits before Rust's direct geometry route is admitted for comparison. Rust
must independently retain its own input bits. A mismatch blocks that pairing
and requires an unchanged-input compiler route; original outputs must never
be supplied as Rust inputs. Coincident cleanup, aspect transformations,
roof/floor automatic reversal, and vertex-count changes must be inactive.

The two unsafe helper rows bypass `GetVertices` and `ProcessSurfaceVertices`.
They invoke genuine plane/normal/area helpers and centroid retention with
explicit prepared values. Their warnings and retained centroid are source
diagnostics, not a claim about full parser rejection. Unwritten nondebug
`Vector3` buffers are observed only through allocation/dimension metadata;
values are copied after actual source assignments.

## Centroid precision

The unchanged `ObjexxFCL::cen` header sums the three binary64 operands in
order, multiplies by a `long double` constant initialized from the binary64
expression `1.0 / 3.0`, then converts to binary64. There are 526 independent
input-only probes: 11 fixed finite edges, 512 deterministic integer-PRNG
significand/exponent inputs, and three signed-zero/cancellation operand
triples. No reference result search defines these inputs.

The wrapper reads type sizes, mantissa sizes, FE rounding, the actual x87
control word (precision and rounding control), and MXCSR rounding before
and after unchanged-header calls and at native initialization snapshots.
It never changes floating-point controls. `LDBL_MANT_DIG` alone cannot
establish the active x87 multiplication precision. The paired cen output
policy is exact bits under the actually observed original configuration;
the other field profiles use their frozen dimensional tolerances and
exact class/zero-sign rules.

## Original-first and implementation evidence

Input/source preparation and independent pre-run review passed. Native
configure/compile/link passed with the source-preserving GNU configuration;
the [native preparation receipt](native-preparation.json) binds the exact
archived source, binary, verbatim setup fragment, commands, preserved failed
attempts, and independent actual-build review. All 643 original compile rows
and 648 original/prior-card rows remain exact. The 13,537 archive source rows
are all present and exact. A separately preserved correction explains the
historical receipt's erroneous subtraction of two Git metadata absences
outside that archive table; no built source or binary was changed.

Original-first execution completed: 15 valid and two source-only helper
quadrilaterals, 526 unchanged-header centroid calls, and 13 ordinary IDFs
(11 valid and two rejected inputs). The [original observations receipt](source-original-first.json)
binds raw commands, inputs, state snapshots and errors. Valid helper retained
coordinate bits equal the frozen request; valid original IDF geometry and
centroids remain unchanged through final return. The 11 valid runs observed
1,440 physical zone callbacks. Both ordinary invalid inputs exited before
initialized geometry or physical callbacks. The separate unsafe helper rows
warned and retained their supplied prior centroids.

Actual helper and initialized original lifetimes observed x87 precision 64
and round-to-nearest without observer control writes. The two named
horizontal-epsilon probes both selected the near-horizontal branch after
normalization; their labels do not establish opposite sides of that threshold.
Other nonhorizontal valid inputs exercise the ordinary branch. Frozen inputs
and tolerances remain unchanged.

The preserved existing Rust baseline has 729 mismatches, including 721
missing geometry/centroid results and eight numerical differences. The
canonical precommit candidate has zero mismatches in 3,182 checks. The
[baseline](baseline-comparison.json) and [candidate](candidate-comparison.json)
records retain their original boundaries, including the candidate's dirty-source
build. The source-preserving GNU core retains its documented O0 reference
configuration, original assertions and warnings, and preserved failed builds.

The final unit example and normal CLI are independently archived builds of
committed implementation `edb6f57822b319692b0edaa30bc276a66bd3319e`.
Their archived available-source inventories contain 3,646 matching Rust files,
including files not selected for each binary. Against committed Git blobs,
3,645 archived files are byte-exact and one unchanged historical file has CRLF
bytes with exact LF-normalized content. These inventory comparisons do not
claim every archived file was compiled. Actual Cargo commands, archived
executables, and scientific-owner source review supply the separate build proof.
The [final unit evidence](final-unit-comparison.json) records zero mismatches
in 3,182 checks: 15 valid quads, 450 paired geometry scalar components, and
526 exact three-operand centroid results. All paired scalar components and
centroid results are bit-identical to the independently executed original.
The two unsafe helper rows remain source-only and unpaired.

Rust reuses the existing geometry entry points through one canonical
`geometry/source_geometry.rs` owner. It preserves the selected producer order,
plain binary64 source grouping, raw-angle cleanup, separate snapped normal,
and stored-gross-area centroid weighting. The centroid precision helper uses
the observed 64-bit product precision without writing floating-point controls.
Initialization assigns one geometry bundle to each actual surface state;
temperature/wind height, convection orientation, solar orientation, and area
consumers read that owned state. Report recomputation suspends only the optional
geometry collector. Neither observers nor comparison tools supply geometry
answers to production calculations.

The first full workspace test run found eight regressions in older tests
whose hand-built horizontal surface winding disagreed with their assumed
floor/roof direction. The shared cube fixture remains unchanged. Geometry
tests now assert its actual Floor +Z / Roof -Z normals. Independent
convection, longwave, solar and conduction tests explicitly prepare the
opposite horizontal winding required by their existing physical assertions.
Their numerical constants and tolerances are unchanged. This test-only
correction does not change the frozen CON or diagnostic inputs, the
canonical geometry calculation, or any original result.

Existing GEO-01 records supply only already observed coordinate/input
identity and area/azimuth/tilt context. They contain no new centroid or
raw/snapped normal observations. The separate
[input family proof](input-families.json) binds the unchanged A/B geometry
families.

## Bounded physical connection and closure

The [production evidence](production-comparison.json) records 72,555 checks
with zero differences for fresh A-24H, A-72H, and B-BOTH-24H Full/Summary runs.
The three Full snapshots contain 540 geometry scalar components across six
surfaces per case, all bit-identical to matching original stored geometry.
These are stored-state observations, not geometry-kernel invocation counts.
The observer retains 23,232 ordered operand events without omissions and
binds their actual contexts to 480 physical zone-loop invocations. Eighteen
unscoped initialization area events are identified separately.

Temperature-height and area consumers cover every actual interval. Other
consumers follow the existing conditional exterior balance and input exposure
branches, as recorded below; no additional calls are made for coverage.

| Actual case | Zone intervals | Wind-height / convection occupied intervals | Solar occupied intervals |
|---|---:|---:|---:|
| A-24H | 96 | 14 / 14 | 37 |
| A-72H | 288 | 157 / 157 | 177 |
| B-BOTH-24H | 96 | 0 / 96 | 0 |

B's frozen surfaces have `NoWind` and `NoSun`. Their absent wind-height and
solar events are verified inactive branches, not numerical validation of those
calculations. Solar events in A copy actual orientation arguments only when
the existing solar path has a real position. Rust still consumes angles rather
than a source outward-normal dot product; no such dot-product parity is claimed.

All reported geometry/consumed-field maximum errors and per-field RMSE are
zero under unchanged dimensional profiles and exact class/zero-sign rules.
Summary emits no Full geometry observer files. Its ordinary series, CSVs,
result store, and run metadata agree with Full; Summary does not directly
observe all sixteen final geometry fields. The [actual command record](actual-commands.json)
retains one helper execution, six successful physical executions, and two
ordinary-input failures. Collinear and coincident first walls produce actual
Rust runtime exit 6 before physical events or result data, while the original
exits 1. Error-code/text equality is not claimed.

Three preserved production reader failures concerned a contract selector,
copied-source path identity, and raw-token versus typed-enum spelling. Their
raw receipts and reader archives remain unchanged. The corrected reader
reused the same engine results, inputs, implementation, and frozen tolerances;
its successful identity is recorded separately from the earlier static review.
Independent final unit and production readers verified the retained data.

This evidence closes GEO-02's selected A/B producer and state-connection
scope. It does not close GEO-03 volume calculation, SRC-04 solar timing or
incidence output, SRC-05 atmospheric equations, native scratch/global/error
parity, other surface families, or whole simulation physics. Fresh Rust
physical coverage is limited to the three listed cases; no new per-limit,
B-72H, or annual physical claim is inferred from the input family proof.
