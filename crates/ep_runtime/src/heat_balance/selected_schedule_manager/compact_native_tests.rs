//! One Native-paired cohort test for the retained Compact producer and clock lookup.
//! Fixture bytes are Rust compiler inputs; production never reads this reference.
use super::{
    SelectedScheduleInput, SelectedScheduleManager, SourceConstantScheduleInput,
    SourceScheduleTypeInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceInputStatisticsInput,
    SourceInputStatisticsValue, SourceObjectDefinition, SourceObjectShape,
};
use crate::heat_balance::source_initialization::ScheduleAuditFileHost;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ScheduleUnitType;
use serde_json::Value;
use std::{collections::BTreeMap, error::Error, io, path::Path};

include!("fixtures/compact_native_b.rs");

type TestResult<T> = Result<T, Box<dyn Error>>;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn array(value: &Value) -> TestResult<&Vec<Value>> {
    Ok(value.as_array().ok_or_else(|| invalid("fixture array"))?)
}
fn object(value: &Value) -> TestResult<&serde_json::Map<String, Value>> {
    Ok(value.as_object().ok_or_else(|| invalid("fixture object"))?)
}
fn string(value: &Value) -> TestResult<&str> {
    Ok(value.as_str().ok_or_else(|| invalid("fixture string"))?)
}
fn member<'a>(value: &'a Value, name: &str) -> TestResult<&'a Value> {
    Ok(object(value)?
        .get(name)
        .ok_or_else(|| invalid("fixture missing member"))?)
}
fn usize_value(value: &Value) -> TestResult<usize> {
    Ok(usize::try_from(
        value
            .as_u64()
            .ok_or_else(|| invalid("fixture unsigned integer"))?,
    )?)
}
fn raw_value(value: &Value) -> TestResult<SourceInputStatisticsValue> {
    Ok(match value {
        Value::Null => SourceInputStatisticsValue::Null,
        Value::Bool(v) => SourceInputStatisticsValue::Bool(*v),
        Value::Number(v) => SourceInputStatisticsValue::Number(v.to_string()),
        Value::String(v) => SourceInputStatisticsValue::String(v.clone()),
        Value::Array(v) => SourceInputStatisticsValue::Array(
            v.iter().map(raw_value).collect::<TestResult<Vec<_>>>()?,
        ),
        Value::Object(v) => SourceInputStatisticsValue::Object(
            v.iter()
                .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                .collect::<TestResult<BTreeMap<_, _>>>()?,
        ),
    })
}
fn json_size(value: &Value) -> usize {
    match value {
        Value::Null => 0,
        Value::Array(rows) => rows.len(),
        Value::Object(fields) => fields.len(),
        _ => 1,
    }
}
fn alpha(fields: &Value, name: &str) -> TestResult<Option<String>> {
    Ok(match object(fields)?.get(name) {
        None | Some(Value::Null) => None,
        Some(Value::String(v)) if v.is_empty() => None,
        Some(Value::String(v)) => Some(v.to_ascii_uppercase()),
        _ => return Err(invalid("actual schedule alpha field").into()),
    })
}
fn number(fields: &Value, name: &str) -> TestResult<Option<f64>> {
    Ok(match object(fields)?.get(name) {
        None | Some(Value::Null) => None,
        Some(Value::Number(v)) => Some(v.as_f64().ok_or_else(|| invalid("schedule number"))?),
        _ => return Err(invalid("actual schedule numeric field").into()),
    })
}
fn names(fixture: &Value, family: &str) -> TestResult<Vec<String>> {
    array(member(member(fixture, "object_order")?, family)?)?
        .iter()
        .map(|v| Ok(string(v)?.to_owned()))
        .collect()
}
fn processor(fixture: &Value, raw: &Value) -> TestResult<SelectedInputProcessor> {
    let mut definitions = Vec::new();
    for row in array(member(fixture, "definitions")?)? {
        let row = array(row)?;
        if row.len() != 6 {
            return Err(invalid("query shape width").into());
        }
        definitions.push(SourceObjectDefinition {
            object_type: string(&row[0])?.to_owned(),
            alpha_fields: usize_value(&row[1])?,
            numeric_fields: usize_value(&row[2])?,
            alpha_extensions: usize_value(&row[3])?,
            numeric_extensions: usize_value(&row[4])?,
            extension: if row[5].is_null() {
                None
            } else {
                Some(string(&row[5])?.to_owned())
            },
        });
    }
    let mut families = BTreeMap::new();
    let mut statistics = SourceInputStatisticsInput::default();
    for (family, objects) in object(raw)? {
        let mut shapes = Vec::new();
        let mut retained = BTreeMap::new();
        for (name, fields) in object(objects)? {
            shapes.push(SourceObjectShape {
                object_name: name.clone(),
                field_sizes: object(fields)?
                    .iter()
                    .map(|(k, v)| (k.clone(), json_size(v)))
                    .collect(),
            });
            retained.insert(
                name.clone(),
                object(fields)?
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                    .collect::<TestResult<BTreeMap<_, _>>>()?,
            );
        }
        families.insert(family.clone(), SourceInputFamily::Objects(shapes));
        statistics.objects.insert(family.clone(), retained);
        statistics
            .object_order
            .insert(family.clone(), names(fixture, family)?);
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(statistics)?;
    Ok(input)
}
fn schedule_declarations(fixture: &Value, raw: &Value) -> TestResult<SelectedScheduleInput> {
    let mut result = SelectedScheduleInput::default();
    for name in names(fixture, "ScheduleTypeLimits")? {
        let fields = member(member(raw, "ScheduleTypeLimits")?, &name)?;
        // All three actual declarations omit unit_type, retaining Native Invalid.
        if alpha(fields, "unit_type")?.is_some() {
            return Err(invalid("cohort type unit provenance changed").into());
        }
        result.types.push(SourceScheduleTypeInput {
            object_name: name.clone(),
            name: name.to_ascii_uppercase(),
            lower: number(fields, "lower_limit_value")?,
            upper: number(fields, "upper_limit_value")?,
            numeric_type: alpha(fields, "numeric_type")?,
            unit: ScheduleUnitType::Invalid,
        });
    }
    for name in names(fixture, "Schedule:Constant")? {
        let fields = member(member(raw, "Schedule:Constant")?, &name)?;
        result.constants.push(SourceConstantScheduleInput {
            object_name: name.clone(),
            name: name.to_ascii_uppercase(),
            type_name: alpha(fields, "schedule_type_limits_name")?,
            value: number(fields, "hourly_value")?
                .ok_or_else(|| invalid("actual constant value"))?,
        });
    }
    Ok(result)
}

#[derive(Default)]
struct MemoryAudit {
    bytes: Vec<u8>,
}
impl ScheduleAuditFileHost for MemoryAudit {
    fn audit_path(&self) -> &Path {
        Path::new("test-only-compact.audit")
    }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn io::Write> {
        Ok(&mut self.bytes)
    }
}

#[test]
fn compact_primary_and_empty_hour_match_native_reported_timestep_bits() -> TestResult<()> {
    let fixture: Value = serde_json::from_str(NATIVE_COMPACT_COHORTS_JSON)?;
    let steps = usize_value(member(&fixture, "time_steps_in_hour")?)?;
    assert_eq!(steps, 4);
    let cohorts = array(member(&fixture, "cohorts")?)?;
    assert_eq!(cohorts.len(), 2);
    for cohort in cohorts {
        let cohort_name = string(member(cohort, "name")?)?;
        let raw = member(cohort, "raw")?;
        let mut input = processor(&fixture, raw)?;
        let declarations = schedule_declarations(&fixture, raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            steps,
            &mut MemoryAudit::default(),
        )?;
        let native_declarations = array(member(cohort, "native_declarations")?)?;
        let mut native_names = Vec::new();
        for declaration in native_declarations {
            let declaration = array(declaration)?;
            if declaration.len() != 2 {
                return Err(invalid("Native dictionary pair width").into());
            }
            native_names.push(string(&declaration[1])?.to_owned());
        }
        assert_eq!(native_names.len(), 5);
        let actual_names = manager
            .schedules
            .iter()
            .skip(2)
            .map(|s| s.name.clone())
            .collect::<Vec<_>>();
        assert_eq!(
            actual_names, native_names,
            "{cohort_name}: actual declaration allocation order"
        );
        let mut indices = Vec::new();
        for (ordinal, name) in native_names.iter().enumerate() {
            let index = manager
                .get_source_schedule(name)?
                .ok_or_else(|| invalid("actual schedule lookup"))?;
            assert_eq!(index.0, ordinal + 2, "{cohort_name}: real manager index");
            indices.push(index);
        }
        let rows = array(member(cohort, "rows")?)?;
        assert_eq!(rows.len(), 96);
        for (ordinal, row) in rows.iter().enumerate() {
            let row = array(row)?;
            if row.len() != 2 {
                return Err(invalid("Native cohort row width").into());
            }
            let tuple = array(&row[0])?;
            let values = array(&row[1])?;
            assert_eq!(tuple.len(), 8);
            assert_eq!(values.len(), indices.len());
            // Consumed operands come from the actual Native ESO tuple. No year
            // exists in that tuple. Unconsumed weather/tomorrow fields remain
            // constructor state: DST0 cannot enter tomorrow's schedule branch.
            let integer = |i: usize| -> TestResult<i32> { Ok(string(&tuple[i])?.trim().parse()?) };
            let month = integer(1)?;
            let day = integer(2)?;
            let dst = integer(3)?;
            let hour = integer(4)?;
            assert_eq!(dst, 0);
            assert_eq!(string(&tuple[7])?, "Tuesday");
            let (end_minute, fraction) = string(&tuple[6])?
                .trim()
                .split_once('.')
                .ok_or_else(|| invalid("Native end-minute token"))?;
            assert!(fraction.bytes().all(|c| c == b'0'));
            let end_minute: i32 = end_minute.parse()?;
            let minutes_per_step = 60 / i32::try_from(steps)?;
            assert_eq!(end_minute % minutes_per_step, 0);
            let clock = WeatherGlobalState {
                hour_of_day: hour,
                time_step: end_minute / minutes_per_step,
                time_steps_in_hour: i32::try_from(steps)?,
                ..WeatherGlobalState::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: crate::weather::raw::source_ordinal_day(month, day, 1)?,
                day_of_week: 3,
                holiday_index: 0,
                dst_indicator: dst,
                ..WeatherEnvironmentState::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            for ((index, expected), name) in indices.iter().zip(values).zip(&native_names) {
                let expected: f64 = string(expected)?.parse()?;
                let actual = manager.source_current_schedule_value(*index)?;
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "{cohort_name}: {name}, Native tuple {ordinal}: {tuple:?}"
                );
            }
        }
    }
    verify_authored_compact_modes_and_diagnostic_prefix(&fixture)?;
    Ok(())
}

// Authored raw profile tables reuse the existing production-IP helper. They
// contain no Native result seed, gated cohort call or synthetic runtime clock.
fn authored_raw(fixture: &Value, mode: &str, overlap: bool) -> Value {
    let mut raw = fixture["cohorts"][0]["raw"].clone();
    raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(200);
    let fields = if overlap {
        vec![
            serde_json::json!("Through: 12/31"),
            serde_json::json!("For: AllDays"),
            serde_json::json!("Interpolate: No"),
            serde_json::json!("Until: 24:00"),
            serde_json::json!(10),
            serde_json::json!("Until: 01:00"),
            serde_json::json!(11),
        ]
    } else {
        vec![
            serde_json::json!("Through: 12/31"),
            serde_json::json!("For: AllDays"),
            serde_json::json!(format!("Interpolate: {mode}")),
            serde_json::json!("Until: 00:20"),
            serde_json::json!(10),
            serde_json::json!("Until: 01:15"),
            serde_json::json!(175),
            serde_json::json!("Until: 24:00"),
            serde_json::json!(175),
        ]
    };
    raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(
        fields
            .into_iter()
            .map(|v| serde_json::json!({"field":v}))
            .collect(),
    );
    raw
}

use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
fn interpolation_error_state() -> SourceErrorState {
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

fn verify_authored_compact_modes_and_diagnostic_prefix(fixture: &Value) -> TestResult<()> {
    use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
    let (kickoff, developer, warmup, sizing) = (false, false, true, false);
    let flags = SourceDiagnosticFlags {
        kickoff: &kickoff,
        developer: &developer,
        warmup: &warmup,
        doing_sizing: &sizing,
    };
    for (mode, first) in [
        ("No", [10.0, 175.0, 175.0, 175.0, 175.0]),
        ("Average", [10.0, 120.0, 175.0, 175.0, 175.0]),
        ("Linear", [10.0, 40.0, 85.0, 130.0, 175.0]),
    ] {
        let raw = authored_raw(fixture, mode, false);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            Vec::new(),
            Vec::new(),
            MemoryAudit::default(),
        );
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut errors),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            manager.process_schedule_input_with_live_backend(
                &mut input,
                &declarations,
                4,
                &mut audit,
                &mut backend,
                &flags,
            )?;
        }
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual authored Compact pointer")?;
        for ordinal in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: ordinal / 4 + 1,
                time_step: ordinal % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            let actual = manager.source_current_schedule_value(index)?;
            let expected: f64 = first.get(ordinal as usize).copied().unwrap_or(175.0);
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{mode} actual live lookup {ordinal}"
            );
        }
        // The last live currentVal is175, yet the same retained Detailed graph
        // contains10. This proves hasVal does not substitute the current value.
        let actual_clock = WeatherGlobalState {
            time_steps_in_hour: 4,
            ..Default::default()
        };
        let constructor_clock = WeatherGlobalState {
            time_steps_in_hour: 0,
            ..Default::default()
        };
        assert_eq!(manager.source_current_schedule_value(index)?, 175.0);
        assert!(manager.source_has_value(index, 10.0, &actual_clock)?);
        assert!(!manager.source_has_value(index, -1.0, &actual_clock)?);
        assert!(!manager.source_has_value(index, 10.0, &constructor_clock)?);
        let always_on = manager
            .get_source_schedule("CONSTANT-1.0")?
            .ok_or("actual built-in constant")?;
        assert!(manager.source_has_value(always_on, 1.0, &constructor_clock)?);
        assert_eq!(state.tracking.total_warning_errors, i32::from(mode == "No"));
        assert_eq!(
            state.tracking.total_warning_errors_during_warmup,
            i32::from(mode == "No")
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        assert!(!state.warmup_flag); // Source cells, not backend ctor, were read.
        assert!(
            std::str::from_utf8(&audit.bytes)?
                .ends_with("  Processing Schedule Input -- Complete\n")
        );
    }
    // One real Warning succeeded, then its Continue fails before population.
    struct FailContinue {
        bytes: Vec<u8>,
    }
    impl io::Write for FailContinue {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if String::from_utf8_lossy(bytes).contains("Other errors may result.") {
                return Err(io::Error::other("authored Compact Continue sink"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let raw = authored_raw(fixture, "No", false);
    let mut input = processor(fixture, &raw)?;
    let declarations = schedule_declarations(fixture, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        FailContinue { bytes: Vec::new() },
        Vec::new(),
        MemoryAudit::default(),
    );
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut audit,
            &mut backend,
            &flags,
        )
    };
    assert!(result.is_err());
    assert_eq!(
        (
            state.tracking.total_warning_errors,
            state.tracking.total_warning_errors_during_warmup
        ),
        (1, 1)
    );
    let day = manager
        .day_schedules
        .iter()
        .find(|d| d.name == "INTERNAL GAIN SCHEDULE_dy_1")
        .ok_or("actual day constructor")?;
    assert!(
        day.timestep_values
            .iter()
            .all(|v| v.to_bits() == 0.0_f64.to_bits())
    );
    assert_eq!(day.sum_timestep_values.to_bits(), 0.0_f64.to_bits());
    // A completed severe minute child emits its parent Continue, populates the
    // retained values, then reaches the real final fatal after validation.
    let raw = authored_raw(fixture, "No", true);
    let mut input = processor(fixture, &raw)?;
    let declarations = schedule_declarations(fixture, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        Vec::new(),
        Vec::new(),
        MemoryAudit::default(),
    );
    let result = {
        let mut backend = SourceErrorBackend {
            state: &mut state,
            error_stream: Some(&mut errors),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        manager.process_schedule_input_with_live_backend(
            &mut input,
            &declarations,
            4,
            &mut audit,
            &mut backend,
            &flags,
        )
    };
    assert!(result.is_err());
    assert_eq!(
        (
            state.tracking.total_severe_errors,
            state.tracking.total_severe_errors_during_warmup
        ),
        (1, 1)
    );
    let day = manager
        .day_schedules
        .iter()
        .find(|d| d.name == "INTERNAL GAIN SCHEDULE_dy_1")
        .ok_or("actual populated day")?;
    assert!(
        day.timestep_values
            .iter()
            .all(|v| v.to_bits() == 10.0_f64.to_bits())
    );
    assert_eq!(day.sum_timestep_values.to_bits(), 960.0_f64.to_bits());
    let error = std::str::from_utf8(&errors)?;
    assert!(error.contains("ref Schedule:Compact=\"INTERNAL GAIN SCHEDULE\""));
    assert!(error.contains("ProcessScheduleInput: Preceding Errors cause termination."));
    assert!(!std::str::from_utf8(&audit.bytes)?.contains("Processing Schedule Input -- Complete"));
    Ok(())
}
