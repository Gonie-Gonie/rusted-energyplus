use super::super::{
    SourceGeneralMeterArguments, SourceMeterEndUse, SourceMeterGroup, SourceMeterResource,
    SourceOutVarReal, SourceOutputReportStreams, SourceOutputUnits,
};
use super::*;
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::selected_project_data::{
    ProjectInputObject, ProjectInputValue, SelectedProjectData,
};
use crate::weather::day::WeatherGlobalState;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct Streams {
    eso: Vec<u8>,
    mtr: Vec<u8>,
    mtd: Vec<u8>,
    rf: Vec<String>,
    eso_path: PathBuf,
    mtr_path: PathBuf,
}
impl SourceOutputReportStreams for Streams {
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
impl SourceMeterReportingHost for Streams {
    fn write_mtd(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.mtd.extend_from_slice(bytes);
        Ok(())
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        &mut self.rf
    }
    fn eso_file_path(&self) -> &Path {
        &self.eso_path
    }
    fn mtr_file_path(&self) -> &Path {
        &self.mtr_path
    }
}

fn source_error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "authored caller".into(),
        idd_version_string: "authored input".into(),
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
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

fn registration() -> Result<SourceOutputRegistration, Box<dyn std::error::Error>> {
    let mut output = SourceOutputRegistration::default();
    // Genuine OutVar constructor followed by the real AddMeter/attachment
    // producer. The getter test does not substitute a time-series value owner.
    output.out_vars.push(SourceOutVarReal {
        units: SourceOutputUnits::J,
        ..SourceOutVarReal::default()
    });
    output.attach_source_general_meters(
        SourceOutputUnits::J,
        SourceGeneralMeterArguments {
            resource: SourceMeterResource::Electricity,
            end_use: SourceMeterEndUse::Invalid,
            end_use_subcategory: "",
            group: SourceMeterGroup::Invalid,
            zone_name: "",
            space_type: "",
            zone_multiplier: 1,
            zone_list_multiplier: 1,
        },
        0,
    )?;
    Ok(output)
}

fn requests(
    frequency: &str,
) -> Result<
    (
        SelectedHeatBalanceInput,
        SelectedInputProcessor,
        SourceInputProcessorShortCut,
    ),
    Box<dyn std::error::Error>,
> {
    let mut raw = SelectedHeatBalanceInput {
        source_is_epjson: true,
        ..SelectedHeatBalanceInput::default()
    };
    let mut families = BTreeMap::new();
    let mut definitions = Vec::new();
    for family in [
        "Output:Meter",
        "Output:Meter:MeterFileOnly",
        "Output:Meter:Cumulative",
        "Output:Meter:Cumulative:MeterFileOnly",
    ] {
        definitions.push(SourceObjectDefinition {
            object_type: family.into(),
            alpha_fields: 2,
            numeric_fields: 0,
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        });
        if family.contains("Cumulative") {
            continue;
        }
        let fields = BTreeMap::from([
            (
                "key_name".into(),
                ProjectInputValue::String("Electricity:Facility".into()),
            ),
            (
                "reporting_frequency".into(),
                ProjectInputValue::String(frequency.into()),
            ),
        ]);
        raw.objects.insert(
            family.into(),
            vec![ProjectInputObject {
                object_name: "".into(),
                fields,
                idf_max_fields: None,
            }],
        );
        families.insert(
            family.into(),
            SourceInputFamily::Objects(vec![SourceObjectShape {
                object_name: "".into(),
                field_sizes: BTreeMap::from([
                    ("key_name".into(), 1),
                    ("reporting_frequency".into(), 1),
                ]),
            }]),
        );
    }
    let input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    let mut shortcut = SourceInputProcessorShortCut::default();
    shortcut.allocate_from_input(&input)?;
    Ok((raw, input, shortcut))
}

#[test]
fn source_meter_getter_preserves_all_frequencies_and_real_dual_stream_duplicate_effects()
-> Result<(), Box<dyn std::error::Error>> {
    for (text, frequency) in [
        ("", SourceReportFrequency::Hour),
        ("Detailed", SourceReportFrequency::EachCall),
        ("Timestep", SourceReportFrequency::TimeStep),
        ("Hourly", SourceReportFrequency::Hour),
        ("Daily", SourceReportFrequency::Day),
        ("Monthly", SourceReportFrequency::Month),
        ("RunPeriod", SourceReportFrequency::Simulation),
        ("Environment", SourceReportFrequency::Simulation),
        ("Annual", SourceReportFrequency::Year),
    ] {
        let mut output = registration()?;
        let (raw, mut input, mut shortcut) = requests(text)?;
        let mut state = source_error_state();
        let clock = WeatherGlobalState {
            warmup_flag: true,
            ..WeatherGlobalState::default()
        };
        let globals = SelectedHeatBalanceGlobals::default();
        let project = SelectedProjectData::default();
        let (mut error, mut console) = (Vec::new(), Vec::new());
        let mut streams = Streams {
            eso_path: PathBuf::from("actual output").join("caller.eso"),
            mtr_path: PathBuf::from("actual output").join("caller.mtr"),
            ..Streams::default()
        };
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            output.update_source_meter_reporting(
                &mut SourceMeterReportingOwners {
                    raw: &raw,
                    input: &mut input,
                    shortcut: &mut shortcut,
                    backend: &mut backend,
                    diagnostic_flags: SourceDiagnosticFlags {
                        kickoff: &globals.kickoff_simulation,
                        developer: &project.system.developer_flag,
                        warmup: &clock.warmup_flag,
                        doing_sizing: &globals.doing_sizing,
                    },
                },
                &mut streams,
            )?;
        }
        let period = if frequency == SourceReportFrequency::EachCall {
            SourceReportFrequency::TimeStep
        } else {
            frequency
        };
        let meter = &output.meters.meters[0];
        assert!(
            meter.periods[period as usize].report
                && !meter.periods[period as usize].report_file_only
        );
        assert!(output.frequency_tracking_variables[frequency as usize]);
        assert_eq!(streams.eso, streams.mtr);
        assert_eq!(streams.rf.len(), 1);
        assert_eq!(std::str::from_utf8(&streams.eso)?.lines().count(), 1);
        assert_eq!(output.meter_values, vec![0.0]);
        let expected = format!(
            "Output:Meter:MeterFileOnly requested for \"Electricity:Facility\" ({}), already on \"Output:Meter\". Will report to both caller.eso and caller.mtr",
            period.dictionary_format().3
        );
        assert!(std::str::from_utf8(&error)?.contains(&expected));
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (1, 1)
        );
        assert!(!state.warmup_flag);
        assert!(
            !input.is_object_unused("Output:Meter", "")
                && !input.is_object_unused("Output:Meter:MeterFileOnly", "")
        );
        assert_eq!(output.meter_dictionary_observations.len(), 1);
        assert_eq!(output.meter_dictionary_observations[0].frequency, frequency);
        let mut reports =
            crate::heat_balance::ctf_initialization_context::CtfInitializationReports {
                output_registration: output,
                selected_project_data: Some(project),
                ..Default::default()
            };
        let files = catalog_files();
        let catalog = reports.source_reported_catalog(&files)?;
        assert_eq!(catalog.declarations.len(), 2);
        for declaration in &catalog.declarations {
            assert_eq!(
                declaration.dictionary_line()?.as_bytes(),
                streams
                    .eso
                    .strip_suffix(b"\n")
                    .ok_or("dictionary newline")?
            );
            assert_eq!(
                declaration.unreached_annual_reporting_boundary,
                frequency == SourceReportFrequency::Year
            );
        }
        if frequency == SourceReportFrequency::Year {
            reports.source_results_framework.meter_frames[frequency as usize]
                .initialize(&reports.output_registration.meters.meters, frequency);
            assert!(
                reports
                    .source_reported_catalog(&files)?
                    .declarations
                    .iter()
                    .all(|d| !d.unreached_annual_reporting_boundary)
            );
        }
    }
    Ok(())
}

struct Stop {
    bytes: Vec<u8>,
}
impl Write for Stop {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.bytes.len() == 9 {
            return Err(io::Error::other("authored meter warning sink failure"));
        }
        let n = (9 - self.bytes.len()).min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn source_meter_duplicate_warning_failure_retains_actual_dictionary_and_period_prefix()
-> Result<(), Box<dyn std::error::Error>> {
    let mut output = registration()?;
    let (raw, mut input, mut shortcut) = requests("Hourly")?;
    let mut state = source_error_state();
    let clock = WeatherGlobalState {
        warmup_flag: true,
        ..WeatherGlobalState::default()
    };
    let globals = SelectedHeatBalanceGlobals::default();
    let project = SelectedProjectData::default();
    let (mut error, mut console) = (Stop { bytes: Vec::new() }, Vec::new());
    let mut streams = Streams {
        eso_path: "caller.eso".into(),
        mtr_path: "caller.mtr".into(),
        ..Streams::default()
    };
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut error),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        output.update_source_meter_reporting(
            &mut SourceMeterReportingOwners {
                raw: &raw,
                input: &mut input,
                shortcut: &mut shortcut,
                backend: &mut backend,
                diagnostic_flags: SourceDiagnosticFlags {
                    kickoff: &globals.kickoff_simulation,
                    developer: &project.system.developer_flag,
                    warmup: &clock.warmup_flag,
                    doing_sizing: &globals.doing_sizing,
                },
            },
            &mut streams,
        )
    };
    assert!(matches!(result, Err(Error::Backend(_))));
    assert_eq!(error.bytes.len(), 9);
    assert_eq!(
        (
            state.tracking.total_warning_errors,
            state.tracking.total_warning_errors_during_warmup
        ),
        (1, 1)
    );
    assert_eq!(streams.eso, streams.mtr);
    assert_eq!(std::str::from_utf8(&streams.eso)?.lines().count(), 1);
    assert!(output.meters.meters[0].periods[SourceReportFrequency::Hour as usize].report);
    assert!(
        !output.meters.meters[0].periods[SourceReportFrequency::Hour as usize].report_file_only
    );
    assert!(output.meter_values.is_empty()); // The successful whole-getter suffix is unreached.
    assert!(streams.mtd.is_empty()); // ReportMeterDetails is after the warning.
    Ok(())
}

fn catalog_files() -> crate::heat_balance::source_initialization::SourceInitializationFiles {
    use crate::heat_balance::source_initialization::{
        SourceInitializationFilePaths, SourceInitializationFiles,
    };
    let path = PathBuf::from("unopened-meter-catalog-test");
    SourceInitializationFiles::new(SourceInitializationFilePaths {
        stat_input: path.clone(),
        eso: path.clone(),
        eio: path.clone(),
        mtr: path.clone(),
        bnd: path.clone(),
        dxf: path.clone(),
        rdd: path.clone(),
        mdd: path.clone(),
        audit: path.clone(),
        debug: path.clone(),
        end: path.clone(),
        error: path.clone(),
        dfs: path.clone(),
        tabular_csv: path.clone(),
        tabular_tab: path.clone(),
        tabular_fixed: path.clone(),
        tabular_html: path.clone(),
        tabular_xml: path,
    })
}
