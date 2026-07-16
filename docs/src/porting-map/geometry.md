---
status: active
claim_level: static-evidence
owner: runtime
last_reviewed: 2026-06-07
---

# Geometry

Implemented evidence:

- `model geometry` summary for zone surface count, floor area, volume, and
  exterior wall area
- `compare geometry` EIO `Zone Information` smoke gate
- `compare surface-geometry` EIO `HeatTransfer Surface` smoke gate for surface
  class, net/gross area, azimuth, tilt, side count, and optional world vertices
- `surface_geometry_001` case manifest with static surface output requests
- typed `GlobalGeometryRules` parsing for all four starting-corner values,
  clockwise/counterclockwise entry, and World/Absolute/Relative surface modes
- detailed opaque-surface canonical vertex ordering and Relative projection
  through zone rotation, zone origin, and building rotation
- `surface_geometry_transform_001` nonblocking `DetailsWithVertices` oracle
  smoke for the normalized rules row and six transformed world-vertex vectors
- `construction_materials_001` case manifest for EIO construction/material
  static summaries
- `internal_gains_001` case manifest for nominal internal gains and the v0.26
  internal convective gain conformance trace
- `official_1zone_static_model_001` conformance case for declared official
  ExampleFile static EIO surface, construction/material, and OtherEquipment
  nominal fields

EnergyPlus evidence source:

- `eplusout.eio`
- `Zone Information`
- `HeatTransfer Surface`
- `Construction CTF`
- `Material CTF Summary`
- `OtherEquipment Internal Gains Nominal`
- `eplusout.eso` for `Zone Total Internal Convective Heating Rate`

Locked v0.5 geometry/static fields:

- zone surface count, floor area, volume, and exterior gross wall area
- surface class, net area, gross area, azimuth, and tilt
- construction count, outside material identity, layer count, thermal
  conductance, material thickness, conductivity, density, specific heat, and
  thermal resistance
- OtherEquipment zone floor area, equipment level, equipment per floor area,
  latent/radiant/lost/convected fractions, and hourly convective gain trace

Locked v0.23 static evidence fields:

- official `1ZoneUncontrolled` heat-transfer surface class, area, azimuth, and
  tilt
- official `1ZoneUncontrolled` Construction CTF and Material CTF Summary rows
- official `1ZoneUncontrolled` OtherEquipment Internal Gains Nominal rows

Locked CP57 diagnostic fields:

- normalized `Surface Geometry` starting corner, vertex direction, surface
  coordinate system, daylight coordinate system, and rectangular coordinate
  system
- detailed opaque-surface side count and canonical world XYZ vertices
- transformed-fixture surface class, area, azimuth, and tilt

EIO parser trust boundary:

- EIO rows are treated as EnergyPlus oracle extraction artifacts for selected
  static input summaries.
- Matching EIO rows is evidence that Rust input interpretation agrees with the
  selected EnergyPlus summaries for the declared fixture or official
  ExampleFile case.
- Matching EIO rows is not evidence of surface heat-transfer, solar,
  fenestration, zone heat-balance, HVAC, or plant conformance.

Unsupported geometry boundaries:

- degenerate surface diagnostics
- source-required missing-rule and cross-coordinate warning parity
- Appendix G rotation, detached surfaces, and `GeometryTransform` aspect changes
- simple rectangular, fenestration, shading, and daylighting coordinate paths
- fenestration and shading geometry rows
- broad geometry-family conformance beyond the declared nonblocking fixtures

Next evidence target:

- blocking official transformed-coordinate ExampleFile families
- Appendix G and detached-surface coordinate branches
- fenestration and shading surface geometry rows
- broader official ExampleFile static variants beyond `1ZoneUncontrolled`
