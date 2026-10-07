//! Source-ordered zone height and closed opaque-zone volume ownership.
//!
//! The admitted automatic route has six initially closed, consistently wound
//! quads. Original repair/open-zone fallbacks, Space state and warning IO remain
//! unpaired. A positive current Volume retains its source priority even when
//! automatic topology is unsupported. Vertices are already world coordinates.

mod topology;

use super::surface_geometry_properties;
use ep_model::{AutoOrNumber, Point3, Surface, SurfaceId, SurfaceType, TypedModel, Zone};
use std::fmt::{Display, Formatter};

/// Original `Constant::AutoCalculate`, retained in actual mutable input fields.
pub const ZONE_GEOMETRY_AUTO_CALCULATE: f64 = -99_999.0;

/// One owned base face in the actual order used for zone calculations.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneVolumeFace {
    /// Independent Rust surface identity.
    pub surface_id: SurfaceId,
    /// Normalized surface name.
    pub name: String,
    /// Actual typed base class.
    pub surface_type: SurfaceType,
    /// Already canonical world points in their retained order.
    pub vertices: Vec<Point3>,
    /// Original-equivalent opaque Area.
    pub area_m2: f64,
    /// Magnitude of the ordered Newell area vector.
    pub gross_area_m2: f64,
    /// Ordered original-equivalent Newell area vector.
    pub newell_area_vector_m2: [f64; 3],
    /// Incoming stored tilt used by the original helper predicates.
    pub tilt_deg: f64,
}

impl ZoneVolumeFace {
    /// Copies a real surface and computes its existing canonical GEO-02 bundle.
    pub fn from_surface(surface: &Surface) -> Result<Self, ZoneVolumeError> {
        let geometry = surface_geometry_properties(&surface.vertices).map_err(|error| {
            ZoneVolumeError::InvalidSurface {
                name: surface.name.0.clone(),
                reason: error.to_string(),
            }
        })?;
        Ok(Self {
            surface_id: surface.id,
            name: surface.name.0.clone(),
            surface_type: surface.surface_type,
            vertices: surface.vertices.clone(),
            area_m2: geometry.area_m2,
            gross_area_m2: geometry.gross_area_m2,
            newell_area_vector_m2: geometry.newell_area_vector_m2,
            tilt_deg: geometry.tilt_deg,
        })
    }
}

/// An initial edge whose undirected use count differs from the original two.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneVolumeEdge {
    /// First independent Rust face using this edge.
    pub surface_id: SurfaceId,
    /// First-seen canonical unique start point.
    pub start_m: Point3,
    /// First-seen canonical unique end point.
    pub end_m: Point3,
    /// Actual number of occurrences in the ordered face traversal.
    pub count: u32,
    /// Subsequent independent Rust faces using this edge.
    pub other_surface_ids: Vec<SurfaceId>,
}

/// Actual numerical observations of this owner, without source warning globals.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ZoneVolumeDiagnostics {
    /// Source-order first-pass unique point count.
    pub initial_unique_vertex_count: usize,
    /// First-pass edges, before any excluded source polygon repair.
    pub initial_edges_not_used_twice: Vec<ZoneVolumeEdge>,
    /// Actual initial undirected-edge closure result.
    pub initially_closed: bool,
    /// Additional bounded-domain requirement for consistent edge directions.
    pub edges_winding_consistent: bool,
    /// Actual signed helper sum, not an unobservable C++ local trace.
    pub signed_polyhedron_volume_m3: Option<f64>,
    /// Why automatic topology is outside the admitted domain, if applicable.
    pub topology_rejection: Option<String>,
    /// Incoming original floor-tilt predicate.
    pub floor_horizontal: bool,
    /// Incoming original roof-tilt predicate.
    pub roof_horizontal: bool,
    /// Incoming original wall-tilt predicate.
    pub walls_vertical: bool,
    /// Original max-Z wall-height predicate with the two-centimetre threshold.
    pub same_wall_height: bool,
    /// Actual last-call numeric five-percent comparison; no warning counter.
    pub volume_differs_by_more_than_five_percent: bool,
}

/// Owned selected Zone fields, retained across height and volume calls.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneGeometryProperties {
    /// Actual current Volume, including its original input sentinel before use.
    pub volume_m3: f64,
    /// Actual current CeilingHeight.
    pub ceiling_height_m: f64,
    /// Set only when height was positive before automatic height assignment.
    pub ceiling_height_entered: bool,
    /// Selected prepared Zone.FloorArea.
    pub floor_area_m2: f64,
    /// Original lexical floor-area input or AutoCalculate sentinel.
    pub user_entered_floor_area_m2: f64,
    /// Prepared geometric FloorArea.
    pub geometric_floor_area_m2: f64,
    /// Prepared CeilingArea.
    pub ceiling_area_m2: f64,
    /// Prepared geometric CeilingArea.
    pub geometric_ceiling_area_m2: f64,
    /// Actual presence of a selected floor.
    pub has_floor: bool,
    /// Actual presence of a selected roof/ceiling.
    pub has_roof: bool,
    /// Ordered independent Rust base faces used by this owner.
    pub faces: Vec<ZoneVolumeFace>,
    /// Selected original VectorsData constructor origin, never Zone.origin.
    pub p0_m: Point3,
    /// Actual owner observations and bounded admission result.
    pub diagnostics: ZoneVolumeDiagnostics,
    /// Number of actual completed invocations of this Rust volume owner.
    pub volume_calculation_count: u32,
}

impl Default for ZoneGeometryProperties {
    fn default() -> Self {
        Self {
            volume_m3: ZONE_GEOMETRY_AUTO_CALCULATE,
            ceiling_height_m: ZONE_GEOMETRY_AUTO_CALCULATE,
            ceiling_height_entered: false,
            floor_area_m2: 0.0,
            user_entered_floor_area_m2: ZONE_GEOMETRY_AUTO_CALCULATE,
            geometric_floor_area_m2: 0.0,
            ceiling_area_m2: 0.0,
            geometric_ceiling_area_m2: 0.0,
            has_floor: false,
            has_roof: false,
            faces: Vec::new(),
            p0_m: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            diagnostics: ZoneVolumeDiagnostics::default(),
            volume_calculation_count: 0,
        }
    }
}

/// Invalid numerical data or automatic geometry outside the selected route.
#[derive(Clone, Debug, PartialEq)]
pub enum ZoneVolumeError {
    /// A required numeric owner value is nonfinite.
    Nonfinite,
    /// Existing canonical surface initialization failed.
    InvalidSurface {
        /// Normalized independent surface name.
        name: String,
        /// Failure returned by the actual GEO-02 surface owner.
        reason: String,
    },
    /// Prepared source height areas cannot supply an admitted denominator.
    InvalidPreparedAreas,
    /// Initial automatic topology cannot use the bounded closed-volume owner.
    UnsupportedTopology(String),
}

impl Display for ZoneVolumeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nonfinite => f.write_str("zone geometry input and derived values must be finite"),
            Self::InvalidSurface { name, reason } => write!(f, "surface {name}: {reason}"),
            Self::InvalidPreparedAreas => f.write_str(
                "zone height requires positive prepared geometric floor and ceiling areas",
            ),
            Self::UnsupportedTopology(reason) => {
                write!(f, "unsupported automatic zone topology: {reason}")
            }
        }
    }
}
impl std::error::Error for ZoneVolumeError {}

/// Maps an existing typed lexical input to its original numeric field value.
#[must_use]
pub fn zone_geometry_input_value(input: AutoOrNumber) -> f64 {
    match input {
        AutoOrNumber::AutoCalculate => ZONE_GEOMETRY_AUTO_CALCULATE,
        AutoOrNumber::Value(value) => value,
    }
}

fn finite_fields(state: &ZoneGeometryProperties) -> bool {
    [
        state.volume_m3,
        state.ceiling_height_m,
        state.floor_area_m2,
        state.user_entered_floor_area_m2,
        state.geometric_floor_area_m2,
        state.ceiling_area_m2,
        state.geometric_ceiling_area_m2,
        state.p0_m.x_m,
        state.p0_m.y_m,
        state.p0_m.z_m,
    ]
    .iter()
    .all(|v| v.is_finite())
}

fn z_limits(face: &ZoneVolumeFace) -> Option<(f64, f64)> {
    let first = face.vertices.first()?.z_m;
    let (mut lo, mut hi) = (first, first);
    for point in &face.vertices[1..] {
        if point.z_m < lo {
            lo = point.z_m;
        }
        if point.z_m > hi {
            hi = point.z_m;
        }
    }
    Some((lo, hi))
}

/// Executes the selected Setup455-546 height preparation once on actual fields.
pub fn prepare_zone_height(state: &mut ZoneGeometryProperties) -> Result<(), ZoneVolumeError> {
    if !finite_fields(state) {
        return Err(ZoneVolumeError::Nonfinite);
    }
    let (mut ceil_count, mut floor_count, mut wall_count) = (0, 0, 0);
    let (mut zmax, mut zmin, mut ceil_average, mut floor_average) = (-99_999.0, 99_999.0, 0.0, 0.0);
    for face in &state.faces {
        let Some((lo, hi)) = z_limits(face) else {
            continue;
        };
        match face.surface_type {
            SurfaceType::Roof | SurfaceType::Ceiling => {
                if state.geometric_ceiling_area_m2 <= 0.0 {
                    return Err(ZoneVolumeError::InvalidPreparedAreas);
                }
                ceil_count += 1;
                ceil_average +=
                    ((lo + hi) / 2.0) * (face.gross_area_m2 / state.geometric_ceiling_area_m2);
            }
            SurfaceType::Floor => {
                if state.geometric_floor_area_m2 <= 0.0 {
                    return Err(ZoneVolumeError::InvalidPreparedAreas);
                }
                floor_count += 1;
                floor_average +=
                    ((lo + hi) / 2.0) * (face.gross_area_m2 / state.geometric_floor_area_m2);
            }
            SurfaceType::Wall => {
                wall_count += 1;
                if wall_count == 1 {
                    zmax = face.vertices[0].z_m;
                    zmin = zmax;
                }
                if hi > zmax {
                    zmax = hi;
                }
                if lo < zmin {
                    zmin = lo;
                }
            }
        }
    }
    let mut average_height = if ceil_count > 0 && floor_count > 0 {
        ceil_average - floor_average
    } else {
        zmax - zmin
    };
    if average_height <= 0.0 {
        average_height = zmax - zmin;
    }
    if state.ceiling_height_m > 0.0 {
        state.ceiling_height_entered = true;
    }
    if state.ceiling_height_m <= 0.0 && average_height > 0.0 {
        state.ceiling_height_m = average_height;
    }
    if !finite_fields(state) {
        return Err(ZoneVolumeError::Nonfinite);
    }
    Ok(())
}

/// Runs signed per-face binary64 dot/3 and current-field priority without bbox.
/// Repair/open-zone fallbacks and nonpositive-result10 are explicitly excluded.
pub fn calculate_zone_volume(state: &mut ZoneGeometryProperties) -> Result<(), ZoneVolumeError> {
    if !finite_fields(state) {
        return Err(ZoneVolumeError::Nonfinite);
    }
    state.diagnostics = topology::inspect(&state.faces, state.p0_m);
    let calculated = state.diagnostics.signed_polyhedron_volume_m3;
    if state.volume_m3 > 0.0 {
        state.diagnostics.volume_differs_by_more_than_five_percent =
            state.diagnostics.initially_closed
                && calculated.is_some_and(|value| {
                    value > 0.0 && (value - state.volume_m3).abs() / state.volume_m3 > 0.05
                });
    } else {
        if let Some(reason) = &state.diagnostics.topology_rejection {
            return Err(ZoneVolumeError::UnsupportedTopology(reason.clone()));
        }
        let calculated = calculated.ok_or(ZoneVolumeError::Nonfinite)?;
        let floor = if state.floor_area_m2 > 0.0 {
            state.floor_area_m2
        } else {
            state.geometric_floor_area_m2
        };
        state.volume_m3 = if state.ceiling_height_entered && floor > 0.0 {
            floor * state.ceiling_height_m
        } else {
            calculated
        };
        if !state.volume_m3.is_finite() {
            return Err(ZoneVolumeError::Nonfinite);
        }
        if state.volume_m3 <= 0.0 {
            return Err(ZoneVolumeError::UnsupportedTopology(
                "selected volume is nonpositive; source-only10m3 fallback is unpaired".into(),
            ));
        }
    }
    state.volume_calculation_count = state.volume_calculation_count.saturating_add(1);
    Ok(())
}

/// Resolves ordinary typed geometry in Wall/Floor/Roof order, retaining the
/// caller's actual within-class order (IDF declaration overlay or epJSON order).
pub fn zone_geometry_properties(
    model: &TypedModel,
    zone: &Zone,
) -> Result<ZoneGeometryProperties, ZoneVolumeError> {
    let mut state = ZoneGeometryProperties {
        volume_m3: zone_geometry_input_value(zone.volume),
        ceiling_height_m: zone_geometry_input_value(zone.ceiling_height),
        user_entered_floor_area_m2: zone_geometry_input_value(zone.floor_area),
        ..ZoneGeometryProperties::default()
    };
    let mut calculated_floor = 0.0;
    for class in [SurfaceType::Wall, SurfaceType::Floor, SurfaceType::Roof] {
        for surface in model.surfaces.iter().filter(|s| {
            s.zone == zone.id
                && (s.surface_type == class
                    || class == SurfaceType::Roof && s.surface_type == SurfaceType::Ceiling)
        }) {
            let face = match ZoneVolumeFace::from_surface(surface) {
                Ok(face) => face,
                Err(error) if state.volume_m3 > 0.0 && state.volume_m3.is_finite() => {
                    // General positive-volume callers retain the supplied value.
                    // Existing surface initialization remains its own admission.
                    state.diagnostics.topology_rejection = Some(error.to_string());
                    state.floor_area_m2 = if state.user_entered_floor_area_m2 > 0.0 {
                        state.user_entered_floor_area_m2
                    } else {
                        calculated_floor
                    };
                    return Ok(state);
                }
                Err(error) => return Err(error),
            };
            if surface.surface_type == SurfaceType::Floor {
                state.has_floor = true;
                calculated_floor += face.area_m2;
                state.geometric_floor_area_m2 += face.gross_area_m2;
            }
            if matches!(
                surface.surface_type,
                SurfaceType::Roof | SurfaceType::Ceiling
            ) {
                state.has_roof = true;
                state.ceiling_area_m2 += face.area_m2;
                state.geometric_ceiling_area_m2 += face.gross_area_m2;
            }
            state.faces.push(face);
        }
    }
    state.floor_area_m2 = if state.user_entered_floor_area_m2 > 0.0 {
        state.has_floor = true;
        state.user_entered_floor_area_m2
    } else {
        calculated_floor
    };
    prepare_zone_height(&mut state)?;
    calculate_zone_volume(&mut state)?;
    Ok(state)
}
