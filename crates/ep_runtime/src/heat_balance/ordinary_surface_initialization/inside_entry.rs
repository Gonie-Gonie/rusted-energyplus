//! Actual CalcHeatBalanceInsideSurf7738–7813 absent-Zone dispatch and recordkeeping.
use super::{
    OrdinarySurfaceInitialization, inside_ctf::set, inside_providers::SourceOrdinaryInsideCtfHost,
    prefix::index,
};
use crate::heat_balance::ordinary_surface_management::provider_host::SourceManageSurfaceOwners;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatTransferModel,
};
use crate::heat_balance::source_output_registration::{
    SourceIntegerOutputBinding, SourceIntegerOutputDeclaration, SourceIntegerOutputOwners,
    SourceIntegerOutputTarget, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputStoreType, SourceOutputUnits, SourceReportVariableInput,
};
mod intermediate;
mod mrt;

impl SourceIntegerOutputOwners for OrdinarySurfaceInitialization {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        match target {
            SourceIntegerOutputTarget::SurfaceInsideIterationCount => {
                Ok(&self.inside_surface_iterations)
            }
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
fn fill(array: &mut Option<Vec<f64>>, value: f64) -> Result<(), Error> {
    array
        .as_mut()
        .ok_or(Error::Unavailable(
            "actual CalcInside environment allocated temperature array",
        ))?
        .fill(value);
    Ok(())
}
impl OrdinarySurfaceInitialization {
    pub(crate) fn calculate_source_inside_surface_heat_balance<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        o: &mut SourceManageSurfaceOwners<'_, '_, H, D>,
        afn_control_type: &i32,
    ) -> Result<(), Error> {
        if self.inside_entry_first_time {
            if o.control.any_empd {
                // Actual DataHeatBalSurface.hh73 MinEMPDIterations.
                self.minimum_inside_surface_iterations = 4;
            }
            if o.globals.display_advanced_report_variables {
                let binding = SourceIntegerOutputBinding::new(
                    SourceIntegerOutputTarget::SurfaceInsideIterationCount,
                    self,
                )
                .map_err(Error::Output)?;
                let mut input = SourceReportVariableInput::new(
                    o.report_inputs,
                    o.input,
                    o.minimum_report_frequency,
                );
                o.registration
                    .setup_bound_zone_integer(
                        &mut input,
                        o.prescan,
                        SourceIntegerOutputDeclaration {
                            name: "Surface Inside Face Heat Balance Calculation Iteration Count",
                            key: "Simulation",
                            units: SourceOutputUnits::Dimensionless,
                            store_type: SourceOutputStoreType::Sum,
                        },
                        o.clock.minutes_in_time_step,
                        o.output_host,
                        o.eio,
                        binding,
                        o.dictionary_host,
                    )
                    .map_err(Error::Output)?;
            }
            let count = usize::try_from(o.globals.num_of_zones)
                .ok()
                .and_then(|n| n.checked_add(1))
                .ok_or(Error::Unavailable(
                    "actual inside mixed-model vector extent",
                ))?;
            self.arrays.zone_has_mixed_ht_models.resize(count, false);
            for zone_number in 1..=o.globals.num_of_zones {
                let zone = o
                    .zones
                    .zones
                    .as_deref()
                    .and_then(|v| v.get(index(zone_number).ok()?))
                    .ok_or(Error::Unavailable("actual inside first-call Zone"))?;
                for &space_number in &zone.space_indexes {
                    let space = o
                        .zones
                        .spaces
                        .as_deref()
                        .and_then(|v| v.get(index(space_number).ok()?))
                        .ok_or(Error::Unavailable("actual inside first-call Space"))?;
                    for surface_number in space.ht_surface_first..=space.ht_surface_last {
                        let row = o
                            .geometry
                            .final_surfaces
                            .as_deref()
                            .and_then(|v| v.get(index(surface_number).ok()?))
                            .ok_or(Error::Unavailable("actual inside first-call Surface"))?;
                        match row.row.heat_transfer_algorithm {
                            SelectedHeatTransferModel::Ctf => {}
                            SelectedHeatTransferModel::Invalid => {
                                return Err(Error::Unavailable(
                                    "actual inside mixed-model algorithm outside selected CTF enum",
                                ));
                            }
                        }
                    }
                }
            }
            self.inside_entry_first_time = false;
        }
        if o.clock.begin_envrn_flag && self.inside_entry_environment_flag {
            fill(&mut self.arrays.surf_temp_ins_old, 23.0)?;
            fill(&mut self.arrays.ref_air_temp, 23.0)?;
            fill(&mut self.arrays.surf_temp_eff_bulk_air, 23.0)?;
            self.ctf_inside_warmup_error_count = 0;
            self.inside_entry_environment_flag = false;
            if o.control.any_kiva {
                return Err(Error::Unavailable(
                    "actual positive initKivaInstances inside environment child",
                ));
            }
        }
        if !o.clock.begin_envrn_flag {
            self.inside_entry_environment_flag = true;
        }
        // The real same Source list is built by radiant equipment getters, never HT substitution.
        for &number in &o.geometry.surface_lists.all_gets_radiant_heat {
            let i = index(number)?;
            let row = self
                .arrays
                .surf_q_rad_from_hvac
                .as_deref()
                .and_then(|v| v.get(i))
                .ok_or(Error::Unavailable(
                    "actual sumSurfQdotRadHVAC struct allocation",
                ))?;
            let get = |v: Option<f64>| {
                v.ok_or(Error::Unavailable(
                    "actual sumSurfQdotRadHVAC unwritten struct operand",
                ))
            };
            let summed = get(row.ht_rad_sys)?
                + get(row.hw_baseboard)?
                + get(row.steam_baseboard)?
                + get(row.elec_baseboard)?
                + get(row.cooling_panel)?;
            set(&mut self.arrays.surf_qdot_rad_hvac_in_per_area, i, summed)?;
        }
        // Actual absent ZoneToResimulate selects the AllCTF flag, not coefficient inference.
        if !o.control.all_ctf {
            return Err(Error::Unavailable(
                "actual general CalcHeatBalanceInsideSurf2 non-AllCTF dispatch",
            ));
        }
        {
            let mut providers = SourceOrdinaryInsideCtfHost {
                convection: o.convection,
                equipment: o.equipment,
                constructions: o.control,
                input: o.input,
                reports: o.report_inputs,
                scan: o.scan,
                globals: o.globals,
                clock: o.clock,
                environment: o.environment,
                hvac: &o.startup.hvac_globals,
                afn_control_type,
                recurring_errors: o.recurring_errors,
            };
            let cadence_scope = crate::heat_balance::source_surface_cadence::enter(
                crate::heat_balance::source_surface_cadence::Procedure::InsideCtf,
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self,
                    o.clock,
                    o.globals,
                    o.environment,
                    o.globals.kickoff_simulation,
                    self.inside_surface_iterations,
                ),
            );
            let cadence_result = self.calculate_source_inside_ctf_all_zones(
                o.radiation,
                o.geometry,
                o.zones,
                o.control,
                &o.ctf.public,
                o.history,
                o.air,
                o.globals,
                o.clock,
                o.project.carroll_method,
                &mut providers,
                o.backend,
            );
            if crate::heat_balance::source_surface_cadence::active() {
                if let Ok(outcome) = &cadence_result {
                    crate::heat_balance::source_surface_cadence::inside_event(
                        "inside_ctf_loop_exited",
                        o.clock,
                        o.globals,
                        o.globals.kickoff_simulation,
                        self.inside_surface_iterations,
                        crate::heat_balance::source_surface_cadence::Detail {
                            converged: Some(outcome.converged),
                            maximum_delta_bits: Some(
                                outcome.maximum_temperature_difference.to_bits(),
                            ),
                            ..crate::heat_balance::source_surface_cadence::Detail::default()
                        },
                    );
                    for &number in &o.geometry.surface_lists.all_ht {
                        crate::heat_balance::source_surface_cadence::inside_event("inside_ctf_exit_surface", o.clock, o.globals, o.globals.kickoff_simulation, self.inside_surface_iterations,
                            crate::heat_balance::source_surface_cadence::Detail {surface:Some(number),inside_snapshot:Some(crate::heat_balance::source_surface_cadence::InsideSnapshot::full(&self.arrays,number)), ..crate::heat_balance::source_surface_cadence::Detail::default()});
                    }
                }
            }
            cadence_scope.finish(
                crate::heat_balance::source_surface_cadence::Frame::from_owners(
                    self,
                    o.clock,
                    o.globals,
                    o.environment,
                    o.globals.kickoff_simulation,
                    self.inside_surface_iterations,
                ),
                cadence_result.is_ok(),
            );
            cadence_result?;
        }
        self.calculate_source_zone_mrt(
            None,
            o.geometry,
            o.zones,
            o.control,
            o.air,
            o.radiation,
            o.globals,
            o.backend,
        )?;
        self.update_source_intermediate_surface_results(
            None,
            o.geometry,
            o.zones,
            o.radiation,
            o.history,
            o.globals,
            o.clock,
        )?;
        Ok(())
    }
}
