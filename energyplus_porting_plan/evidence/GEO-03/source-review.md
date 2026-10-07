# GEO-03 bounded source review

This is a preparation review. No GEO-03 reference or Rust numerical comparison
has run, and all four gates remain pending. The original pin is EnergyPlus
26.1.0, `6f2e40d10250a105b49966baa24d843711e61048`.

The source, input and tolerance drafts are
[`GEO-03-source.json`](../../contracts/GEO-03-source.json),
[`GEO-03-cases.json`](../../contracts/GEO-03-cases.json), and
[`GEO-03-tolerances.json`](../../contracts/GEO-03-tolerances.json).
The source contract records whole-file hashes and exact byte-range hashes for
the selected owner, immediate helpers, declarations and incoming geometry.
It must pass Root and independent review before any original-first execution.

## Owner and call order

The owner is `SurfaceGeometry::CalculateZoneVolume`,
`SurfaceGeometry.cc:12010–12322`. It is called by `SetupZoneGeometry:548` after
the GEO-02 surface geometry and the caller's area/height preparation.

`HeatBalanceManager.cc:2333–2340` writes the Zone numeric ceiling height, volume
and user floor area. It does not set `ceilingHeightEntered`.
`GetSurfaceData:2302–2456` resolves Zone/Space floor areas, ceiling area and
floor/roof flags. `SetupZoneGeometry:376–391` separately accumulates geometric
floor/ceiling gross areas. Its `455–546` height loop sets the entered-height
flag when the current height is positive, then assigns an automatic height
only if the prior height is nonpositive. A positive calculated final height
does not imply an entered height.

The height loop uses area-weighted midpoints of each roof/floor's minimum and
maximum vertex z. It falls back to wall maximum-minus-minimum height where
required. An axis-aligned bounding-box height happens to agree for the selected
flat rectangular prisms; it is not the source's general height algorithm.

`CalculateZoneVolume` constructs each Wall/Floor/Roof face in its actual stored
order, computes its Newell area vector, and evaluates horizontal/vertical,
wall-height and enclosure predicates before deciding the final priority.
Positive entered volume does not bypass those evaluations.

Normal input has a further order dependency. `BaseSurfIDs` declares
Wall/Floor/Roof order (`SurfaceGeometry.hh:545–558`), and
`GetSurfaceData:1504–1530` moves those classes in that order while retaining
their `SurfaceTmp` ordinal within each Space. `GetHTSurfaceData:3889–3915`
obtains successive instances through `getObjectItem`. For ordinary IDF input,
`InputProcessor::getJSONObjNum:1216–1259` maps the saved `idf_order` when
`isEpJSON` is false and `preserveIDFOrder` is true. Native flags and actual
ordered surfaces must be observed. Rust's alphabetical TypedModel order and
its independent IDs cannot stand in for this source volume sum order.
The prepared helper route deliberately retains the explicit request face
order because it does not execute the normal parser's class reordering.

The initial-closed branch calls `Vectors::CalcPolyhedronVolume:505–529`:
each ordered term is `dot(NewellAreaVector, FacePoints(2) - actual p0) / 3.0`,
added to a signed binary64 sum. The selected actual `VectorsData` constructor
sets `p0` to zero and the owner does not write it. Face points are already world
coordinates; applying the Zone origin again or shifting them to a convenient
reference point would change the source arithmetic. This `/3.0` is unrelated
to the extended-product third used for GEO-02 centroids.

Final priority at `12193–12233` is:

1. Retain a current positive `Volume`.
2. Otherwise, if `ceilingHeightEntered` and selected floor area are positive,
   use selected floor area times current ceiling height.
3. Otherwise use the calculated volume.

Selected floor area is positive `FloorArea`, or else `geometricFloorArea`.
The original can warn and replace a nonpositive postselection volume with
10 m³. It then assigns missing Space volumes and fractions. Those unsupported
shape fallback paths and Space/global warning histories are retained as
original-only observations until corresponding Rust owners are verified.

A second call reads the first call's now-positive numeric volume. It must not
be described as a fresh user declaration. A discrepancy can increment the
original `ErrCount5` again, even where the first positive volume came from an
entered-height override. The warning threshold is strictly greater than 5%.

## Selected reference boundary

The input-only helper request has 16 paired closed boxes and three original-only
diagnostics. The paired shapes have six finite, outward, nondegenerate planar
quads: one horizontal floor, one horizontal roof and four vertical walls.
Shared endpoints and ordered vertex bits must match the frozen inputs in both
implementations. The first original edge-count probe must already be closed,
so collinear repair and geometry reorientation are inactive. Moderate
translation, horizontal rotation and changed face enumeration exercise the
signed source sum without claiming arbitrary polyhedra or floating ranges.

For prepared units, real `EnergyPlusData`, original `GetVertices`, explicit
prepared area read fields, the byte-exact original `455–546` height fragment,
and two original volume calls are used. The fragment's `DetailedWWR` branch is
inactive and its routine label is source-bound. Prepared area fields are
declared inputs; this route does not claim original area producer or parser
admission. Additional genuine helper closure/tilt/height/volume probes are
labelled observations of separate calls, not an internal-local trace.

The reversed-winding diagnostic explicitly bypasses `GetVertices`, which can
reorient roofs and floors. It invokes genuine direct geometry helpers on
declared prepared vertices. The missing-roof and duplicated-wall diagnostics
retain actual open-zone fallback behavior. None promises an original fatal
status, Rust diagnostic-text parity, or normal Rust admission.

Twelve ordinary original IDFs are planned separately: seven unchanged CON
representatives, four Zone-field-only A-24H diagnostics and one duplicate-wall
topology diagnostic. The eleven positive rows require successful initialized
and retained lifecycles. They observe the
real parser, floor/ceiling area producers, height flags, initialized volume,
implicit Space fields and later retained values in fresh original lifetimes.
No original output becomes a Rust input.

The additional ordinary input copies `Zn001:Wall003` vertices to
`Zn001:Wall004`, preserving distinct names and all six finite nondegenerate
faces. Its original exit is not prescribed: either source rejection or
warning/fallback with continued physics is retained, together with actual
callbacks and nullable available phases. The normal Rust physical route must
reject unsupported topology before zone physics. This is separate admission
evidence and makes no original/Rust exit or error-text parity claim.

The helper also captures newly allocated actual owner defaults before names,
indices, declared numeric inputs and geometry preparation. A pre-allocation
constructor snapshot with empty arrays is not evidence for fabricated Zone or
Space field values. Unwritten vertex/scratch elements are never read.

## Current Rust reuse and remaining work

`geometry.rs::zone_volume_m3` currently returns a positive declared volume
early, otherwise prefers bounding-box volume before floor-area times entered
height. This misses the original prepriority evaluations and reverses the
entered-height override. Its bounding-box helper also adds a Zone origin to
vertices that the compiler has already made world coordinates. GEO-03 must
reuse the incoming GEO-02 geometry, preserve the source priority and reject
unsupported topology rather than generalize the box approximation.

The actual initialization stores `ZoneHeatBalanceState.volume_m3` and passes it
to the standard air capacity consumer. `heat_balance/air_manager.rs` passes
that stored value to the weather-context capacity consumer. The intended
connection proof copies the actual volume argument, caller and real execution
context. It does not certify complete `AirPowerCap`, rho/Cp, multiplier,
system timestep or ZON-02 physics.

Three fresh Rust physical cases are planned: A-24H, A-72H and B-BOTH-24H,
each Full and Summary. Full can expose stored state and genuine consumer
operands; Summary proves ordinary output invariance without a fabricated
geometry snapshot. There is no fresh B72, annual or per-limit consumption
claim. Existing GEO-02 results remain historical geometry evidence.

## Precision and exclusions

Draft frozen profiles are volume `atol=1e-9 m³`, area `1e-10 m²`, height
`1e-10 m`, each `rtol=1e-13`. Input bits, booleans, array order/cardinality,
phase labels and actual passed consumer operand bits are exact. Finite class
and the sign of two zero values are exact. Profiles cannot widen after a
failure.

No general polyhedron, active repair, multi-zone/multi-space, air boundary,
window/shading/IntMass, Space allocation history, warning IO, ZON-02/SYS or
whole EnergyPlus compatibility claim follows from this bounded card.
