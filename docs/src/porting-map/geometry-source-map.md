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

CP97 adds a bounded partition-membership composite after Space declaration
state. Its `GetHTSurfaceData` cross-section resolves optional Space names for
typed `BuildingSurface:Detailed` records against the complete pre-remainder
Space arena and validates same-Zone membership while the full routine remains
`source_mapped`. The later state-mapped `CreateMissingSpaces` finalizer consumes
those resolved, preclassified assignments, creates one General
`AutoZoneRemainder` for each mixed Zone, and applies fallback `SpaceId`s. This
does not promote the parent geometry routines or extend the
transformed-coordinate smoke claim.

## Source Order

| Order | EnergyPlus routine | Ledger status | Bounded obligation |
|---|---|---|---|
| 1 | `SetupZoneGeometry` | `source_mapped` | Owns building and zone rotation trigonometric initialization, calls `GetSurfaceData`, and later tears down temporary zone arrays; equipment, window, shading, and solar setup remain outside this checkpoint. |
| 2 | `GetSurfaceData` | `source_mapped` | Calls `GetGeometryParameters`, applies world-coordinate warning policy, inventories every surface family, and dispatches detailed heat-transfer input; allocation, sorting, interzone matching, fenestration, shading, and diagnostics remain deferred. |
| 3 | `GetGeometryParameters` | `state_mapped` | Maps the unique `GlobalGeometryRules` fields, coordinate-mode flags, mismatch checks, diagnostics, and EIO reporting. The typed Rust parser covers field normalization and fallbacks, but source-required-object and cross-coordinate warning parity remain deferred. |
| 4 | `GetHTSurfaceData` | `source_mapped` | Reads detailed heat-transfer surface objects and delegates vertex processing to `GetVertices`; the CP97 cross-section bounds optional Space lookup and same-Zone validation for typed detailed opaque surfaces, but the full routine and all non-opaque families remain deferred. |
| 5 | `GetVertices` | `state_mapped` | The bounded detailed opaque-surface ordering and relative/world projection branch is implemented and oracle-smoke tested. The ledger status remains `state_mapped` until an Algorithm Port Ticket and blocking family gate promote the routine; the rest of the source routine remains deferred. |
| 6 | `CreateMissingSpaces` | `state_mapped` | After the source has read every surface family, generated adjacent surfaces, and reconciled base links, the bounded Rust finalizer consumes the preceding GetHT-resolved typed `BuildingSurface:Detailed` assignment and explicit/implicit classification, creates mixed-Zone remainders, preserves or applies fallback SpaceIds, and redirects mixed implicit surfaces; reordering and all later geometry stay deferred. |

The source call chain is `SetupZoneGeometry` -> `GetSurfaceData`.
`GetSurfaceData` invokes `GetGeometryParameters` before surface-family input,
then `GetHTSurfaceData` delegates detailed vertices to `GetVertices`. Much later
in the same parent routine it calls `CreateMissingSpaces` immediately before
surface reordering. The CP97 composite therefore spans a bounded input-time
cross-section owned by `GetHTSurfaceData` and the later state-mapped
`CreateMissingSpaces` finalizer; it does not map the full former routine.

## CP57 and CP97 State Contracts

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

### `CreateMissingSpaces` (`create_missing_spaces`)

<!-- routine-state-contract:v1 begin create_missing_spaces -->
CreateMissingSpaces

read_state:
- EnergyPlus calls `CreateMissingSpaces` inside `GetSurfaceData` after all surface-family input, the fatal input-error gate, adjacent Zone/Space surface generation, and base-surface reconciliation, but before surface reordering; bounded Rust calls its partition pass after validated typed `BuildingSurface:Detailed` materialization and covers only that base detailed opaque family
- the preceding bounded GetHTSurfaceData cross-section owns raw optional `space_name` handling: `resolve_surface_space` treats missing or blank input as implicit, rejects malformed values, searches the full pre-remainder Space arena in dense order with case-insensitive normalized matching including authored and `AutoZoneDefault` entries, and requires every explicit target to belong to the surface Zone; full GetHTSurfaceData remains source-mapped
- as a prerequisite to `create_missing_spaces`, every retained typed surface therefore supplies a validated ZoneId, a provisional final-candidate SpaceId equal to either its explicit target or the Zone's existing last Space, and an ephemeral explicit/implicit classification; the pass reads those preclassified assignments to detect whether each Zone has both classes, while a Zone with no retained detailed surface has neither

write_state:
- every retained typed `BuildingSurface:Detailed` exits with a final `Surface.space: SpaceId`: an explicit valid target remains unchanged, an all-implicit Zone uses its existing last Space, and an implicit surface in a mixed explicit/implicit Zone is redirected to the newly appended remainder
- each mixed Zone receives exactly one Zone-order appended Space named `<ZONE>-REMAINDER`, linked last in `Zone::spaces`, with `SpaceOrigin::AutoZoneRemainder`, AutoCalculate ceiling height/volume/floor area, no tags, and the General SpaceTypeId reused or appended through `GetGeneralSpaceTypeNum`; the remainder is deliberately absent from the authored Space name map and generated Spaces do not increase typed input object count
- unknown, malformed, or cross-Zone explicit references fail compilation in the preceding bounded GetHTSurfaceData cross-section before this partition pass; every authored Space, SpaceList, and generated remainder is reported as `UnsupportedSpacePartitioning` and `RunBlocked`, while a sole `AutoZoneDefault` remains inactive when no remainder is needed, including when every valid typed surface explicitly references it

history_state_ownership:
- TypedModel owns immutable final detailed-surface SpaceIds, generated remainder descriptors, per-Zone ordered SpaceIds, and the shared SpaceType registry; this bounded pass allocates no mutable surface, geometry, space-air, zone-air, load, HVAC, sizing, or reporting history

unsupported_state:
- all non-`BuildingSurface:Detailed` source families, non-heat-transfer and shading filters, InternalMass exclusion from remainder detection, subsurface inheritance, auto-generated interzone surfaces and warnings, and Outside Boundary Condition Zone/Space opposite-surface creation
- the remainder of `GetSurfaceData` and `SetupZoneGeometry`: surface reordering, `Space.surfaces`, first/last ranges, calculated floor area/ceiling height/volume and fractions, enclosures, vertex realization/correction beyond the existing CP57 subset, reports, and runtime consumers

inactive_branches:
- a Zone with no retained typed detailed surface creates no remainder; a Zone whose retained surfaces are all implicit creates no remainder and assigns all of them to its existing last Space
- a Zone whose retained surfaces are all explicit creates no remainder; prevalidated assignments to its sole generated whole-zone default add no `UnsupportedSpacePartitioning` boundary
- General is appended only when a mixed Zone needs a remainder and the ordered type registry does not already contain it; otherwise the first existing General identity is reused

unsupported_active_branches:
- every generated `AutoZoneRemainder` blocks arbitrary runtime execution until space-level geometry, heat balance, loads, HVAC, and reporting consume the partition; authored Spaces and SpaceLists remain blocked by their existing parent contracts
- final SurfaceId-to-SpaceId ownership is typed but does not claim surface sorting, per-Space surface lists, space heat-balance execution, or any numerical effect

not_claimed_branches:
- complete `SetupZoneGeometry`, `GetSurfaceData`, `GetHTSurfaceData`, or `CreateMissingSpaces` parity; source allocation/resize capacity, one-based counters, partial invalid-record side effects, whitespace-preserving or ambiguous case-colliding first-match behavior, exact diagnostics/order/multiplicity, legacy detailed surfaces, InternalMass, fenestration/subsurface inheritance, auto-generated adjacent surfaces and warnings, surface reordering, Space surface lists/ranges, geometry, loads, HVAC, reporting, numerical parity, and conformance
<!-- routine-state-contract:v1 end create_missing_spaces -->

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

This inventory contains six routines: three `source_mapped` and three
`state_mapped`. Every routine has `required_for_full_domain = false`.
The canonical world-vertex path and source-vector tests now exist, but routine
promotion still requires an Algorithm Port Ticket and blocking transformed
geometry EIO families. Neither `surface_geometry_001` nor
`surface_geometry_transform_001` satisfies that blocking requirement.
