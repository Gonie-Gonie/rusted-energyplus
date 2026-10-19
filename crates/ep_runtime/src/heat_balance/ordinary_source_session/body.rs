//! Build one temporary view from the actual session, reports and weather owners.
use super::OrdinarySourceSession;
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_heat_balance_initialization::body_host::SourceHeatBalanceBodyHost;
use crate::heat_balance::ordinary_heat_balance_initialization::manage::manage_source_heat_balance_after_input;
use crate::heat_balance::ordinary_surface_management::provider_host::SourceManageSurfaceOwners;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::SourceOutputFileHost;
use crate::psychrometrics::SourcePsychrometricAccess;
use crate::weather::day::WeatherDayState;
use ep_model::TypedModel;
use std::path::Path;

impl OrdinarySourceSession {
    /// The actual first-call prefix must return before any of these views exist.
    /// This method does not create/reset a psychrometric state scope or clock.
    pub(crate) fn run_heat_balance_body<H: SourceOutputFileHost>(
        &mut self,
        reports: &mut CtfInitializationReports,
        weather: &mut WeatherDayState,
        model: &TypedModel,
        output_host: &mut H,
        files: &mut SourceInitializationFiles,
        backend: &mut SourceErrorBackend<'_>,
        shading_path: &Path,
    ) -> Result<(), Error> {
        if self.heat_balance.manage_heat_balance_get_input_flag {
            return Err(Error::Unavailable(
                "actual GetHBInput/computed geometry prefix before body",
            ));
        }
        // Report-frequency text is immutable input metadata after GetProjectData.
        // A short local owned text prevents aliasing it with the SAME project's
        // convergence arrays, which actual HVAC legitimately mutates. No flag,
        // scientific value, IP, output counter or cache is copied.
        let minimum_report_frequency = reports
            .selected_project_data
            .as_ref()
            .ok_or(Error::Unavailable(
                "same actual GetProjectData owner before HB body",
            ))?
            .system
            .minimum_report_frequency
            .clone();
        let project = reports
            .selected_project_data
            .as_mut()
            .ok_or(Error::Unavailable(
                "same actual GetProjectData owner before HB body",
            ))?;
        let raw_initialization =
            reports
                .source_initialization_input
                .as_ref()
                .ok_or(Error::Unavailable(
                    "same raw initialization input before HB body",
                ))?;
        let surface_owners = self.surfaces.as_mut().ok_or(Error::Unavailable(
            "same actual input-created Source surface owners",
        ))?;
        let mut psychrometrics = SourcePsychrometricAccess;
        // Split only the actual ESO/MTR/EIO and RF metadata fields. Retaining a
        // whole reports borrow in the stream host would alias the body owners.
        let (mut dictionary_host, eio) = files.source_report_streams_and_eio(
            &reports.source_sqlite,
            &mut reports.source_results_output_variables,
        );
        let mut owners = SourceManageSurfaceOwners {
            geometry: &mut reports.surface_geometry_input,
            physical_surfaces: &mut surface_owners.outdoor,
            zones: reports
                .selected_zone_input
                .as_mut()
                .ok_or(Error::Unavailable(
                    "same actual Zone/Space input before HB body",
                ))?,
            control: &mut reports.heat_balance_input_state,
            radiation: reports
                .ordinary_solar_radiation
                .as_mut()
                .ok_or(Error::Unavailable(
                    "same actual InitSolarViewFactors owner before HB body",
                ))?,
            air: &mut self.air,
            history: &mut surface_owners.history,
            ctf: &mut self.ctf,
            globals: reports
                .selected_heat_balance_globals
                .as_mut()
                .ok_or(Error::Unavailable("same actual DataGlobal before HB body"))?,
            clock: &mut weather.global,
            environment: &mut weather.environment,
            weather: &weather.weather,
            project,
            shadowing: reports.shadowing_input.as_ref().ok_or(Error::Unavailable(
                "same actual GetShadowingInput owner before HB body",
            ))?,
            gains: &mut reports.selected_internal_heat_gains,
            gain_modules: &mut self.gain_modules,
            facility: reports.facility_electric_service.as_mut(),
            exterior: &self.exterior,
            schedules: reports
                .selected_schedule_manager
                .as_mut()
                .ok_or(Error::Unavailable(
                    "same actual ScheduleManager before HB body",
                ))?,
            input: reports
                .selected_input_processor
                .as_mut()
                .ok_or(Error::Unavailable(
                    "same actual InputProcessor before HB body",
                ))?,
            shortcut: &mut reports.input_processor_shortcut,
            faults: &reports.source_faults_manager,
            source_is_epjson: &reports.heat_balance_input.as_ref()
                .ok_or(Error::Unavailable("same actual input format before thermostat getter"))?.source_is_epjson,
            scan: &mut reports.scan,
            report_inputs: &model.report_inputs,
            registration: &mut reports.output_registration,
            output_control: &reports.source_output_control,
            tabular_raw: &raw_initialization.tabular,
            sizing_raw: &raw_initialization.sizing,
            results: &mut reports.source_results_framework,
            standard_output_record_count: &mut reports.source_standard_output_record_count,
            standard_meter_record_count: &mut reports.source_standard_meter_record_count,
            prescan: &reports.output_prescan,
            minimum_report_frequency: &minimum_report_frequency,
            output_host,
            dictionary_host: &mut dictionary_host,
            eio,
            materials: &reports.selected_material_data,
            predefined_registry: &mut reports.registry,
            predefined_table: &mut reports.table,
            backend,
            fan: &mut self.fan,
            equipment: reports
                .source_zone_equipment
                .as_mut()
                .ok_or(Error::Unavailable(
                    "same actual Setup ZoneEquipment before HB body",
                ))?,
            sizing: &mut reports.selected_sizing_manager,
            convection: &mut surface_owners.convection,
            daylighting: &mut self.daylighting,
            startup: reports
                .selected_simulation_startup
                .as_mut()
                .ok_or(Error::Unavailable(
                    "same actual simulation startup before HB body",
                ))?,
            space_controls: &reports.heat_balance_space_controls,
            psychrometrics: &mut psychrometrics,
            comfort: &mut self.comfort,
            comfort_summary: &mut self.comfort_summary,
            heat_balance: &mut self.heat_balance,
            tabular: &mut self.tabular,
            final_surface: &mut self.final_surface,
            plugin_data: &mut reports.source_plugin_data,
            report_flags: &mut reports.source_report_flags,
            economic_tariff: &mut reports.economic_tariff,
            cost_estimate_monetary: &mut reports.cost_estimate_monetary,
        };
        manage_source_heat_balance_after_input(&mut SourceHeatBalanceBodyHost {
            owners: &mut owners,
            surface: &mut self.surface,
            controller: &mut self.controller,
            hvac: &mut self.hvac,
            surface_management: &mut self.surface_management,
            window: &mut self.window,
            model,
            shading_stream: &mut self.shading_stream,
            shading_path,
        })
    }
}
