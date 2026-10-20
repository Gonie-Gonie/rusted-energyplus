# GEO-03 bounded source review

GEO-03's four gates are closed for the frozen A/B projection. The original is
EnergyPlus 26.1.0, `6f2e40d10250a105b49966baa24d843711e61048`; the implementation
is `966af17aa5fc6bd68cddb9cfa23079dffc09ddc7`, crates tree
`c5120d6a1b6210c0badca9df98e46fa756f762dc`. The metadata-reader repair is
`6288c2b3153b7c4851efb387c585f0f68d56af7b` and changes no scientific source.

The frozen [source](../../contracts/GEO-03-source.json),
[cases](../../contracts/GEO-03-cases.json) and
[tolerances](../../contracts/GEO-03-tolerances.json) retain 14 original files and
68 byte-hashed ranges. Original execution and independent review preceded
Rust numerical execution. [Original-first](original-first.json),
[final unit](final-unit-comparison.json),
[production connection](production-comparison.json),
[actual commands](actual-commands.json) and
[independent review](independent-review.json) bind preserved raw evidence.

## Source owner, fields and order

`SurfaceGeometry::CalculateZoneVolume`, `SurfaceGeometry.cc:12010-12322`, runs
from `SetupZoneGeometry:548` after incoming GEO-02 surface geometry and the
area/height preparation. `HeatBalanceManager.cc:2333-2340` writes the declared
height, volume and user floor area; it does not set `ceilingHeightEntered`.
`GetSurfaceData:2302-2456` resolves floor/ceiling area and floor/roof flags.
`SetupZoneGeometry:376-391` accumulates geometric floor/ceiling gross areas.
Its `455-546` loop sets the entered-height flag from the prior positive height,
then assigns automatic height only when the prior height is nonpositive.
Positive final height alone does not prove a lexical entered height.

Height is the difference between area-weighted roof/floor min/max-z midpoints,
with the selected wall max/min fallback. A bounding-box height may coincide
for a flat rectangular prism but is not the general source algorithm.

The volume owner constructs stored Wall/Floor/Roof faces, recomputes their
Newell area vectors and evaluates horizontal/vertical, wall-height and initial
enclosure conditions before applying volume priority. `BaseSurfIDs`
(`SurfaceGeometry.hh:545-558`) and the stable class transfer at
`GetSurfaceData:1504-1530` determine ordinary Wall/Floor/Roof order. The selected
`GetHTSurfaceData` instance/write blocks and `InputProcessor::getJSONObjNum`
bind saved IDF ordinals; actual native parser flags and ordered faces are
observed rather than assumed. Direct units preserve their request face order.
Own Rust IDs are separate from native IDs and pair by canonical names/order.

`Vectors::CalcPolyhedronVolume:505-529` adds each ordered signed binary64 term
`dot(NewellAreaVector, FacePoints(2) - actual p0) / 3.0`. The actual p0 constructor
is zero and this owner does not write it. Stored face points are already world
coordinates, so Rust does not add Zone origin again, translate them to a
convenient center, take an absolute value or substitute bbox volume. The plain
`/3.0` is distinct from GEO-02's centroid extended-product third.

Final selection at `12193-12233` retains a current positive Volume; otherwise
entered-height plus positive selected floor area has priority, then calculated
volume. Selected floor area is positive FloorArea, else geometricFloorArea.
The source nonpositive fallback to 10m3, warning/error IO, ErrCount5 and implicit
Space assignment/fractions remain original-only lifetimes. The source discrepancy
threshold is strictly greater than 5%. A second call reads the first call's
current positive volume and is not a new lexical declaration.

## Original-first and input identity

The original helper executes 19 input-only rows: 16 closed boxes and three
unsupported source-only diagnostics. Paired shapes have six finite, outward,
nondegenerate planar quads, exact shared endpoints and ordered input bits.
Initial edge closure is already valid; no collinear healing or reorientation
is needed. Moderate translation, horizontal rotation and changed enumeration
exercise the signed sum without admitting arbitrary polyhedra or ranges.

The genuine-state helper uses original GetVertices, declared prepared area
read fields, the SHA-pinned original height fragment and two volume calls.
Prepared areas are inputs, not a claim of executing the normal area parser.
Newly allocated constructor defaults, declared reads and later owner states
are separate observations; unwritten vertex/scratch values are never read.
Additional closure/tilt/height/signed-volume helper calls are observations of
separate calls, not fabricated internal-local traces.

The reversed-winding helper explicitly bypasses GetVertices' roof/floor
reorientation. Missing-roof and duplicate-wall helpers retain original warning
and fallback behavior. All three are unpaired source-only routes and promise
no ordinary Rust parser rejection or error-text equivalence.

Twelve fresh ordinary original IDFs separately observe actual area/height/flag
producers and Zone/Space field retention: seven unchanged CON cases, four
Zone-field A24 diagnostics and one duplicate-wall topology diagnostic. Its
outcome was not prescribed: it actually warns and continues with exit 0 and
96 physical callbacks. Original outputs are never supplied as Rust inputs.

## Canonical reuse and actual connection

Rust now uses `geometry/zone_volume.rs` and `zone_volume/topology.rs` as the
mutable Zone geometry owner, reusing real GEO-02 area/Gross/Newell values.
The validated BuildingSurface:Detailed IDF-order overlay and stable class
selection preserve the ordinary face sum order. Prepared units use their
explicit ordered faces. Height runs once, then volume runs twice on the same
owner in direct units. Automatic unsupported topology returns an explicit
error rather than an approximate positive box volume.

The default committed unit comparison passes 2,672 checks with zero mismatches:
ten Zone fields, five observed phases including the separate declared-read
stage, four required phases, strict input bits/flags/order and second-call
retention. Volume (96), height (80) and area (880) scalar comparisons each have
maximum error and RMSE 0 under their unchanged dimensional profiles.

Six actual Rust normal CLI commands execute A24/A72/BBoth24 Full and Summary.
The final production comparison passes 758 checks with zero mismatches. Full
copies the actual initializer's ten-field returned bundle; only Volume is
stored in ZoneHeatBalanceState and handed to real capacity calculations. It
does not assert ten retained Rust fields across physical timesteps. Actual
`RustZoneAirHeatCapacity` arguments, `air_manager.rs` callers, the
`zone_air_heat_capacity_update` phase and real zone/system cursors qualify
672 physical calls covering all 480 ordered zone intervals, with no omissions.
Report recomputation and initialization calls are separately classified.
These are actual Rust labels, not inferred native stage or system-clock parity.

B's positive declared Volume has identical declared, returned, original,
stored and consumed bits for 1m3. Its own diagnostics are genuinely winding
inconsistent with signed sum -1/3 and a rejection reason; positive-volume
priority preserves the entered value. This is not passing automatic topology
or an inference of original local CalcVolume/method.

The automatic duplicate-wall normal Rust case reaches the actual owner and
rejects before zone physics with Runtime exit 6, zero recorded zone hooks and
no final physics output. The original continues exit 0/96, as observed above;
exit code, text and fallback parity are not claimed. Summary has no Full
observer files and proves equality of ordinary outputs only, not direct
observation of the ten fields. No fresh B72/annual/per-limit physics is claimed.

## Failure history, precision and exclusions

The [baseline](baseline.json) remains fail-incomplete: 582 checks, 192 missing
owner observations and three volume mismatches. [Candidate](candidate-unit.json)
remains non-gate evidence. The first workspace regression preserves exit 101
and 69 failures. Its legacy inward-wound thermal fixture had relied on bbox
volume; only its intended 1m3 was made explicit, keeping vertices, Auto height/
floor and thermal assertions unchanged. A separate retention-versus-Auto-
rejection regression and [independent fixture review](independent-fixture-review.json)
remain preserved. The final workspace suite passes 4,548 tests and Clippy.

Two actual production-reader failures remain raw receipts: an assumed setup
phase and an unconditional Auto topology guard on entered Volume. Independent
source-bound metadata amendments corrected those assumptions. No Rust math,
case input, tolerance or engine run was changed to repair the readers.
Archived available-source inventories bind build and execution bytes and
record one CRLF-only difference; they do not establish which files each
compiler selected.

Frozen profiles are volume atol1e-9m3, area1e-10m2 and height1e-10m, each
rtol1e-13. Input and actual consumer bits, booleans, identities, order and phases
are exact, as are finite class and two-zero sign. No tolerance widened after a
failure. The selected closure rules are not general polyhedron certification.

Implicit Space, source ErrCount5/global counters, scratch/repair allocation,
warning IO and unsupported fallback remain unpaired. The observed volume
handoff does not close original AirPowerCap/rhoCp/multiplier/systemdt, ZON-02,
SYS, annual warmup or whole EnergyPlus physical equivalence.
