---
status: active
claim_level: smoke
owner: runtime
last_reviewed: 2026-06-07
---

# Geometry

Implemented evidence:

- `model geometry` summary for zone surface count, floor area, volume, and
  exterior wall area
- `compare geometry` EIO `Zone Information` smoke gate
- `compare surface-geometry` EIO `HeatTransfer Surface` smoke gate for surface
  class, net/gross area, azimuth, and tilt
- `surface_geometry_001` case manifest with static surface output requests
- `construction_materials_001` case manifest for EIO construction/material
  static summaries
- `internal_gains_001` case manifest for nominal internal gains and the first
  internal convective gain trace

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
  latent/radiant/lost/convected fractions, and first hourly convective gain
  trace

EIO parser trust boundary:

- EIO rows are treated as EnergyPlus oracle extraction artifacts for selected
  static input summaries.
- Matching EIO rows is evidence that Rust input interpretation agrees with the
  selected EnergyPlus summaries for these fixtures.
- Matching EIO rows is not evidence of surface heat-transfer, solar,
  fenestration, zone heat-balance, HVAC, or plant conformance.

Unsupported v0.5 geometry boundaries:

- `GlobalGeometryRules` variants beyond
  `UpperLeftCorner,CounterClockWise,World`
- relative or non-world coordinate systems
- zone origin/rotation variants
- alternate vertex-ordering cases
- degenerate surface diagnostics
- fenestration and shading geometry rows

Next evidence target:

- coordinate-system and `GlobalGeometryRules` variants
- fenestration and shading surface geometry rows
- declared tolerances and blocking report gates for a future conformance class
