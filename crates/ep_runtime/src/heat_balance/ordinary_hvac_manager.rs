//! Genuine ManageHVAC129-687 orchestration on the retained ordinary owners.
//! Mandatory module children have no default implementation or success adapter.
use super::ordinary_air_heat_balance::predictor_corrector::SourceAirUpdate;
use super::selected_heat_balance_input::HeatBalanceInputError as Error;
mod air_report;
mod ancillary;
mod endrun;
pub(crate) mod heat_emission;
pub(crate) mod non_zone_equipment;
mod owners;
mod postlude;
pub(crate) mod provider_host;
mod providers;
pub(crate) mod remaining;
mod reporting;
mod simple_airflow;
pub(crate) mod simulation;
mod state;
pub(crate) mod water_manager;
pub(crate) use ancillary::{SourceHvacAncillaryModules, SourceOutdoorAirNodes};
pub(crate) use owners::SourceHvacOwners;
pub(crate) use providers::{SourceHvacChild, SourceHvacChildren};
pub(crate) use state::SourceHvacManager;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SourceHvacEmsCallingPoint {
    BeginTimestepBeforePredictor,
    BeforeHvacManagers,
    AfterHvacManagers,
    HvacIterationLoop,
    EndSystemTimestepBeforeHvacReporting,
    EndSystemTimestepAfterHvacReporting,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual positive HVAC Source ordinal"))
}
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
impl SourceHvacManager {
    pub(crate) fn manage_source_hvac(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
        children: &mut impl SourceHvacChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        use SourceHvacChild as Child;
        let mut temperature_change = 0.0;
        if self.trigger_get_afn {
            self.trigger_get_afn = false;
            o.backend.display_string("Initializing HVAC")?;
            o.controller
                .subsystems
                .airflow_network
                .manage_source_balance(o.input, eio)?;
        }
        for row in o
            .air
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("actual HVAC Zone AirHB collection"))?
        {
            row.air.zt = row.air.mat;
            row.air.ztav = 0.0;
            row.air.air_hum_rat_avg = 0.0;
        }
        for row in o
            .air
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("actual HVAC Space AirHB collection"))?
        {
            row.air.zt = row.air.mat;
            row.air.ztav = 0.0;
            row.air.air_hum_rat_avg = 0.0;
        }
        // Native EPVector traversal is empty before its genuine later Init allocation.
        if let Some(rows) = o.fan.thermostat.zone_tstat_setpoints.as_mut() {
            for row in rows {
                row.setpt_lo_average = 0.0;
                row.setpt_hi_average = 0.0;
            }
        }
        self.printed_warmup = false;
        if o.control.carbon_dioxide_simulation {
            return Err(Error::Unavailable(
                "actual HVAC CO2OutdoorSched current-value dereference185",
            ));
        }
        if o.control.generic_contaminant_simulation {
            return Err(Error::Unavailable(
                "actual HVAC genericOutdoorSched current-value dereference189",
            ));
        }
        if o.clock.begin_envrn_flag && self.my_environment_flag {
            o.hvac.air_loops_sim_once = false;
            self.my_environment_flag = false;
            o.hvac.number_system_time_steps_last_zone = 1;
            o.hvac.previous_time_step = o.clock.time_step_zone;
        }
        if !o.clock.begin_envrn_flag {
            self.my_environment_flag = true;
        }
        o.surface_arrays
            .q_rad_surf_afn_duct
            .as_mut()
            .ok_or(Error::Unavailable("actual dimensioned QRadSurfAFNDuct"))?
            .fill(0.0);
        o.hvac.system_time_elapsed = 0.0;
        o.hvac.time_step_sys = o.clock.time_step_zone;
        o.hvac.time_step_sys_seconds = o.hvac.time_step_sys * 3600.0;
        o.hvac.first_time_step_sys = true;
        o.hvac.shorten_time_step_sys = false;
        o.hvac.use_zone_time_step_history = true;
        let prior_time_step = o.clock.time_step_zone;
        o.hvac.number_system_time_steps = 1;
        o.hvac.fraction_time_step_zone = o.hvac.time_step_sys / o.clock.time_step_zone;
        let mut any_ems_ran = None; // Rust representation of the Native unwritten local.
        self.manage_source_ems(
            o,
            SourceHvacEmsCallingPoint::BeginTimestepBeforePredictor,
            &mut any_ems_ran,
        )?;
        self.ancillary
            .outdoor_air_nodes
            .set_source_outdoor_air_nodes(o.input)?;
        self.manage_source_refrigerated_case_racks(o)?;
        o.zone_air_update(
            &self.ancillary.availability,
            SourceAirUpdate::GetZoneSetPoints,
            &mut temperature_change,
            prior_time_step,
            children,
            eio,
        )?;
        if o.control.simulate_contaminants {
            children.run(
                Child::ContaminantGetZoneSetPoints { prior_time_step },
                self,
                o,
                eio,
            )?;
        }
        self.ancillary
            .hybrid_ventilation
            .manage_source_hybrid_ventilation(o.input)?;
        children.run(
            Child::CalculateSimpleAirflow { system_step: None },
            self,
            o,
            eio,
        )?;
        if o.controller.subsystems.airflow_network.control_type != 0 {
            o.controller.subsystems.airflow_network.rollback_flag = false;
            o.controller
                .subsystems
                .airflow_network
                .manage_source_balance(o.input, eio)?;
        }
        children.run(Child::SetHeatToReturnAirFlag, self, o, eio)?;
        for number in 1..=o.globals.num_of_zones {
            let at = index(number)?;
            let zone_air = &mut o
                .air
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(at))
                .ok_or(Error::Unavailable("actual HVAC SysDepZoneLoads Zone"))?
                .air;
            zone_air.sys_dep_zone_loads_lagged = zone_air.sys_dep_zone_loads;
            if o.space_controls.current {
                let zone = o
                    .zones
                    .zones
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable("actual HVAC Zone spaceIndexes"))?;
                for &space in &zone.space_indexes {
                    let si = index(space)?;
                    let fraction = o
                        .zones
                        .spaces
                        .as_ref()
                        .and_then(|v| v.get(si))
                        .ok_or(Error::Unavailable("actual HVAC Space fracZoneVolume"))?
                        .fraction_zone_volume;
                    o.air
                        .spaces
                        .as_mut()
                        .and_then(|v| v.get_mut(si))
                        .ok_or(Error::Unavailable(
                            "actual HVAC Space SysDepZoneLoadsLagged",
                        ))?
                        .air
                        .sys_dep_zone_loads_lagged = zone_air.sys_dep_zone_loads * fraction;
                }
            }
        }
        self.ancillary
            .indoor_green
            .simulate_source_indoor_green(o.input)?;
        o.update_source_internal_gains()?;
        o.zone_air_update(
            &self.ancillary.availability,
            SourceAirUpdate::PredictStep,
            &mut temperature_change,
            prior_time_step,
            children,
            eio,
        )?;
        if o.control.simulate_contaminants {
            children.run(
                Child::ContaminantPredictStep { prior_time_step },
                self,
                o,
                eio,
            )?;
        }
        self.simulate_source_hvac_and_ideal_condenser(o, children, eio)?;
        children.run(Child::ManageWaterInits, self, o, eio)?;
        if o.hvac.first_time_step_sys && o.globals.meters_have_been_initialized {
            children.run(Child::ManageDemand, self, o, eio)?;
        }
        o.clock.begin_time_step_flag = false;
        o.zone_air_update(
            &self.ancillary.availability,
            SourceAirUpdate::CorrectStep,
            &mut temperature_change,
            prior_time_step,
            children,
            eio,
        )?;
        if o.control.simulate_contaminants {
            children.run(
                Child::ContaminantCorrectStep { prior_time_step },
                self,
                o,
                eio,
            )?;
        }
        if temperature_change > o.project.convergence.max_zone_temp_diff
            && !o.globals.kickoff_simulation
        {
            let trends_value = temperature_change / o.project.convergence.max_zone_temp_diff + 1.0;
            // C++ int truncates toward zero; reject only its undefined conversion domain.
            if !trends_value.is_finite()
                || trends_value.trunc() < f64::from(i32::MIN)
                || trends_value.trunc() > f64::from(i32::MAX)
            {
                return Err(Error::Unavailable(
                    "actual HVAC adaptive int conversion outside Native defined domain315",
                ));
            }
            let trends = trends_value as i32;
            o.hvac.number_system_time_steps = if trends < o.hvac.limit_num_sys_steps {
                trends
            } else {
                o.hvac.limit_num_sys_steps
            };
            if o.hvac.number_system_time_steps > 0 {
                o.hvac.time_step_sys =
                    o.clock.time_step_zone / f64::from(o.hvac.number_system_time_steps);
            }
            o.hvac.time_step_sys = cpp_max(
                o.hvac.time_step_sys,
                o.project.convergence.min_time_step_sys,
            );
            o.hvac.time_step_sys_seconds = o.hvac.time_step_sys * 3600.0;
            o.hvac.use_zone_time_step_history = false;
            o.hvac.shorten_time_step_sys = true;
        } else {
            o.hvac.number_system_time_steps = 1;
            o.hvac.use_zone_time_step_history = true;
        }
        if o.hvac.use_zone_time_step_history {
            o.hvac.previous_time_step = o.clock.time_step_zone;
        }
        for step in 1..=o.hvac.number_system_time_steps {
            if o.globals.stop_simulation {
                break;
            }
            if o.hvac.time_step_sys < o.clock.time_step_zone {
                self.ancillary
                    .hybrid_ventilation
                    .manage_source_hybrid_ventilation(o.input)?;
                children.run(
                    Child::CalculateSimpleAirflow {
                        system_step: Some(step),
                    },
                    self,
                    o,
                    eio,
                )?;
                if o.controller.subsystems.airflow_network.control_type != 0 {
                    o.controller.subsystems.airflow_network.rollback_flag = false;
                    o.controller
                        .subsystems
                        .airflow_network
                        .manage_source_balance(o.input, eio)?;
                }
                o.update_source_internal_gains()?;
                o.zone_air_update(
                    &self.ancillary.availability,
                    SourceAirUpdate::PredictStep,
                    &mut temperature_change,
                    prior_time_step,
                    children,
                    eio,
                )?;
                if o.control.simulate_contaminants {
                    children.run(
                        Child::ContaminantPredictStep { prior_time_step },
                        self,
                        o,
                        eio,
                    )?;
                }
                self.simulate_source_hvac_and_ideal_condenser(o, children, eio)?;
                children.run(Child::ManageWaterInits, self, o, eio)?;
                o.hvac.shorten_time_step_sys = false;
                o.zone_air_update(
                    &self.ancillary.availability,
                    SourceAirUpdate::CorrectStep,
                    &mut temperature_change,
                    prior_time_step,
                    children,
                    eio,
                )?;
                if o.control.simulate_contaminants {
                    children.run(
                        Child::ContaminantCorrectStep { prior_time_step },
                        self,
                        o,
                        eio,
                    )?;
                }
                o.zone_air_update(
                    &self.ancillary.availability,
                    SourceAirUpdate::PushSystemHistories,
                    &mut temperature_change,
                    prior_time_step,
                    children,
                    eio,
                )?;
                if o.control.simulate_contaminants {
                    children.run(
                        Child::ContaminantPushSystemHistories { prior_time_step },
                        self,
                        o,
                        eio,
                    )?;
                }
                o.hvac.previous_time_step = o.hvac.time_step_sys;
            }
            o.hvac.fraction_time_step_zone = o.hvac.time_step_sys / o.clock.time_step_zone;
            o.accumulate_source_system_step_averages()?;
            children.run(Child::DetectOscillatingZoneTemperature, self, o, eio)?;
            children.run(Child::UpdateZoneListAndGroupLoads, self, o, eio)?;
            children.run(Child::UpdateIceFractions, self, o, eio)?;
            children.run(Child::ManageWater, self, o, eio)?;
            let mut dummy_logical = false;
            children.manage_electric_facility(false, &mut dummy_logical, true, self, o, eio)?;
            children.run(Child::UpdatePlantNodeThermalHistory, self, o, eio)?;
            if o.tabular.display_heat_emissions_summary {
                children.run(Child::CalculateHeatEmissionReport, self, o, eio)?;
            }
            self.manage_source_ems(
                o,
                SourceHvacEmsCallingPoint::EndSystemTimestepBeforeHvacReporting,
                &mut any_ems_ran,
            )?;
            self.report_source_system_step(o, children, eio)?;
            self.manage_source_ems(
                o,
                SourceHvacEmsCallingPoint::EndSystemTimestepAfterHvacReporting,
                &mut any_ems_ran,
            )?;
            o.hvac.system_time_elapsed += o.hvac.time_step_sys;
            o.hvac.first_time_step_sys = false;
        }
        for row in o.air.zones.as_mut().ok_or(Error::Unavailable(
            "actual HVAC final Zone AirHB collection",
        ))? {
            row.air.ztav_comf = row.air.ztav;
            row.air.air_hum_rat_avg_comf = row.air.air_hum_rat_avg;
        }
        for row in o.air.spaces.as_mut().ok_or(Error::Unavailable(
            "actual HVAC final Space AirHB collection",
        ))? {
            row.air.ztav_comf = row.air.ztav;
            row.air.air_hum_rat_avg_comf = row.air.air_hum_rat_avg;
        }
        o.zone_air_update(
            &self.ancillary.availability,
            SourceAirUpdate::PushZoneHistories,
            &mut temperature_change,
            prior_time_step,
            children,
            eio,
        )?;
        if o.control.simulate_contaminants {
            children.run(
                Child::ContaminantPushZoneHistories { prior_time_step },
                self,
                o,
                eio,
            )?;
        }
        o.hvac.number_system_time_steps_last_zone = o.hvac.number_system_time_steps;
        children.run(Child::UpdateDemandManagers, self, o, eio)?;
        if o.project.debug_output {
            return Err(Error::Unavailable(
                "actual HVAC debug ReportDebug/Node list and physical stream subtree599",
            ));
        }
        Ok(())
    }
    fn simulate_source_hvac_and_ideal_condenser(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
        children: &mut impl SourceHvacChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        children.run(SourceHvacChild::SimulateHvac, self, o, eio)?;
        if o.globals.any_ideal_cond_ent_setpoint_in_model
            && o.globals.meters_have_been_initialized
            && !o.clock.warmup_flag
        {
            o.globals.run_opt_cond_ent_temp = true;
            while o.globals.run_opt_cond_ent_temp {
                children.run(SourceHvacChild::SimulateHvac, self, o, eio)?;
            }
        }
        Ok(())
    }
    fn manage_source_ems(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
        _calling_point: SourceHvacEmsCallingPoint,
        any_program_ran: &mut Option<bool>,
    ) -> Result<(), Error> {
        *any_program_ran = Some(false); // Actual ManageEMS, before its genuine flag quick return.
        if !o.globals.any_energy_management_system_in_model {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual positive ManageEMS InitEMS/plugin/RuntimeLanguage child",
        ))
    }
    fn manage_source_refrigerated_case_racks(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !o.gain_modules.refrigeration.manage_refrigeration {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual refrigeration CheckRefrigerationInput344 before InitRefrigeration",
        ))
    }
}
