//! Authored live-owner cases for Native OutputProcessor538-545,3360,3470.
use super::super::*;
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

#[test]
fn scheduled_filter_resolves_the_live_schedule_and_rejects_a_missing_reference() -> TestResult {
    for (name, expected, dictionary_name) in [
        ("FILTER", Some(FILTER), "FILTER"),
        ("CONSTANT-1.0", Some(SourceScheduleIndex(1)), "Constant-1.0"),
        ("ABSENT", None, ""),
    ] {
        let (mut schedules, mut input) = live_schedules()?;
        let requests = request(name, "HOURLY");
        let mut operands = super::SourceReportVariableInput::with_schedules(
            &requests,
            &mut input,
            "",
            &mut schedules,
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
            prescan.add_record("*", "Schedule Value")?;
            let mut operands = super::SourceReportVariableInput::new(&requests, &mut input, "");
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
            let text = std::str::from_utf8(&dictionary.eso)?;
            assert!(
                text.ends_with(&format!(" !Hourly,{dictionary_name}\n")),
                "{text}"
            );
            assert_eq!(dictionary.variables.len(), 1);
        }
    }
    Ok(())
}

#[test]
fn scheduled_filter_without_its_live_owner_stays_rejected() -> TestResult {
    let (mut schedules, mut input) = live_schedules()?;
    let requests = request("FILTER", "HOURLY");
    let mut operands = super::SourceReportVariableInput::new(&requests, &mut input, "");
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
            "",
            &mut schedules,
        );
        let mut output = SourceOutputRegistration::default();
        output.get_report_variable_input(&mut operands, &Files::default(), &mut Vec::new())?;
        assert_eq!(
            output.requested_variables[0].reporting_schedule,
            Some(FILTER)
        );
    }
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
    let (mut schedules, _) = live_schedules()?;
    let mut output = output(
        SourceReportFrequency::Hour,
        SourceOutputStoreType::Sum,
        true,
    );
    let mut update = Update::new();
    let mut files = Files::default();
    output.initialize_output(15, &mut files)?;
    output.attach_source_energy_transfer_meters(
        SourceOutputUnits::J,
        SourceEnergyTransferMeterArguments {
            end_use: SourceMeterEndUse::Heating,
            end_use_subcategory: "",
            zone_name: "",
            zone_multiplier: 1,
            zone_list_multiplier: 1,
        },
        0,
    )?;
    output.meter_values = vec![0.0; output.meters.meters.len()];
    output.out_vars[0].time_step_type = TimeStepKind::System;
    update.hvac.time_step_sys = 0.25;
    schedules.schedules[VALUE.0].current_value = 8.0;
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
            8.0
        );
    }
    Ok(())
}
