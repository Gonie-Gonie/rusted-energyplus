//! Native normal wrapper and EndEnergyPlus on the same retained Source owners.
//! Finite ctor/empty guards are real state; unsupported positive children stop
//! at their first reached operation, preserving every prior diagnostic/effect.
use super::{SourceInitializationFiles, SourceSimulationInformation};
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_source_session::OrdinarySourceSession;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_registration::{
    SourceOutputRegistration, SourceReportFrequency,
};
use crate::weather::day::{ProductionWeatherTimestepSeries, WeatherSession};
use std::collections::BTreeSet;
#[path = "program_completion_tables.rs"]
mod tables;

fn missing(name: &'static str) -> Error {
    Error::Unavailable(name)
}
fn audit_requested(
    reg: &mut SourceOutputRegistration,
    advanced: bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    for i in 0..reg.requested_variables.len() {
        if reg.requested_variables[i].used {
            continue;
        }
        if reg.requested_variables[i].key.is_empty() {
            reg.requested_variables[i].key = "*".into();
        }
        if reg.requested_variables[i]
            .name
            .contains("OPAQUE SURFACE INSIDE FACE CONDUCTION")
            && !advanced
            && !reg.opaque_surface_warning
        {
            backend.show_warning_message("Variables containing \"Opaque Surface Inside Face Conduction\" are now \"advanced\" variables.")?;
            backend.show_continue_error("You must enter the \"Output:Diagnostics,DisplayAdvancedReportVariables;\" statement to view.")?;
            backend.show_continue_error("First, though, read cautionary statements in the \"InputOutputReference\" document.")?;
            reg.opaque_surface_warning = true;
        }
        if !reg.ungenerated_report_warning {
            backend.show_warning_message(
                "The following Report Variables were requested but not generated -- check.rdd file",
            )?;
            backend.show_continue_error(
                "Either the IDF did not contain these elements, the variable name is misspelled,",
            )?;
            backend.show_continue_error("or the requested variable is an advanced output which requires Output : Diagnostics, DisplayAdvancedReportVariables;")?;
            reg.ungenerated_report_warning = true;
        }
        let row = &reg.requested_variables[i];
        let frequency = match row.frequency {
            SourceReportFrequency::EachCall => "Detailed",
            SourceReportFrequency::TimeStep => "TimeStep",
            SourceReportFrequency::Hour => "Hourly",
            SourceReportFrequency::Day => "Daily",
            SourceReportFrequency::Month => "Monthly",
            SourceReportFrequency::Simulation => "RunPeriod",
            SourceReportFrequency::Year => "Annual",
        };
        backend.show_message(&format!(
            "Key={}, VarName={}, Frequency={frequency}",
            row.key, row.name
        ))?;
    }
    Ok(())
}
fn orphan_objects(
    input: &SelectedInputProcessor,
    display_unused: bool,
    display_all: bool,
    flags: &SourceDiagnosticFlags<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let count = input.source_unused_objects().count();
    if count > 0 && display_unused {
        backend.show_source_warning_error(
            "The following lines are \"Unused Objects\".  These objects are in the input",
            flags,
        )?;
        backend.show_continue_error(
            " file but are never obtained by the simulation and therefore are NOT used.",
        )?;
        if !display_all {
            backend.show_continue_error(" Only the first unused named object of an object class is shown.  Use Output:Diagnostics,DisplayAllWarnings; to see all.")?;
        } else {
            backend.show_continue_error(" Each unused object is shown.")?;
        }
        backend.show_continue_error(" See InputOutputReference document for more details.")?;
    }
    let mut types = BTreeSet::new();
    let mut first = true;
    for (kind, name) in input.source_unused_objects() {
        // This severe is deliberately outside DisplayUnusedObjects.
        if kind.starts_with("ZoneHVAC:") {
            backend.show_source_severe_error("Orphaned ZoneHVAC object found.  This was object never referenced in the input, and was not used.",flags)?;
            backend.show_continue_error(&format!(" -- Object type: {kind}"))?;
            backend.show_continue_error(&format!(" -- Object name: {name}"))?;
        }
        if !display_unused {
            continue;
        }
        if !display_all && !types.insert(kind) {
            continue;
        }
        let message = if name.is_empty() {
            format!("Object={kind}")
        } else {
            format!("Object={kind}={name}")
        };
        if first {
            backend.show_message(&message)?;
            first = false;
        } else {
            backend.show_continue_error(&message)?;
        }
    }
    if count > 0 && !display_unused {
        backend.show_message(&format!("There are {count} unused objects in input."))?;
        backend.show_message("Use Output:Diagnostics,DisplayUnusedObjects; to see them.")?;
    }
    Ok(())
}
fn orphan_fluids(
    reports: &CtfInitializationReports,
    display: bool,
    flags: &SourceDiagnosticFlags<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let fluid = reports
        .selected_fluid_properties
        .as_ref()
        .ok_or(missing("actual Fluid owner at ReportOrphanFluids"))?;
    let mut first = true;
    let mut refrigerants = 0;
    let mut glycols = 0;
    for row in &fluid.refrigerants {
        if row.used || row.name == "STEAM" {
            continue;
        }
        if first && display {
            backend.show_source_warning_error(
                "The following fluid names are \"Unused Fluids\".  These fluids are in the idf",
                flags,
            )?;
            backend.show_continue_error(
                " file but are never obtained by the simulation and therefore are NOT used.",
            )?;
            first = false;
        }
        if display {
            backend.show_message(&format!("Refrigerant={}", row.name))?;
        } else {
            refrigerants += 1;
        }
    }
    for row in &fluid.glycols {
        if row.used
            || row.name == "WATER"
            || row.name == "ETHYLENEGLYCOL"
            || row.name == "PROPYLENEGLYCOL"
        {
            continue;
        }
        if first && display {
            backend.show_source_warning_error(
                "The following fluid names are \"Unused Fluids\".  These fluids are in the idf",
                flags,
            )?;
            backend.show_continue_error(
                " file but are never obtained by the simulation and therefore are NOT used.",
            )?;
            first = false;
        }
        if display {
            backend.show_message(&format!("Glycol={}", row.name))?;
        } else {
            glycols += 1;
        }
    }
    if refrigerants > 0 || glycols > 0 {
        if refrigerants > 0 {
            backend.show_message(&format!(
                "There are {refrigerants} unused refrigerants in input."
            ))?;
        }
        if glycols > 0 {
            backend.show_message(&format!("There are {glycols} unused glycols in input."))?;
        }
        backend.show_message("Use Output:Diagnostics,DisplayUnusedObjects; to see them.")?;
    }
    Ok(())
}
fn orphan_schedules(
    reports: &CtfInitializationReports,
    display: bool,
    flags: &SourceDiagnosticFlags<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let schedules = reports
        .selected_schedule_manager
        .as_ref()
        .ok_or(missing("actual Schedule owner at ReportOrphanSchedules"))?;
    let mut use_message = false;
    let mut first = true;
    let mut count = 0;
    for row in &schedules.schedules {
        if row.is_used {
            continue;
        }
        if first && display {
            backend.show_source_warning_error("The following schedule names are \"Unused Schedules\".  These schedules are in the idf",flags)?;
            backend.show_continue_error(
                " file but are never obtained by the simulation and therefore are NOT used.",
            )?;
            first = false;
        }
        if display {
            backend.show_message(&format!(
                "Schedule:Year or Schedule:Compact or Schedule:File or Schedule:Constant={}",
                row.name
            ))?;
        } else {
            count += 1;
        }
    }
    if count > 0 {
        backend.show_message(&format!("There are {count} unused schedules in input."))?;
        use_message = true;
    }
    first = true;
    count = 0;
    for row in &schedules.week_schedules {
        if row.is_used || row.name.is_empty() {
            continue;
        }
        if first && display {
            backend.show_source_warning_error("The following week schedule names are \"Unused Schedules\".  These schedules are in the idf",flags)?;
            backend.show_continue_error(
                " file but are never obtained by the simulation and therefore are NOT used.",
            )?;
            first = false;
        }
        if display {
            backend.show_message(&format!(
                "Schedule:Week:Daily or Schedule:Week:Compact={}",
                row.name
            ))?;
        } else {
            count += 1;
        }
    }
    if count > 0 {
        backend.show_message(&format!(
            "There are {count} unused week schedules in input."
        ))?;
        use_message = true;
    }
    first = true;
    count = 0;
    for row in &schedules.day_schedules {
        if row.is_used || row.name.is_empty() {
            continue;
        }
        if first && display {
            backend.show_source_warning_error("The following day schedule names are \"Unused Schedules\".  These schedules are in the idf",flags)?;
            backend.show_continue_error(
                " file but are never obtained by the simulation and therefore are NOT used.",
            )?;
            first = false;
        }
        if display {
            backend.show_message(&format!(
                "Schedule:Day:Hourly or Schedule:Day:Interval or Schedule:Day:List={}",
                row.name
            ))?;
        } else {
            count += 1;
        }
    }
    if count > 0 {
        backend.show_message(&format!("There are {count} unused day schedules in input."))?;
        use_message = true;
    }
    if use_message {
        backend.show_message("Use Output:Diagnostics,DisplayUnusedSchedules; to see them.")?;
    }
    Ok(())
}
fn summarize(backend: &mut SourceErrorBackend<'_>) -> Result<(), Error> {
    if !backend.state.tracking.match_counts.iter().any(|&v| v > 0) {
        return Ok(());
    }
    backend.show_message("")?;
    backend.show_message("===== Final Error Summary =====")?;
    backend
        .show_message("The following error categories occurred.  Consider correcting or noting.")?;
    for i in 0..20 {
        if backend.state.tracking.match_counts[i] <= 0 {
            continue;
        }
        backend.show_message(tables::SUMMARIES[i])?;
        let mut text = tables::MORE_DETAILS[i];
        while !text.is_empty() {
            let end = text
                .find("<CR")
                .ok_or(missing("actual SummarizeErrors MoreDetails delimiter"))?;
            backend.show_message(&format!("..{}", &text[..end]))?;
            if text[end..].starts_with("<CRE>") {
                break;
            }
            text = &text[end + 4..];
        }
    }
    backend.show_message("")?;
    Ok(())
}
impl SourceInitializationFiles {
    fn complete_source_program_on_session(
        &mut self,
        reports: &mut CtfInitializationReports,
        session: &mut Option<OrdinarySourceSession>,
        weather: &WeatherSession,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        backend.show_message("Simulation Error Summary *************")?;
        let globals = reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or(missing("actual DataGlobal before wrapUp"))?;
        let project = reports.selected_project_data.as_ref().ok_or(missing(
            "actual same Project before wrapper diagnostic flags",
        ))?;
        let flags = SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &project.system.developer_flag,
            warmup: &weather.state.global.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        audit_requested(
            &mut reports.output_registration,
            globals.display_advanced_report_variables,
            backend,
        )?;
        // Matching pinned original-core Psychrometrics compilation does not
        // define EP_psych_stats: ShowPsychrometricSummary136-197 is compiled empty.
        orphan_objects(
            reports
                .selected_input_processor
                .as_ref()
                .ok_or(missing("actual IP before wrapUp orphan objects"))?,
            globals.display_unused_objects,
            globals.display_all_warnings,
            &flags,
            backend,
        )?;
        orphan_fluids(reports, globals.display_unused_objects, &flags, backend)?;
        orphan_schedules(reports, globals.display_unused_schedules, &flags, backend)?;
        if reports.source_sqlite.present() {
            reports.source_sqlite.reset_source_pointer();
        }
        if reports.selected_input_processor.is_some() {
            reports.selected_input_processor = None;
        }
        if globals.run_read_vars {
            if reports.source_output_control.csv {
                backend.show_warning_message("Native CSV output requested in input file, but running ReadVarsESO due to command line argument.")?;
                backend.show_warning_message("This will overwrite the native CSV output.")?;
            }
            return Err(missing("actual wrapUp runReadVarsESO386"));
        }
        // End526 executes after the wrapper's real pointer reset above.
        if reports.source_sqlite.present() {
            return Err(missing(
                "actual EndEnergyPlus updateSQLiteSimulationRecord526",
            ));
        }
        let solar = reports
            .ordinary_solar_radiation
            .as_ref()
            .ok_or(missing("actual SolarShading before ReportSurfaceErrors530"))?;
        let counts = &solar.solar_timestep;
        let surface_count = counts
            .num_too_many_figures
            .checked_add(counts.num_too_many_vertices)
            .and_then(|v| v.checked_add(counts.num_base_sub_surround))
            .ok_or(missing("actual ReportSurfaceErrors signed count sum11641"))?;
        if surface_count > 0 {
            backend.show_message("")?;
            backend.show_message("===== Recurring Surface Error Summary =====")?;
            backend.show_message("The following surface error messages occurred.")?;
            backend.show_message("")?;
            if counts.num_base_sub_surround > 0 {
                backend.show_message(
                    "Base Surface does not surround subsurface errors occurring...",
                )?;
                backend.show_message("Check that the GlobalGeometryRules object is expressing the proper starting corner and direction [CounterClockwise/Clockwise]")?;
                backend.show_message("")?;
            }
            return Err(missing(
                "actual positive ReportSurfaceErrors SurfErrorReported.dimension11661",
            ));
        }
        backend.show_source_recurring_errors(&reports.source_recurring_errors)?;
        summarize(backend)?;
        session
            .as_ref()
            .ok_or(missing("actual Source session before CloseMiscOpenFiles"))?
            .daylighting
            .close_source_illuminance_maps()?;
        let project = reports
            .selected_project_data
            .as_mut()
            .ok_or(missing("actual Project before EndEnergyPlus"))?;
        self.close_source_misc_files(project.debug_output)?;
        // Six local strings are made only AFTER every reached warning producer.
        let tracking = &backend.state.tracking;
        let warnings = tracking.total_warning_errors.to_string();
        let severe = tracking.total_severe_errors.to_string();
        let warnings_warmup = tracking.total_warning_errors_during_warmup.to_string();
        let severe_warmup = tracking.total_severe_errors_during_warmup.to_string();
        let warnings_sizing = tracking.total_warning_errors_during_sizing.to_string();
        let severe_sizing = tracking.total_severe_errors_during_sizing.to_string();
        project.system.runtime_timer.tock()?;
        if globals.create_perf_log {
            return Err(missing(
                "actual EndEnergyPlus appendPerfLog runtime seconds549",
            ));
        }
        let elapsed = project.system.runtime_timer.format_as_hour_min_secs()?;
        let info: &mut SourceSimulationInformation =
            &mut reports.source_results_framework.simulation_information;
        info.run_time = elapsed.clone();
        info.warnings_warmup = warnings_warmup.clone();
        info.severe_warmup = severe_warmup.clone();
        info.warnings_sizing = warnings_sizing.clone();
        info.severe_sizing = severe_sizing.clone();
        info.warnings = warnings.clone();
        info.severe = severe.clone();
        backend.show_message(&format!("EnergyPlus Warmup Error Summary. During Warmup: {warnings_warmup} Warning; {severe_warmup} Severe Errors."))?;
        backend.show_message(&format!("EnergyPlus Sizing Error Summary. During Sizing: {warnings_sizing} Warning; {severe_sizing} Severe Errors."))?;
        let completed = format!(
            "EnergyPlus Completed Successfully-- {warnings} Warning; {severe} Severe Errors; Elapsed Time={elapsed}"
        );
        backend.show_message(&completed)?;
        let flags = SourceDiagnosticFlags {
            kickoff: &globals.kickoff_simulation,
            developer: &project.system.developer_flag,
            warmup: &weather.state.global.warmup_flag,
            doing_sizing: &globals.doing_sizing,
        };
        backend.display_source_string(&format!("EnergyPlus Run Time={elapsed}"), &flags)?;
        self.write_source_program_end(&reports.source_output_control, &completed, backend, &flags)?;
        let result = &reports.source_results_framework;
        if reports.source_output_control.csv {
            return Err(missing("actual ResultsFramework.writeCSVOutput1055"));
        }
        if result.time_series_enabled
            && (result.output_json || result.output_cbor || result.output_msg_pack)
        {
            return Err(missing(
                "actual ResultsFramework.writeTimeSeriesReports1059",
            ));
        }
        if result.time_series_and_tabular_enabled
            && (result.output_json || result.output_cbor || result.output_msg_pack)
        {
            return Err(missing("actual ResultsFramework.writeReport1063"));
        }
        if backend.state.print_console_output {
            writeln!(backend.console, "EnergyPlus Completed Successfully.")?;
            backend.console.flush()?;
        }
        reports
            .source_external_interface
            .close_source_socket_at_success()?;
        if let Some(callback) = globals.progress_callbacks.f_progress {
            (callback.0)(100);
        }
        if globals.progress_callbacks.progress_callback.is_some() {
            return Err(missing("actual captured progressCallback100 invocation598"));
        }
        if globals.running_via_api {
            return Err(missing("actual EndEnergyPlus API files.flushAll602"));
        }
        self.close_source_program_audit();
        Ok(())
    }
    /// Run actual normal wrapUp/End only after successful same-owner620.
    /// No diagnostic, timer, weather state or physical file owner is recreated.
    /// # Errors
    /// Returns the first real unsupported child or IO failure with prefix retained.
    pub fn complete_source_program(
        &mut self,
        reports: &mut CtfInitializationReports,
        session: &mut Option<OrdinarySourceSession>,
        weather: &ProductionWeatherTimestepSeries,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        weather.with_source_weather_session_mut(|actual| {
            self.complete_source_program_on_session(reports, session, actual, backend)
                .map_err(|e| RuntimeError::HeatBalanceInput(Box::new(e)))
        })
    }
}
