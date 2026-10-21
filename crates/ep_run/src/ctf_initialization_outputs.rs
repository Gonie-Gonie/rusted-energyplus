//! Real Rust ordinary-run initialization owners, independent of runtime Result.
//!
//! These are Rust-owned header strings and explicit absent optional adapters;
//! Native version/header/callback/SQLite byte parity is not claimed.

// Retain the existing test module and later completion impl order; this exception concerns only this file-module layout.
#![allow(clippy::items_after_test_module)]

use ep_runtime::heat_balance::ctf_initialization_context::{
    CtfInitializationContext, CtfInitializationReports,
};
use ep_runtime::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use ep_runtime::heat_balance::source_initialization::{
    SourceInitializationFilePaths, SourceInitializationFiles, SourceInitializationInput,
};
use std::path::Path;

pub(crate) struct CtfRunOutputs {
    reports: Box<CtfInitializationReports>,
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
    reached: bool,
    initialization_files: Option<SourceInitializationFiles>,
    source_session: Option<ep_runtime::heat_balance::OrdinarySourceSession>,
    source_simulation_loop_locals: Option<ep_runtime::heat_balance::SourceSimulationLoopLocals>,
    mtd: Option<ep_runtime::heat_balance::source_output_registration::SourceMtdFileHost>,
}

impl CtfRunOutputs {
    pub(crate) fn new(typed_input_version: &str) -> Self {
        Self {
            reports: Box::default(),
            state: SourceErrorState {
                output_error_header: true,
                version_string: concat!("rusted-energyplus ", env!("CARGO_PKG_VERSION")).into(),
                idd_version_string: format!("Rust typed input version {typed_input_version}"),
                // Compilation has returned; construction generation precedes the
                // ordinary warmup/run loops. This caller has no sizing/kickoff path.
                doing_input_processing: false,
                print_console_output: true,
                kickoff_simulation: false,
                developer_flag: false,
                warmup_flag: false,
                doing_sizing: false,
                tracking: SourceErrorTracking {
                    ask_for_plant_check_on_abort: false,
                    exit_during_simulations: false,
                    ask_for_connections_report: false,
                    ask_for_surfaces_report: false,
                    total_multiplied_windows: 0,
                    total_coincident_vertices: 0,
                    total_degenerate_surfaces: 0,
                    total_receiving_non_convex_surfaces: 0,
                    total_casting_non_convex_surfaces: 0,
                    match_counts: [0; 20],
                    abort_processing: false,
                    total_severe_errors: 0,
                    last_severe_error: String::new(),
                    total_severe_errors_during_warmup: 0,
                    total_severe_errors_during_sizing: 0,
                    total_warning_errors: 0,
                    total_warning_errors_during_warmup: 0,
                    total_warning_errors_during_sizing: 0,
                },
            },
            error: Vec::new(),
            console: Vec::new(),
            eio: Vec::new(),
            reached: false,
            initialization_files: None,
            source_session: None,
            source_simulation_loop_locals: None,
            mtd: None,
        }
    }

    pub(crate) fn prepare_source_environment(
        &mut self,
        output_dir: &Path,
        input: SourceInitializationInput,
    ) -> Result<(), String> {
        self.reached = true;
        let first_entry = self.initialization_files.is_none();
        if first_entry {
            self.mtd = Some(
                ep_runtime::heat_balance::source_output_registration::SourceMtdFileHost::new(
                    output_dir.join("eplusrs.mtd"),
                ),
            );
            // Actual CLI583-584: copy the supplied EPW path, then replace its extension.
            let mut stat_input = input.input_weather_file_path.clone();
            stat_input.set_extension("stat");
            let source_input_file = input.input_file_path.clone();
            self.reports.source_initialization_input = Some(input);
            let paths = SourceInitializationFilePaths {
                stat_input,
                eso: output_dir.join("eplusrs.eso"),
                eio: output_dir.join("eplusrs.eio"),
                mtr: output_dir.join("eplusrs.mtr"),
                bnd: output_dir.join("eplusrs.bnd"),
                dxf: output_dir.join("eplusrs.dxf"),
                sln: output_dir.join("eplusrs.sln"),
                sci: output_dir.join("eplusrs.sci"),
                rdd: output_dir.join("eplusrs.rdd"),
                mdd: output_dir.join("eplusrs.mdd"),
                audit: output_dir.join("eplusrs.audit"),
                debug: output_dir.join("eplusrs.dbg"),
                end: output_dir.join("eplusrs.end"),
                error: output_dir.join("eplusrs.err"),
                dfs: output_dir.join("eplusrs.dfs"),
                tabular_csv: output_dir.join("eplustbl.csv"),
                tabular_tab: output_dir.join("eplustbl.tab"),
                tabular_fixed: output_dir.join("eplustbl.txt"),
                tabular_html: output_dir.join("eplustbl.htm"),
                tabular_xml: output_dir.join("eplustbl.xml"),
            };
            self.initialization_files = Some(match source_input_file {
                Some(path) => SourceInitializationFiles::new_with_source_input_file(paths, &path)
                    .map_err(|error| error.to_string())?,
                None => SourceInitializationFiles::new(paths),
            });
        }
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        if first_entry {
            self.reports
                .prepare_source_environment(&mut backend)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub(crate) fn initialize_source_reporting_and_post_input(&mut self) -> Result<(), String> {
        let files = self
            .initialization_files
            .as_mut()
            .ok_or("actual initialization files missing before ManageSimulation183")?;
        self.source_simulation_loop_locals =
            Some(ep_runtime::heat_balance::SourceSimulationLoopLocals::at_simulation_prologue());
        self.reports
            .prepare_source_reporting_prefix(files)
            .map_err(|e| e.to_string())?;
        self.context()
            .post_input_processing()
            .map_err(|e| e.to_string())?;
        self.reports
            .begin_source_simulation_after_post_input_processing()
            .map_err(|e| e.to_string())
    }

    pub(crate) fn initialize_source_state(
        &mut self,
        report_inputs: &ep_model::ReportInputs,
    ) -> Result<(), String> {
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let files = self
            .initialization_files
            .as_mut()
            .ok_or("actual initialization files missing")?;
        self.reports
            .initialize_source_state(files, report_inputs, &mut backend)
            .map_err(|e| e.to_string())
    }

    pub(crate) fn initialize_source_run_prefix(
        &mut self,
        report_inputs: &ep_model::ReportInputs,
    ) -> Result<(), String> {
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        self.reports
            .initialize_source_run_prefix(report_inputs, &mut backend)
            .map_err(|e| e.to_string())
    }

    pub(crate) fn source_zone_timesteps_per_hour(&self) -> Result<u32, String> {
        self.reports
            .source_zone_timesteps_per_hour()
            .map_err(|e| e.to_string())
    }

    pub(crate) fn apply_source_reporting_controls(
        &self,
        reporting: &mut ep_runtime::HeatBalanceOrdinaryWarmupReportInputs,
    ) -> Result<(), String> {
        self.reports
            .apply_source_reporting_controls(reporting)
            .map_err(|e| e.to_string())
    }

    pub(crate) fn initialize_source_weather(
        &mut self,
        bytes: Vec<u8>,
        configuration: ep_runtime::weather::day::WeatherEnvironmentConfiguration,
    ) -> Result<ep_runtime::weather::day::ProductionWeatherTimestepSeries, String> {
        self.reports
            .initialize_source_weather(bytes, configuration)
            .map_err(|e| e.to_string())
    }

    /// Genuine unopened Source weather attachment; old pipeline entry remains separate.
    pub(crate) fn initialize_unopened_source_weather(
        &mut self,
        bytes: Vec<u8>,
        configuration: ep_runtime::weather::day::WeatherEnvironmentConfiguration,
    ) -> Result<ep_runtime::weather::day::ProductionWeatherTimestepSeries, String> {
        self.reports
            .initialize_unopened_source_weather(bytes, configuration)
            .map_err(|e| e.to_string())
    }

    /// Explicit genuine Source entry; legacy pipeline does not invoke it yet.
    // Retain the genuine direct Source timestep entry API without deleting its real owner/caller path.
    #[allow(dead_code)]
    pub(crate) fn source_ordinary_timestep(
        &mut self,
        weather: &ep_runtime::weather::day::ProductionWeatherTimestepSeries,
        model: &ep_model::TypedModel,
        reporting: &ep_runtime::HeatBalanceOrdinaryWarmupReportInputs,
        shading_path: &Path,
    ) -> Result<(), ep_runtime::RuntimeError> {
        use ep_runtime::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
        let missing = |name| {
            ep_runtime::RuntimeError::HeatBalanceInput(Box::new(
                HeatBalanceInputError::Unavailable(name),
            ))
        };
        let files = self
            .initialization_files
            .as_mut()
            .ok_or_else(|| missing("actual initialized Source files before genuine timestep"))?;
        let mtd = self
            .mtd
            .as_mut()
            .ok_or_else(|| missing("actual retained MTD before genuine timestep"))?;
        self.reached = true;
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        files.source_ordinary_timestep(
            &mut self.source_session,
            &mut self.reports,
            weather,
            model,
            reporting,
            mtd,
            &mut backend,
            shading_path,
        )
    }

    /// Explicit Source continuation through Native primary564; production dispatch
    /// and final566-620 processing remain separate caller work. The one outer PSY
    /// scope from before input processing must remain active across this call.
    pub(crate) fn source_simulation_through_primary(
        &mut self,
        weather: &ep_runtime::weather::day::ProductionWeatherTimestepSeries,
        model: &ep_model::TypedModel,
        reporting: &ep_runtime::HeatBalanceOrdinaryWarmupReportInputs,
        shading_path: &Path,
    ) -> Result<(), ep_runtime::RuntimeError> {
        use ep_runtime::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
        let missing = |name| {
            ep_runtime::RuntimeError::HeatBalanceInput(Box::new(
                HeatBalanceInputError::Unavailable(name),
            ))
        };
        let files = self
            .initialization_files
            .as_mut()
            .ok_or_else(|| missing("actual initialized Source files before continuation268"))?;
        let mtd = self
            .mtd
            .as_mut()
            .ok_or_else(|| missing("actual retained Source MTD before continuation268"))?;
        let locals = self
            .source_simulation_loop_locals
            .as_mut()
            .ok_or_else(|| missing("actual RunSimulation180 locals before continuation268"))?;
        // Same concrete caller capability as source_ordinary_timestep: RunConfig
        // owns these two real streams and exposes no callback/SQL/unit adapters.
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        files.source_simulation_through_primary(
            &mut self.source_session,
            &mut self.reports,
            weather,
            model,
            reporting,
            mtd,
            &mut backend,
            shading_path,
            locals,
        )
    }

    /// Same-owner genuine endrun566-590 after the primary continuation returns.
    /// The existing outer PSY scope remains active; no run owner is recreated.
    pub(crate) fn source_simulation_endrun_prefix(
        &mut self,
        weather: &ep_runtime::weather::day::ProductionWeatherTimestepSeries,
    ) -> Result<(), ep_runtime::RuntimeError> {
        use ep_runtime::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
        let files = self.initialization_files.as_mut().ok_or_else(|| {
            ep_runtime::RuntimeError::HeatBalanceInput(Box::new(
                HeatBalanceInputError::Unavailable("actual retained Source files before endrun566"),
            ))
        })?;
        let mut backend = SourceErrorBackend {
            state: &mut self.state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        files.source_simulation_endrun_prefix(
            &mut self.source_session,
            &mut self.reports,
            weather,
            &mut backend,
        )
    }

    /// Only the caller's successful genuine endrun620 branch may invoke this.
    /// Paths and registration/counters belong to the same completed Source run.
    pub(crate) fn read_completed_source_reports(
        &self,
    ) -> Result<crate::source_reported_results::SourceReportedStreams, String> {
        let files = self
            .initialization_files
            .as_ref()
            .ok_or("actual retained Source file owner after620 is missing")?;
        let catalog = self
            .reports
            .source_reported_catalog(files)
            .map_err(|error| error.to_string())?;
        let (eso, mtr) = files.source_reported_paths();
        crate::source_reported_results::read_completed_source_reports(eso, mtr, &catalog)
            .map_err(|error| error.to_string())
    }

    pub(crate) fn reports(&self) -> &CtfInitializationReports {
        &self.reports
    }

    pub(crate) fn context(&mut self) -> CtfInitializationContext<'_> {
        self.reached = true;
        CtfInitializationContext {
            reports: &mut self.reports,
            eio: match self.initialization_files.as_mut() {
                Some(files) => files.eio_stream(),
                None => &mut self.eio,
            },
            backend: SourceErrorBackend {
                state: &mut self.state,
                error_stream: Some(&mut self.error),
                console: &mut self.console,
                // RunConfig owns no SQLite, callback or optional output-unit adapter.
                // The actual buffered console stream is exported separately from
                // CLI JSON stdout, rather than silently replacing it with a no-op.
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        }
    }

    pub(crate) fn context_with_mtd(
        &mut self,
    ) -> Result<
        (
            CtfInitializationContext<'_>,
            &mut ep_runtime::heat_balance::source_output_registration::SourceMtdFileHost,
        ),
        String,
    > {
        self.reached = true;
        let mtd = self.mtd.as_mut().ok_or("actual MTD host missing")?;
        Ok((
            CtfInitializationContext {
                reports: &mut self.reports,
                eio: match self.initialization_files.as_mut() {
                    Some(files) => files.eio_stream(),
                    None => &mut self.eio,
                },
                backend: SourceErrorBackend {
                    state: &mut self.state,
                    error_stream: Some(&mut self.error),
                    console: &mut self.console,
                    // RunConfig owns no SQLite, callback or optional output-unit adapter.
                    // The actual buffered console stream is exported separately from
                    // CLI JSON stdout, rather than silently replacing it with a no-op.
                    out_unit1: None,
                    out_unit2: None,
                    f_message: None,
                    message_callback: None,
                    error_callback: None,
                    sqlite: None,
                },
            },
            mtd,
        ))
    }

    // Retain the existing output-prefix compatibility entry used by tests; production uses persist_after_runtime.
    #[allow(dead_code)]
    pub(crate) fn persist(&mut self, output_dir: &Path) -> Result<(), String> {
        self.persist_after_runtime(output_dir, false)
    }

    /// Keep the Source console in the application log directory after actual
    /// completion. Its primary ERR already belongs to the successful Source
    /// file owner; failure and Legacy prefixes retain the original layout.
    pub(crate) fn persist_after_runtime(
        &mut self,
        output_dir: &Path,
        source_completed: bool,
    ) -> Result<(), String> {
        if !self.reached {
            return Ok(());
        }
        // Write the genuine error prefix first, including when runtime returned Err.
        // Later summary rendering reads this independent exact-byte artifact.
        for (name, bytes) in [
            ("eplusrs.ctf.err", &self.error),
            ("eplusrs.eio", &self.eio),
            ("eplusrs.ctf.log", &self.console),
        ] {
            if name == "eplusrs.eio" && self.initialization_files.is_some() {
                continue;
            }
            if source_completed && name == "eplusrs.ctf.err" {
                continue;
            }
            let path = if source_completed && name == "eplusrs.ctf.log" {
                let logs = output_dir.join("logs");
                std::fs::create_dir_all(&logs)
                    .map_err(|error| format!("failed to create Source log directory: {error}"))?;
                logs.join("source-console.log")
            } else {
                output_dir.join(name)
            };
            std::fs::write(path, bytes)
                .map_err(|error| format!("failed to write {name}: {error}"))?;
        }
        if let Some(files) = self.initialization_files.as_mut() {
            files.flush_reached().map_err(|e| e.to_string())?;
        }
        if let Some(host) = self.mtd.as_mut() {
            host.flush_reached().map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}

/// Preserve the reached Source prefix before the ordinary structured summary.
pub(crate) fn write_error_summary(output_dir: &Path, summary: &str) -> Result<(), String> {
    let path = output_dir.join("eplusrs.ctf.err");
    let mut bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(format!("failed to read {}: {error}", path.display())),
    };
    bytes.extend_from_slice(summary.as_bytes());
    std::fs::write(output_dir.join("eplusrs.err"), bytes)
        .map_err(|error| format!("failed to write eplusrs.err: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_clock_and_version_reach_source_getters_before_axis_construction()
    -> Result<(), Box<dyn std::error::Error>> {
        use serde_json::json;
        let cases = [
            ("absent timestep", Some("26.1"), None, 4, 1, "26.1"),
            (
                "empty timestep",
                Some("26.1"),
                Some(json!({})),
                6,
                0,
                "26.1",
            ),
            (
                "blank IDF timestep",
                Some("26.1"),
                Some(json!({})),
                6,
                0,
                "26.1",
            ),
            (
                "seven",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":7})),
                6,
                1,
                "26.1",
            ),
            (
                "eight",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":8})),
                10,
                1,
                "26.1",
            ),
            (
                "nine",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":9})),
                10,
                1,
                "26.1",
            ),
            (
                "fourteen",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":14})),
                15,
                1,
                "26.1",
            ),
            (
                "forty-five",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":45})),
                60,
                1,
                "26.1",
            ),
            (
                "matching patch",
                Some("26.1.0"),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                0,
                "26.1.0",
            ),
            (
                "advisory patch",
                Some("26.1.99"),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                0,
                "26.1.99",
            ),
            (
                "older version",
                Some("25.2"),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                2,
                "25.2",
            ),
            (
                "arbitrary version",
                Some("not-a-version"),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                2,
                "NOT-A-VERSION",
            ),
            (
                "absent version",
                None,
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                1,
                "",
            ),
            (
                "blank version",
                Some(""),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                1,
                "26.1",
            ),
            (
                "empty singleton keys",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                0,
                "26.1",
            ),
            (
                "whitespace singleton keys",
                Some("26.1"),
                Some(json!({"number_of_timesteps_per_hour":4})),
                4,
                0,
                "26.1",
            ),
        ];
        let forced = std::env::var("CI_FORCE_TIME_STEP")
            .ok()
            .is_some_and(|value| {
                value
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| matches!(byte, b'Y' | b'y' | b'T' | b't'))
            });
        let dir = std::env::temp_dir().join(format!(
            "ep-project-data-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        std::fs::create_dir(&dir)?;
        for (name, version, timestep, expected, warnings, eio_version) in cases {
            let has_timestep = timestep.is_some();
            let converted_idf = name == "blank IDF timestep";
            let version_key = match name {
                "blank IDF timestep" => "Version 1",
                "empty singleton keys" => "",
                "whitespace singleton keys" => " \t ",
                _ => "v",
            };
            let timestep_key = if converted_idf {
                "Timestep 1"
            } else if name.contains("singleton keys") {
                version_key
            } else {
                "t"
            };
            let mut value =
                json!({"Building":{"b":{"solar_distribution":"MinimalShadowing"}},"Zone":{"z":{}}});
            if let Some(version) = version {
                value["Version"] = json!({(version_key):{"version_identifier":version}});
            }
            if let Some(timestep) = timestep {
                value["Timestep"] = json!({(timestep_key):timestep});
            }
            let raw = if converted_idf {
                ep_raw_model::parse_epjson_str_with_idf_order(
                    &value.to_string(),
                    "Version,26.1;Timestep,;Building,b,,,,,MinimalShadowing;Zone,z;",
                )?
            } else {
                ep_raw_model::parse_epjson_str(&value.to_string())?
            };
            let compiled = ep_compiler::compile_raw_model(&raw);
            crate::ordinary_equipment_input::resolve(&raw)?;
            let mut model = compiled
                .model
                .ok_or_else(|| format!("{name}: {:?}", compiled.report.diagnostics))?;
            assert_eq!(
                model.timestep.number_of_timesteps_per_hour, expected,
                "{name}"
            );
            assert_eq!(
                model.authored_version_identifier.as_deref(),
                version,
                "{name}"
            );
            let input = crate::ordinary_input_processor::resolve(&raw)?;
            let operands = SourceInitializationInput {
                project: crate::ordinary_project_data::resolve(&raw, raw.is_epjson(), &input)?,
                schedules: crate::ordinary_schedule_input::resolve(&raw)?,
                sizing: crate::ordinary_sizing_input::resolve(&raw)?,
                ..SourceInitializationInput::default()
            };
            let mut output = CtfRunOutputs::new(&model.version.to_string());
            output.prepare_source_environment(&dir, operands)?;
            {
                let mut context = output.context();
                let matches = input.check_source_version_match(&mut context.backend)?;
                assert_eq!(
                    matches,
                    version.is_none_or(|v| v.is_empty() || v.starts_with("26.1")),
                    "{name}"
                );
                assert!(
                    version.is_none() || input.is_object_unused("Version", version_key),
                    "precheck must not perform getObjectItem"
                );
                context.reports.initialize_source_input_processor(input)?;
            }
            output.initialize_source_state(&model.report_inputs)?;
            let reached = output
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("same retained IP missing")?;
            assert!(
                version.is_none() || !reached.is_object_unused("Version", version_key),
                "Version exact-key getter consumption: {name}"
            );
            assert!(
                !has_timestep || !reached.is_object_unused("Timestep", timestep_key),
                "Timestep exact-key getter consumption: {name}"
            );
            let actual = output.source_zone_timesteps_per_hour()?;
            assert_eq!(
                actual,
                if forced && has_timestep { 2 } else { expected },
                "{name}"
            );
            model.timestep.number_of_timesteps_per_hour = actual;
            let axes = ep_runtime::time_axis::build_environment_time_axes(&model)?;
            assert_eq!(axes[0].zone_timestep.timesteps_per_hour, actual, "{name}");
            assert_eq!(axes[0].sample_count(), 24 * actual as usize, "{name}");
            let version_warnings = if version.is_none() || version == Some("") {
                1
            } else if version.is_some_and(|v| !v.starts_with("26.1")) {
                2
            } else {
                0
            };
            assert_eq!(
                output.state.tracking.total_warning_errors,
                if forced && has_timestep {
                    version_warnings + 1
                } else {
                    warnings
                },
                "{name}"
            );
            assert_eq!(output.state.tracking.total_severe_errors, 0, "{name}");
            if version == Some("not-a-version") {
                let messages = String::from_utf8(output.error.clone())?;
                let first = messages
                    .find("IDF=\"not-a-version\"")
                    .ok_or("raw Version warning missing")?;
                let second = messages
                    .find("IDF=\"NOT-A-VERSION\"")
                    .ok_or("canonical Version warning missing")?;
                assert!(
                    first < second,
                    "raw warning precedes canonical getter warning"
                );
            }
            output.persist(&dir)?;
            let eio = std::fs::read_to_string(dir.join("eplusrs.eio"))?;
            assert!(
                eio.lines()
                    .any(|line| line == format!(" Version, {eio_version}")),
                "{name}: Version EIO"
            );
            drop(output);
            for entry in std::fs::read_dir(&dir)? {
                std::fs::remove_file(entry?.path())?;
            }
        }
        std::fs::remove_dir(&dir)?;
        Ok(())
    }

    #[test]
    fn actual_fatal_prefix_survives_runtime_failure_and_summary()
    -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::temp_dir().join(format!(
            "ep-run-ctf-prefix-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        std::fs::create_dir(&dir)?;
        let mut output = CtfRunOutputs::new("authored test input version");
        {
            let mut context = output.context();
            context
                .backend
                .show_severe_error("authored reached severe")?;
            assert!(
                context
                    .backend
                    .show_fatal_error("authored reached fatal")
                    .is_err()
            );
        }
        let prefix = output.error.clone();
        assert!(!prefix.is_empty());
        // Genuine ShowFatalError preserves AbortProcessing and throws FatalError.
        assert!(!output.state.tracking.abort_processing);
        output.persist(&dir)?;
        write_error_summary(&dir, "actual structured runtime failure\n")?;
        let combined = std::fs::read(dir.join("eplusrs.err"))?;
        assert_eq!(&combined[..prefix.len()], prefix.as_slice());
        assert_eq!(
            &combined[prefix.len()..],
            b"actual structured runtime failure\n"
        );
        assert_eq!(std::fs::read(dir.join("eplusrs.ctf.err"))?, prefix);
        for name in [
            "eplusrs.ctf.err",
            "eplusrs.err",
            "eplusrs.eio",
            "eplusrs.ctf.log",
        ] {
            std::fs::remove_file(dir.join(name))?;
        }
        std::fs::remove_dir(&dir)?;
        let raw = ep_raw_model::parse_epjson_str(
            r#"{
            "Version":{"v":{"version_identifier":"26.1"}},
            "Building":{"b":{"solar_distribution":"MinimalShadowing"}},
            "Timestep":{"t":{}},"Zone":{"z":{}},
            "Output:Table:SummaryReports":{"summary":{"reports":[{"report_name":"AllSummary"}]}},
            "Output:Diagnostics":{"d":{"diagnostics":[{"key":"DisplayAdvancedReportVariables"}]}}
        }"#,
        )?;
        let make_input = || -> Result<_, String> {
            let input = crate::ordinary_input_processor::resolve(&raw)?;
            let operands = SourceInitializationInput {
                project: crate::ordinary_project_data::resolve(&raw, raw.is_epjson(), &input)?,
                schedules: crate::ordinary_schedule_input::resolve(&raw)?,
                sizing: crate::ordinary_sizing_input::resolve(&raw)?,
                ..SourceInitializationInput::default()
            };
            Ok((input, operands))
        };
        std::fs::create_dir(&dir)?;
        std::fs::create_dir(dir.join("eplusrs.mtr"))?;
        let mut failure = CtfRunOutputs::new("26.1");
        let (input, operands) = make_input()?;
        failure.reports.initialize_input_processor(input)?;
        failure.prepare_source_environment(&dir, operands)?;
        let error = failure
            .initialize_source_state(&ep_model::ReportInputs::default())
            .err()
            .ok_or("actual MTR open did not fail")?;
        assert!(error.contains("eplusrs.mtr"));
        assert!(
            failure
                .reports
                .source_initialization_guards
                .init_constant_state_called
        );
        assert!(
            failure
                .reports
                .source_initialization_guards
                .init_state_called
        );
        assert!(
            failure
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("moved IP missing")?
                .is_object_unused("Version", "v")
        );
        let version_line = format!(
            "Program Version,{}{}",
            failure.state.version_string,
            if cfg!(windows) { "\r\n" } else { "\n" }
        )
        .into_bytes();
        assert_eq!(std::fs::read(dir.join("eplusrs.eso"))?, version_line);
        assert_eq!(std::fs::read(dir.join("eplusrs.eio"))?, version_line);
        assert!(!dir.join("eplusrs.bnd").exists());
        assert!(!dir.join("eplusrs.audit").exists());
        // Both actual guards remain true after fatal. Reentry neither retries
        // the failed file nor truncates already reached output/header prefixes.
        failure.initialize_source_state(&ep_model::ReportInputs::default())?;
        assert!(failure.source_zone_timesteps_per_hour().is_err());
        failure.persist(&dir)?;
        assert!(
            String::from_utf8(std::fs::read(dir.join("eplusrs.ctf.err"))?)?
                .contains("OpenOutputFiles: Could not open file")
        );
        assert_eq!(std::fs::read(dir.join("eplusrs.eio"))?, version_line);
        drop(failure);
        for name in [
            "eplusrs.eso",
            "eplusrs.eio",
            "eplusrs.ctf.err",
            "eplusrs.ctf.log",
        ] {
            std::fs::remove_file(dir.join(name))?;
        }
        std::fs::remove_dir(dir.join("eplusrs.mtr"))?;
        let mut success = CtfRunOutputs::new("26.1");
        let (input, operands) = make_input()?;
        success.reports.initialize_input_processor(input)?;
        success.prepare_source_environment(&dir, operands)?;
        success.initialize_source_state(&ep_model::ReportInputs::default())?;
        let steps = success.source_zone_timesteps_per_hour()?;
        assert!(matches!(steps, 2 | 6)); // Source schema6, or real CI_FORCE_TIME_STEP2.
        let ip = success
            .reports
            .selected_input_processor
            .as_ref()
            .ok_or("sole IP missing")?;
        assert!(!ip.is_object_unused("Version", "v"));
        assert!(!ip.is_object_unused("Timestep", "t"));
        assert!(!ip.is_object_unused("Output:Diagnostics", "d"));
        assert!(
            ip.is_object_unused("Building", "b"),
            "later HB getter must not be fabricated"
        );
        let mut reporting = crate::warmup_report_controls::resolve(&raw)?;
        reporting.advanced_report_variables = false;
        success.apply_source_reporting_controls(&mut reporting)?;
        assert!(
            reporting.advanced_report_variables,
            "effective actual diagnostic owner reaches HB"
        );
        assert!(
            success
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("sole IP missing")?
                .is_object_unused("Output:Table:SummaryReports", "summary")
        );
        success.initialize_source_run_prefix(&ep_model::ReportInputs::default())?;
        assert_eq!(success.reports.source_run_available, Some(true));
        let expected_progress = [
            "Adjusting Air System Sizing",
            "Adjusting Standard 62.1 Ventilation Sizing",
            "Initializing Simulation",
        ];
        let progress = String::from_utf8(success.console.clone())?;
        let reached_progress = progress
            .lines()
            .filter(|line| expected_progress.contains(line))
            .collect::<Vec<_>>();
        assert_eq!(reached_progress, expected_progress.to_vec());
        assert!(!success.state.kickoff_simulation);
        assert!(success.reports.setup_simulation_registry_prepared);
        assert!(success.reports.registry.report_count > 0);
        assert!(
            !success.state.doing_sizing,
            "actual240 must follow successful234"
        );
        assert!(
            !success
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("sole IP missing")?
                .is_object_unused("Output:Table:SummaryReports", "summary")
        );
        success.persist(&dir)?;
        let actual_eio = std::fs::read(dir.join("eplusrs.eio"))?;
        let eio = String::from_utf8(actual_eio.clone())?;
        for section in [
            "! <Version>",
            "! <Timesteps per Hour>",
            "! <System Convergence Limits>",
            "! <Simulation Control>",
            "! <Performance Precision Tradeoffs>",
            "! <Output Reporting Tolerances>",
        ] {
            assert!(
                eio.contains(section),
                "actual Project EIO section {section}"
            );
        }
        assert_eq!(
            std::fs::read(dir.join("eplusrs.audit"))?,
            if cfg!(windows) {
                b"  Processing Schedule Input -- Start\r\n  Processing Schedule Input -- Complete\r\n"
                    .as_slice()
            } else {
                b"  Processing Schedule Input -- Start\n  Processing Schedule Input -- Complete\n"
                    .as_slice()
            }
        );
        success.initialize_source_state(&ep_model::ReportInputs::default())?;
        success.persist(&dir)?;
        assert_eq!(std::fs::read(dir.join("eplusrs.eio"))?, actual_eio);
        let reached_reports = success.reports.registry.report_count;
        let repeated = success
            .initialize_source_run_prefix(&ep_model::ReportInputs::default())
            .err()
            .ok_or("actual duplicate pointer did not stop")?;
        assert!(repeated.contains("SetupTimePointers was already called for Zone"));
        assert!(
            success.reports.registry.report_count > reached_reports,
            "actual repeated215 occurs before duplicate218 fatal"
        );
        assert!(!success.state.doing_sizing);
        assert!(
            !success
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("sole IP missing")?
                .is_object_unused("Output:Table:SummaryReports", "summary")
        );
        success.persist(&dir)?;
        assert_eq!(std::fs::read(dir.join("eplusrs.eio"))?, actual_eio);
        drop(success);

        // A genuine association failure reached inside234 keeps233 true and
        // retains all real six-handler/file/registry prefixes without retry.
        let mut sizing_failure = CtfRunOutputs::new("26.1");
        let (input, mut operands) = make_input()?;
        operands.sizing.summary_report_objects[0].object_name = "wrong permanent key".into();
        sizing_failure.reports.initialize_input_processor(input)?;
        sizing_failure.prepare_source_environment(&dir, operands)?;
        sizing_failure.initialize_source_state(&ep_model::ReportInputs::default())?;
        let sizing_error = sizing_failure
            .initialize_source_run_prefix(&ep_model::ReportInputs::default())
            .err()
            .ok_or("actual234 association did not fail")?;
        assert!(sizing_error.contains("summary permanent/extensible association"));
        assert!(
            sizing_failure.state.doing_sizing,
            "Err234 must not execute240"
        );
        assert!(sizing_failure.reports.setup_simulation_registry_prepared);
        assert!(sizing_failure.reports.registry.report_count > 0);
        assert!(
            sizing_failure
                .reports
                .selected_input_processor
                .as_ref()
                .ok_or("sole IP missing")?
                .is_object_unused("Output:Table:SummaryReports", "summary")
        );
        sizing_failure.persist(&dir)?;
        assert_eq!(std::fs::read(dir.join("eplusrs.eio"))?, actual_eio);
        drop(sizing_failure);
        for name in [
            "eplusrs.eso",
            "eplusrs.eio",
            "eplusrs.mtr",
            "eplusrs.bnd",
            "eplusrs.audit",
            "eplusrs.ctf.err",
            "eplusrs.ctf.log",
        ] {
            std::fs::remove_file(dir.join(name))?;
        }
        std::fs::remove_dir(dir)?;
        Ok(())
    }
}

impl CtfRunOutputs {
    pub(crate) fn complete_source_program(
        &mut self,
        weather: &ep_runtime::weather::day::ProductionWeatherTimestepSeries,
    ) -> Result<(), String> {
        let files = self
            .initialization_files
            .as_mut()
            .ok_or("actual files before normal program completion")?;
        {
            let mut backend = SourceErrorBackend {
                state: &mut self.state,
                error_stream: Some(&mut self.error),
                console: &mut self.console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            files
                .complete_source_program(
                    &mut self.reports,
                    &mut self.source_session,
                    weather,
                    &mut backend,
                )
                .map_err(|e| e.to_string())?;
        }
        files
            .persist_source_error(&self.error)
            .map_err(|e| e.to_string())
    }
}
