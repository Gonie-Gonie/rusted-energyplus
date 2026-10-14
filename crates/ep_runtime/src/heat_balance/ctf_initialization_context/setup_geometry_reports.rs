//! Actual SetupZoneGeometry641-792 reporting from the same retained owners.
//! Solar enclosure setup and stratosphere checks are not activated here.
use super::{CtfInitializationError, CtfInitializationReports, hb_input_error};
use crate::RuntimeError;
use crate::heat_balance::ctf_geometry_reports::nominal_u_with_films;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as E, surface::SurfaceClass,
};
use ep_model::{
    InsideSurfaceConvectionAlgorithm as Inside, OutsideSurfaceConvectionAlgorithm as Outside,
};
use std::io::Write;

fn unavailable(message: &'static str) -> RuntimeError {
    hb_input_error(E::Unavailable(message))
}

fn film_coefficient(class: SurfaceClass) -> Result<f64, RuntimeError> {
    Ok(match class {
        SurfaceClass::Wall
        | SurfaceClass::Window
        | SurfaceClass::GlassDoor
        | SurfaceClass::Door => 0.1197548,
        SurfaceClass::Floor => 0.1620212,
        SurfaceClass::Roof => 0.1074271,
        SurfaceClass::None
        | SurfaceClass::IntMass
        | SurfaceClass::DetachedBuilding
        | SurfaceClass::DetachedFixed
        | SurfaceClass::Shading
        | SurfaceClass::TddDome
        | SurfaceClass::TddDiffuser => 0.0,
        SurfaceClass::Invalid => return Err(unavailable("invalid Source SurfaceClass film index")),
    })
}

fn source_index(number: i32) -> Result<usize, RuntimeError> {
    number
        .checked_sub(1)
        .and_then(|index| usize::try_from(index).ok())
        .ok_or_else(|| unavailable("positive Source report Surface association"))
}

fn rounded(value: f64, precision: usize) -> Result<String, RuntimeError> {
    crate::heat_balance::source_round_format::round(value, precision).map_err(|error| {
        hb_input_error(E::Format(format!(
            "SetupZoneGeometry Zone Information: {error:?}"
        )))
    })
}

impl CtfInitializationReports {
    pub(crate) fn write_selected_setup_geometry_reports(
        &mut self,
        eio: &mut dyn Write,
    ) -> Result<(), RuntimeError> {
        // The real report list, not physical model order or a new typed sort.
        for number in &self.surface_geometry_input.all_surface_list_report_order {
            let surfaces = self
                .surface_geometry_input
                .final_surfaces
                .as_ref()
                .ok_or_else(|| unavailable("actual report Surface allocation"))?;
            let surface = &surfaces
                .get(source_index(*number)?)
                .ok_or_else(|| unavailable("actual report Surface row"))?
                .row;
            let mut value = 0.0; // Original643: invalid/zero Construction keeps this default.
            if surface.construction > 0
                && usize::try_from(surface.construction)
                    .is_ok_and(|n| n <= self.heat_balance_input_state.total_constructions)
            {
                // ComputeNominalUwithConvCoeffs selects outside film before testing U.
                let mut outside = match surface.exterior_boundary {
                    0 => 0.0299387,
                    -1 | -2 | -4 | -5 | -6 => 0.0,
                    -3 => return Err(unavailable("positive OSC film coefficient report branch")),
                    number => film_coefficient(
                        surfaces
                            .get(source_index(number)?)
                            .ok_or_else(|| unavailable("actual adjacent report Surface"))?
                            .row
                            .class,
                    )?,
                };
                let construction = source_index(surface.construction)?;
                let nominal = self
                    .heat_balance_input_state
                    .nominal_u
                    .as_ref()
                    .and_then(|rows| rows.get(construction))
                    .copied()
                    .ok_or_else(|| unavailable("same actual construction NominalU report slot"))?;
                if nominal > 0.0 {
                    let inside = film_coefficient(surface.class)?;
                    if inside == 0.0 {
                        outside = 0.0;
                    }
                    value = nominal_u_with_films(nominal, inside, outside);
                } else {
                    value = nominal; // isWithConvCoefValid=false is not read by the caller.
                }
            }
            let exterior = matches!(surface.exterior_boundary, 0 | -1 | -5 | -6);
            let key = match (surface.class, exterior) {
                (SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof, true) => {
                    "pdchOpUfactFilm"
                }
                (SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof, false) => {
                    "pdchIntOpUfactFilm"
                }
                (SurfaceClass::Door, true) => "pdchDrUfactFilm",
                (SurfaceClass::Door, false) => "pdchIntDrUfactFilm",
                _ => continue,
            };
            let column =
                *self
                    .registry
                    .named_indices
                    .get(key)
                    .ok_or(CtfInitializationError::Owner(
                        crate::heat_balance::ctf_reports::CtfReportUnavailable::TableOwner,
                    ))?;
            if column <= 0 || column > self.registry.column_count {
                return Err(CtfInitializationError::Owner(
                    crate::heat_balance::ctf_reports::CtfReportUnavailable::TableOwner,
                )
                .into());
            }
            self.table
                .fixed_real_entry_3(column, &surface.name, value)
                .map_err(CtfInitializationError::Owner)?;
        }

        // Each original print is a separate reached write. No additional flush.
        eio.write_all(b"! <Shading Summary>, Number of Fixed Detached Shades, Number of Building Detached Shades, Number of Attached Shades\n")
            .map_err(|error| hb_input_error(E::Io(error)))?;
        let shading = &self.surface_geometry_input.setup_shading;
        writeln!(
            eio,
            " Shading Summary,{},{},{}",
            shading.fixed, shading.building, shading.attached
        )
        .map_err(|error| hb_input_error(E::Io(error)))?;
        eio.write_all(
            b"! <Zone Summary>, Number of Zones, Number of Zone Surfaces, Number of SubSurfaces\n",
        )
        .map_err(|error| hb_input_error(E::Io(error)))?;
        let globals = self
            .selected_heat_balance_globals
            .as_ref()
            .ok_or_else(|| unavailable("actual Zone Summary DataGlobal owner"))?;
        let zones = self
            .selected_zone_input
            .as_ref()
            .and_then(|owner| owner.zones.as_ref())
            .ok_or_else(|| unavailable("actual Zone Summary Zone allocation"))?;
        let sub_surfaces = zones
            .iter()
            .try_fold(0_i32, |sum, row| sum.checked_add(row.num_sub_surfaces))
            .ok_or_else(|| unavailable("Zone Summary subsurface sum int capacity"))?;
        let zone_surfaces = self
            .surface_geometry_input
            .total_surfaces
            .checked_sub(shading.fixed)
            .and_then(|n| n.checked_sub(shading.building))
            .and_then(|n| n.checked_sub(shading.attached))
            .ok_or_else(|| unavailable("Zone Summary Surface count int capacity"))?;
        writeln!(
            eio,
            " Zone Summary,{},{},{}",
            globals.num_of_zones, zone_surfaces, sub_surfaces
        )
        .map_err(|error| hb_input_error(E::Io(error)))?;
        eio.write_all(concat!(
            "! <Zone Information>,Zone Name,North Axis {deg},Origin X-Coordinate {m},Origin Y-Coordinate {m},Origin Z-Coordinate ",
            "{m},Centroid X-Coordinate {m},Centroid Y-Coordinate {m},Centroid Z-Coordinate {m},Type,Zone Multiplier,Zone List ",
            "Multiplier,Minimum X {m},Maximum X {m},Minimum Y {m},Maximum Y {m},Minimum Z {m},Maximum Z {m},Ceiling Height {m},Volume ",
            "{m3},Zone Inside Convection Algorithm {Simple-Detailed-CeilingDiffuser-TrombeWall},Zone Outside Convection Algorithm ",
            "{Simple-Detailed-Tarp-MoWitt-DOE-2-BLAST}, Floor Area {m2},Exterior Gross Wall Area {m2},Exterior Net Wall Area {m2},Exterior ",
            "Window ",
            "Area {m2}, Number of Surfaces, Number of SubSurfaces, Number of Shading SubSurfaces,  Part of Total Building Area\n"
        ).as_bytes()).map_err(|error| hb_input_error(E::Io(error)))?;
        for number in 1..=globals.num_of_zones {
            let zone = zones
                .get(source_index(number)?)
                .ok_or_else(|| unavailable("actual Zone Information row"))?;
            let inside = match zone.inside_algorithm {
                Inside::Simple => "Simple",
                Inside::Tarp => "TARP",
                Inside::CeilingDiffuser => "CeilingDiffuser",
                Inside::TrombeWall => "TrombeWall",
                Inside::AdaptiveConvectionAlgorithm => "AdaptiveConvectionAlgorithm",
                Inside::AstmC1340 => "ASTMC1340",
            };
            let outside = match zone.outside_algorithm {
                Outside::SimpleCombined => "Simple",
                Outside::Tarp => "TARP",
                Outside::MoWitt => "MoWitt",
                Outside::Doe2 => "DOE-2",
                Outside::AdaptiveConvectionAlgorithm => "AdaptiveConvectionAlgorithm",
            };
            let centroid = zone
                .centroid_m
                .ok_or_else(|| unavailable("actual Zone Information Centroid"))?;
            // Exact Format_720 field order. Strings are transient formatting,
            // not another persistent Zone or report owner.
            let fields = [
                zone.name.clone(),
                rounded(zone.relative_north, 1)?,
                rounded(zone.origin_m[0], 2)?,
                rounded(zone.origin_m[1], 2)?,
                rounded(zone.origin_m[2], 2)?,
                rounded(centroid[0], 2)?,
                rounded(centroid[1], 2)?,
                rounded(centroid[2], 2)?,
                zone.of_type.to_string(),
                zone.multiplier.to_string(),
                zone.list_multiplier.to_string(),
                rounded(zone.bounds_m[0], 2)?,
                rounded(zone.bounds_m[1], 2)?,
                rounded(zone.bounds_m[2], 2)?,
                rounded(zone.bounds_m[3], 2)?,
                rounded(zone.bounds_m[4], 2)?,
                rounded(zone.bounds_m[5], 2)?,
                rounded(zone.ceiling_height, 2)?,
                rounded(zone.volume, 2)?,
                inside.into(),
                outside.into(),
                rounded(zone.floor_area, 2)?,
                rounded(zone.exterior_gross_wall_area, 2)?,
                rounded(zone.exterior_net_wall_area, 2)?,
                rounded(zone.exterior_window_area, 2)?,
                zone.num_surfaces.to_string(),
                zone.num_sub_surfaces.to_string(),
                zone.num_shading_surfaces.to_string(),
                if zone.is_part_of_total_area {
                    "Yes"
                } else {
                    "No"
                }
                .into(),
            ];
            writeln!(eio, " Zone Information, {}", fields.join(","))
                .map_err(|error| hb_input_error(E::Io(error)))?;
        }
        Ok(())
    }
}
