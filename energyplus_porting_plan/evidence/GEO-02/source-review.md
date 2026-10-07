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
These checks establish source/input identity; numerical and lifecycle proof
still requires actual execution.

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

There are 11 planned successful native IDF lifetimes: seven unchanged CON
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

## Preparation status

Input/source preparation and independent pre-run review passed. Native
configure/compile/link passed with the source-preserving GNU configuration;
the [native preparation receipt](native-preparation.json) binds the exact
archived source, binary, verbatim setup fragment, commands, preserved failed
attempts, and independent actual-build review. All 643 original compile rows
and 648 original/prior-card rows remain exact. The 13,537 archive source rows
are all present and exact. A separately preserved correction explains the
historical receipt's erroneous subtraction of two Git metadata absences
outside that archive table; no built source or binary was changed.

Original helper and IDF execution, Rust baseline/final comparison,
production consumer evidence, and all four card gates remain pending. The source-preserving GNU core
retains its documented O0 reference configuration, original assertions and
warnings, and all previously preserved failed build histories.

Existing GEO-01 records supply only already observed coordinate/input
identity and area/azimuth/tilt context. They contain no new centroid or
raw/snapped normal observations. The separate
[input family proof](input-families.json) binds the unchanged A/B geometry
families. Fresh Rust physical proof is planned for A-24H, A-72H, and
B-BOTH-24H with Full/Summary; it does not claim new B-72H or annual physics.
