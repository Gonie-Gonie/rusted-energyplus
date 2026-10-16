//! Literal UpdateIntermediateSurfaceHeatBalanceResults4951-5020.
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::ordinary_surface_initialization::inside_ctf::{set, value};
use crate::heat_balance::ordinary_surface_initialization::prefix::index;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::weather::day::WeatherGlobalState;
impl OrdinarySurfaceInitialization {
    pub(crate) fn update_source_intermediate_surface_results(
        &mut self,
        zone_to_resimulate: Option<i32>,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        radiation: &mut OrdinarySolarRadiation,
        history: &mut CtfThermalHistoryState,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
    ) -> Result<(), Error> {
        let mut first_zone = 1;
        let mut last_zone = globals.num_of_zones;
        if let Some(zone) = zone_to_resimulate {
            first_zone = zone;
            last_zone = zone;
        }
        for zone in first_zone..=last_zone {
            let zone_row = zones
                .zones
                .as_ref()
                .and_then(|v| v.get(index(zone).ok()?))
                .ok_or(Error::Unavailable("actual intermediate Zone spaceIndexes"))?;
            for &space_number in &zone_row.space_indexes {
                let space = zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(space_number).ok()?))
                    .ok_or(Error::Unavailable("actual intermediate Space window range"))?;
                for number in space.window_first..=space.window_last {
                    let surface = geometry
                        .final_surfaces
                        .as_ref()
                        .and_then(|v| v.get(index(number).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual intermediate window Surface owner",
                        ))?;
                    if surface.row.exterior_solar {
                        let gain = geometry
                            .window_arrays
                            .as_ref()
                            .ok_or(Error::Unavailable("same actual DataSurface window arrays"))?
                            .surf_win_heat_gain
                            .read([index(number)? + 1])?;
                        let target = &mut radiation.solar_timestep.heat_balance.zone_win_heat_gain;
                        target.set([index(zone)? + 1], target.read([index(zone)? + 1])? + gain)?;
                    }
                }
                let hb = &mut radiation.solar_timestep.heat_balance;
                if hb.zone_win_heat_gain.read([index(zone)? + 1])? >= 0.0 {
                    hb.zone_win_heat_gain_rep.set(
                        [index(zone)? + 1],
                        hb.zone_win_heat_gain.read([index(zone)? + 1])?,
                    )?;
                    hb.zone_win_heat_gain_rep_energy.set(
                        [index(zone)? + 1],
                        hb.zone_win_heat_gain_rep.read([index(zone)? + 1])?
                            * clock.time_step_zone_sec,
                    )?;
                } else {
                    hb.zone_win_heat_loss_rep.set(
                        [index(zone)? + 1],
                        -hb.zone_win_heat_gain.read([index(zone)? + 1])?,
                    )?;
                    hb.zone_win_heat_loss_rep_energy.set(
                        [index(zone)? + 1],
                        hb.zone_win_heat_loss_rep.read([index(zone)? + 1])?
                            * clock.time_step_zone_sec,
                    )?;
                }
            }
        }
        if geometry.use_representative_surface_calculations {
            return Err(Error::Unavailable(
                "actual UpdateIntermediate4985 UpdateNonRepresentativeSurfaceResults",
            ));
        }
        for zone in first_zone..=last_zone {
            let zone_row = zones
                .zones
                .as_ref()
                .and_then(|v| v.get(index(zone).ok()?))
                .ok_or(Error::Unavailable("actual intermediate opaque/window Zone"))?;
            for &space_number in &zone_row.space_indexes {
                let space = zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(space_number).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual intermediate opaque/window Space",
                    ))?;
                for number in space.opaque_or_window_first..=space.opaque_or_window_last {
                    let at = index(number)?;
                    let flux = -value(&self.arrays.surf_h_conv_int, at)?
                        * (value(&self.arrays.surf_temp_in, at)?
                            - value(&self.arrays.ref_air_temp, at)?);
                    set(&mut self.arrays.surf_qdot_conv_in_per_area, at, flux)?;
                }
            }
        }
        for zone in first_zone..=last_zone {
            let zone_row = zones
                .zones
                .as_ref()
                .and_then(|v| v.get(index(zone).ok()?))
                .ok_or(Error::Unavailable("actual intermediate opaque Zone"))?;
            for &space_number in &zone_row.space_indexes {
                let space = zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(space_number).ok()?))
                    .ok_or(Error::Unavailable("actual intermediate opaque Space"))?;
                for number in space.opaque_or_intmass_first..=space.opaque_or_intmass_last {
                    let at = index(number)?;
                    let flux = value(&self.arrays.surf_opaq_q_rad_sw_in_abs, at)?
                        - value(&self.arrays.surf_qdot_rad_lights_in_per_area, at)?;
                    set(
                        &mut self.arrays.surf_qdot_rad_solar_in_rep_per_area,
                        at,
                        flux,
                    )?;
                }
            }
        }
        // Native Kiva traversal is global even when a ZoneToResimulate is supplied.
        // Its conduction flux is the same retained DataHeatBalSurf field already
        // carried by history, with the existing exact SurfaceId association.
        for &number in &geometry.surface_lists.all_ht_kiva {
            let at = index(number)?;
            let flux = -(value(&self.arrays.surf_qdot_conv_in_per_area, at)?
                + value(&self.arrays.surf_qdot_rad_net_lw_in_per_area, at)?
                + value(&self.arrays.surf_qdot_rad_hvac_in_per_area, at)?
                + value(&self.arrays.surf_qdot_rad_int_gains_in_per_area, at)?
                + value(&self.arrays.surf_opaq_q_rad_sw_in_abs, at)?);
            let id = geometry
                .final_surfaces
                .as_ref()
                .and_then(|v| v.get(at))
                .and_then(|row| row.typed_id)
                .ok_or(Error::Unavailable(
                    "intermediate actual Kiva Surface identity transport",
                ))?;
            let target = history
                .surface_ids
                .iter()
                .position(|candidate| *candidate == id)
                .ok_or(Error::Unavailable(
                    "intermediate same allocated Kiva conduction flux identity",
                ))?;
            *history
                .flux_reports
                .inside_flux
                .get_mut(target)
                .ok_or(Error::Unavailable(
                    "intermediate same allocated SurfOpaqInsFaceCondFlux",
                ))? = flux;
        }
        Ok(())
    }
}
