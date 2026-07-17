use crate::{NormalizedName, SurfaceId, SurfaceVaporCoefficientsId};

/// One side of a surface's constant vapor-transfer coefficient input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceVaporCoefficient {
    /// Whether EnergyPlus should use the supplied constant coefficient.
    pub is_constant: bool,
    /// Supplied coefficient, retained even when `is_constant` is false.
    pub value_kg_per_pa_s_m2: f64,
}

/// Typed `SurfaceProperties:VaporCoefficients` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceVaporCoefficients {
    /// Typed ID.
    pub id: SurfaceVaporCoefficientsId,
    /// Normalized nonsemantic outer object key retained for diagnostics.
    pub name: NormalizedName,
    /// Resolved `BuildingSurface:Detailed` target.
    pub reference_surface: SurfaceId,
    /// Exterior-face vapor-transfer coefficient input.
    pub external: SurfaceVaporCoefficient,
    /// Interior-face vapor-transfer coefficient input.
    pub internal: SurfaceVaporCoefficient,
}
