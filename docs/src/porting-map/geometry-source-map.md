---
status: active
claim_level: source-mapped
owner: runtime
last_reviewed: 2026-07-15
---

# Geometry Coordinate Transformation Source Map

Reference version: EnergyPlus 26.1.0

Reference sources at the repository-locked EnergyPlus `v26.1.0` commit:

- `src/EnergyPlus/SurfaceGeometry.cc` for geometry rules, vertex ordering, and
  world-coordinate projection
- `src/EnergyPlus/Vectors.cc::DetermineAzimuthAndTilt` for the horizontal
  surface azimuth convention exercised by the transformed fixture

This CP57 checkpoint implements the bounded detailed opaque-surface coordinate
path. Rust types and parses `GlobalGeometryRules`, including EnergyPlus aliases
and warning fallbacks, preserves an explicit compatibility deviation for an
absent source-required singleton, canonicalizes input vertex order, and projects
Relative coordinates through zone rotation, zone origin, and building rotation
into world coordinates. `surface_geometry_transform_001` locks those results
against EnergyPlus `DetailsWithVertices` EIO rows. The case remains smoke,
nonclaim, and nonblocking; broad geometry diagnostics and the other source
families remain outside this checkpoint.

## Source Order

| Order | EnergyPlus routine | Ledger status | Bounded obligation |
|---|---|---|---|
| 1 | `SetupZoneGeometry` | `source_mapped` | Owns building and zone rotation trigonometric initialization, calls `GetSurfaceData`, and later tears down temporary zone arrays; equipment, window, shading, and solar setup remain outside this checkpoint. |
| 2 | `GetSurfaceData` | `source_mapped` | Calls `GetGeometryParameters`, applies world-coordinate warning policy, inventories every surface family, and dispatches detailed heat-transfer input; allocation, sorting, interzone matching, fenestration, shading, and diagnostics remain deferred. |
| 3 | `GetGeometryParameters` | `state_mapped` | Maps the unique `GlobalGeometryRules` fields, coordinate-mode flags, mismatch checks, diagnostics, and EIO reporting. The typed Rust parser covers field normalization and fallbacks, but source-required-object and cross-coordinate warning parity remain deferred. |
| 4 | `GetHTSurfaceData` | `source_mapped` | Reads detailed heat-transfer surface objects and delegates vertex processing to `GetVertices`; construction, boundary, zone/space, validation, and all non-opaque families remain deferred. |
| 5 | `GetVertices` | `state_mapped` | The bounded detailed opaque-surface ordering and relative/world projection branch is implemented and oracle-smoke tested. The ledger status remains `state_mapped` until an Algorithm Port Ticket and blocking family gate promote the routine; the rest of the source routine remains deferred. |

The source call chain is `SetupZoneGeometry` -> `GetSurfaceData`.
`GetSurfaceData` invokes `GetGeometryParameters` before surface-family input,
then `GetHTSurfaceData` delegates detailed vertices to `GetVertices`.

## CP57 State Contracts

### `GetGeometryParameters` (`get_geometry_parameters`)

<!-- routine-state-contract:v1 begin get_geometry_parameters -->
GetGeometryParameters

read_state:
- the unique `GlobalGeometryRules` object; its starting-vertex-position, vertex-entry-direction, surface coordinate-system, daylight-reference-point coordinate-system, and rectangular-surface coordinate-system alpha fields; zone origins used by coordinate-mismatch checks; and the input object count

write_state:
- `DataSurfaces::Corner`, `DataSurfaces::CCW`, `DataSurfaces::WorldCoordSystem`, `DataSurfaces::DaylRefWorldCoordSystem`, `SurfaceGeometryData::RectSurfRefWorldCoordSystem`, the caller-owned error flag, warning and severe diagnostic streams, and the `Surface Geometry` EIO row

history_state_ownership:
- all geometry-rule fields are `EnergyPlusData`-owned initialization state set during surface input; the routine has no cross-call numerical cache, while diagnostics and EIO output accumulate in simulation-owned reporting state

unsupported_state:
- daylight-reference-point and rectangular-simple-surface coordinate-system consumers, exact diagnostic counts and text, and exact EIO formatting
- lifecycle interactions with repeated input processing, shared input scratch arrays, and caller error aggregation

inactive_branches:
- exactly one object parses four starting corners, counterclockwise or clockwise entry, and World or Absolute versus Relative surface coordinates; zero objects and multiple objects set severe errors
- invalid surface-coordinate text warns and defaults to World, invalid optional coordinate text warns and defaults to Relative, and mixed coordinate modes or nonzero zone origins can emit mismatch warnings

unsupported_active_branches:
- exact diagnostic counts and text, cross-coordinate mismatch warnings, EIO emission, and repeated-input lifecycle behavior
- downstream use of the parsed flags by simple rectangular, daylighting, fenestration, shading, solar, and heat-balance paths

not_claimed_branches:
- source-required-object absence parity, cross-coordinate mismatch warning parity, exact EnergyPlus diagnostic or general EIO-emission parity, alternate surface families, daylighting, `GeometryTransform`, Appendix G rotation, fenestration, shading, solar, or broad geometry conformance
<!-- routine-state-contract:v1 end get_geometry_parameters -->

### `GetVertices` (`get_vertices`)

<!-- routine-state-contract:v1 begin get_vertices -->
GetVertices

read_state:
- the target `SurfaceTmp` record, `SurfNum`, `NSides`, and packed input coordinates; `Corner`, `CCW`, and `WorldCoordSystem`; building and Appendix G rotation trigonometry; per-zone relative-north trigonometry and origins; maximum-vertex, warning, error-counter, surface-class, multiplier, view-factor, and `GeometryTransform` state

write_state:
- reordered and world-projected surface vertices; maximum vertices per surface; perimeter, side count, coincident and degenerate counters, area vectors, gross and net area, normal and local-coordinate vectors, azimuth, tilt, orientation, trigonometric fields, view factors, diagnostics, possible vertex reversal, and optional aspect-transformed coordinates

history_state_ownership:
- input flags and rotation arrays are initialized earlier in the same `EnergyPlusData` geometry lifecycle and each surface result is owned by that simulation state; the routine has no independent cross-call numerical cache, but diagnostics and aggregate error counters persist across surfaces

unsupported_state:
- coincident-vertex removal, degenerate-surface accounting, exact source-order normal and area derivation, roof or floor correction, orientation diagnostics, multipliers, view-factor autocalculation, local coordinate systems, and `GeometryTransform` aspect changes
- detached-building, daylighting, simple rectangular, fenestration, shading, interzone, and downstream solar or heat-balance state

inactive_branches:
- clockwise input reverses vertices two through the final vertex, non-upper-left starting corners rotate vertex order, and counterclockwise upper-left input preserves order
- Relative coordinates apply zone relative-north rotation, zone-origin translation, and building rotation for zoned surfaces; World coordinates ignore those values and apply only Appendix G rotation, while detached-building surfaces follow their source-specific building branch

unsupported_active_branches:
- all `GetVertices` behavior outside the implemented detailed opaque-surface order and relative/world projection, including diagnostics, vertex deletion, source-order derived geometry, correction, and aspect transformation
- Appendix G rotation, detached surfaces, non-opaque surface families, and downstream consumers beyond the existing typed opaque-surface runtime

not_claimed_branches:
- full-routine `GetVertices` parity, exact trigonometric last-bit parity, diagnostic and counter parity, EIO equality outside the declared transformed fixture, zone volume closure, fenestration, shading, daylighting, solar, heat balance, or broad geometry conformance
<!-- routine-state-contract:v1 end get_vertices -->

## CP57 Transformed-Coordinate Evidence

`surface_geometry_transform_001` uses `UpperLeftCorner`,
`CounterClockWise`, and `Relative` coordinates with Building North Axis
30 degrees, Zone Direction of Relative North 45 degrees, and zone origin
`(10, 20, 3)`. Its six detailed opaque surfaces request
`Output:Surfaces:List,DetailsWithVertices`.

The smoke gate compares the normalized `Surface Geometry` rules row, side
counts, canonical world XYZ triples, surface class, net and gross area,
azimuth, and tilt. All six world-vertex vectors match EnergyPlus 26.1.0 at the
declared 0.01 m absolute and 1e-6 relative tolerance. This evidence proves only
the declared fixture and does not create a geometry conformance claim.

For horizontal surfaces, `DetermineAzimuthAndTilt` defines the local X axis
from vertex 2 to vertex 3 before computing azimuth. Rust follows that convention
instead of selecting the first available horizontal edge; the fixture locks
FLOOR at 345 degrees and ROOF at 75 degrees after the combined -75 degree
zone/building rotation.

## Promotion Boundary

This inventory contains five routines: three `source_mapped` and two
`state_mapped`. Every routine has `required_for_full_domain = false`.
The canonical world-vertex path and source-vector tests now exist, but routine
promotion still requires an Algorithm Port Ticket and blocking transformed
geometry EIO families. Neither `surface_geometry_001` nor
`surface_geometry_transform_001` satisfies that blocking requirement.
