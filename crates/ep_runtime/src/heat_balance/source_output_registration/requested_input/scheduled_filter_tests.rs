//! Authored live-owner cases for Native OutputProcessor538-545,3360,3470.
use super::super::*;
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::selected_project_data::SelectedProjectData;
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleInput, SelectedScheduleManager, SourceConstantScheduleInput,
    SourceScheduleIndex, SourceScheduleTypeInput,
};
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::ScheduleAuditFileHost;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::{ReportInputs, ScheduleUnitType, SourceOutputVariableInput};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const FILTER: SourceScheduleIndex = SourceScheduleIndex(2);
const VALUE: SourceScheduleIndex = SourceScheduleIndex(3);

#[derive(Default)]
struct Files {
    audit: Vec<u8>,
    eso: Vec<u8>,
    mtr: Vec<u8>,
    variables: Vec<String>,
}
impl ScheduleAuditFileHost for Files {
    fn audit_path(&self) -> &Path {
        Path::new("authored-memory-audit")
    }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn Write> {
        Ok(&mut self.audit)
    }
}
impl SourceOutputFileHost for Files {
    fn ensure_mtd_open(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl SourceOutputDictionaryHost for Files {
    fn eso_good(&self) -> bool {
        true
    }
    fn write_eso_dictionary(&mut self, line: &str) -> io::Result<()> {
        self.eso.extend_from_slice(line.as_bytes());
        Ok(())
    }
    fn sqlite_present(&self) -> bool {
        false
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        &mut self.variables
    }
}
impl SourceOutputReportStreams for Files {
    fn eso_good(&self) -> bool {
        true
    }
    fn mtr_good(&self) -> bool {
        true
    }
    fn write_eso(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.eso.extend_from_slice(bytes);
        Ok(())
    }
    fn write_mtr(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.mtr.extend_from_slice(bytes);
        Ok(())
    }
    fn sqlite_present(&self) -> bool {
        false
    }
}
impl Files {
    fn reported_values(&self) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let text = std::str::from_utf8(&self.eso)?;
        Ok(text
            .lines()
            .filter_map(|line| line.strip_prefix("100,"))
            .filter_map(|fields| fields.split(',').next())
            .map(str::parse::<f64>)
            .collect::<Result<Vec<_>, _>>()?)
    }
}

fn live_schedules()
-> Result<(SelectedScheduleManager, SelectedInputProcessor), Box<dyn std::error::Error>> {
    let definitions = [
        ("ScheduleTypeLimits", 3, 2),
        ("Schedule:Constant", 2, 1),
        ("Output:Schedules", 1, 0),
        ("Output:Variable", 4, 0),
    ]
    .map(|(kind, alphas, numerics)| SourceObjectDefinition {
        object_type: kind.into(),
        alpha_fields: alphas,
        numeric_fields: numerics,
        alpha_extensions: 0,
        numeric_extensions: 0,
        extension: None,
    });
    let families = [
        ("ScheduleTypeLimits", vec!["Filter Type"]),
        ("Schedule:Constant", vec!["Filter", "Value"]),
        ("Output:Variable", vec!["request"]),
    ]
    .map(|(kind, names)| {
        (
            kind.into(),
            SourceInputFamily::Objects(
                names
                    .into_iter()
                    .map(|name| SourceObjectShape {
                        object_name: name.into(),
                        field_sizes: BTreeMap::new(),
                    })
                    .collect(),
            ),
        )
    });
    let mut input = SelectedInputProcessor::from_admitted_input(
        definitions.to_vec(),
        BTreeMap::from(families),
    )?;
    let declarations = SelectedScheduleInput {
        source_is_epjson: None,
        types: vec![SourceScheduleTypeInput {
            object_name: "Filter Type".into(),
            name: "FILTER TYPE".into(),
            lower: None,
            upper: None,
            numeric_type: None,
            unit: ScheduleUnitType::Invalid,
        }],
        constants: ["Filter", "Value"]
            .map(|name| SourceConstantScheduleInput {
                object_name: name.into(),
                name: name.to_ascii_uppercase(),
                type_name: Some("FILTER TYPE".into()),
                value: 1.0,
            })
            .to_vec(),
    };
    let mut schedules = SelectedScheduleManager::default();
    schedules.init_constant_state(4)?;
    schedules.process_schedule_input(&mut input, &declarations, 4, &mut Files::default())?;
    Ok((schedules, input))
}

fn request(schedule: &str, frequency: &str) -> ReportInputs {
    ReportInputs {
        output_variables: vec![SourceOutputVariableInput {
            object_name: "request".into(),
            key: "*".into(),
            variable_name: "Schedule Value".into(),
            reporting_frequency: frequency.into(),
            schedule_name: schedule.into(),
            alpha_blank: [false; 4],
            default_applied: [false; 4],
            num_alphas: 4,
            is_epjson: true,
        }],
        ..ReportInputs::default()
    }
}

// Real authored phase owners and error streams, borrowed at each getter call.
struct GetterDiagnostics {
    state: SourceErrorState,
    errors: Vec<u8>,
    console: Vec<u8>,
    globals: crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
    project: SelectedProjectData,
    clock: WeatherGlobalState,
}
impl GetterDiagnostics {
    fn new() -> Self {
        Self {
            state: SourceErrorState {
                output_error_header: true,
                version_string: "authored Rust schedule caller".into(),
                idd_version_string: "authored typed input".into(),
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
            errors: Vec::new(),
            console: Vec::new(),
            globals: Default::default(),
            project: Default::default(),
            clock: Default::default(),
        }
    }
    fn borrow<'a>(
        &'a mut self,
        text: &'a str,
    ) -> (SourceReportVariableEnvironment<'a>, SourceErrorBackend<'a>) {
        (
            SourceReportVariableEnvironment::from_owners(
                text,
                &self.globals,
                &self.project.system.developer_flag,
                Some(&self.clock),
            ),
            SourceErrorBackend {
                state: &mut self.state,
                error_stream: Some(&mut self.errors),
                console: &mut self.console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            },
        )
    }
}

struct StopWrite {
    bytes: Vec<u8>,
    calls: usize,
    fail_at: usize,
}
impl Write for StopWrite {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.calls += 1;
        if self.calls == self.fail_at {
            return Err(io::Error::other("authored stopped stream"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn scheduled_filter_resolves_the_live_schedule_and_rejects_a_missing_reference() -> TestResult {
    for (key, name, expected, dictionary_name) in [
        ("*", "FILTER", Some(FILTER), "FILTER"),
        (
            "*",
            "CONSTANT-1.0",
            Some(SourceScheduleIndex(1)),
            "Constant-1.0",
        ),
        ("*", "ABSENT", None, ""),
        ("VaLu[ae]", "FILTER", Some(FILTER), "FILTER"),
    ] {
        let (mut schedules, mut input) = live_schedules()?;
        let mut requests = request(name, "HOURLY");
        requests.output_variables[0].key = key.into();
        let mut diagnostics = GetterDiagnostics::new();
        let (environment, mut backend) = diagnostics.borrow("");
        let mut operands = super::SourceReportVariableInput::with_schedules(
            &requests,
            &mut input,
            environment,
            &mut schedules,
            &mut backend,
        );
        let mut output = SourceOutputRegistration::default();
        let result =
            output.get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new());
        assert_eq!(result.is_ok(), expected.is_some());
        assert_eq!(output.requested_variables.len(), 1);
        assert_eq!(output.requested_variables[0].reporting_schedule, expected);
        assert!(!input.is_object_unused("Output:Variable", "request"));
        assert_eq!(
            schedules.schedules[FILTER.0].is_used,
            expected == Some(FILTER)
        );
        assert!(!schedules.schedules[VALUE.0].is_used);
        if expected.is_some() {
            assert_eq!(
                output.requested_variables[0].reporting_schedule_name,
                dictionary_name
            );
            let mut prescan = SourceOutputPreScan::default();
            // Actual backend and in-memory sinks, rather than a successful stub.
            prescan.add_record(key, "Schedule Value", &mut backend)?;
            let mut operands = super::SourceReportVariableInput::new(
                &requests,
                &mut input,
                environment,
                &mut backend,
            );
            let binding = SourceRealOutputBinding::new(
                SourceRealOutputTarget::Schedule { index: VALUE },
                &schedules,
            )?;
            let mut dictionary = Files::default();
            output.setup_bound_unmetered_zone_real(
                &mut operands,
                &prescan,
                SourceRealOutputDeclaration {
                    name: "Schedule Value",
                    key: "VALUE",
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                15,
                &mut Files::default(),
                &mut Vec::new(),
                binding,
                &mut dictionary,
            )?;
            assert_eq!(output.out_vars.len(), 1);
            assert_eq!(output.out_vars[0].schedule, expected);
            assert!(output.requested_variables[0].used);
            if key != "*" {
                assert_eq!(
                    output.requested_variables[0]
                        .case_insensitive_pattern
                        .as_ref()
                        .map(|pattern| pattern.pattern()),
                    Some("(?i)VaLu[ae]")
                );
            }
            let text = std::str::from_utf8(&dictionary.eso)?;
            assert!(
                text.ends_with(&format!(" !Hourly,{dictionary_name}\n")),
                "{text}"
            );
            assert_eq!(dictionary.variables.len(), 1);
        }
    }
    use SourceReportFrequency as F;
    const HEADER: &[u8] =
        b"! <Minimum Reporting Frequency (overriding input value)>, Value, Input Value\n";
    for (text, initial, requested, minimum, effective, warning) in [
        ("", F::Year, "TIMESTEP", F::Year, F::Year, None),
        (
            "Detailed",
            F::EachCall,
            "DETAILED",
            F::EachCall,
            F::EachCall,
            None,
        ),
        (
            "Timestep",
            F::EachCall,
            "DETAILED",
            F::TimeStep,
            F::TimeStep,
            None,
        ),
        ("Hourly", F::EachCall, "TIMESTEP", F::Hour, F::Hour, None),
        ("Daily", F::EachCall, "TIMESTEP", F::Day, F::Day, None),
        ("Monthly", F::EachCall, "TIMESTEP", F::Month, F::Month, None),
        (
            "RunPeriod",
            F::EachCall,
            "TIMESTEP",
            F::Simulation,
            F::Simulation,
            None,
        ),
        (
            "Environment",
            F::EachCall,
            "TIMESTEP",
            F::Simulation,
            F::Simulation,
            None,
        ),
        ("Annual", F::EachCall, "TIMESTEP", F::Year, F::Year, None),
        (
            "hour",
            F::EachCall,
            "TIMESTEP",
            F::Hour,
            F::Hour,
            Some(("hour", "Hourly")),
        ),
        (
            "Daily",
            F::EachCall,
            "hour",
            F::Day,
            F::Day,
            Some(("HOUR", "Hourly")),
        ),
        ("Ho", F::Year, "TIMESTEP", F::Hour, F::Hour, None),
        ("unknown", F::Year, "TIMESTEP", F::Hour, F::Hour, None),
        (" Hourly", F::Year, "TIMESTEP", F::Hour, F::Hour, None),
        ("éé", F::Year, "TIMESTEP", F::Hour, F::Hour, None),
        ("Hourly", F::Year, "TIMESTEP", F::Year, F::Year, None),
    ] {
        let (mut schedules, mut input) = live_schedules()?;
        let mut requests = request("FILTER", requested);
        requests.output_variables[0].key = "VaLu[ae]".into();
        requests.output_variables[0].variable_name = "Schedule Value [scalar]".into();
        let mut diagnostics = GetterDiagnostics::new();
        // These are independent real phase cells, deliberately different from
        // legacy backend fields, so the diagnostic must read the supplied view.
        diagnostics.clock.warmup_flag = true;
        diagnostics.globals.doing_sizing = text == "hour";
        let (environment, mut backend) = diagnostics.borrow(text);
        let mut operands = super::SourceReportVariableInput::with_schedules(
            &requests,
            &mut input,
            environment,
            &mut schedules,
            &mut backend,
        );
        let mut output = SourceOutputRegistration {
            minimum_report_frequency: initial,
            ..SourceOutputRegistration::default()
        };
        let mut eio = Vec::new();
        output.get_report_variable_input(&mut operands, &Files::default(), &mut eio)?;
        assert_eq!(output.minimum_report_frequency, minimum, "{text}");
        assert_eq!(output.requested_variables.len(), 1);
        let row = &output.requested_variables[0];
        assert_eq!(row.frequency, effective);
        assert_eq!(row.key, "VaLu[ae]");
        assert_eq!(row.name, "Schedule Value");
        assert_eq!(row.reporting_schedule, Some(FILTER));
        assert!(!row.used);
        assert_eq!(
            row.case_insensitive_pattern.as_ref().map(|r| r.pattern()),
            Some("(?i)VaLu[ae]")
        );
        let expected = if text.is_empty() {
            Vec::new()
        } else {
            let mut bytes = HEADER.to_vec();
            bytes.extend_from_slice(
                format!(
                    " Minimum Reporting Frequency, {},{}\n",
                    minimum.dictionary_format().2,
                    text
                )
                .as_bytes(),
            );
            bytes
        };
        assert_eq!(eio, expected);
        // A second read is a genuine no-op, including an already-consumed IP
        // request and the two EIO records.
        output.get_report_variable_input(&mut operands, &Files::default(), &mut eio)?;
        assert_eq!(output.requested_variables.len(), 1);
        assert_eq!(eio, expected);
        assert!(!input.is_object_unused("Output:Variable", "request"));
        assert!(schedules.schedules[FILTER.0].is_used);
        assert_eq!(
            diagnostics.state.tracking.total_warning_errors,
            i32::from(warning.is_some())
        );
        assert_eq!(
            diagnostics
                .state
                .tracking
                .total_warning_errors_during_warmup,
            i32::from(warning.is_some() && text != "hour")
        );
        assert_eq!(
            diagnostics
                .state
                .tracking
                .total_warning_errors_during_sizing,
            i32::from(warning.is_some() && text == "hour")
        );
        let error_text = std::str::from_utf8(&diagnostics.errors)?;
        if let Some((raw, canonical)) = warning {
            let first = format!(
                "DetermineFrequency: Entered frequency=\"{raw}\" is not an exact match to key strings."
            );
            let second = format!("Frequency={canonical} will be used.");
            assert!(
                error_text.find(&first).ok_or("missing frequency warning")?
                    < error_text
                        .find(&second)
                        .ok_or("missing frequency continue")?
            );
        } else {
            assert!(error_text.is_empty());
        }
    }

    // Failure prefixes preserve Native flag/assignment/object-getter ordering.
    for failure in [
        "legacy-warning-unavailable",
        "warning-stream",
        "eio-header",
        "eio-value",
    ] {
        let (mut schedules, mut input) = live_schedules()?;
        let requests = request("FILTER", "TIMESTEP");
        let mut diagnostics = GetterDiagnostics::new();
        let mut stopped_error = StopWrite {
            bytes: Vec::new(),
            calls: 0,
            fail_at: 1,
        };
        let text = if failure.starts_with("eio-") {
            "Daily"
        } else {
            "hour"
        };
        let (mut environment, mut backend) = diagnostics.borrow(text);
        if failure == "legacy-warning-unavailable" {
            environment.diagnostic_flags = None;
        }
        if failure == "warning-stream" {
            backend.error_stream = Some(&mut stopped_error);
        }
        let mut operands = super::SourceReportVariableInput::with_schedules(
            &requests,
            &mut input,
            environment,
            &mut schedules,
            &mut backend,
        );
        let mut eio = StopWrite {
            bytes: Vec::new(),
            calls: 0,
            fail_at: if failure == "eio-header" { 1 } else { 2 },
        };
        let mut output = SourceOutputRegistration::default();
        let error =
            match output.get_report_variable_input(&mut operands, &Files::default(), &mut eio) {
                Err(error) => error,
                Ok(()) => {
                    return Err(format!("unexpected successful frequency prefix: {failure}").into());
                }
            };
        assert!(!output.get_output_input);
        assert_eq!(
            output.minimum_report_frequency,
            if failure.starts_with("eio-") {
                F::Day
            } else {
                F::EachCall
            }
        );
        assert!(output.requested_variables.is_empty());
        assert!(matches!(
            error,
            OrdinaryRadiationError::Input(_) | OrdinaryRadiationError::Output(_)
        ));
        output.get_report_variable_input(&mut operands, &Files::default(), &mut eio)?;
        assert!(input.is_object_unused("Output:Variable", "request"));
        assert!(!schedules.schedules[FILTER.0].is_used);
        if failure == "eio-value" {
            assert_eq!(eio.bytes, HEADER);
        } else {
            assert!(eio.bytes.is_empty());
        }
        assert_eq!(
            eio.calls,
            if failure == "eio-header" {
                1
            } else if failure == "eio-value" {
                2
            } else {
                0
            }
        );
        assert_eq!(
            diagnostics.state.tracking.total_warning_errors,
            i32::from(failure == "warning-stream")
        );
    }

    // A canonical override does not need unavailable legacy phase cells. A
    // later abbreviated request does: retain CI/name writes before that stop,
    // while frequency/schedule retain their original constructor state.
    {
        let (mut schedules, mut input) = live_schedules()?;
        let mut requests = request("FILTER", "hour");
        requests.output_variables[0].key = "VaLu[ae]".into();
        requests.output_variables[0].variable_name = "Schedule Value [scalar]".into();
        let mut diagnostics = GetterDiagnostics::new();
        let (_, mut backend) = diagnostics.borrow("Daily");
        let environment = SourceReportVariableEnvironment::new("Daily", None);
        let mut operands = super::SourceReportVariableInput::with_schedules(
            &requests,
            &mut input,
            environment,
            &mut schedules,
            &mut backend,
        );
        let mut output = SourceOutputRegistration::default();
        let mut eio = Vec::new();
        assert!(
            output
                .get_report_variable_input(&mut operands, &Files::default(), &mut eio)
                .is_err()
        );
        assert!(!output.get_output_input);
        assert_eq!(output.minimum_report_frequency, F::Day);
        assert_eq!(output.requested_variables.len(), 1);
        let retained = &output.requested_variables[0];
        assert_eq!(retained.key, "VaLu[ae]");
        assert_eq!(retained.name, "Schedule Value");
        assert_eq!(
            retained
                .case_insensitive_pattern
                .as_ref()
                .map(|r| r.pattern()),
            Some("(?i)VaLu[ae]")
        );
        assert_eq!(retained.frequency, F::Hour);
        assert_eq!(retained.reporting_schedule, None);
        assert!(!retained.used);
        output.get_report_variable_input(&mut operands, &Files::default(), &mut eio)?;
        assert!(!input.is_object_unused("Output:Variable", "request"));
        assert!(!schedules.schedules[FILTER.0].is_used);
        assert_eq!(diagnostics.state.tracking.total_warning_errors, 0);
        assert!(diagnostics.errors.is_empty());
        let mut expected = HEADER.to_vec();
        expected.extend_from_slice(
            format!(
                " Minimum Reporting Frequency, {},Daily\n",
                F::Day.dictionary_format().2
            )
            .as_bytes(),
        );
        assert_eq!(eio, expected);
    }

    // Override effects precede even the zero-object count; the same minimum is
    // later consumed by the shared Output:Meter frequency helper.
    let empty = ReportInputs::default();
    let mut empty_input = SelectedInputProcessor::from_admitted_input(Vec::new(), BTreeMap::new())?;
    let mut diagnostics = GetterDiagnostics::new();
    let (environment, mut backend) = diagnostics.borrow("Hourly");
    let mut operands =
        super::SourceReportVariableInput::new(&empty, &mut empty_input, environment, &mut backend);
    let mut output = SourceOutputRegistration::default();
    let mut eio = Vec::new();
    output.get_report_variable_input(&mut operands, &Files::default(), &mut eio)?;
    assert_eq!(output.minimum_report_frequency, F::Hour);
    assert_eq!(
        eio,
        [HEADER, b" Minimum Reporting Frequency,  !Hourly,Hourly\n"].concat()
    );
    assert!(output.requested_variables.is_empty());

    Ok(())
}

#[test]
fn scheduled_filter_without_its_live_owner_stays_rejected() -> TestResult {
    let mut diagnostics = GetterDiagnostics::new();
    let (environment, mut backend) = diagnostics.borrow("");
    let (mut schedules, mut input) = live_schedules()?;
    let requests = request("FILTER", "HOURLY");
    let mut operands =
        super::SourceReportVariableInput::new(&requests, &mut input, environment, &mut backend);
    assert!(
        SourceOutputRegistration::default()
            .get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new())
            .is_err()
    );
    for frequency in [
        "DETAILED",
        "TIMESTEP",
        "HOURLY",
        "DAILY",
        "MONTHLY",
        "RUNPERIOD",
        "ANNUAL",
    ] {
        let requests = request("FILTER", frequency);
        let mut operands = super::SourceReportVariableInput::with_schedules(
            &requests,
            &mut input,
            environment,
            &mut schedules,
            &mut backend,
        );
        let mut output = SourceOutputRegistration::default();
        output.get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new())?;
        assert_eq!(
            output.requested_variables[0].reporting_schedule,
            Some(FILTER)
        );
    }
    // The actual getter retains invalid CI construction and writes later fields.
    let mut invalid_requests = request("FILTER", "HOURLY");
    invalid_requests.output_variables[0].key = "[".into();
    let mut operands = super::SourceReportVariableInput::with_schedules(
        &invalid_requests,
        &mut input,
        environment,
        &mut schedules,
        &mut backend,
    );
    let mut invalid = SourceOutputRegistration::default();
    invalid.get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new())?;
    let retained = &invalid.requested_variables[0];
    assert!(!retained.is_simple_string);
    let pattern = retained
        .case_insensitive_pattern
        .as_ref()
        .ok_or("CI object not retained")?;
    assert_eq!(pattern.pattern(), "(?i)[");
    assert!(!pattern.ok());
    assert_eq!(retained.name, "Schedule Value");
    assert_eq!(retained.frequency, SourceReportFrequency::Hour);
    assert_eq!(retained.reporting_schedule, Some(FILTER));
    assert!(!retained.used);
    assert!(
        invalid
            .match_report_variables("Schedule Value", "VALUE")
            .is_empty()
    );
    assert!(!invalid.requested_variables[0].used);

    // Four genuine requests preserve first frequency plus live-schedule identity.
    let mut requests = ReportInputs::default();
    for (index, (key, frequency, schedule)) in [
        ("VaLu[ae]", "HOURLY", "FILTER"),
        ("Value", "HOURLY", "FILTER"),
        ("VaLu[ae]", "TIMESTEP", "FILTER"),
        ("VaLu[ae]", "HOURLY", "CONSTANT-1.0"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut row = request(schedule, frequency)
            .output_variables
            .into_iter()
            .next()
            .ok_or("actual request missing")?;
        row.object_name = format!("request{index}");
        row.key = key.into();
        requests.output_variables.push(row);
    }
    let mut request_input = SelectedInputProcessor::from_admitted_input(
        vec![SourceObjectDefinition {
            object_type: "Output:Variable".into(),
            alpha_fields: 4,
            numeric_fields: 0,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        }],
        BTreeMap::from([(
            "Output:Variable".into(),
            SourceInputFamily::Objects(
                requests
                    .output_variables
                    .iter()
                    .map(|row| SourceObjectShape {
                        object_name: row.object_name.clone(),
                        field_sizes: BTreeMap::new(),
                    })
                    .collect(),
            ),
        )]),
    )?;
    let mut operands = super::SourceReportVariableInput::with_schedules(
        &requests,
        &mut request_input,
        environment,
        &mut schedules,
        &mut backend,
    );
    let mut output = SourceOutputRegistration::default();
    output.get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new())?;
    assert!(
        output
            .requested_variables
            .iter()
            .all(|request| !request.used)
    );
    assert!(
        output
            .match_report_variables("unrelated variable", "VALUE")
            .is_empty()
    );
    assert!(
        output
            .requested_variables
            .iter()
            .all(|request| !request.used)
    );
    assert_eq!(
        output.match_report_variables("SCHEDULE VALUE", "VALUE"),
        [0, 2, 3]
    );
    assert!(
        output
            .requested_variables
            .iter()
            .all(|request| request.used)
    );
    // Non-simple requested keys have no DataOutputs literal-key escape.
    assert!(
        output
            .match_report_variables("Schedule Value", "VaLu[ae]")
            .is_empty()
    );
    output.get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new())?;
    assert_eq!(output.requested_variables.len(), 4);
    Ok(())
}

struct NoIntegers;
impl SourceIntegerOutputOwners for NoIntegers {
    fn integer_output(
        &self,
        _target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        Err(OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}

struct Update {
    clock: WeatherGlobalState,
    environment: WeatherEnvironmentState,
    hvac: SelectedHvacGlobals,
    project: SelectedProjectData,
    results: SourceResultsFrameworkOptions,
    records: i64,
    meter_records: i64,
}
impl Update {
    fn new() -> Self {
        Self {
            clock: WeatherGlobalState {
                time_step_zone: 0.25,
                minutes_in_time_step: 15,
                hour_of_day: 1,
                day_of_sim_chr: "1".into(),
                calendar_year: 2013,
                calendar_year_chr: "2013".into(),
                warmup_flag: false,
                ..WeatherGlobalState::default()
            },
            environment: WeatherEnvironmentState {
                month: 1,
                day_of_month: 1,
                day_of_week: 3,
                ..WeatherEnvironmentState::default()
            },
            hvac: SelectedHvacGlobals::default(),
            project: SelectedProjectData::default(),
            results: SourceResultsFrameworkOptions::default(),
            records: 0,
            meter_records: 0,
        }
    }
    fn step(
        &mut self,
        output: &mut SourceOutputRegistration,
        schedules: &SelectedScheduleManager,
        kind: TimeStepKind,
        files: &mut Files,
    ) -> TestResult {
        output.update_source_data_and_report(
            kind,
            &mut SourceOutputUpdateOwners {
                clock: &mut self.clock,
                environment: SourceOutputUpdateEnvironment {
                    month: &self.environment.month,
                    day_of_month: &self.environment.day_of_month,
                    day_of_week: &self.environment.day_of_week,
                    holiday_index: &self.environment.holiday_index,
                    dst_indicator: &self.environment.dst_indicator,
                    end_month_flag: &mut self.environment.end_month_flag,
                    end_year_flag: &self.environment.end_year_flag,
                },
                hvac: &self.hvac,
                project: &self.project,
                results: &mut self.results,
                standard_output_record_count: &mut self.records,
                standard_meter_record_count: &mut self.meter_records,
                real: schedules,
                integers: &NoIntegers,
            },
            files,
        )?;
        Ok(())
    }
}

fn output(
    frequency: SourceReportFrequency,
    store: SourceOutputStoreType,
    filtered: bool,
) -> SourceOutputRegistration {
    let mut output = SourceOutputRegistration::default();
    output.time_pointers.zone.target = Some(TimeStepTarget::GlobalZoneTimeStep);
    output.time_pointers.system.target = Some(TimeStepTarget::HvacSystemTimeStep);
    output.frequency_tracking_variables[SourceReportFrequency::Hour as usize] =
        frequency == SourceReportFrequency::Hour;
    output.out_vars.push(SourceOutVarReal {
        time_step_type: TimeStepKind::Zone,
        store_type: store,
        report: true,
        frequency,
        report_id: 100,
        schedule: filtered.then_some(FILTER),
        which: Some(SourceRealOutputTarget::Schedule { index: VALUE }),
        ..SourceOutVarReal::default()
    });
    output
}

#[test]
fn scheduled_filter_positive_zero_negative_transitions_preserve_average_sum_and_counts()
-> TestResult {
    for frequency in [
        SourceReportFrequency::EachCall,
        SourceReportFrequency::TimeStep,
        SourceReportFrequency::Hour,
    ] {
        for store in [SourceOutputStoreType::Average, SourceOutputStoreType::Sum] {
            for filtered in [false, true] {
                let (mut schedules, _) = live_schedules()?;
                let mut output = output(frequency, store, filtered);
                let mut update = Update::new();
                let mut files = Files::default();
                for (index, (filter, value)) in [(1.0, 2.0), (0.0, 4.0), (-1.0, 6.0), (0.0, 8.0)]
                    .into_iter()
                    .enumerate()
                {
                    schedules.schedules[FILTER.0].current_value = filter;
                    schedules.schedules[VALUE.0].current_value = value;
                    update.clock.end_hour_flag = index == 3;
                    update.step(&mut output, &schedules, TimeStepKind::Zone, &mut files)?;
                }
                let expected = match (frequency, store, filtered) {
                    (
                        SourceReportFrequency::EachCall | SourceReportFrequency::TimeStep,
                        _,
                        true,
                    ) => vec![2.0, 6.0],
                    (
                        SourceReportFrequency::EachCall | SourceReportFrequency::TimeStep,
                        _,
                        false,
                    ) => vec![2.0, 4.0, 6.0, 8.0],
                    (_, SourceOutputStoreType::Average, true) => vec![4.0],
                    (_, SourceOutputStoreType::Average, false) => vec![5.0],
                    (_, SourceOutputStoreType::Sum, true) => vec![8.0],
                    (_, SourceOutputStoreType::Sum, false) => vec![20.0],
                };
                assert_eq!(files.reported_values()?, expected);
                assert_eq!(update.records, expected.len() as i64);
                assert_eq!(output.out_vars[0].external_interface_time_step_value, 8.0);
            }
        }
    }
    Ok(())
}

#[test]
fn scheduled_filter_preserves_existing_period_averages_sums_and_extrema() -> TestResult {
    for frequency in [
        SourceReportFrequency::Day,
        SourceReportFrequency::Month,
        SourceReportFrequency::Simulation,
        SourceReportFrequency::Year,
    ] {
        for store in [SourceOutputStoreType::Average, SourceOutputStoreType::Sum] {
            let (mut schedules, _) = live_schedules()?;
            let mut output = output(frequency, store, true);
            output.frequency_tracking_variables[frequency as usize] = true;
            let mut update = Update::new();
            update.environment.month = 12;
            update.environment.day_of_month = 31;
            update.clock.hour_of_day = 24;
            let mut files = Files::default();
            for (index, (filter, value)) in [(1.0, 2.0), (0.0, 4.0), (-1.0, 6.0), (0.0, 8.0)]
                .into_iter()
                .enumerate()
            {
                schedules.schedules[FILTER.0].current_value = filter;
                schedules.schedules[VALUE.0].current_value = value;
                let final_step = index == 3;
                update.clock.end_hour_flag = final_step;
                update.clock.end_day_flag = final_step;
                update.clock.end_envrn_flag = final_step;
                update.environment.end_month_flag = final_step;
                update.environment.end_year_flag = final_step;
                update.step(&mut output, &schedules, TimeStepKind::Zone, &mut files)?;
            }
            let expected = if store == SourceOutputStoreType::Average {
                4.0
            } else {
                8.0
            };
            assert_eq!(files.reported_values()?, vec![expected]);
            assert_eq!(update.records, 1);
            let text = std::str::from_utf8(&files.eso)?;
            let fields = text
                .lines()
                .find_map(|line| line.strip_prefix("100,"))
                .ok_or("missing period variable row")?
                .split(',')
                .map(str::trim)
                .collect::<Vec<_>>();
            // Extrema are updated before Native3360's reporting filter: the
            // OFF interval's value8 remains the maximum; no filter-specific reset.
            assert_eq!(fields[1].parse::<f64>()?, 2.0);
            let maximum_index = match frequency {
                SourceReportFrequency::Day => 4,
                SourceReportFrequency::Month => 5,
                _ => 6,
            };
            assert_eq!(fields[maximum_index].parse::<f64>()?, 8.0);
        }
    }
    Ok(())
}

#[test]
fn scheduled_filter_reads_live_ems_override_without_changing_schedule_report_value() -> TestResult {
    let (mut schedules, _) = live_schedules()?;
    let mut output = output(
        SourceReportFrequency::TimeStep,
        SourceOutputStoreType::Average,
        true,
    );
    let mut update = Update::new();
    let mut files = Files::default();
    schedules.schedules[FILTER.0].current_value = 1.0;
    schedules.schedules[FILTER.0].ems_actuated_on = true;
    schedules.schedules[FILTER.0].ems_value = 0.0;
    update.step(&mut output, &schedules, TimeStepKind::Zone, &mut files)?;
    assert!(files.reported_values()?.is_empty());
    schedules.schedules[FILTER.0].ems_value = -2.0;
    update.step(&mut output, &schedules, TimeStepKind::Zone, &mut files)?;
    assert_eq!(files.reported_values()?, vec![1.0]);
    assert_eq!(
        *schedules.real_output(SourceRealOutputTarget::Schedule { index: FILTER })?,
        1.0
    );
    assert_eq!(
        *schedules.real_output(SourceRealOutputTarget::ReportingSchedule { index: FILTER })?,
        -2.0
    );
    Ok(())
}

#[test]
fn scheduled_filter_is_read_again_at_zone_handoff_and_does_not_filter_meter_energy() -> TestResult {
    for multiplier in [1_i32, 2, i32::MAX] {
        let (mut schedules, _) = live_schedules()?;
        let mut output = output(
            SourceReportFrequency::Hour,
            SourceOutputStoreType::Sum,
            true,
        );
        // Authored registered-variable factors exercise the real live meter path;
        // the source-backed getter/cohort and physical cases own input custody.
        output.out_vars[0].zone_multiplier = multiplier;
        let mut update = Update::new();
        let mut files = Files::default();
        output.initialize_output(15, &mut files)?;
        output.attach_source_energy_transfer_meters(
            SourceOutputUnits::J,
            SourceEnergyTransferMeterArguments {
                end_use: SourceMeterEndUse::Heating,
                end_use_subcategory: "",
                zone_name: "",
                zone_multiplier: multiplier,
                zone_list_multiplier: 1,
            },
            0,
        )?;
        output.meter_values = vec![0.0; output.meters.meters.len()];
        output.out_vars[0].time_step_type = TimeStepKind::System;
        update.hvac.time_step_sys = 0.25;
        schedules.schedules[VALUE.0].current_value = 8.0;
        assert_eq!(
            output.instant_source_meter_value(0, TimeStepKind::System, &schedules)?,
            8.0 * f64::from(multiplier),
        );
        update.step(&mut output, &schedules, TimeStepKind::System, &mut files)?;
        assert_eq!(output.out_vars[0].this_time_step_count, 1);
        schedules.schedules[FILTER.0].current_value = 0.0;
        update.clock.end_hour_flag = true;
        update.step(&mut output, &schedules, TimeStepKind::Zone, &mut files)?;
        assert_eq!(files.reported_values()?, vec![0.0]);
        assert_eq!(output.out_vars[0].external_interface_time_step_value, 8.0);
        assert!(!output.meters.meters.is_empty());
        for meter in &output.meters.meters {
            assert_eq!(
                meter.periods[SourceReportFrequency::Simulation as usize].value,
                8.0 * f64::from(multiplier)
            );
        }
    }

    Ok(())
}
