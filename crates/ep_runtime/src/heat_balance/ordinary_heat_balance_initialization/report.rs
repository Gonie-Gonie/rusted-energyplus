//! Genuine HeatBalanceManager ReportHeatBalance3321-3418.
//! Each child reborrows the same run owners; no numerical report snapshot exists.
use super::report_warmup::{
    SourceHeatBalanceWarmupChildren, report_source_heat_balance_warmup_branch,
    write_source_heat_balance_warmup_environment_stamp,
};
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use crate::heat_balance::ordinary_hvac_manager::remaining::SourceOrdinaryHvacRemaining;
use crate::heat_balance::ordinary_hvac_manager::simulation::output_registration::SourceHvacOutputContext;
use crate::heat_balance::ordinary_hvac_manager::{SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::ordinary_surface_management::provider_host::SourceManageSurfaceOwners;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_simulation_startup::SourceHvacSizingSimulationData;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputReportStreams,
    SourceReportVariableInput, TimeStepKind,
};
use crate::weather::day::WeatherGlobalState;

/// Borrowed only during actual reporting, after ManageSurface completes.
pub(crate) struct SourceHeatBalanceReportHost<'a, 'owners, 'backend, H, D> {
    pub(crate) owners: &'a mut SourceManageSurfaceOwners<'owners, 'backend, H, D>,
    pub(crate) surface: &'a mut OrdinarySurfaceInitialization,
    pub(crate) controller: &'a mut SourceZoneAirPredictorCorrector,
    pub(crate) hvac: &'a mut SourceHvacManager,
}
enum ZoneChild {
    Output,
    Tabular,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost + SourceOutputReportStreams>
    SourceHeatBalanceReportHost<'_, '_, '_, H, D>
{
    pub(crate) fn report_source_heat_balance(&mut self) -> Result<(), Error> {
        {
            let o = &mut self.owners;
            o.schedules.report_source_schedule_values(
                &mut SourceReportVariableInput::new(
                    o.report_inputs,
                    o.input,
                    o.minimum_report_frequency,
                ),
                o.registration,
                o.prescan,
                o.clock.minutes_in_time_step,
                o.clock,
                o.environment,
                o.output_host,
                o.eio,
                o.dictionary_host,
            )?; //3340, before any report guard
        }
        // End the mutable schedule borrow before any Which/shared output view.
        if !self.owners.clock.warmup_flag && self.owners.clock.do_output_reporting {
            if !self.owners.globals.doing_sizing {
                self.calculate_more_node_info()?;
            }
            self.update_zone_data_and_report()?; //3346
            if self.owners.clock.kind_of_sim == 4 || self.owners.clock.kind_of_sim == 5 {
                self.owners
                    .startup
                    .hvac_sizing_simulation
                    .update_source_zone_step_logs()?;
            }
            self.zone_child(ZoneChild::Tabular)?; //3354, after actual sizing logs
            let o = &mut self.owners;
            o.economic_tariff.update_utility_bills(
                o.cost_estimate_monetary,
                o.input,
                o.shortcut,
                o.clock,
                o.backend,
            )?; //3355
        } else {
            report_source_heat_balance_warmup_branch(self)?;
        }
        Ok(())
    }
    fn zone_child(&mut self, child: ZoneChild) -> Result<(), Error> {
        let o = &mut self.owners;
        let mut output = SourceHvacOutputContext {
            registration: o.registration,
            report_flags: o.report_flags,
            output_control: o.output_control,
            tabular_raw: o.tabular_raw,
            sizing_raw: o.sizing_raw,
            results: o.results,
            standard_output_record_count: o.standard_output_record_count,
            standard_meter_record_count: o.standard_meter_record_count,
            prescan: o.prescan,
            reports: o.report_inputs,
            minimum_report_frequency: o.minimum_report_frequency,
            host: o.output_host,
            dictionary_host: o.dictionary_host,
        };
        let mut owners = SourceHvacOwners {
            runtime_language: &o.startup.runtime_language,
            air: o.air,
            controller: self.controller,
            zones: o.zones,
            sizing: o.sizing,
            equipment: o.equipment,
            geometry: o.geometry,
            surface_arrays: &mut self.surface.arrays,
            fan: o.fan,
            gains: o.gains,
            control: o.control,
            space_controls: o.space_controls,
            globals: o.globals,
            clock: o.clock,
            environment: o.environment,
            hvac: &mut o.startup.hvac_globals,
            input: o.input,
            shortcut: o.shortcut,
            source_is_epjson: o.source_is_epjson,
            faults: o.faults,
            psychrometrics: o.psychrometrics,
            backend: o.backend,
            recurring_errors: o.recurring_errors,
            plant_loops: &mut o.startup.plant_loops,
            project: o.project,
            contaminants: &mut o.heat_balance.contaminants,
            gain_modules: o.gain_modules,
            facility: o.facility.as_deref_mut(),
            schedules: o.schedules,
            tabular: o.tabular,
            radiation: o.radiation,
            history: o.history,
            physical_surfaces: o.physical_surfaces,
            comfort: o.comfort,
            comfort_summary: o.comfort_summary,
            predefined_registry: o.predefined_registry,
            predefined_table: o.predefined_table,
            weather: o.weather,
            inside_surface_iterations: &self.surface.inside_surface_iterations,
            surface_heat_emission: &self.surface.sum_surface_heat_emission,
            surface_averages: o.final_surface,
            exterior: o.exterior,
        };
        match child {
            ZoneChild::Output => SourceOrdinaryHvacRemaining::update_source_ordinary_output(
                TimeStepKind::Zone,
                self.hvac,
                &mut owners,
                &mut output,
            ),
            ZoneChild::Tabular => SourceOrdinaryHvacRemaining::update_source_ordinary_tabular(
                TimeStepKind::Zone,
                self.hvac,
                &mut owners,
                &mut output,
                o.eio,
            ),
        }
    }
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost + SourceOutputReportStreams>
    SourceHeatBalanceWarmupChildren for SourceHeatBalanceReportHost<'_, '_, '_, H, D>
{
    fn clock(&self) -> &WeatherGlobalState {
        self.owners.clock
    }
    fn globals(&self) -> &SelectedHeatBalanceGlobals {
        self.owners.globals
    }
    fn project(&self) -> &SelectedProjectData {
        self.owners.project
    }
    fn hvac_sizing_simulation(&self) -> &SourceHvacSizingSimulationData {
        &self.owners.startup.hvac_sizing_simulation
    }
    fn write_warmup_environment_stamp(&mut self) -> Result<(), Error> {
        let o = &mut self.owners;
        write_source_heat_balance_warmup_environment_stamp(
            o.clock,
            o.environment,
            o.report_flags,
            o.dictionary_host,
        )
    }
    fn calculate_more_node_info(&mut self) -> Result<(), Error> {
        self.owners.equipment.calculate_source_more_node_info(
            self.owners.environment,
            &*self.owners.registration,
            &*self.owners.schedules,
            &self.owners.startup.runtime_language,
        )
    }
    fn update_zone_data_and_report(&mut self) -> Result<(), Error> {
        self.zone_child(ZoneChild::Output)
    }
}
