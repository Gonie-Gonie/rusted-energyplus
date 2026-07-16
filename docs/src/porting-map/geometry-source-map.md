---
status: active
claim_level: source-mapped
owner: runtime
last_reviewed: 2026-07-15
---

# Geometry Coordinate Transformation Source Map

Reference version: EnergyPlus 26.1.0

Reference source: `src/EnergyPlus/SurfaceGeometry.cc` at the repository-locked
EnergyPlus `v26.1.0` commit.

This CP57 checkpoint records the detailed opaque-surface coordinate path. It
does not claim that Rust currently parses `GlobalGeometryRules`, transforms
surface vertices, or matches EnergyPlus geometry diagnostics. The existing
`surface_geometry_001` case remains smoke, nonclaim, and nonblocking evidence
for the default zero-rotation world-coordinate fixture.

## Source Order

| Order | EnergyPlus routine | Ledger status | Bounded obligation |
|---|---|---|---|
| 1 | `SetupZoneGeometry` | `source_mapped` | Owns building and zone rotation trigonometric initialization, calls `GetSurfaceData`, and later tears down temporary zone arrays; equipment, window, shading, and solar setup remain outside this checkpoint. |
| 2 | `GetSurfaceData` | `source_mapped` | Calls `GetGeometryParameters`, applies world-coordinate warning policy, inventories every surface family, and dispatches detailed heat-transfer input; allocation, sorting, interzone matching, fenestration, shading, and diagnostics remain deferred. |
| 3 | `GetGeometryParameters` | `state_mapped` | Maps the unique `GlobalGeometryRules` fields, coordinate-mode flags, mismatch checks, diagnostics, and EIO reporting without claiming a Rust implementation. |
| 4 | `GetHTSurfaceData` | `source_mapped` | Reads detailed heat-transfer surface objects and delegates vertex processing to `GetVertices`; construction, boundary, zone/space, validation, and all non-opaque families remain deferred. |
| 5 | `GetVertices` | `state_mapped` | Maps vertex ordering and the relative/world coordinate branch plus the broader derived-geometry state that surrounds it; only the coordinate branch is the bounded future implementation target. |

The source call chain is `SetupZoneGeometry` -> `GetSurfaceData`.
`GetSurfaceData` invokes `GetGeometryParameters` before surface-family input,
then `GetHTSurfaceData` delegates detailed vertices to `GetVertices`.

## CP57 State Contracts

### `GetGeometryParameters` (`get_geometry_parameters`)

<!-- routine-state-contract:v1 begin get_geometry_parameters -->
GetGeometryParameters

read_state:
- the unique `GlobalGeometryRules` object; its starting-vertex-position, vertex-entry-direction, surface coordinate-system, daylight-reference-point coordinate-system, and rectangular-surface coordinate-system alpha fields; zone origins used by coordinate-mismatch checks; the input object count; and the caller-owned error flag

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
- every diagnostic and reporting side effect, optional daylight and rectangular coordinate mode, and repeated-input lifecycle behavior
- downstream use of the parsed flags by detailed, simple, daylighting, fenestration, shading, solar, and heat-balance paths

not_claimed_branches:
- a typed Rust `GlobalGeometryRules` object, compiler acceptance or rejection parity, implemented coordinate transformation, exact EnergyPlus warning or EIO parity, alternate surface families, daylighting, `GeometryTransform`, Appendix G rotation, fenestration, shading, solar, or broad geometry conformance
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
- coincident-vertex removal, degenerate-surface accounting, normal and area derivation, roof or floor reversal, orientation diagnostics, multipliers, view-factor autocalculation, local coordinate systems, and `GeometryTransform` aspect changes
- detached-building, daylighting, simple rectangular, fenestration, shading, interzone, and downstream solar or heat-balance state

inactive_branches:
- clockwise input reverses vertices two through the final vertex, non-upper-left starting corners rotate vertex order, and counterclockwise upper-left input preserves order
- Relative coordinates apply zone relative-north rotation, zone-origin translation, and building rotation for zoned surfaces; World coordinates ignore those values and apply only Appendix G rotation, while detached-building surfaces follow their source-specific building branch

unsupported_active_branches:
- all `GetVertices` behavior outside the bounded detailed opaque-surface order and relative/world projection, including diagnostics, vertex deletion, derived geometry, correction, and aspect transformation
- Appendix G rotation, detached surfaces, non-opaque surface families, and every downstream consumer of transformed coordinates

not_claimed_branches:
- a Rust `GetVertices` implementation, exact trigonometric or last-bit parity, complete starting-corner and entry-direction behavior, diagnostic and counter parity, EIO row equality for transformed cases, zone volume closure, fenestration, shading, daylighting, solar, heat balance, or broad geometry conformance
<!-- routine-state-contract:v1 end get_vertices -->

## Promotion Boundary

This inventory contains five routines: three `source_mapped` and two
`state_mapped`. Every routine has `required_for_full_domain = false`.
Promotion requires a typed geometry-rules owner, an implemented canonical
world-vertex path, source-vector tests, and blocking transformed geometry EIO
families. The default `surface_geometry_001` smoke case does not satisfy those
requirements.
