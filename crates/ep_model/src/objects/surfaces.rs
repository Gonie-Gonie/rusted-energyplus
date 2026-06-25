use crate::{AutoOrNumber, ConstructionId, NormalizedName, Point3, SurfaceId, ZoneId};

/// Building surface type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceType {
    /// Ceiling surface.
    Ceiling,
    /// Floor surface.
    Floor,
    /// Roof surface.
    Roof,
    /// Wall surface.
    Wall,
}

/// Outside boundary condition for the first detailed surface subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutsideBoundaryCondition {
    /// Adiabatic boundary.
    Adiabatic,
    /// Foundation boundary.
    Foundation,
    /// Ground boundary.
    Ground,
    /// Outdoors boundary.
    Outdoors,
    /// Space boundary.
    Space,
    /// Adjacent surface boundary.
    Surface,
    /// Adjacent zone boundary.
    Zone,
    /// Other supported boundary condition represented but not simulated yet.
    Other,
}

/// Sun exposure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SunExposure {
    /// No sun exposure.
    NoSun,
    /// Sun exposed.
    SunExposed,
}

/// Wind exposure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindExposure {
    /// No wind exposure.
    NoWind,
    /// Wind exposed.
    WindExposed,
}

/// Detailed building surface.
#[derive(Clone, Debug, PartialEq)]
pub struct Surface {
    /// Typed ID.
    pub id: SurfaceId,
    /// Surface name.
    pub name: NormalizedName,
    /// Surface type.
    pub surface_type: SurfaceType,
    /// Resolved construction ID.
    pub construction: ConstructionId,
    /// Resolved zone ID.
    pub zone: ZoneId,
    /// Outside boundary condition.
    pub outside_boundary_condition: OutsideBoundaryCondition,
    /// Optional outside boundary condition object name.
    pub outside_boundary_condition_object: Option<NormalizedName>,
    /// Sun exposure.
    pub sun_exposure: SunExposure,
    /// Wind exposure.
    pub wind_exposure: WindExposure,
    /// View factor to ground.
    pub view_factor_to_ground: AutoOrNumber,
    /// Surface vertices.
    pub vertices: Vec<Point3>,
}
