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
- `surface_geometry_001` case manifest with static output requests

EnergyPlus evidence source:

- `eplusout.eio`
- `Zone Information`
- `HeatTransfer Surface`

Next evidence target:

- coordinate-system and `GlobalGeometryRules` variants
- fenestration and shading surface geometry rows
- declared tolerances and blocking report gates for a future conformance class
