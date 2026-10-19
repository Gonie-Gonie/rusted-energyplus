//! Reached Source five-property copy and CheckGlazingShadingStatusChange.
use super::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedWindowOpticalModel,
    SelectedWindowThermalModel,
};
use crate::weather::day::WeatherGlobalState;

pub(super) fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or(Error::Unavailable(
            "positive Source ordinal in surface caller",
        ))
}

/// The Source Zone.spaceIndexes and Space first..last ranges, in their actual
/// retained order. Empty native ranges remain empty; typed IDs never define it.
pub(super) fn source_groups(
    zones: &SelectedZoneInputOwners,
    num_of_zones: i32,
    opaque: bool,
) -> Result<Vec<Vec<i32>>, Error> {
    let zone_rows = zones
        .zones
        .as_ref()
        .ok_or(Error::Unavailable("same Source Zone allocation"))?;
    let spaces = zones
        .spaces
        .as_ref()
        .ok_or(Error::Unavailable("same Source Space allocation"))?;
    let num_of_zones =
        usize::try_from(num_of_zones).map_err(|_| Error::Unavailable("actual NumOfZones"))?;
    if zone_rows.len() != num_of_zones {
        return Err(Error::Unavailable(
            "actual NumOfZones and Zone allocation association",
        ));
    }
    let mut groups = Vec::new();
    for zone in zone_rows {
        for &space_number in &zone.space_indexes {
            let space = spaces
                .get(index(space_number)?)
                .ok_or(Error::Unavailable("actual Zone Space association"))?;
            let (first, last) = if opaque {
                (space.opaque_or_intmass_first, space.opaque_or_intmass_last)
            } else {
                (space.ht_surface_first, space.ht_surface_last)
            };
            groups.push((first..=last).collect());
        }
    }
    Ok(groups)
}

impl SourceSurfaceHeatBalanceArrays {
    /// HBSM357-373. The actual active-construction ordinal selects the same
    /// returned Construction owner independently for every Source HT surface.
    pub(crate) fn copy_source_construction_absorptance(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        constructions: &SelectedHeatBalanceInputState,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        let active = &geometry
            .window_arrays
            .as_ref()
            .ok_or(Error::Unavailable("same SurfActiveConstruction allocation"))?
            .surf_active_construction;
        for group in source_groups(zones, globals.num_of_zones, false)? {
            for number in group {
                let i = index(number)?;
                let construction_number = *active
                    .get(i)
                    .ok_or(Error::Unavailable("SurfActiveConstruction Source ordinal"))?;
                let construction = constructions
                    .construction_usage
                    .get(index(construction_number)?)
                    .ok_or(Error::Unavailable("actual active Construction owner"))?;
                *self
                    .surf_abs_solar_int
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfAbsSolarInt allocation"))? =
                    construction.inside_absorp_solar;
                *self
                    .surf_abs_thermal_int
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfAbsThermalInt allocation"))? =
                    construction.inside_absorp_thermal;
                *self
                    .surf_roughness_ext
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfRoughnessExt allocation"))? =
                    construction.outside_roughness;
                *self
                    .surf_abs_solar_ext
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfAbsSolarExt allocation"))? =
                    construction.outside_absorp_solar;
                *self
                    .surf_abs_thermal_ext
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfAbsThermalExt allocation"))? =
                    construction.outside_absorp_thermal;
            }
        }
        Ok(())
    }
}

fn set_recalculation(radiation: &mut OrdinarySolarRadiation, value: bool) -> Result<(), Error> {
    // SolarShading10258/10277/10285: complete Solar traversal before Radiant.
    for row in radiation
        .source_solar_enclosures_mut()
        .ok_or(Error::Unavailable("same Source EnclSolInfo allocation"))?
    {
        row.rad_recalc = value;
    }
    for row in radiation.source_radiant_enclosures_mut() {
        row.rad_recalc = value;
    }
    Ok(())
}

/// SolarShading10234-10293, including genuine constructor flags and reached
/// false writes before the unsupported positive shading-control continuation.
pub(crate) fn check_glazing_shading_status_change(
    geometry: &SurfaceGeometryInputState,
    zones: &SelectedZoneInputOwners,
    constructions: &mut SelectedHeatBalanceInputState,
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    radiation: &mut OrdinarySolarRadiation,
) -> Result<(), Error> {
    if clock.begin_sim_flag {
        // The sole real input getter admits the built-in thermal model. Missing
        // model selection is unavailable, never interpreted as external=false.
        match (
            constructions.window_thermal_model,
            constructions.window_optical_model,
        ) {
            (
                Some(SelectedWindowThermalModel::BuiltIn),
                Some(SelectedWindowOpticalModel::Simplified),
            ) => {}
            _ => {
                return Err(Error::Unavailable(
                    "actual admitted WindowManager thermal/optical owners",
                ));
            }
        }
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("actual final Surface allocation"))?;
        for group in source_groups(zones, globals.num_of_zones, false)? {
            for number in group {
                let surface = surfaces
                    .get(index(number)?)
                    .ok_or(Error::Unavailable("Source HT Surface ordinal"))?;
                let construction = constructions
                    .construction_usage
                    .get(index(surface.row.construction)?)
                    .ok_or(Error::Unavailable(
                        "CheckGlazing actual Surface Construction owner",
                    ))?;
                if construction.is_tc_window || construction.window_type_eql {
                    constructions.encl_rad_always_recalc = true;
                    break;
                }
            }
        }
        if constructions.encl_rad_always_recalc {
            set_recalculation(radiation, true)?;
        }
    }
    if constructions.encl_rad_always_recalc {
        return Ok(());
    }
    if clock.begin_envrn_flag
        || globals.any_construction_overrides_in_model
        || globals.any_surface_property_overrides_in_model
    {
        set_recalculation(radiation, true)?;
        return Ok(());
    }
    set_recalculation(radiation, false)?;
    if !globals.any_shading_control_in_model {
        return Ok(());
    }
    Err(Error::Unavailable(
        "CheckGlazingShadingStatusChange positive shading control10294",
    ))
}

/// HBSM235-270. The genuine retained dynamic-coating list governs the loop;
/// its constructor-empty admitted route performs no property writes.
pub(crate) fn update_source_variable_absorptances(
    geometry: &SurfaceGeometryInputState,
) -> Result<(), Error> {
    // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
    #[allow(clippy::never_loop)]
    for _number in &geometry.surface_lists.all_vary_absorptance_opaque {
        return Err(Error::Unavailable(
            "positive actual dynamic-coating Surface/Material owner",
        ));
    }
    Ok(())
}

/// HBSM10037-10046 retains the actual early return before any surface read.
pub(crate) fn get_source_ground_surface_reflectance_average(
    globals: &SelectedHeatBalanceGlobals,
) -> Result<(), Error> {
    if !globals.any_local_environments_in_model {
        return Ok(());
    }
    Err(Error::Unavailable(
        "positive actual ground/local-environment reflectance owner",
    ))
}

/// SolarShading13071-13111. WindowModel::BSDF is the pinned enum value1;
/// every actual TotSurfaces model cell is read at the Source loop position.
pub(crate) fn timestep_source_complex_fenestration(
    geometry: &SurfaceGeometryInputState,
) -> Result<(), Error> {
    let models = &geometry
        .window_arrays
        .as_ref()
        .ok_or(Error::Unavailable("same SurfWinWindowModelType allocation"))?
        .surf_win_window_model_type;
    let count = usize::try_from(geometry.total_surfaces)
        .map_err(|_| Error::Unavailable("actual TotSurfaces"))?;
    for i in 0..count {
        if *models
            .get(i)
            .ok_or(Error::Unavailable("actual window model Source ordinal"))?
            == 1
        {
            return Err(Error::Unavailable(
                "positive actual CheckCFSStates producer",
            ));
        }
    }
    Ok(())
}

/// SolarShading9586-10231 has no writes outside its actual Zone/Space/window
/// ranges. This admits the genuine empty window traversal, not a typed no-window
/// inference; a reached window stops before the first unported shade write.
pub(crate) fn window_source_shading_manager(
    zones: &SelectedZoneInputOwners,
    globals: &SelectedHeatBalanceGlobals,
) -> Result<(), Error> {
    let zone_rows = zones
        .zones
        .as_ref()
        .ok_or(Error::Unavailable("same Source Zone allocation"))?;
    let spaces = zones
        .spaces
        .as_ref()
        .ok_or(Error::Unavailable("same Source Space allocation"))?;
    let count = usize::try_from(globals.num_of_zones)
        .map_err(|_| Error::Unavailable("actual NumOfZones"))?;
    if zone_rows.len() != count {
        return Err(Error::Unavailable("actual Zone count association"));
    }
    for zone in zone_rows {
        for &number in &zone.space_indexes {
            let space = spaces
                .get(index(number)?)
                .ok_or(Error::Unavailable("actual Source window Space association"))?;
            // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
            #[allow(clippy::never_loop)]
            for _surface in space.window_first..=space.window_last {
                return Err(Error::Unavailable(
                    "positive WindowShadingManager first shade-status write",
                ));
            }
        }
    }
    Ok(())
}
