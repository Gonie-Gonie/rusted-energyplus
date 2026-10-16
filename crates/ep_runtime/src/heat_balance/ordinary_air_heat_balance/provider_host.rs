//! Concrete ManageAir children using per-call borrows of the retained run owners.
use super::predictor_corrector::{
    SourceZoneAirPredictorCorrector,
    simple_air::{
        SourceAirReportRegistrationContext, SourceManageAirHeatBalanceChildren,
        SourceMeanAirReportOwners, SourceSimpleAirInputOwners,
    },
};
use crate::heat_balance::ordinary_hvac_manager::provider_host::{
    SourceHvacProviderHost, SourceHvacRemainingChildren,
};
use crate::heat_balance::ordinary_hvac_manager::simulation::output_registration::SourceHvacOutputContext;
use crate::heat_balance::ordinary_hvac_manager::{SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::ordinary_surface_initialization::OrdinarySurfaceInitialization;
use crate::heat_balance::ordinary_surface_management::provider_host::{
    SourceManageSurfaceAirChild, SourceManageSurfaceOwners,
};
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost,
};

pub(crate) struct SourceOrdinaryAirProvider<'a, R> {
    pub(crate) hvac: &'a mut SourceHvacManager,
    pub(crate) remaining: &'a mut R,
}
struct SourceManageAirCall<'a, 'o, 'backend, H, D, R> {
    surface: &'a mut OrdinarySurfaceInitialization,
    owners: &'a mut SourceManageSurfaceOwners<'o, 'backend, H, D>,
    hvac: &'a mut SourceHvacManager,
    remaining: &'a mut R,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, R: SourceHvacRemainingChildren<H, D>>
    SourceManageSurfaceAirChild<H, D> for SourceOrdinaryAirProvider<'_, R>
{
    fn manage_air_heat_balance(
        &mut self,
        surface: &mut OrdinarySurfaceInitialization,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceManageSurfaceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        controller.manage_source_air_heat_balance(&mut SourceManageAirCall {
            surface,
            owners,
            hvac: self.hvac,
            remaining: self.remaining,
        })
    }
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost, R: SourceHvacRemainingChildren<H, D>>
    SourceManageAirHeatBalanceChildren for SourceManageAirCall<'_, '_, '_, H, D, R>
{
    fn get_air_heat_balance_input(
        &mut self,
        controller: &mut SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error> {
        let o = &mut self.owners;
        let mut output = SourceAirReportRegistrationContext {
            registration: o.registration,
            prescan: o.prescan,
            reports: o.report_inputs,
            minimum_report_frequency: o.minimum_report_frequency,
            minutes_in_time_step: o.clock.minutes_in_time_step,
            host: o.output_host,
            eio: o.eio,
            dictionary_host: o.dictionary_host,
        };
        controller.get_source_air_heat_balance_input(
            &mut SourceSimpleAirInputOwners {
                input: o.input,
                zones: o.zones,
                globals: o.globals,
                space_controls: o.space_controls,
                control: o.control,
                fan: o.fan,
                shortcut: o.shortcut,
                backend: o.backend,
            },
            &mut output,
        )
    }
    fn heat_balance_control(&self) -> &SelectedHeatBalanceInputState {
        self.owners.control
    }
    fn calculate_heat_balance_air(
        &mut self,
        controller: &mut SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error> {
        let o = &mut self.owners;
        // Actual DataGlobals function object, not an inferred HVAC readiness flag.
        if o.globals.external_hvac_manager.is_some() {
            if !o.globals.external_hvac_manager_initialized {
                return Err(Error::Unavailable(
                    "actual initializeForExternalHVACManager4598",
                ));
            }
            return Err(Error::Unavailable(
                "actual externalHVACManager callback4600",
            ));
        }
        let mut children = SourceHvacProviderHost {
            output: SourceHvacOutputContext {
                registration: o.registration,
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
            },
            remaining: self.remaining,
        };
        self.hvac.manage_source_hvac(
            &mut SourceHvacOwners {
                air: o.air,
                controller,
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
                psychrometrics: o.psychrometrics,
                backend: o.backend,
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
            },
            &mut children,
            o.eio,
        )
    }
    fn report_zone_mean_air_temperature(
        &mut self,
        controller: &mut SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error> {
        let o = &mut self.owners;
        controller.report_source_zone_mean_air_temp(&mut SourceMeanAirReportOwners {
            air: o.air,
            zones: o.zones,
            globals: o.globals,
            space_controls: o.space_controls,
            environment: o.environment,
            registration: o.registration,
            runtime_language: &o.startup.runtime_language,
            psychrometrics: o.psychrometrics,
        })
    }
}
