//! Sole OutputReportTabular owner and genuine first-call input/initialization chain.
//! Pollution factors borrow the same module that actual simulation startup uses.
mod beps;
mod beps_gather;
mod constants;
mod date_time;
mod fuel_factors;
mod gather;
mod getters;
mod heat_emission_gather;
mod heat_gain;
mod input;
mod monthly_initialization;
mod output_files;
mod source_demand;
mod state;
mod style;
mod summary;
mod table_of_contents;
mod unit_conversions;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_output_control::{
    SourceOutputControl, SourceResultsFrameworkOptions,
};
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_sizing::SelectedSizingInput;
use crate::heat_balance::source_output_registration::{SourceOutputRegistration, TimeStepKind};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
pub use input::SelectedTabularInput;
use std::io::Write;
// Source fatal has an uninhabited success type; preserve its actual typed cause
// using the established crate idiom without an unwrap/panic path.
fn source_fatal_error(backend: &mut SourceErrorBackend<'_>, message: &str) -> Error {
    match backend.show_fatal_error(message) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error),
    }
}
/// Per-call disjoint actual owners; no host-retained IP/backend/report aliases.
pub(crate) struct SourceTabularInputOwners<'a, 'backend> {
    pub(crate) files: &'a SourceOutputControl,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) raw: &'a SelectedTabularInput,
    pub(crate) sizing_raw: &'a SelectedSizingInput,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) schedules: &'a super::selected_schedule_manager::SelectedScheduleManager,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) registry: &'a mut PredefinedRegistry,
    pub(crate) registration: &'a SourceOutputRegistration,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) pollution: &'a mut super::ordinary_pollution::SourcePollutionModule,
    pub(crate) equipment: &'a super::ordinary_equipment::OrdinaryEquipment,
    pub(crate) space_controls:
        &'a super::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls,
    pub(crate) hvac: &'a super::selected_simulation_startup::SelectedHvacGlobals,
    pub(crate) control: &'a mut super::selected_heat_balance_input::SelectedHeatBalanceInputState,
    pub(crate) gain_modules: &'a super::selected_gain_modules::SourceGainModules,
    pub(crate) heat_emission:
        &'a super::ordinary_hvac_manager::heat_emission::state::SourceHeatEmissionDeviceModules,
    pub(crate) surface_heat_emission: &'a f64,
    pub(crate) controller:
        &'a super::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector,
    pub(crate) radiation: &'a super::radiation::ordinary::OrdinarySolarRadiation,
    pub(crate) surface_averages:
        &'a super::ordinary_surface_source_averages::SourceFinalSurfaceHeatBalance,
}
#[derive(Debug)]
pub(crate) struct SourceTabularReports {
    pub(crate) display_tabular_beps: bool,
    pub(crate) display_leed_summary: bool,
    pub(crate) display_tabular_comp_costs: bool,
    pub(crate) display_tabular_veri_sum: bool,
    pub(crate) display_component_sizing: bool,
    pub(crate) display_surface_shadowing: bool,
    pub(crate) display_demand_end_use: bool,
    pub(crate) display_adaptive_comfort: bool,
    pub(crate) display_source_energy_end_use_summary: bool,
    pub(crate) display_zone_component_load_summary: bool,
    pub(crate) display_air_loop_component_load_summary: bool,
    pub(crate) display_facility_component_load_summary: bool,
    pub(crate) display_life_cycle_cost_report: bool,
    pub(crate) display_tariff_report: bool,
    pub(crate) display_economic_result_summary: bool,
    pub(crate) display_heat_emissions_summary: bool,
    pub(crate) display_eio_summary: bool,
    pub(crate) display_thermal_resilience_summary: bool,
    pub(crate) display_co2_resilience_summary: bool,
    pub(crate) display_visual_resilience_summary: bool,
    pub(crate) display_thermal_resilience_summary_explicitly: bool,
    pub(crate) display_co2_resilience_summary_explicitly: bool,
    pub(crate) display_visual_resilience_summary_explicitly: bool,
    pub(crate) update_tabular_reports_get_input: bool,
    input_state: state::SourceTabularInputState,
    output_streams: output_files::SourceTabularOutputStreams,
}
impl Default for SourceTabularReports {
    fn default() -> Self {
        Self {
            display_tabular_beps: false,
            display_leed_summary: false,
            display_tabular_comp_costs: false,
            display_tabular_veri_sum: false,
            display_component_sizing: false,
            display_surface_shadowing: false,
            display_demand_end_use: false,
            display_adaptive_comfort: false,
            display_source_energy_end_use_summary: false,
            display_zone_component_load_summary: false,
            display_air_loop_component_load_summary: false,
            display_facility_component_load_summary: false,
            display_life_cycle_cost_report: false,
            display_tariff_report: false,
            display_economic_result_summary: false,
            display_heat_emissions_summary: false,
            display_eio_summary: false,
            display_thermal_resilience_summary: false,
            display_co2_resilience_summary: false,
            display_visual_resilience_summary: false,
            display_thermal_resilience_summary_explicitly: false,
            display_co2_resilience_summary_explicitly: false,
            display_visual_resilience_summary_explicitly: false,
            update_tabular_reports_get_input: true,
            input_state: state::SourceTabularInputState::default(),
            output_streams: output_files::SourceTabularOutputStreams::default(),
        }
    }
}
impl SourceTabularReports {
    /// Borrow the sole actual OutputReportTabular WriteTabularFiles cell.
    /// EconomicTariff writes this same field at its reached native positions.
    pub(crate) fn source_write_tabular_files_mut(&mut self) -> &mut bool {
        &mut self.input_state.write_tabular_files
    }
    /// Actual UpdateTabularReports prefix. A positive unported child returns at
    /// its first real operation, after all preceding reached Source writes.
    pub(crate) fn update_source_tabular_reports(
        &mut self,
        time_step: TimeStepKind,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        match time_step {
            TimeStepKind::Zone | TimeStepKind::System => {}
        }
        if self.update_tabular_reports_get_input {
            self.get_source_tabular_monthly(o)?;
            self.get_source_tabular_annual(o)?;
            self.check_source_annual_aggregation(o)?;
            self.get_source_tabular_time_bins(o)?;
            self.get_source_tabular_style(o)?;
            self.get_source_summary_reports(o)?;
            if self.display_thermal_resilience_summary {
                let count = usize::try_from(o.globals.num_of_zones)
                    .map_err(|_| Error::Unavailable("actual TAB thermal threshold zone count"))?;
                for i in 0..count {
                    let row = o
                        .zones
                        .resilience
                        .as_mut()
                        .and_then(|v| v.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "same actual Resilience Zone threshold owner",
                        ))?;
                    row.cold_stress_temp_thresh = -999.0;
                    row.heat_stress_temp_thresh = -999.0;
                }
                let people = usize::try_from(o.gains.totals[0])
                    .map_err(|_| Error::Unavailable("actual TAB TotPeople count"))?;
                for i in 0..people {
                    let person = o
                        .gains
                        .allocations
                        .people
                        .get(i)
                        .ok_or(Error::Unavailable("same actual TAB People threshold owner"))?;
                    let zone = person.zone_ptr;
                    let at = zone
                        .checked_sub(1)
                        .and_then(|v| usize::try_from(v).ok())
                        .ok_or(Error::Unavailable("actual TAB People ZonePtr"))?;
                    let cold = person.cold_stress_temp_thresh;
                    let row = o
                        .zones
                        .resilience
                        .as_mut()
                        .and_then(|v| v.get_mut(at))
                        .ok_or(Error::Unavailable(
                            "same actual TAB ZoneResilience threshold owner",
                        ))?;
                    if row.cold_stress_temp_thresh < -999.0 + 1.0 {
                        row.cold_stress_temp_thresh = cold;
                    } else if row.cold_stress_temp_thresh != cold {
                        o.backend.show_warning_message(&format!("Zone {zone} has multiple people objects with different Cold Stress Temperature Threshold."))?;
                    }
                    let heat = person.heat_stress_temp_thresh;
                    if row.heat_stress_temp_thresh < -999.0 + 1.0 {
                        row.heat_stress_temp_thresh = heat;
                    } else if row.heat_stress_temp_thresh != heat {
                        o.backend.show_warning_message(&format!("Zone {zone} has multiple people objects with different Heat Stress Temperature Threshold."))?;
                    }
                }
            }
            self.initialize_source_tabular_monthly(o)?;
            if self.source_invalid_monthly_aggregation(o)? {
                return Err(source_fatal_error(
                    o.backend,
                    "OutputReportTabular: Invalid aggregations detected, no simulation performed.",
                ));
            }
            self.get_source_fuel_and_pollution_factors(o)?;
            self.setup_source_unit_conversions()?;
            self.add_source_load_component_table_summaries(o)?;
            self.update_tabular_reports_get_input = false;
            self.input_state.write_source_current_date_and_time();
        }
        if o.clock.do_output_reporting
            && self.input_state.write_tabular_files
            && o.clock.kind_of_sim == 3
        {
            if time_step == TimeStepKind::Zone {
                self.input_state.gather_elapsed_time_beps += o.clock.time_step_zone;
            }
            if o.clock.do_weath_sim {
                self.gather_source_monthly_results(time_step, o)?;
                self.gather_source_annual_results()?;
                self.gather_source_bin_results(time_step, o)?;
                self.gather_source_beps_results(time_step, o)?;
                self.gather_source_energy_end_use_results(time_step, o)?;
                self.gather_source_peak_demand(time_step, o)?;
                self.gather_source_heat_gain_report(time_step, o)?;
                self.gather_source_heat_emission_report(time_step, o)?;
            }
        }
        Ok(())
    }
    fn source_invalid_monthly_aggregation(
        &self,
        o: &SourceTabularInputOwners<'_, '_>,
    ) -> Result<bool, Error> {
        if !o.clock.do_weath_sim {
            return Ok(false);
        }
        if self.input_state.monthly_input_count > 0 {
            return Err(Error::Unavailable(
                "actual isInvalidAggregationOrder969 MonthlyInput numTables",
            ));
        }
        Ok(false)
    }
}
