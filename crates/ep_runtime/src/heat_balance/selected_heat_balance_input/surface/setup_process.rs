//! Actual selected Setup313-369 owners; full Setup and370+ remain pending.
use super::final_mapping::FinalSurface;
use super::parallel_arrays::SurfaceParallelArrays;
use super::{Error, SurfaceClass, SurfaceGeometryInputState};
use crate::geometry::{
    SourceProcessShape, source_process_coplanarity, source_process_distance,
    source_process_is_rectangle, source_process_shift, source_process_sky_rad_split,
    source_process_translation,
};
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use ep_model::Point3;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProcessVerticesScratch {
    pub(crate) one_time: bool,
    pub(crate) x: Option<Vec<f64>>,
    pub(crate) y: Option<Vec<f64>>,
    pub(crate) z: Option<Vec<f64>>,
}
impl Default for ProcessVerticesScratch {
    fn default() -> Self {
        Self {
            one_time: true,
            x: None,
            y: None,
            z: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SetupShadingCounters {
    pub(crate) building: i32,
    pub(crate) fixed: i32,
    pub(crate) attached: i32,
    pub(crate) first: i32,
    pub(crate) last: i32,
}
impl Default for SetupShadingCounters {
    fn default() -> Self {
        Self {
            building: 0,
            fixed: 0,
            attached: 0,
            first: 0,
            last: -1,
        }
    }
}

fn process_base_surface(
    surface: &mut FinalSurface,
    index: usize,
    scratch: &mut ProcessVerticesScratch,
    arrays: &mut SurfaceParallelArrays,
    max_vertices: usize,
) -> Result<(), Error> {
    let row = &mut surface.row;
    if row.vertices_processed {
        return Ok(());
    } // Actual12900 short circuit.
    if scratch.one_time {
        scratch.x = Some(vec![0.0; max_vertices]);
        scratch.y = Some(vec![0.0; max_vertices]);
        scratch.z = Some(vec![0.0; max_vertices]);
        scratch.one_time = false;
    }
    let source_number = super::signed(index + 1)?;
    // This is the actual raw self-base writer plus the final copy bijection,
    // checked at use; no typed index or model-only assumption substitutes.
    if row.base_surface != source_number || !row.heat_transfer_surface || row.is_shadowing {
        return Err(Error::Unavailable(
            "nonopaque/nonself ProcessSurfaceVertices branch",
        ));
    }
    let sides =
        usize::try_from(row.sides).map_err(|_| Error::Unavailable("actual Process sides"))?;
    let vertices = row
        .vertex
        .as_ref()
        .ok_or(Error::Unavailable("actual final Process vertices"))?;
    if sides < 3 || vertices.len() != sides {
        return Err(Error::Unavailable("complete actual Process vertex bounds"));
    }
    let points = vertices
        .iter()
        .map(|value| {
            let [x_m, y_m, z_m] =
                value.ok_or(Error::Unavailable("actual written Process vertex"))?;
            Ok(Point3 { x_m, y_m, z_m })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    if !row.mirrored_surface {
        let coplanarity = source_process_coplanarity(&points);
        if !coplanarity.is_coplanar {
            // Native emits warning/severe with .5T and then continues. That
            // real formatter branch is pending, not silently suppressed.
            return Err(Error::Unavailable(
                "nonplanar Process .5T diagnostic branch",
            ));
        }
    }
    let x = scratch
        .x
        .as_mut()
        .ok_or(Error::Unavailable("actual Xpsv allocation"))?;
    let y = scratch
        .y
        .as_mut()
        .ok_or(Error::Unavailable("actual Ypsv allocation"))?;
    let z = scratch
        .z
        .as_mut()
        .ok_or(Error::Unavailable("actual Zpsv allocation"))?;
    if x.len() != max_vertices
        || y.len() != max_vertices
        || z.len() != max_vertices
        || sides > max_vertices
    {
        return Err(Error::Unavailable("actual Process scratch bounds"));
    }
    for (i, point) in points.iter().enumerate() {
        x[i] = point.x_m;
        y[i] = point.y_m;
        z[i] = point.z_m;
    } // Tail slots remain the same persistent prior values.
    row.vertex_progress.width = source_process_distance(points[2], points[1]);
    row.vertex_progress.height = source_process_distance(points[1], points[0]);
    row.shape = if sides == 3 {
        SourceProcessShape::Triangle
    } else if sides == 4 {
        let quad = [points[0], points[1], points[2], points[3]];
        if source_process_is_rectangle(&quad) {
            SourceProcessShape::Rectangle
        } else {
            SourceProcessShape::Quadrilateral
        }
    } else {
        SourceProcessShape::Polygonal
    };
    if sides > 4 {
        let gross_area = row
            .vertex_progress
            .normal_area
            .ok_or(Error::Unavailable("actual Process GrossArea"))?
            .gross_area_m2;
        if (row.vertex_progress.height * row.vertex_progress.width - gross_area).abs() > 0.001 {
            row.vertex_progress.width = gross_area.sqrt();
            row.vertex_progress.height = row.vertex_progress.width;
        }
    }
    // Native selected BaseSurface ErrorInSurface remains false. Its other
    // true branch sets ErrorsFound and returns only from Process; Setup would
    // continue. No false Native abort or fabricated ErrorsFound write is added.
    let shade = arrays
        .shade_vertices
        .get_mut(index)
        .ok_or(Error::Unavailable("same ShadeV owner"))?;
    shade.n_vert = row.sides;
    shade.xv = Some(vec![0.0; sides]);
    shade.yv = Some(vec![0.0; sides]);
    shade.zv = Some(vec![0.0; sides]);
    for (i, &x_value) in x.iter().take(sides).enumerate() {
        shade
            .xv
            .as_mut()
            .ok_or(Error::Unavailable("actual ShadeV XV"))?[i] = x_value;
        shade
            .yv
            .as_mut()
            .ok_or(Error::Unavailable("actual ShadeV YV"))?[i] = y[i];
        shade
            .zv
            .as_mut()
            .ok_or(Error::Unavailable("actual ShadeV ZV"))?[i] = z[i];
    }
    let origin = source_process_translation(&points).ok_or(Error::Unavailable(
        "CalcCoordinateTransformation invalid-dot severe/vertex/optional-ESO fatal branch",
    ))?; // Retain every preceding ShadeV allocation/value on this failure.
    *arrays
        .x0
        .get_mut(index)
        .ok_or(Error::Unavailable("actual X0 owner"))? = origin[0];
    *arrays
        .y0
        .get_mut(index)
        .ok_or(Error::Unavailable("actual Y0 owner"))? = origin[1];
    *arrays
        .z0
        .get_mut(index)
        .ok_or(Error::Unavailable("actual Z0 owner"))? = origin[2];
    let properties = row
        .vertex_progress
        .properties
        .ok_or(Error::Unavailable("actual stored Process local axes"))?;
    row.x_shift = source_process_shift(properties.lcsx, points[1], origin);
    row.y_shift = source_process_shift(properties.lcsy, points[1], origin);
    row.vertices_processed = true;
    Ok(()) // Selected actual ProcessSurfaceVertices return, not whole Setup.
}

impl SurfaceGeometryInputState {
    pub(crate) fn process_opaque_setup_surfaces(
        &mut self,
        zones: &mut SelectedZoneInputOwners,
        control: &mut SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        control.calculate_window_reveal_reflection = false;
        self.setup_shading.building = 0;
        self.setup_shading.fixed = 0;
        self.setup_shading.attached = 0;
        self.setup_shading.first = 0;
        self.setup_shading.last = -1;
        let total = usize::try_from(self.total_surfaces)
            .map_err(|_| Error::Unavailable("actual Setup total"))?;
        super::lists::reserve(
            &mut self.surface_lists.all_exterior_solar_and_shading,
            total,
        );
        let surfaces = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("same final Setup Surface owner"))?;
        let arrays = self
            .parallel_arrays
            .as_mut()
            .ok_or(Error::Unavailable("same Setup parallel arrays"))?;
        if surfaces.len() != total || arrays.surf_air_sky_rad_split.len() != total {
            return Err(Error::Unavailable("actual Setup Surface/parallel bounds"));
        }
        for (index, surface) in surfaces.iter_mut().enumerate() {
            let properties = surface
                .row
                .vertex_progress
                .properties
                .ok_or(Error::Unavailable("actual Setup CosTilt"))?;
            arrays.surf_air_sky_rad_split[index] =
                source_process_sky_rad_split(properties.cos_tilt);
            surface.row.is_shadowing = false;
            if matches!(
                surface.row.class,
                SurfaceClass::Shading
                    | SurfaceClass::DetachedFixed
                    | SurfaceClass::DetachedBuilding
            ) {
                return Err(Error::Unavailable(
                    "positive Setup shadowing/list/counter class branch",
                ));
            }
            if !matches!(
                surface.row.class,
                SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof
            ) {
                return Err(Error::Unavailable(
                    "nonopaque Setup Process/IntMass class branch",
                ));
            }
            if surface.row.heat_transfer_surface && surface.row.exterior_solar {
                self.surface_lists
                    .all_exterior_solar_and_shading
                    .push(super::signed(index + 1)?);
            }
            // The three shading counter predicates and IntMass skip are false
            // for this actual W/F/R class; positive getters stay excluded.
            process_base_surface(
                surface,
                index,
                &mut self.process_vertices_scratch,
                arrays,
                self.coordinate.max_vertices_per_surface,
            )?;
        }
        for zone in zones
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Setup Zone owner"))?
        {
            zone.exterior_window_area = 0.0;
            zone.has_window = false;
            zone.exterior_gross_wall_area = 0.0;
            zone.exterior_net_wall_area = 0.0;
            zone.total_surface_area = 0.0;
        }
        for space in zones
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("same Setup Space owner"))?
        {
            space.exterior_window_area = 0.0;
            space.total_surface_area = 0.0;
        }
        Ok(()) // Stop before actual370 DetailedWWR/376 area accumulation.
    }
}
