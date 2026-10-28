//! Reached explicit opaque Surface-name checks and bounded final reconciliation.
//! Native SurfaceGeometry4396-4425,1785-2109; no inferred typed ordinals.
use super::final_mapping::InterzoneExposurePath;
use super::{
    Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners,
    SurfaceTmpInput,
};
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_material_data::{SelectedMaterialData, SourceMaterialGroup};

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Error::Unavailable("actual positive paired Surface ordinal"))
}

// Read the existing Source array only when the Native branch reaches it.
fn nominal_u_difference(values: Option<&[f64]>, current: i32, found: i32) -> Result<f64, Error> {
    let values = values.ok_or(Error::Unavailable("actual paired NominalU allocation"))?;
    let current_u = *values
        .get(index(current)?)
        .ok_or(Error::Unavailable("actual paired NominalU slot"))?;
    let found_u = *values
        .get(index(found)?)
        .ok_or(Error::Unavailable("actual adjacent NominalU slot"))?;
    if !current_u.is_finite() || !found_u.is_finite() {
        return Err(Error::Unavailable("bounded finite paired NominalU values"));
    }
    Ok((current_u - found_u).abs())
}

fn world_vertices(row: &SurfaceTmpInput) -> Result<Vec<[f64; 3]>, Error> {
    row.vertex
        .as_ref()
        .ok_or(Error::Unavailable("actual paired World Vertex allocation"))?
        .iter()
        .map(|point| {
            let point = point.ok_or(Error::Unavailable("actual written paired World Vertex"))?;
            if point.iter().any(|value| !value.is_finite()) {
                return Err(Error::Unavailable("finite paired World Vertex domain"));
            }
            Ok(point)
        })
        .collect()
}

fn reversed_world_vertices(a: &SurfaceTmpInput, b: &SurfaceTmpInput) -> Result<bool, Error> {
    let a = world_vertices(a)?;
    let b = world_vertices(b)?;
    if a.len() < 3 || a.len() != b.len() {
        return Ok(false);
    }
    // Permit any legal starting vertex, retaining the opposite cyclic order.
    Ok((0..b.len()).any(|offset| {
        a.iter()
            .zip(b.iter().rev().cycle().skip(offset))
            .all(|(left, right)| left == right)
    }))
}

impl SurfaceGeometryInputState {
    /// The post-opaque GetHTSurfaceData name/Vertex.size check, before later gets.
    /// Each ErrorsFound write follows all of its actual reached diagnostics.
    pub(crate) fn check_explicit_pair_vertices(
        &self,
        module: &str,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        let rows = self.surface_tmp.as_ref().ok_or(Error::Unavailable(
            "actual SurfaceTmp pair-check allocation",
        ))?;
        if self.pending_vertices.is_some() || self.surfaces_read > rows.len() {
            return Err(Error::Unavailable(
                "all actual opaque Vertex returns before pair check",
            ));
        }
        for surface in rows.iter().take(self.surfaces_read) {
            if surface.exterior_boundary != -999 || surface.exterior_boundary_name.is_empty() {
                continue;
            }
            // UtilityRoutines.hh441-447 uses exact already-uppercased Source names.
            let found = rows
                .iter()
                .find(|candidate| candidate.name == surface.exterior_boundary_name);
            let Some(found) = found else {
                backend.show_severe_error(&format!(
                    "{module}=\"{}\" references an outside boundary surface that cannot be found:{}",
                    surface.name, surface.exterior_boundary_name
                ))?;
                *errors = true;
                continue;
            };
            // The genuine default/unallocated Native Vertex array has size0.
            let size = surface.vertex.as_ref().map_or(0, Vec::len);
            let found_size = found.vertex.as_ref().map_or(0, Vec::len);
            if size != found_size {
                backend.show_severe_error(&format!(
                    "{module}=\"{}\", Vertex size mismatch between base surface :{} and outside boundary surface: {}",
                    surface.name, surface.name, found.name
                ))?;
                backend.show_continue_error(&format!(
                    "The vertex sizes are {size} for base surface and {found_size} for outside boundary surface. Please check inputs."
                ))?;
                *errors = true;
            }
        }
        Ok(())
    }

    /// Resolve final Source IDs in their actual moved order. Unsupported warning
    /// branches stop at their first unavailable operation; no diagnostics vanish.
    pub(crate) fn reconcile_opaque_surface_boundaries(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
        materials: Option<&SelectedMaterialData>,
    ) -> Result<(), Error> {
        let rows = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("actual final paired Surface allocation"))?;
        if self.moved_surfaces != rows.len() {
            return Err(Error::Unavailable("actual MovedSurfs pair-search bounds"));
        }
        self.iz_const_diff_message = Some(false); // Actual1776 invocation local.
        for current in 0..self.moved_surfaces {
            if !rows[current].row.heat_transfer_surface
                || rows[current].row.exterior_boundary != -999
            {
                continue;
            }
            let number = super::signed(current + 1)?;
            if rows[current].row.exterior_boundary_name == rows[current].row.name {
                // Preserve the established self-Adiabatic1788-1794 write.
                rows[current].row.exterior_boundary = number;
                continue;
            }
            if rows[current].row.exterior_boundary_name.is_empty() {
                return Err(Error::Unavailable(
                    "positive opaque empty adjacency continuation",
                ));
            }
            let found = rows
                .iter()
                .take(self.moved_surfaces)
                .position(|candidate| {
                    candidate.row.name == rows[current].row.exterior_boundary_name
                })
                .ok_or(Error::Unavailable(
                    "actual adjacent final Surface not found2133",
                ))?;
            // Actual1794 assigns Found BEFORE checking the matching boundary.
            rows[current].row.exterior_boundary = super::signed(found + 1)?;
            let surface = &rows[current].row;
            let adjacent = &rows[found].row;
            if adjacent.exterior_boundary <= 0 && adjacent.exterior_boundary != -999 {
                return Err(Error::Unavailable(
                    "positive OtherZoneSurface mismatch diagnostic1798",
                ));
            }
            if surface.space == adjacent.space {
                return Err(Error::Unavailable(
                    "positive same-Space adjacency counter1814",
                ));
            }
            if surface.zone == adjacent.zone
                || !matches!(
                    surface.class,
                    SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof
                )
                || !matches!(
                    adjacent.class,
                    SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof
                )
                || surface.base_surface != number
                || adjacent.base_surface != super::signed(found + 1)?
                || adjacent.exterior_boundary_name != surface.name
            {
                return Err(Error::Unavailable(
                    "bounded explicit reciprocal interzone opaque class domain",
                ));
            }
            let construction = owners
                .control
                .construction_usage
                .get(index(surface.construction)?)
                .ok_or(Error::Unavailable("actual paired Source Construction"))?;
            let adjacent_construction = owners
                .control
                .construction_usage
                .get(index(adjacent.construction)?)
                .ok_or(Error::Unavailable("actual adjacent Source Construction"))?;
            // Ceiling already passed the real getter's CEILING-to-ROOF alias.
            // Only normal opaque classes expand; never extend the IRT Wall lane
            // through a prelatched or exact-identity construction shortcut.
            if (construction.type_is_irt || adjacent_construction.type_is_irt)
                && (surface.class != SurfaceClass::Wall || adjacent.class != SurfaceClass::Wall)
            {
                return Err(Error::Unavailable("existing IRT paired Wall-only domain"));
            }
            // Native1842/1846 continue AFTER current Found but BEFORE all
            // layer/area/exposure checks and the opposing reciprocal write.
            if construction.reverse_construction_num_layers_warning
                && adjacent_construction.reverse_construction_num_layers_warning
            {
                rows[current].interzone_exposure_path =
                    InterzoneExposurePath::SkippedNumLayersWarning;
                continue;
            }
            if construction.reverse_construction_layers_order_warning
                && adjacent_construction.reverse_construction_layers_order_warning
            {
                rows[current].interzone_exposure_path =
                    InterzoneExposurePath::SkippedLayerOrderWarning;
                continue;
            }
            if !(1..=10).contains(&construction.total_layers)
                || !(1..=10).contains(&adjacent_construction.total_layers)
                || (construction.type_is_irt && construction.total_layers != 1)
                || (adjacent_construction.type_is_irt && adjacent_construction.total_layers != 1)
                || construction.type_is_window
                || adjacent_construction.type_is_window
                || construction.type_is_air_boundary
                || adjacent_construction.type_is_air_boundary
                || construction.source_sink_present
                || adjacent_construction.source_sink_present
            {
                return Err(Error::Unavailable(
                    "bounded opaque Source Construction counts",
                ));
            }
            if construction.total_layers != adjacent_construction.total_layers {
                // Native1851-1871: unequal counts read only the ACTUAL NominalU
                // owner; there is no layer comparison, warning or latch write
                // when the difference is within0.001. Nonfinite values remain
                // an explicit boundary of this bounded normal-opaque stage.
                if construction.type_is_irt || adjacent_construction.type_is_irt {
                    return Err(Error::Unavailable(
                        "unequal-count normal opaque pair domain",
                    ));
                }
                if nominal_u_difference(
                    owners.control.nominal_u.as_deref(),
                    surface.construction,
                    adjacent.construction,
                )? > 0.001
                {
                    return Err(Error::Unavailable(
                        "unequal-count actual NominalU difference",
                    ));
                }
            } else {
                let layers = usize::try_from(construction.total_layers)
                    .map_err(|_| Error::Unavailable("actual paired TotLayers"))?;
                let mut different = false;
                for layer in 0..layers {
                    let original = construction.layer_points[layer];
                    let reversed = adjacent_construction.layer_points[layers - layer - 1];
                    if original <= 0 || reversed <= 0 {
                        return Err(Error::Unavailable(
                            "actual positive reversed Source LayerPoint",
                        ));
                    }
                    if original == reversed {
                        continue;
                    }
                    // Native15495-15599: fetch BOTH actual material groups only
                    // for distinct IDs. No typed property-equivalence surrogate.
                    let data = materials
                        .ok_or(Error::Unavailable("actual paired Material group owner"))?;
                    let group = data.source_group_at(original)?;
                    let reverse_group = data.source_group_at(reversed)?;
                    if !matches!(
                        group,
                        SourceMaterialGroup::Regular | SourceMaterialGroup::AirGap
                    ) || !matches!(
                        reverse_group,
                        SourceMaterialGroup::Regular | SourceMaterialGroup::AirGap
                    ) || construction.type_is_irt
                        || adjacent_construction.type_is_irt
                    {
                        return Err(Error::Unavailable(
                            "normal opaque reversed Material group domain",
                        ));
                    }
                    different = true;
                    break; // Native opaque mismatch immediately exits.
                }
                if different {
                    let difference = nominal_u_difference(
                        owners.control.nominal_u.as_deref(),
                        surface.construction,
                        adjacent.construction,
                    )?;
                    if difference > 0.001 {
                        return Err(Error::Unavailable(
                            "mismatched-layer actual NominalU Severe branch",
                        ));
                    }
                    let flags = flags.ok_or(Error::Unavailable(
                        "actual interzone Construction warning phase owners",
                    ))?;
                    backend.show_source_warning_error(&format!(
                        "GetSurfaceData: Construction {} of interzone surface {} does not have the same materials in the reverse order as the construction {} of adjacent surface {}",
                        construction.name,surface.name,adjacent_construction.name,adjacent.name),flags)?;
                    backend.show_continue_error("or the properties of the reversed layers are not correct due to differing layer front and back side values")?;
                    // Formatting occurs at the reached THIRD record, after the
                    // warning and preceding Continue have both returned.
                    backend.show_continue_error(&format!(
                        "...but Nominal U values are similar, diff=[{}] ... simulation proceeds.",
                        super::super::rounded(difference, 4)?
                    ))?;
                    if self.iz_const_diff_message == Some(false) {
                        backend.show_continue_error("...if the two zones are expected to have significantly different temperatures, the proper \"reverse\" construction should be created.")?;
                        self.iz_const_diff_message = Some(true);
                    }
                    let current_construction = index(surface.construction)?;
                    let found_construction = index(adjacent.construction)?;
                    if !owners.control.construction_usage[current_construction]
                        .reverse_construction_layers_order_warning
                        || !owners.control.construction_usage[found_construction]
                            .reverse_construction_layers_order_warning
                    {
                        backend.show_continue_error(
                            "...this problem for this pair will not be reported again.",
                        )?;
                        owners.control.construction_usage[current_construction]
                            .reverse_construction_layers_order_warning = true;
                        owners.control.construction_usage[found_construction]
                            .reverse_construction_layers_order_warning = true;
                    }
                }
            }
            let zones = owners
                .zones
                .zones
                .as_ref()
                .ok_or(Error::Unavailable("actual paired Source Zone allocation"))?;
            let zone = zones
                .get(index(surface.zone)?)
                .ok_or(Error::Unavailable("actual paired Source Zone"))?;
            let adjacent_zone = zones
                .get(index(adjacent.zone)?)
                .ok_or(Error::Unavailable("actual adjacent Source Zone"))?;
            let mult_found = adjacent_zone
                .multiplier
                .checked_mul(adjacent_zone.list_multiplier)
                .ok_or(Error::Unavailable("actual adjacent signed Zone factors"))?;
            let mult_surface = zone
                .multiplier
                .checked_mul(zone.list_multiplier)
                .ok_or(Error::Unavailable("actual paired signed Zone factors"))?;
            if mult_found <= 0
                || mult_surface <= 0
                || zone.multiplier != adjacent_zone.multiplier
                || zone.list_multiplier != adjacent_zone.list_multiplier
                || !surface.area.is_finite()
                || !adjacent.area.is_finite()
                || surface.area <= 0.0
                || adjacent.area <= 0.0
            {
                return Err(Error::Unavailable(
                    "bounded equal positive Zone factors and paired area",
                ));
            }
            // Preserve Native1937-1939 parentheses, including /Area * MultFound.
            if ((adjacent.area * f64::from(mult_found) - surface.area * f64::from(mult_surface))
                / adjacent.area
                * f64::from(mult_found))
            .abs()
                > 0.02
            {
                return Err(Error::Unavailable(
                    "positive interzone area mismatch counter1940",
                ));
            }
            let properties = surface
                .vertex_progress
                .properties
                .as_ref()
                .ok_or(Error::Unavailable("actual paired GetVertices properties"))?;
            let adjacent_properties = adjacent
                .vertex_progress
                .properties
                .as_ref()
                .ok_or(Error::Unavailable("actual adjacent GetVertices properties"))?;
            if !properties.tilt_deg.is_finite()
                || (!matches!(surface.class, SurfaceClass::Roof | SurfaceClass::Floor)
                    && (!properties.azimuth_deg.is_finite() || !properties.sin_tilt.is_finite()))
                || !adjacent_properties.tilt_deg.is_finite()
                || (!matches!(surface.class, SurfaceClass::Roof | SurfaceClass::Floor)
                    && !adjacent_properties.azimuth_deg.is_finite())
            {
                return Err(Error::Unavailable(
                    "finite actual paired Source orientation",
                ));
            }
            if ((adjacent_properties.tilt_deg + properties.tilt_deg).abs() - 180.0).abs() > 1.0 {
                return Err(Error::Unavailable("positive interzone tilt diagnostic1993"));
            }
            // Native2007-2036 has class-warning effects for the other opaque
            // combinations. Hold them at their reached point, after tilt and
            // every earlier86 effect; do not silently omit those diagnostics.
            if !matches!(
                (surface.class, adjacent.class),
                (SurfaceClass::Wall, SurfaceClass::Wall)
                    | (SurfaceClass::Roof, SurfaceClass::Floor)
                    | (SurfaceClass::Floor, SurfaceClass::Roof)
            ) {
                return Err(Error::Unavailable(
                    "positive interzone class-mismatch diagnostics2007",
                ));
            }
            // Native2037 never reads azimuth or SinTilt for current Roof/Floor.
            if !matches!(surface.class, SurfaceClass::Roof | SurfaceClass::Floor)
                && ((properties.azimuth_deg - adjacent_properties.azimuth_deg).abs() - 180.0).abs()
                    > 1.0
                && (properties.sin_tilt.abs() > 0.5 || owners.globals.display_extra_warnings)
            {
                return Err(Error::Unavailable(
                    "positive interzone azimuth diagnostic2054",
                ));
            }
            if surface.exterior_solar || adjacent.exterior_solar {
                let flags = flags.ok_or(Error::Unavailable(
                    "actual interzone SunExposed warning phase owners",
                ))?;
                backend.show_source_warning_error(
                    "GetSurfaceData: Interzone surfaces cannot be \"SunExposed\" -- removing SunExposed",
                    flags,
                )?;
                backend.show_continue_error(&format!(
                    "  Surface={}, Zone={}",
                    surface.name, surface.zone_name,
                ))?;
                backend.show_continue_error(&format!(
                    "  Surface={}, Zone={}",
                    adjacent.name, adjacent.zone_name,
                ))?;
                rows[current].row.exterior_solar = false;
                rows[found].row.exterior_solar = false;
            }
            if rows[current].row.exterior_wind || rows[found].row.exterior_wind {
                let flags = flags.ok_or(Error::Unavailable(
                    "actual interzone WindExposed warning phase owners",
                ))?;
                backend.show_source_warning_error(
                    "GetSurfaceData: Interzone surfaces cannot be \"WindExposed\" -- removing WindExposed",
                    flags,
                )?;
                backend.show_continue_error(&format!(
                    "  Surface={}, Zone={}",
                    rows[current].row.name, rows[current].row.zone_name,
                ))?;
                backend.show_continue_error(&format!(
                    "  Surface={}, Zone={}",
                    rows[found].row.name, rows[found].row.zone_name,
                ))?;
                rows[current].row.exterior_wind = false;
                rows[found].row.exterior_wind = false;
            }
            if !reversed_world_vertices(&rows[current].row, &rows[found].row)? {
                return Err(Error::Unavailable(
                    "bounded reverse-coincident actual World vertices",
                ));
            }
            // Actual2109 after every reached check: mutate the genuine opposing row.
            rows[found].row.exterior_boundary = number;
            // Rust-only validation bookkeeping observes the completed body;
            // it never supplies or changes either scientific exposure value.
            rows[current].interzone_exposure_path = InterzoneExposurePath::Completed;
            rows[found].interzone_exposure_path = InterzoneExposurePath::Completed;
        }
        Ok(())
    }
}
