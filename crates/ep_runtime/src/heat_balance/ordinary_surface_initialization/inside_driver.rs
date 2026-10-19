//! Genuine CTF-only inside loop; same Source arrays, history and AirHB owners.
use super::inside_ctf::{set, value};
use super::prefix::index;
use super::{OrdinarySurfaceInitialization, SourceSurfaceHeatBalanceArrays};
use crate::heat_balance::ctf_public_initialization::OrderedCtfPublicInitialization;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::weather::day::WeatherGlobalState;

/// No default callback. Final assembly supplies genuine convection/diagnostics
/// over the same live owners; unavailable positive branches cannot become success.
pub(crate) trait SourceInsideCtfProviders {
    fn initialize_inside_convection(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        arrays: &mut SourceSurfaceHeatBalanceArrays,
        zones: &SelectedZoneInputOwners,
        air: &SourceAirHeatBalance,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;

    fn test_surface_temperature(
        &mut self,
        temperature: f64,
        surface: i32,
        zone: i32,
        geometry: &SurfaceGeometryInputState,
        zones: &mut SelectedZoneInputOwners,
        warmup_error_count: &mut i32,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;

    fn warn_nonconvergence(
        &mut self,
        maximum_difference: f64,
        maximum_allowed_cond_fd: f64,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;

    fn recurring_nonconvergence(
        &mut self,
        maximum_difference: f64,
        error_pointer: &mut i32,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error>;
}

/// Read-only outcome of the actual loop, not a numerical comparison certificate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SourceInsideCtfOutcome {
    pub(crate) converged: bool,
    pub(crate) iterations: i32,
    pub(crate) maximum_temperature_difference: f64,
}

#[derive(Clone)]
struct SpaceRanges {
    zone: i32,
    opaque: std::ops::RangeInclusive<i32>,
    opaque_or_window: std::ops::RangeInclusive<i32>,
    windows: std::ops::RangeInclusive<i32>,
}

fn source_spaces(
    zones: &SelectedZoneInputOwners,
    num_zones: i32,
) -> Result<Vec<SpaceRanges>, Error> {
    let zone_rows = zones
        .zones
        .as_deref()
        .ok_or(Error::Unavailable("inside Zone allocation"))?;
    let spaces = zones
        .spaces
        .as_deref()
        .ok_or(Error::Unavailable("inside Space allocation"))?;
    let count = usize::try_from(num_zones).map_err(|_| Error::Unavailable("inside NumOfZones"))?;
    if zone_rows.len() != count {
        return Err(Error::Unavailable("inside actual all-Zone range"));
    }
    let mut ranges = Vec::new();
    for zone_number in 1..=num_zones {
        for &space in &zone_rows[index(zone_number)?].space_indexes {
            let row = spaces
                .get(index(space)?)
                .ok_or(Error::Unavailable("inside Source Space ordinal"))?;
            ranges.push(SpaceRanges {
                zone: zone_number,
                opaque: row.opaque_or_intmass_first..=row.opaque_or_intmass_last,
                opaque_or_window: row.opaque_or_window_first..=row.opaque_or_window_last,
                windows: row.window_first..=row.window_last,
            });
        }
    }
    Ok(ranges)
}

fn set_flag(values: &mut Option<Vec<i32>>, i: usize, number: i32) -> Result<(), Error> {
    *values
        .as_mut()
        .and_then(|rows| rows.get_mut(i))
        .ok_or(Error::Unavailable("inside Source flag allocation"))? = number;
    Ok(())
}

fn history_index(
    geometry: &SurfaceGeometryInputState,
    history: &CtfThermalHistoryState,
    i: usize,
) -> Result<usize, Error> {
    let id = geometry
        .final_surfaces
        .as_deref()
        .and_then(|rows| rows.get(i))
        .and_then(|row| row.typed_id)
        .ok_or(Error::Unavailable("inside actual Surface/history custody"))?;
    history
        .surface_ids
        .iter()
        .position(|candidate| *candidate == id)
        .ok_or(Error::Unavailable("inside same normal history owner"))
}

impl OrdinarySurfaceInitialization {
    /// HBSM8658-9275, actual absent-ZoneToResimulate/all-Zone branch.
    /// No outside/zone solve, old lifecycle or SimplifiedAnalytical path runs.
    pub(crate) fn calculate_source_inside_ctf_all_zones(
        &mut self,
        radiation: &mut OrdinarySolarRadiation,
        geometry: &mut SurfaceGeometryInputState,
        zones: &mut SelectedZoneInputOwners,
        constructions: &SelectedHeatBalanceInputState,
        public: &OrderedCtfPublicInitialization,
        history: &mut CtfThermalHistoryState,
        air: &SourceAirHeatBalance,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        carroll_method: bool,
        providers: &mut impl SourceInsideCtfProviders,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<SourceInsideCtfOutcome, Error> {
        let spaces = source_spaces(zones, globals.num_of_zones)?;
        // Source8677-8701 clears only this independent CTF-only member after
        // every actual first-call coefficient flag write has completed.
        if self.ctf_only_first_time {
            for space in &spaces {
                for number in space.opaque.clone() {
                    let i = index(number)?;
                    let surface = geometry
                        .final_surfaces
                        .as_deref()
                        .and_then(|rows| rows.get(i))
                        .ok_or(Error::Unavailable("inside first-call Surface"))?;
                    let construction = constructions
                        .construction_usage
                        .get(index(surface.row.construction)?)
                        .ok_or(Error::Unavailable("inside first-call Construction"))?;
                    set_flag(
                        &mut self.arrays.surf_is_adiabatic,
                        i,
                        i32::from(surface.row.exterior_boundary == number),
                    )?;
                    set_flag(
                        &mut self.arrays.surf_is_source_or_sink,
                        i,
                        i32::from(construction.source_sink_present),
                    )?;
                }
            }
            self.ctf_only_first_time = false;
        }

        for space in &spaces {
            for number in space.opaque_or_window.clone() {
                let i = index(number)?;
                let surface = geometry
                    .final_surfaces
                    .as_deref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("inside reference-air Surface"))?;
                if geometry.use_representative_surface_calculations
                    && number != surface.representative_calc_surface_num
                {
                    continue;
                }
                let construction = constructions
                    .construction_usage
                    .get(index(surface.row.construction)?)
                    .ok_or(Error::Unavailable("inside coefficient Construction"))?;
                let construction_id = construction
                    .construction_id
                    .ok_or(Error::Unavailable("inside public Construction identity"))?;
                let coefficient = &public
                    .constructions
                    .iter()
                    .find(|row| row.construction_id == construction_id)
                    .ok_or(Error::Unavailable("inside returned public Construction"))?
                    .result
                    .as_ref()
                    .map_err(|_| Error::Unavailable("inside unavailable public Construction"))?
                    .storage;
                set(&mut self.arrays.surf_ctf_cross0, i, coefficient.cross[0])?;
                set(&mut self.arrays.surf_ctf_inside0, i, coefficient.inside[0])?;
                set(
                    &mut self.arrays.surf_ctf_source_in0,
                    i,
                    coefficient.source_in[0],
                )?;
                let h = history_index(geometry, history, i)?;
                let outside = *history.normal.outside_temperature[0]
                    .get(h)
                    .ok_or(Error::Unavailable("inside outside history term1"))?;
                set(&mut self.arrays.surf_temp_out_hist, i, outside)?;
                if construction.source_sink_present {
                    return Err(Error::Unavailable(
                        "inside actual SurfQsrcHist source/sink owner",
                    ));
                }
                let parallel = geometry
                    .parallel_arrays
                    .as_ref()
                    .ok_or(Error::Unavailable("inside parallel Surface arrays"))?;
                if *parallel
                    .surf_is_pool
                    .get(i)
                    .ok_or(Error::Unavailable("inside actual SurfIsPool"))?
                {
                    let operating = value(&self.arrays.q_pool_surf_numerator, i)?.abs() >= 0.0001
                        || value(&self.arrays.pool_heat_trans_coefs, i)?.abs() >= 0.0001;
                    set_flag(
                        &mut self.arrays.surf_is_operating_pool,
                        i,
                        i32::from(operating),
                    )?;
                }
                // DataSurfaces199-265: the actual Space AirHB is selected before
                // the switch; Source ordinal is independent of a typed Zone ID.
                let mat = air.source_space_mat(surface.row.space)?;
                let reference = match *parallel
                    .surf_t_air_ref
                    .get(i)
                    .ok_or(Error::Unavailable("inside actual SurfTAirRef"))?
                {
                    0 => *mat,
                    1 => value(&self.arrays.surf_temp_eff_bulk_air, i)?,
                    2 => {
                        return Err(Error::Unavailable(
                            "inside ZoneSupplyAirTemp actual controlled-zone/node owner",
                        ));
                    }
                    _ => *mat,
                };
                set(&mut self.arrays.ref_air_temp, i, reference)?;
                set(
                    &mut self.arrays.surf_temp_eff_bulk_air,
                    i,
                    value(&self.arrays.ref_air_temp, i)?,
                )?;
            }
            // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
            #[allow(clippy::never_loop)]
            for _number in space.windows.clone() {
                return Err(Error::Unavailable(
                    "inside positive window first report reset8751",
                ));
            }
            // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
            #[allow(clippy::never_loop)]
            for _number in &geometry.surface_lists.all_inside_source {
                return Err(Error::Unavailable(
                    "inside positive inside-source schedule8768",
                ));
            }
            for number in space.opaque.clone() {
                let i = index(number)?;
                let surface = geometry
                    .final_surfaces
                    .as_deref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("inside precalculation Surface"))?;
                if geometry.use_representative_surface_calculations
                    && number != surface.representative_calc_surface_num
                {
                    continue;
                }
                self.arrays
                    .precalculate_source_inside_terms_at(i, clock.time_step_zone_sec)?;
            }
        }

        self.inside_surface_iterations = 0;
        let mut converged = false;
        loop {
            // Source8809 whole Old copy, then actual same-owner radiation.
            self.exchange_source_inside_iteration(
                radiation,
                geometry,
                constructions,
                globals,
                clock,
                carroll_method,
                backend,
            )?;
            if self.inside_surface_iterations > 0 && self.inside_surface_iterations % 30 == 0 {
                crate::heat_balance::source_surface_cadence::inside_event(
                    "inside_convection_entry",
                    clock,
                    globals,
                    backend.state.kickoff_simulation,
                    self.inside_surface_iterations,
                    if crate::heat_balance::source_surface_cadence::active() {
                        let surface = spaces.iter().find_map(|space| space.opaque.clone().next());
                        crate::heat_balance::source_surface_cadence::Detail {
                            surface,
                            convection_bits: surface
                                .and_then(|number| index(number).ok())
                                .and_then(|i| {
                                    self.arrays
                                        .surf_h_conv_int
                                        .as_deref()
                                        .and_then(|v| v.get(i))
                                })
                                .map(|value| value.to_bits()),
                            convection_storage: self
                                .arrays
                                .surf_h_conv_int
                                .as_ref()
                                .map(|v| v.as_ptr() as usize),
                            ..crate::heat_balance::source_surface_cadence::Detail::default()
                        }
                    } else {
                        crate::heat_balance::source_surface_cadence::Detail::default()
                    },
                );
                providers.initialize_inside_convection(
                    geometry,
                    &mut self.arrays,
                    zones,
                    air,
                    globals,
                    clock,
                    backend,
                )?;
                crate::heat_balance::source_surface_cadence::inside_event(
                    "inside_convection_return",
                    clock,
                    globals,
                    backend.state.kickoff_simulation,
                    self.inside_surface_iterations,
                    if crate::heat_balance::source_surface_cadence::active() {
                        let surface = spaces.iter().find_map(|space| space.opaque.clone().next());
                        crate::heat_balance::source_surface_cadence::Detail {
                            surface,
                            convection_bits: surface
                                .and_then(|number| index(number).ok())
                                .and_then(|i| {
                                    self.arrays
                                        .surf_h_conv_int
                                        .as_deref()
                                        .and_then(|v| v.get(i))
                                })
                                .map(|value| value.to_bits()),
                            convection_storage: self
                                .arrays
                                .surf_h_conv_int
                                .as_ref()
                                .map(|v| v.as_ptr() as usize),
                            ..crate::heat_balance::source_surface_cadence::Detail::default()
                        }
                    } else {
                        crate::heat_balance::source_surface_cadence::Detail::default()
                    },
                );
                self.arrays.precalculate_source_inside_terms(
                    geometry,
                    zones,
                    clock.time_step_zone_sec,
                    globals,
                )?;
                crate::heat_balance::source_surface_cadence::inside_event(
                    "inside_precalculation_return",
                    clock,
                    globals,
                    backend.state.kickoff_simulation,
                    self.inside_surface_iterations,
                    if crate::heat_balance::source_surface_cadence::active() {
                        let surface = spaces.iter().find_map(|space| space.opaque.clone().next());
                        crate::heat_balance::source_surface_cadence::Detail {
                            surface,
                            convection_bits: surface
                                .and_then(|number| index(number).ok())
                                .and_then(|i| {
                                    self.arrays
                                        .surf_h_conv_int
                                        .as_deref()
                                        .and_then(|v| v.get(i))
                                })
                                .map(|value| value.to_bits()),
                            convection_storage: self
                                .arrays
                                .surf_h_conv_int
                                .as_ref()
                                .map(|v| v.as_ptr() as usize),
                            ..crate::heat_balance::source_surface_cadence::Detail::default()
                        }
                    } else {
                        crate::heat_balance::source_surface_cadence::Detail::default()
                    },
                );
            }
            let mut cadence_first_solve = crate::heat_balance::source_surface_cadence::active();
            for space in &spaces {
                for number in space.opaque.clone() {
                    if cadence_first_solve {
                        crate::heat_balance::source_surface_cadence::inside_event(
                            "inside_first_immediate_solve",
                            clock,
                            globals,
                            backend.state.kickoff_simulation,
                            self.inside_surface_iterations,
                            crate::heat_balance::source_surface_cadence::Detail {
                                surface: Some(number),
                                convection_bits: self
                                    .arrays
                                    .surf_h_conv_int
                                    .as_deref()
                                    .and_then(|v| index(number).ok().and_then(|i| v.get(i)))
                                    .map(|v| v.to_bits()),
                                convection_storage: self
                                    .arrays
                                    .surf_h_conv_int
                                    .as_ref()
                                    .map(|v| v.as_ptr() as usize),
                                ..crate::heat_balance::source_surface_cadence::Detail::default()
                            },
                        );
                        cadence_first_solve = false;
                    }
                    if crate::heat_balance::source_surface_cadence::active()
                        && self.inside_surface_iterations < 2
                    {
                        crate::heat_balance::source_surface_cadence::inside_event("inside_ctf_before_temperature_assignment", clock, globals, backend.state.kickoff_simulation, self.inside_surface_iterations, crate::heat_balance::source_surface_cadence::Detail {surface:Some(number), inside_snapshot:Some(crate::heat_balance::source_surface_cadence::InsideSnapshot::full(&self.arrays,number)), ..crate::heat_balance::source_surface_cadence::Detail::default()});
                    }
                    self.arrays
                        .solve_source_inside_opaque_ctf_at(index(number)?)?;
                    if crate::heat_balance::source_surface_cadence::active()
                        && self.inside_surface_iterations < 2
                    {
                        crate::heat_balance::source_surface_cadence::inside_event("inside_ctf_after_temperature_assignment", clock, globals, backend.state.kickoff_simulation, self.inside_surface_iterations, crate::heat_balance::source_surface_cadence::Detail {surface:Some(number), inside_snapshot:Some(crate::heat_balance::source_surface_cadence::InsideSnapshot::temperature(&self.arrays,number)), ..crate::heat_balance::source_surface_cadence::Detail::default()});
                    }
                }
                for number in space.opaque.clone() {
                    let i = index(number)?;
                    let insulation = geometry
                        .parallel_arrays
                        .as_ref()
                        .and_then(|parallel| parallel.interior_movable_insulation.get(i))
                        .ok_or(Error::Unavailable("inside actual intMovInsuls allocation"))?;
                    if geometry.any_movable_insulation && insulation.present {
                        return Err(Error::Unavailable(
                            "inside positive movable-insulation solve8942",
                        ));
                    }
                    if constructions.any_internal_heat_source_in_input {
                        let surface = geometry
                            .final_surfaces
                            .as_deref()
                            .and_then(|rows| rows.get(i))
                            .ok_or(Error::Unavailable("inside radiant-system Surface"))?;
                        let construction = constructions
                            .construction_usage
                            .get(index(surface.row.construction)?)
                            .ok_or(Error::Unavailable("inside radiant-system Construction"))?;
                        if construction.source_sink_present {
                            return Err(Error::Unavailable(
                                "inside positive radiant-system coefficient8967",
                            ));
                        }
                    }
                }
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _number in space.windows.clone() {
                    return Err(Error::Unavailable(
                        "inside positive actual window thermal solve9021",
                    ));
                }
                for number in space.opaque_or_window.clone() {
                    let i = index(number)?;
                    let h = history_index(geometry, history, i)?;
                    let temperature = value(&self.arrays.surf_temp_in, i)?;
                    *history.normal.inside_temperature[0]
                        .get_mut(h)
                        .ok_or(Error::Unavailable("inside same history term1 output"))? =
                        temperature;
                    let outside = *history.normal.outside_temperature[0]
                        .get(h)
                        .ok_or(Error::Unavailable("inside same outside history term1"))?;
                    set(&mut self.arrays.surf_temp_out, i, outside)?;
                    let surface = geometry
                        .final_surfaces
                        .as_deref()
                        .and_then(|rows| rows.get(i))
                        .ok_or(Error::Unavailable("inside reporting Surface"))?;
                    if surface.original_class == SurfaceClass::TddDiffuser {
                        return Err(Error::Unavailable(
                            "inside positive TDD report transfer9183",
                        ));
                    }
                    if temperature > constructions.max_surface_temperature_limit
                        || temperature < -100.0
                    {
                        providers.test_surface_temperature(
                            temperature,
                            number,
                            space.zone,
                            geometry,
                            zones,
                            &mut self.ctf_inside_warmup_error_count,
                            backend,
                        )?;
                    }
                }
            }
            for &number in &geometry.surface_lists.all_interzone {
                let i = index(number)?;
                let other_number = geometry
                    .final_surfaces
                    .as_deref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("inside interzone Surface"))?
                    .row
                    .exterior_boundary;
                let h = history_index(geometry, history, i)?;
                let other = history_index(geometry, history, index(other_number)?)?;
                let temperature = *history.normal.inside_temperature[0]
                    .get(other)
                    .ok_or(Error::Unavailable("inside interzone paired history"))?;
                *history.normal.outside_temperature[0]
                    .get_mut(h)
                    .ok_or(Error::Unavailable("inside interzone outside history"))? = temperature;
                set(&mut self.arrays.surf_temp_out, i, temperature)?;
                set(
                    &mut self.arrays.surf_temp_out_hist,
                    i,
                    value(&self.arrays.surf_temp_out, i)?,
                )?;
            }
            self.inside_surface_iterations = self
                .inside_surface_iterations
                .checked_add(1)
                .ok_or(Error::Unavailable("inside Source iteration signed domain"))?;
            let mut maximum_difference = 0.0;
            for space in &spaces {
                for number in space.opaque.clone() {
                    let i = index(number)?;
                    let delta = value(&self.arrays.surf_temp_in, i)?
                        - value(&self.arrays.surf_temp_ins_old, i)?;
                    let absolute_difference = delta.abs();
                    // std::max(absDif, MaxDelTemp), including NaN and signed-zero
                    // operand order. Rust f64::max would alter this operation.
                    maximum_difference = if absolute_difference < maximum_difference {
                        maximum_difference
                    } else {
                        absolute_difference
                    };
                }
            }
            if let Some(outcome) = self.check_source_inside_convergence(
                maximum_difference,
                &mut converged,
                globals,
                clock,
                providers,
                backend,
            )? {
                return Ok(outcome);
            }
        }
    }
}

impl OrdinarySurfaceInitialization {
    /// Same actual production suffix; no max/min/tolerance override. The loop
    /// passes its actual reduced maximum and local convergence state once.
    pub(super) fn check_source_inside_convergence(
        &mut self,
        maximum_difference: f64,
        converged: &mut bool,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        providers: &mut impl SourceInsideCtfProviders,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<Option<SourceInsideCtfOutcome>, Error> {
        if maximum_difference <= self.maximum_allowed_temperature_difference {
            *converged = true;
        }
        // EP_Count_Calls conditional timing instrumentation is unpaired.
        if self.inside_surface_iterations < self.minimum_inside_surface_iterations {
            *converged = false;
        }
        crate::heat_balance::source_surface_cadence::inside_event(
            "inside_ctf_convergence_checks_reached",
            clock,
            globals,
            backend.state.kickoff_simulation,
            self.inside_surface_iterations,
            crate::heat_balance::source_surface_cadence::Detail {
                maximum_delta_bits: Some(maximum_difference.to_bits()),
                converged: Some(*converged),
                ..crate::heat_balance::source_surface_cadence::Detail::default()
            },
        );
        if self.inside_surface_iterations > 500 {
            if !clock.warmup_flag {
                self.ctf_inside_error_count = self
                    .ctf_inside_error_count
                    .checked_add(1)
                    .ok_or(Error::Unavailable("inside Source diagnostic count domain"))?;
                if self.ctf_inside_error_count < 16 {
                    providers.warn_nonconvergence(
                        maximum_difference,
                        self.maximum_allowed_temperature_difference_cond_fd,
                        backend,
                    )?;
                } else {
                    providers.recurring_nonconvergence(
                        maximum_difference,
                        &mut self.ctf_inside_error_pointer,
                        backend,
                    )?;
                }
            }
            return Ok(Some(SourceInsideCtfOutcome {
                converged: *converged,
                iterations: self.inside_surface_iterations,
                maximum_temperature_difference: maximum_difference,
            }));
        }
        if *converged {
            return Ok(Some(SourceInsideCtfOutcome {
                converged: *converged,
                iterations: self.inside_surface_iterations,
                maximum_temperature_difference: maximum_difference,
            }));
        }
        Ok(None)
    }
}
