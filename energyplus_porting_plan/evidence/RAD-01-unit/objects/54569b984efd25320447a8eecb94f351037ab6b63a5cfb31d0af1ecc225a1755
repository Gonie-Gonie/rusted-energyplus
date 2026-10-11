use crate::{AutoOrNumber, ConstructionId, NormalizedName, Point3, SpaceId, SurfaceId, ZoneId};

/// Declared first vertex for detailed surface input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartingVertexPosition {
    /// Upper-left corner as viewed from outside the surface.
    UpperLeftCorner,
    /// Lower-left corner as viewed from outside the surface.
    LowerLeftCorner,
    /// Upper-right corner as viewed from outside the surface.
    UpperRightCorner,
    /// Lower-right corner as viewed from outside the surface.
    LowerRightCorner,
}

/// Declared detailed-surface vertex entry direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VertexEntryDirection {
    /// Counter-clockwise as viewed from outside the surface.
    CounterClockwise,
    /// Clockwise as viewed from outside the surface.
    Clockwise,
}

/// Coordinate system selected by `GlobalGeometryRules`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryCoordinateSystem {
    /// Coordinates are relative to the zone origin.
    Relative,
    /// Coordinates are absolute facility/world coordinates.
    World,
}

/// Typed `GlobalGeometryRules` input settings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalGeometryRules {
    /// Declared starting vertex position.
    pub starting_vertex_position: StartingVertexPosition,
    /// Declared vertex entry direction.
    pub vertex_entry_direction: VertexEntryDirection,
    /// Detailed-surface coordinate system.
    pub coordinate_system: GeometryCoordinateSystem,
    /// Daylighting reference point coordinate system.
    pub daylighting_reference_point_coordinate_system: GeometryCoordinateSystem,
    /// Rectangular/simple surface coordinate system.
    pub rectangular_surface_coordinate_system: GeometryCoordinateSystem,
}

impl Default for GlobalGeometryRules {
    fn default() -> Self {
        Self {
            starting_vertex_position: StartingVertexPosition::UpperLeftCorner,
            vertex_entry_direction: VertexEntryDirection::CounterClockwise,
            coordinate_system: GeometryCoordinateSystem::World,
            daylighting_reference_point_coordinate_system: GeometryCoordinateSystem::Relative,
            rectangular_surface_coordinate_system: GeometryCoordinateSystem::Relative,
        }
    }
}

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

/// Source-recognized shape category for bounded computed surface geometry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceShapeCategory {
    /// Three-vertex surface.
    Triangular,
    /// Four-vertex surface admitted by the bounded rectangle predicate.
    Rectangular,
}

/// Axis removed when projecting surface vertices into two dimensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceProjectionAxis {
    /// Remove X and retain Y/Z coordinates.
    X,
    /// Remove Y and retain X/Z coordinates.
    Y,
    /// Remove Z and retain X/Y coordinates.
    Z,
}

/// Point or vector in the source-shaped two-dimensional surface projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceProjectedPoint {
    /// First projected coordinate in meters.
    pub x_m: f64,
    /// Second projected coordinate in meters.
    pub y_m: f64,
}

/// Immutable geometry derived from one admitted detailed surface.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceComputedGeometry {
    /// Source-recognized bounded shape category.
    pub shape_category: SurfaceShapeCategory,
    /// Unnormalized Newell plane coefficients `[a, b, c, d]`.
    pub plane: [f64; 4],
    /// Axis removed from the three-dimensional vertices.
    pub projection_axis: SurfaceProjectionAxis,
    /// Counter-clockwise projected vertices; only vertices 2 through N may be reversed.
    pub projected_vertices: Vec<SurfaceProjectedPoint>,
    /// Lower corner of the projected bounding box.
    pub projected_lower_bound: SurfaceProjectedPoint,
    /// Upper corner of the projected bounding box.
    pub projected_upper_bound: SurfaceProjectedPoint,
    /// Wraparound edge vectors in projected-vertex order.
    pub projected_edges: Vec<SurfaceProjectedPoint>,
    /// Squared first-side length for rectangles, or zero for triangles.
    pub rectangle_side_1_squared_m2: f64,
    /// Squared fourth-side length for rectangles, or zero for triangles.
    pub rectangle_side_3_squared_m2: f64,
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
    /// Final Space assignment after the bounded `CreateMissingSpaces` pass.
    pub space: SpaceId,
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
    /// Bounded source-order computed geometry, when the surface is admitted.
    ///
    /// This derived attachment is not a typed input object and does not change
    /// object identity, object counts, or model-graph edges.
    pub computed_geometry: Option<SurfaceComputedGeometry>,
}
