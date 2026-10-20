//! Actual selected GetVertices row writes, before later final-Surface centroid.
use super::{
    Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners,
};
use crate::geometry::{GetVerticesProperties, Orientation, SourceNormalArea};
use ep_model::Point3;

/// Reached actual row quantities; absence distinguishes unexecuted writes.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct VertexProgress {
    pub(crate) convection_orientation: SurfaceConvectionOrientation,
    pub(crate) perimeter: f64,
    pub(crate) normal_area: Option<SourceNormalArea>,
    pub(crate) orientation_return: Option<Orientation>,
    pub(crate) properties: Option<GetVerticesProperties>,
    pub(crate) view_factor_sky: f64,
    pub(crate) view_factor_sky_ir: f64,
    pub(crate) view_factor_ground_ir: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

/// Original ConvectionConstants418-426 and GetSurfConvOrientation6596-6613.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SurfaceConvectionOrientation {
    #[default]
    Invalid,
    HorizontalDown,
    TiltedDownward,
    Vertical,
    TiltedUpward,
    HorizontalUp,
}
fn convection_orientation(tilt: f64) -> SurfaceConvectionOrientation {
    use SurfaceConvectionOrientation as O;
    if tilt < 5.0 {
        return O::HorizontalDown;
    }
    if (5.0..85.0).contains(&tilt) {
        return O::TiltedDownward;
    }
    if (85.0..95.0).contains(&tilt) {
        return O::Vertical;
    }
    if (95.0..175.0).contains(&tilt) {
        return O::TiltedUpward;
    }
    if tilt >= 175.0 {
        return O::HorizontalUp;
    }
    O::Invalid
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoordinateOwner {
    pub(crate) cos_building_relative: f64,
    pub(crate) sin_building_relative: f64,
    pub(crate) cos_appendix_g: f64,
    pub(crate) sin_appendix_g: f64,
    pub(crate) cos_zone_relative: Option<Vec<f64>>,
    pub(crate) sin_zone_relative: Option<Vec<f64>>,
    pub(crate) max_vertices_per_surface: usize,
    pub(crate) aspect_first_time: bool,
    pub(crate) no_aspect_transform: bool,
}
impl Default for CoordinateOwner {
    fn default() -> Self {
        Self {
            cos_building_relative: 0.0,
            sin_building_relative: 0.0,
            cos_appendix_g: 0.0,
            sin_appendix_g: 0.0,
            cos_zone_relative: None,
            sin_zone_relative: None,
            max_vertices_per_surface: 4,
            aspect_first_time: true,
            no_aspect_transform: true,
        }
    }
}

fn point(value: [f64; 3]) -> Point3 {
    Point3 {
        x_m: value[0],
        y_m: value[1],
        z_m: value[2],
    }
}
fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    let x = a[0] - b[0];
    let y = a[1] - b[1];
    let z = a[2] - b[2];
    (x * x + y * y + z * z).sqrt()
}
fn rounded(value: f64, precision: usize) -> Result<String, Error> {
    super::super::rounded(value, precision)
}
fn counter(value: &mut i32) -> Result<(), Error> {
    *value = value.checked_add(1).ok_or(Error::Unavailable(
        "Original signed geometry counter overflow",
    ))?;
    Ok(())
}
fn coincident(vertices: &[[f64; 3]]) -> (f64, Option<(usize, usize)>) {
    let mut distances = Vec::with_capacity(vertices.len());
    let mut minimum = f64::MAX;
    let mut perimeter = 0.0;
    for i in 0..vertices.len() {
        let d = distance(vertices[i], vertices[(i + 1) % vertices.len()]);
        distances.push(d);
        if d < minimum {
            minimum = d;
        }
        perimeter += d;
    }
    if minimum >= 0.01 {
        return (perimeter, None);
    }
    let mut minimum_weight = f64::MAX;
    let mut selected = None;
    for i in 0..vertices.len() {
        let previous = if i == 0 { vertices.len() - 1 } else { i - 1 };
        let next_distance = distances[i];
        let previous_distance = distances[previous];
        if next_distance >= 0.01 && previous_distance >= 0.01 {
            continue;
        }
        let weight = next_distance + previous_distance;
        if weight < minimum_weight {
            minimum_weight = weight;
            selected = Some((
                i,
                if previous_distance < next_distance {
                    previous
                } else {
                    (i + 1) % vertices.len()
                },
            ));
        }
    }
    (perimeter, selected)
}

impl SurfaceGeometryInputState {
    /// Actual SetupZoneGeometry272-289, before GetSurfaceData's first getter.
    pub(crate) fn initialize_coordinate_owners(
        &mut self,
        owners: &SurfaceGetterOwners<'_>,
    ) -> Result<(), Error> {
        let rad = std::f64::consts::PI / 180.0;
        let rotation = owners.control.building_rotation_appendix_g;
        let angle = -(owners.control.building_azimuth + rotation) * rad;
        self.coordinate.cos_building_relative = angle.cos();
        self.coordinate.sin_building_relative = angle.sin();
        let angle = -rotation * rad;
        self.coordinate.cos_appendix_g = angle.cos();
        self.coordinate.sin_appendix_g = angle.sin();
        let zones = owners.zones.zones.as_ref().ok_or(Error::Unavailable(
            "actual Zone allocation before SetupZoneGeometry",
        ))?;
        self.coordinate.cos_zone_relative = Some(vec![0.0; zones.len()]);
        self.coordinate.sin_zone_relative = Some(vec![0.0; zones.len()]);
        for (i, zone) in zones.iter().enumerate() {
            self.coordinate
                .cos_zone_relative
                .as_mut()
                .ok_or(Error::Unavailable("Zone trig allocation"))?[i] =
                (-zone.relative_north * rad).cos();
            self.coordinate
                .sin_zone_relative
                .as_mut()
                .ok_or(Error::Unavailable("Zone trig allocation"))?[i] =
                (-zone.relative_north * rad).sin();
        }
        Ok(())
    }

    /// Caller supplies the same currently filled numeric N3 onward arguments.
    pub(crate) fn get_world_vertices(
        &mut self,
        slot: usize,
        sides: usize,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.pending_vertices != Some(slot) || !(1..=4).contains(&self.corner) {
            return Err(Error::Unavailable(
                "actual pending GetVertices/geometry corner owner",
            ));
        }
        if sides > self.coordinate.max_vertices_per_surface {
            self.coordinate.max_vertices_per_surface = sides;
        }
        let row = self
            .surface_tmp
            .as_mut()
            .and_then(|rows| rows.get_mut(slot))
            .ok_or(Error::Unavailable("actual SurfaceTmp row"))?;
        let vertex = row
            .vertex
            .as_mut()
            .ok_or(Error::Unavailable("actual allocated Vertex"))?;
        if vertex.len() != sides {
            return Err(Error::Unavailable("actual Vertex allocation bounds"));
        }
        let mut ptr = 2;
        for value in vertex.iter_mut() {
            let x = *owners
                .shortcut
                .numeric_args
                .get(ptr)
                .ok_or(Error::Unavailable("actual GetVertices numeric slice"))?;
            ptr += 1;
            let y = *owners
                .shortcut
                .numeric_args
                .get(ptr)
                .ok_or(Error::Unavailable("actual GetVertices numeric slice"))?;
            ptr += 1;
            let z = *owners
                .shortcut
                .numeric_args
                .get(ptr)
                .ok_or(Error::Unavailable("actual GetVertices numeric slice"))?;
            ptr += 1;
            *value = Some([x, y, z]);
        }
        // The admitted producer is an opaque surface with a genuine owning
        // Zone. Detached_B/shading transforms require their own real getters;
        // a missing Zone is never replaced by a synthetic zero origin.
        if row.zone <= 0
            || !matches!(
                row.class,
                SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof
            )
        {
            return Err(Error::Unavailable("actual opaque GetVertices owning Zone"));
        }
        // Original9230-9243 preserves vertex1 and reverses vertices2..N.
        if !self.counterclockwise {
            let mut source = sides;
            let mut target = 2;
            // Preserve Original9237's independent source/target counter writes.
            #[allow(clippy::explicit_counter_loop)]
            for _ in 0..sides.saturating_sub(1) / 2 {
                vertex.swap(source - 1, target - 1);
                source -= 1;
                target += 1;
            }
        }
        // Original9248-9276 is a sequence of swaps, not a generic rotation.
        // The triangle UpperRightCorner break is intentional.
        let mut this_corner = usize::try_from(self.corner)
            .map_err(|_| Error::Unavailable("actual positive geometry corner"))?;
        while this_corner != 1 {
            if sides < 4 && this_corner == 4 {
                break;
            }
            let mut target = this_corner;
            let mut source = this_corner + 1;
            if source > sides {
                source = 1;
            }
            if target > sides || source > sides {
                return Err(Error::Unavailable(
                    "actual geometry corner permutation bounds",
                ));
            }
            for _ in 0..sides.saturating_sub(1) {
                vertex.swap(target - 1, source - 1);
                target += 1;
                source += 1;
                if target > sides {
                    target = 1;
                }
                if source > sides {
                    source = 1;
                }
            }
            this_corner += 1;
            if this_corner > sides {
                this_corner = 1;
            }
        }
        if !self.world_coordinates {
            let zone_index = usize::try_from(row.zone - 1)
                .map_err(|_| Error::Unavailable("actual GetVertices Zone index"))?;
            let zone = owners
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(zone_index))
                .ok_or(Error::Unavailable(
                    "same actual GetVertices Zone allocation",
                ))?;
            let cos_zone = self
                .coordinate
                .cos_zone_relative
                .as_ref()
                .and_then(|rows| rows.get(zone_index))
                .copied()
                .ok_or(Error::Unavailable("same actual Zone cosine owner"))?;
            let sin_zone = self
                .coordinate
                .sin_zone_relative
                .as_ref()
                .and_then(|rows| rows.get(zone_index))
                .copied()
                .ok_or(Error::Unavailable("same actual Zone sine owner"))?;
            // Original9283-9290: both intermediate coordinates read the OLD
            // x/y before either write; preserve each multiply/add and z +=.
            for value in vertex.iter_mut() {
                let xyz = value
                    .as_mut()
                    .ok_or(Error::Unavailable("actual written Vertex"))?;
                let xb = xyz[0] * cos_zone - xyz[1] * sin_zone + zone.origin_m[0];
                let yb = xyz[0] * sin_zone + xyz[1] * cos_zone + zone.origin_m[1];
                xyz[0] = xb * self.coordinate.cos_building_relative
                    - yb * self.coordinate.sin_building_relative;
                xyz[1] = xb * self.coordinate.sin_building_relative
                    + yb * self.coordinate.cos_building_relative;
                xyz[2] += zone.origin_m[2];
            }
        } else if row.zone > 0 {
            // Existing genuine World Appendix-G-only operations stay intact.
            for value in vertex.iter_mut() {
                let xyz = value
                    .as_mut()
                    .ok_or(Error::Unavailable("actual written Vertex"))?;
                let x = xyz[0];
                let y = xyz[1];
                xyz[0] = x * self.coordinate.cos_appendix_g - y * self.coordinate.sin_appendix_g;
                xyz[1] = x * self.coordinate.sin_appendix_g + y * self.coordinate.cos_appendix_g;
            }
        }
        if sides <= 2 {
            return match backend.show_fatal_error(&format!(
                "GetVertices: Called with less than 2 sides, Surface={}",
                row.name
            )) {
                Err(error) => Err(Error::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        loop {
            let values = vertex
                .iter()
                .copied()
                .collect::<Option<Vec<_>>>()
                .ok_or(Error::Unavailable("complete written Vertex"))?;
            let (perimeter, selected) = coincident(&values);
            let Some((popped, kept)) = selected else {
                row.vertex_progress.perimeter = perimeter;
                break;
            };
            if owners.globals.display_extra_warnings {
                backend.show_warning_error(&format!("GetVertices: Distance between two vertices < .01, possibly coincident. for Surface={}, in Zone={}",row.name,row.zone_name))?;
                let popped_index = popped + 1;
                let kept_index = kept + 1;
                let popped_first = if popped_index < kept_index {
                    !(popped_index == 1 && kept_index == vertex.len())
                } else {
                    popped_index == vertex.len() && kept_index == 1
                };
                for index in if popped_first {
                    [popped, kept]
                } else {
                    [kept, popped]
                } {
                    let xyz = values[index];
                    backend.show_continue_error(&format!(
                        "Vertex [{}]=({},{},{})",
                        index + 1,
                        rounded(xyz[0], 2)?,
                        rounded(xyz[1], 2)?,
                        rounded(xyz[2], 2)?
                    ))?;
                }
            }
            counter(&mut backend.state.tracking.total_coincident_vertices)?;
            if vertex.len() <= 3 {
                if owners.globals.display_extra_warnings {
                    backend.show_continue_error(&format!("Cannot Drop Vertex [{}]; Number of Surface Sides at minimum. This surface is now a degenerate surface.",popped+1))?;
                }
                counter(&mut backend.state.tracking.total_degenerate_surfaces)?;
                row.vertex_progress.perimeter = perimeter;
                break;
            }
            if owners.globals.display_extra_warnings {
                backend.show_continue_error(&format!("Dropping Vertex [{}].", popped + 1))?;
            }
            row.sides -= 1;
            vertex.remove(popped);
        }
        let vertices = vertex
            .iter()
            .copied()
            .collect::<Option<Vec<_>>>()
            .ok_or(Error::Unavailable("written SurfaceTmp coordinates"))?;
        let points: Vec<_> = vertices.iter().copied().map(point).collect();
        let normal_area = crate::geometry::source_normal_area(&points);
        row.vertex_progress.normal_area = Some(normal_area); // Actual9383-9388 retained before warning failures.
        row.gross_area = normal_area.gross_area_m2; // Original9387
        row.area = row.gross_area; //9388 same mutable SurfaceData member
        row.net_area_shadow_calc = row.area; //9389, before orientation/warnings
        let orientation =
            crate::geometry::orientation_from_normal(&points, normal_area.newell_normal);
        row.vertex_progress.orientation_return = Some(orientation); // actual local angles + owner local axes.
        let dot = normal_area.newell_normal[2];
        let (inverted, steep, noun, expected) = match row.class {
            SurfaceClass::Roof => (
                dot < -0.000001,
                orientation.tilt_deg > 80.0,
                "Roof/Ceiling",
                "0",
            ),
            SurfaceClass::Floor => (dot > 0.000001, orientation.tilt_deg < 158.2, "Floor", "180"),
            _ => (false, false, "", ""),
        };
        if inverted || steep {
            let description = if inverted {
                "is upside down!"
            } else {
                "is not oriented correctly!"
            };
            backend.show_warning_error(&format!("GetVertices: {noun} {description} Tilt angle=[{}], should be near {expected}, Surface=\"{}\", in Zone=\"{}\".",rounded(orientation.tilt_deg,1)?,row.name,row.zone_name))?;
            if inverted {
                backend.show_continue_error("Automatic fix is attempted.")?;
                // The genuine next call writes the real debug stream. This selected
                // warning-free route cannot substitute a discarded/fake debug sink.
                return Err(Error::Unavailable(
                    "ReverseAndRecalculate requires genuine debug-file host",
                ));
            }
        }
        row.vertex_progress.convection_orientation = convection_orientation(orientation.tilt_deg);
        let quantities = crate::geometry::finish_get_vertices_properties(normal_area, orientation);
        row.vertex_progress.properties = Some(quantities);
        if row.view_factor_ground == -99999.0 {
            row.view_factor_ground = 0.5 * (1.0 - quantities.cos_tilt);
        }
        row.vertex_progress.view_factor_sky = 0.5 * (1.0 + quantities.cos_tilt);
        row.vertex_progress.view_factor_sky_ir = row.vertex_progress.view_factor_sky;
        row.vertex_progress.view_factor_ground_ir = 0.5 * (1.0 - quantities.cos_tilt);
        if self.coordinate.aspect_first_time {
            if owners.input.get_num_objects_found("GeometryTransform") == 1 {
                return Err(Error::Unavailable(
                    "positive GeometryTransform actual local getter",
                ));
            }
            self.coordinate.aspect_first_time = false;
        }
        if !self.coordinate.no_aspect_transform {
            return Err(Error::Unavailable("actual positive aspect transform"));
        }
        let width = distance(vertices[2], vertices[1]);
        let height = distance(vertices[1], vertices[0]);
        row.vertex_progress.height = height;
        row.vertex_progress.width = width;
        // All actual row writes are retained before this bounded physical-domain stop.
        if !quantities.is_finite() {
            return Err(Error::Unavailable(
                "nonfinite derived GetVertices owner cannot initialize physical surface",
            ));
        }
        self.returned_from_vertices(slot)
    }
}
