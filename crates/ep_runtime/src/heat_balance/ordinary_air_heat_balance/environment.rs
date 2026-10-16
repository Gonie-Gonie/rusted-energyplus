//! Actual InitThermalAndFluxHistories owner and numerical stage order.
use super::SourceAirHeatBalance;
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_public_initialization::OrderedCtfPublicInitialization;
use crate::heat_balance::ctf_thermal_history::{
    CtfThermalHistoryAlgorithm, CtfThermalHistoryContext, CtfThermalHistoryState,
    CtfThermalHistorySurface, SourceCtfGroundTemperatures,
    initialize_source_ctf_history_boundary_flux, initialize_source_ctf_history_bulk,
    initialize_source_ctf_history_master_boundary,
};
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedHeatTransferModel,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::{OutsideBoundaryCondition, SurfaceId};
mod fan_zone;
pub(crate) use fan_zone::SourceZoneFanHeatBalanceArrays;

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual environment Source ordinal"))
}
fn value(values: &Option<Vec<f64>>, i: usize) -> Result<f64, Error> {
    values
        .as_deref()
        .and_then(|rows| rows.get(i))
        .copied()
        .ok_or(Error::Unavailable("actual environment surface scalar"))
}
fn source_id(geometry: &SurfaceGeometryInputState, number: i32) -> Result<SurfaceId, Error> {
    geometry
        .final_surfaces
        .as_deref()
        .and_then(|rows| rows.get(index(number).ok()?))
        .and_then(|row| row.typed_id)
        .ok_or(Error::Unavailable("actual environment Surface identity"))
}

/// Reuses the sole normal/master history and actual Source public coefficients.
/// The complete method is the admitted opaque no-source boundary route; positive
/// cavity/Kiva/OSCM and unsupported boundary owners stop at their reached branch.
pub(crate) fn initialize_source_air_surface_environment(
    surface: &mut OrdinarySurfaceInitialization,
    air: &mut SourceAirHeatBalance,
    fan_zone: &mut SourceZoneFanHeatBalanceArrays,
    radiation: &mut OrdinarySolarRadiation,
    geometry: &SurfaceGeometryInputState,
    zones: &SelectedZoneInputOwners,
    control: &SelectedHeatBalanceInputState,
    public: &OrderedCtfPublicInitialization,
    history: &mut CtfThermalHistoryState,
    physical: &[SourceSurfaceOutdoorState],
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    environment: &WeatherEnvironmentState,
) -> Result<(), Error> {
    let humidity = environment.current_weather.out_hum_rat;
    air.reconstruct_source_zone_environment(
        globals,
        humidity,
        fan_zone
            .temp_tstat_air
            .as_deref_mut()
            .ok_or(Error::Unavailable(
                "actual TempTstatAir allocation before reconstruction",
            ))?,
    )?;
    // Source2240 traverses the actual allocated EnclRadInfo, not its count.
    for enclosure in radiation.source_radiant_enclosures_mut() {
        enclosure.mean_radiant_temperature = 23.0;
    }
    air.reconstruct_source_space_environment(humidity)?;
    surface.initialize_source_surface_environment_bulk(geometry, zones, globals, history)?;

    let zone_rows = zones
        .zones
        .as_deref()
        .ok_or(Error::Unavailable("actual environment Zone allocation"))?;
    let space_rows = zones
        .spaces
        .as_deref()
        .ok_or(Error::Unavailable("actual environment Space allocation"))?;
    let final_rows = geometry
        .final_surfaces
        .as_deref()
        .ok_or(Error::Unavailable("actual environment Surface allocation"))?;
    let mut ht_groups = Vec::new();
    let mut opaque_groups = Vec::new();
    for zone_number in 1..=globals.num_of_zones {
        for &space in &zone_rows
            .get(index(zone_number)?)
            .ok_or(Error::Unavailable("actual environment Zone domain"))?
            .space_indexes
        {
            let row = space_rows
                .get(index(space)?)
                .ok_or(Error::Unavailable("actual environment Space domain"))?;
            ht_groups.push(
                (row.ht_surface_first..=row.ht_surface_last)
                    .map(|number| source_id(geometry, number))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            opaque_groups.push(
                (row.opaque_or_intmass_first..=row.opaque_or_intmass_last)
                    .map(|number| source_id(geometry, number))
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
    }
    let all_ht = geometry
        .surface_lists
        .all_ht
        .iter()
        .map(|&number| source_id(geometry, number))
        .collect::<Result<Vec<_>, _>>()?;
    let mut history_surfaces = Vec::new();
    let mut source_boundary_codes = Vec::new();
    for &number in &geometry.surface_lists.all_ht {
        let i = index(number)?;
        let actual = final_rows
            .get(i)
            .ok_or(Error::Unavailable("actual history Surface domain"))?;
        let construction = control
            .construction_usage
            .get(index(actual.row.construction)?)
            .ok_or(Error::Unavailable("actual history Construction domain"))?;
        let construction_id = construction
            .construction_id
            .ok_or(Error::Unavailable("actual public Construction identity"))?;
        let coefficients = &public
            .constructions
            .iter()
            .find(|row| row.construction_id == construction_id)
            .ok_or(Error::Unavailable("actual returned public Construction"))?
            .result
            .as_ref()
            .map_err(|_| Error::Unavailable("unavailable actual public Construction"))?
            .storage;
        let surface_id = actual
            .typed_id
            .ok_or(Error::Unavailable("actual history Source identity"))?;
        let physical = physical
            .iter()
            .find(|row| row.surface_id == surface_id)
            .ok_or(Error::Unavailable("same moved Source outdoor owner"))?;
        let outside_boundary =
            if actual.row.exterior_boundary == 0 || actual.row.exterior_boundary == -4 {
                OutsideBoundaryCondition::Outdoors
            } else if actual.row.exterior_boundary == number {
                OutsideBoundaryCondition::Adiabatic
            } else if actual.row.exterior_boundary == -1 || actual.row.exterior_boundary == -5 {
                OutsideBoundaryCondition::Ground
            } else if actual.row.exterior_boundary > 0 {
                OutsideBoundaryCondition::Surface
            } else {
                OutsideBoundaryCondition::Other
            };
        source_boundary_codes.push((surface_id, actual.row.exterior_boundary));
        history_surfaces.push(CtfThermalHistorySurface {
            surface_id,
            construction_id,
            public_owner_construction_id: construction_id,
            coefficients,
            algorithm: if actual.row.heat_transfer_algorithm == SelectedHeatTransferModel::Ctf {
                CtfThermalHistoryAlgorithm::Ctf
            } else {
                CtfThermalHistoryAlgorithm::Other
            },
            ordinary_opaque: !construction.type_is_window,
            source_sink_present: construction.source_sink_present,
            outside_boundary,
            outside_dry_bulb: physical.outdoor_environment.row.dry_bulb_c,
            area: actual.row.area,
            inside_surface_temperature: value(&surface.arrays.surf_temp_in, i)?,
            inside_constant: value(&surface.arrays.surf_ctf_const_in_part, i)?,
            outside_constant: value(&surface.arrays.surf_ctf_const_out_part, i)?,
        });
    }
    let context = CtfThermalHistoryContext {
        simple_ctf_only: public.aggregate_after_returned_calls.simple_ctf_only,
        any_ems_in_model: globals.any_energy_management_system_in_model,
        any_construction_overrides: globals.any_construction_overrides_in_model,
        any_internal_heat_source: control.any_internal_heat_source_in_input,
        global_max_ctf_terms: public.aggregate_after_returned_calls.max_ctf_terms,
        time_step_zone_hours: clock.time_step_zone,
        heat_transfer_groups: &ht_groups,
        opaque_groups: &opaque_groups,
        all_heat_transfer_surfaces: &all_ht,
    };
    let source_ground = SourceCtfGroundTemperatures {
        surface_boundary_codes: &source_boundary_codes,
        ground_temp: &environment.ground_temp,
    };
    initialize_source_ctf_history_bulk(history, &history_surfaces, context)
        .map_err(|_| Error::Unavailable("actual Source history bulk scope/shape"))?;
    // Positive source-history allocation/write continuation2359-2377 is not
    // represented by the accepted no-source history body. Its live flag is false.
    if control.any_internal_heat_source_in_input {
        return Err(Error::Unavailable(
            "actual source/sink environment histories2359",
        ));
    }
    surface.reset_source_cond_fd_relaxation_factor();
    initialize_source_ctf_history_boundary_flux(history, &history_surfaces, context, source_ground)
        .map_err(|_| Error::Unavailable("actual Source boundary/flux history scope/shape"))?;
    let parallel = geometry
        .parallel_arrays
        .as_ref()
        .ok_or(Error::Unavailable("actual cavity Surface array"))?;
    for &number in &geometry.surface_lists.all_ht {
        if *parallel
            .surf_ext_cavity_present
            .get(index(number)?)
            .ok_or(Error::Unavailable("actual cavity Surface ordinal"))?
        {
            return Err(Error::Unavailable(
                "actual positive vented-cavity initialization2403",
            ));
        }
    }
    if control.any_kiva {
        return Err(Error::Unavailable(
            "actual positive Kiva surface convection initialization2409",
        ));
    }
    initialize_source_ctf_history_master_boundary(
        history,
        &history_surfaces,
        context,
        source_ground,
    )
    .map_err(|_| Error::Unavailable("actual Source master boundary history scope/shape"))?;
    if geometry.total_oscm >= 1 {
        return Err(Error::Unavailable(
            "actual positive OSCM environment initialization2440",
        ));
    }
    Ok(())
}
