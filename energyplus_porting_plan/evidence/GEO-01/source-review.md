# GEO-01 original source and preparation review

Status: original-first source/input/state preparation and the 37-case Rust unit
comparison passed. Selected physical connection comparisons and committed-source
refresh are pending. This file does not set plan or card gates.

The reference is the unchanged EnergyPlus 26.1.0 source at
`6f2e40d10250a105b49966baa24d843711e61048`. The source contract pins complete
file hashes and 35 bounded ranges across 12 files. `GetHTSurfaceData` reads the
actual Detailed objects, binds their positive zone, and allocates an explicit
four-vertex array before calling `GetVertices` (4294–4348). The selected
`GetVertices` block (9169–9314) copies numeric inputs in order, updates the
original maximum-vertex counter with an `if` assignment, reverses clockwise
vertices 2..N while retaining vertex 1, applies the original starting-corner
swap loop, and converts coordinates in the original operation order.

`SetupZoneGeometry` (251–311) computes coefficients using the original
`Constant::DegToRad` multiplication. Relative coordinates use zone rotation,
then X/Y origin, then building rotation; Z adds the zone origin. World
coordinates still execute the AppendixG-only X/Y multiply/add. AppendixG is
fixed to zero here; its sine is an observable negative zero. World is therefore
not modeled as a literal algebraic bypass. Building modulo normalization is a
CON-01 prerequisite. Nonzero AppendixG remains excluded.

The reference wrapper dynamically loads the verified same-GCC original API
DLL. That DLL constructs, parses, initializes, simulates and deletes its own
`EnergyPlusData`. The wrapper performs only const reads and public callback
registration. It adds no geometry calls, resets, actuation or expected answers.
It observes the first before-init-heat-balance callback after original geometry
setup (possibly during warmup), the first physical weather callback, the final
weather callback before wrapup, and the retained arrays immediately before
`stateDelete`. Initial/final vertex bits and name/zone/cardinality identity must
remain exact. The original parsed epJSON and numeric bits establish actual
input signs; a textual `-0.0` is not assumed to survive parsing without evidence.

Constructor and later native-only state are retained honestly: Corner=0,
CCW=false, World=false, MaxVerticesPerSurface=4, zero trig coefficients and
one-time flags change during original initialization. Zone trig arrays are
allocated during setup and deallocated at 310–311. The observer reports their
actual later deallocated state, rather than reconstructing cached values. Rust
has no corresponding maximum-vertex lifecycle field. Four-vertex inputs keep
growth inactive, so the native counter is unpaired source-state evidence.

The unchanged CON15 cases remain World/UpperLeft/CCW with zero Building/Zone
axes/origins. Twenty-two separate A-24H geometry diagnostics cover Relative
rotation/translation/composition, negative and greater-than-360 angles,
near-zero angles, finite signed-zero lexemes, World ignored axes/origins, four
starting corners, and both winding directions. Their six opaque planar
quadrilateral faces retain the original orientation through input-only cyclic
permutations and clockwise reversal. They do not expand CON admission.
Declared vertex count equals the supplied array length. Triangle/5+ vertex,
invalid/missing GGR, extra/auto-count, nonfinite, detached, subsurface,
GeometryTransform and AppendixG control branches are excluded.

The full original `GetVertices` body subsequently evaluates normals, area,
azimuth and tilt and can remove coincident vertices, reverse an upside-down
roof/floor, or apply aspect transforms. Those are not silently treated as the
coordinate block. Required domain observations are unchanged cardinality,
zero actual coincident/degenerate counters, absent aspect/AppendixG controls,
and no geometry-specific removal/reversal warnings. Other weather/warmup
warnings do not constitute geometry failure. `SurfaceTmp` relocation into the
final owned array (1504–1530), later base-surface readers, and first/final
retention connect the coordinate producer to actual initialized state. Each
engine retains its own IDs; pairing uses case-normalized surface/zone names and
exact vertex order.

Coordinate tolerance was frozen before comparison at 1e-10 m absolute plus
1e-13 relative. Both-zero output sign is exact when the same actual parsed
input sign is established. Finite/nonfinite classification, input identity,
names/bindings/order/cardinality and inactive branch predicates are exact.
Nonzero coordinate bits remain recorded; libm results use the fixed numeric
tolerance. No tolerance is widened after failure. Selected CON physical
connection profiles also freeze area (1e-10 m2), zone volume (1e-9 m3), azimuth
and tilt (1e-10 deg), all with 1e-13 relative. Source 0/360 azimuth convention is
preserved without angle wrapping. These actual stored-field connection checks
do not close GEO-02/GEO-03 algorithms. A autocalculated volume and B declared
volume retain distinct provenance. Diagnostic dry-run consumers are unpaired.

The native target inherits the actual original directory and target compile
definitions and `project_options`, `project_fp_options`, `project_warnings`.
It uses the existing genuine GCC13.2/O0 core with assertions and Werror enabled,
FP contraction off, no fast math and matching non-debug container ABI. Prior
CLK/PSY target source/binary bytes and all frozen original compilation rows
must remain unchanged. Historical original Debug/optimized compiler failures
remain in the native core provenance and are not reclassified as successful.

Preparation commands:

```text
python -B tools/porting/geo01_reference.py --prepare
python -B tools/porting/geo01_reference.py --check
python -B tools/porting/geo01_reference.py --build --output-dir .runtime/porting/GEO-01/native-driver-first
```

`--prepare` and `--check` completed; they generate/check input-only contracts
and hashes, not numerical answers. Final native configure, compile and link
completed with the selected original flags. First draft contract bytes are archived under
`.runtime/porting/GEO-01/precomparison-initial-contracts`; a connection-profile
wording draft is archived separately. No original numerical output or Rust
comparison has been used to choose the frozen precision policy.

Original execution and review commands:

```text
python -B tools/porting/geo01_reference.py --run-original --driver-build .runtime/porting/GEO-01/native-driver-final/native-driver-build.json --output-dir .runtime/porting/GEO-01/original-first
python -B tools/porting/geo01_reference.py --review-original --original-matrix .runtime/porting/GEO-01/original-first/matrix.json --output-dir .runtime/porting/GEO-01/original-first-review
```

Both completed with exit 0. The original matrix retains all 37 per-case argv,
stdout/stderr, exits, input/weather/IDD and native build/output hashes. The
review checked 222 quadrilaterals and 2,664 actual parsed input coordinates,
including signed zero. All first/final vertices retained identical bits;
all supplied counts equaled actual four-vertex counts. Actual coincident and
degenerate counters were zero, aspect flags were inactive, and no selected
geometry removal/orientation/aspect warning was found. Each actual
`eplusout.err` is hashed in the preparation review. The native runs recorded
179,232 physical weather-zone callbacks, including unchanged CON annual inputs;
this does not claim Rust annual physics. The 22 original diagnostic simulations
provide input-level initialization references, not admitted diagnostic Rust
production physics.

An additional early fresh-process `WORLD-NEGATIVE-ZERO` execution retained
96 callbacks separately. Genuine parsed epJSON preserved the negative-zero
coordinates. Original World/AppendixG-zero algebra changed Wall001's first
input `(-0,-0,z)` to stored `(-0,+0,z)`; a literal World bypass would preserve a
different sign. This is an actual original output observation, not an answer
fed to Rust. Its raw input/result/build bindings remain immutable.

The initial wrapper compiled but failed to link the original container symbol
`ObjexxFCL::IndexRange::npos`. The wrapper-only target then linked the unchanged
original `objexx` library. Its SHA/size and actual link command are retained.
The final receipt includes a fresh compile and link of unchanged wrapper bytes,
the same-GCC DLL/core hashes, all 643 unchanged frozen original compilation
rows, and unchanged prior CLK/PSY executables. The failed and intermediate
attempts were preserved. No source scientific patch or math/diagnostic waiver
was introduced. `source-preparation.json` binds the compact review and each
historical attempt; the native lock is now idle.
