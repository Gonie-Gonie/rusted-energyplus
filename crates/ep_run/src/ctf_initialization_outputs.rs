//! Real Rust ordinary-run initialization owners, independent of runtime Result.
//!
//! These are Rust-owned header strings and explicit absent optional adapters;
//! Native version/header/callback/SQLite byte parity is not claimed.

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
    reports: CtfInitializationReports,
    state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
    reached: bool,
    initialization_files: Option<SourceInitializationFiles>,
}

impl CtfRunOutputs {
    pub(crate) fn new(typed_input_version: &str) -> Self {
        Self {
            reports: CtfInitializationReports::default(),
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
            self.reports.source_initialization_input = Some(input);
            self.initialization_files = Some(SourceInitializationFiles::new(
                SourceInitializationFilePaths {
                    eso: output_dir.join("eplusrs.eso"),
                    eio: output_dir.join("eplusrs.eio"),
                    mtr: output_dir.join("eplusrs.mtr"),
                    bnd: output_dir.join("eplusrs.bnd"),
                    audit: output_dir.join("eplusrs.audit"),
                },
            ));
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

    pub(crate) fn persist(&mut self, output_dir: &Path) -> Result<(), String> {
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
            std::fs::write(output_dir.join(name), bytes)
                .map_err(|error| format!("failed to write {name}: {error}"))?;
        }
        if let Some(files) = self.initialization_files.as_mut() {
            files.flush_reached().map_err(|e| e.to_string())?;
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
            "Output:Diagnostics":{"d":{"diagnostics":[{"key":"DisplayAdvancedReportVariables"}]}}
        }"#,
        )?;
        let make_input = || -> Result<_, String> {
            let input = crate::ordinary_input_processor::resolve(&raw)?;
            let operands = SourceInitializationInput {
                project: crate::ordinary_project_data::resolve(&raw, raw.is_epjson(), &input)?,
                schedules: crate::ordinary_schedule_input::resolve(&raw)?,
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
        let version_line =
            format!("Program Version,{}\n", failure.state.version_string).into_bytes();
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
            b"  Processing Schedule Input -- Start\n  Processing Schedule Input -- Complete\n"
        );
        success.initialize_source_state(&ep_model::ReportInputs::default())?;
        success.persist(&dir)?;
        assert_eq!(std::fs::read(dir.join("eplusrs.eio"))?, actual_eio);
        drop(success);
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
