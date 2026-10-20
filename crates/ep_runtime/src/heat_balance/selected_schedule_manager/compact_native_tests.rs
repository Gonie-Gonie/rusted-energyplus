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
    // This existing owner cohort also exercises the common numeric child:
    // preserve Native error-zero vs non-finite-success before any real cast.
    use ep_model::{
        SourceNumberOutcome, process_source_number, process_source_number_outcome,
        source_finite_number_prefix,
    };
    for (text, expected) in [
        ("1e-9999", 0.0_f64),
        ("-1e-9999", -0.0),
        ("1D-9999", 0.0),
        ("-1D-9999", -0.0),
        ("0e9999", 0.0),
        ("-0D9999", -0.0),
        ("15+E0", 15.0),
    ] {
        let SourceNumberOutcome::Finite(actual) = process_source_number_outcome(text) else {
            return Err("actual finite ProcessNumber outcome".into());
        };
        assert_eq!(actual.to_bits(), expected.to_bits(), "{text}");
        assert_eq!(
            process_source_number(text).map(f64::to_bits),
            Ok(expected.to_bits()),
            "{text}"
        );
    }
    for text in [
        "nan",
        "-nan",
        "INF",
        "-infinity",
        "nan(payload)",
        "nan()",
        "nan(",
        "nan(bad!)E",
        "nan(payload)X",
        "nanE+1",
        "1e9999",
        "-1e9999",
        "1e9999X",
        "1e9999E+1",
        "+1",
        "junk",
    ] {
        assert_eq!(
            process_source_number_outcome(text),
            SourceNumberOutcome::ErrorZero,
            "{text}"
        );
        assert!(process_source_number(text).is_err(), "{text}");
    }
    for text in [
        "nanE",
        "nan+E",
        "nan-E",
        "nanE12",
        "nan(payload)E",
        "nan()D",
        "infE",
        "infinityE",
        "-infE",
        "nanD",
        "infD",
        "1e9999E",
        "1e9999+E",
        "1D9999E",
    ] {
        assert_eq!(
            process_source_number_outcome(text),
            SourceNumberOutcome::NonFinite,
            "{text}"
        );
        assert!(process_source_number(text).is_err(), "{text}");
    }
    for text in ["1e-9999", "-1e-9999", "1e9999"] {
        assert_eq!(
            source_finite_number_prefix(text),
            Err("decimal magnitude outside the finite list-read domain"),
            "{text}"
        );
    }
    assert_eq!(
        source_finite_number_prefix("nan(payload)E"),
        Err("non-finite mandatory number")
    );
    assert_eq!(source_finite_number_prefix("1D-9999"), Ok(Some((1.0, 1))));
    // Real General must stop unsupported non-finite->int before writing any
    // MonthDay or fabricated error flag/zero. Its actual diagnostic is unreached.
    // A successful finite value is truncated BEFORE the defined-i32 test,
    // even when its year token is unused by this actual General caller.
    {
        use crate::weather::raw::{SourceDateScratch, process_source_date_string_with_diagnostic};
        let mut date = SourceDateScratch::default();
        let mut failed = false;
        let mut diagnostics = Vec::new();
        process_source_date_string_with_diagnostic(
            "2147483647.9/12/31",
            &mut date,
            false,
            &mut failed,
            &mut |message| {
                diagnostics.push(message.to_owned());
                Ok(())
            },
        )?;
        assert_eq!((date.month, date.day), (Some(12), Some(31)));
        assert!(!failed && diagnostics.is_empty());
    }
    for text in [
        "NANE",
        "INFE",
        "1E9999E",
        "2013/NANE/31",
        "2147483648/13/31",
        "2013/13/2147483648",
    ] {
        use crate::weather::raw::{SourceDateScratch, process_source_date_string_with_diagnostic};
        let mut date = SourceDateScratch::default();
        let mut failed = false;
        let mut diagnostics = Vec::new();
        assert!(
            process_source_date_string_with_diagnostic(
                text,
                &mut date,
                false,
                &mut failed,
                &mut |message| {
                    diagnostics.push(message.to_owned());
                    Ok(())
                }
            )
            .is_err(),
            "{text}"
        );
        assert!(!failed, "{text}");
        assert!(date.month.is_none() && date.day.is_none(), "{text}");
        assert!(diagnostics.is_empty(), "{text}");
    }
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
    // Equivalent integral HH/MM spellings feed the same real minute producer.
    // Blank HH and MM are existing operand substrings, not absent directives.
    for untils in [
        ["Until::20", "Until:1D0:15.0", "Until:24:"],
        ["Until :20", "Until1E:15+E0", "Until24.0:0D"],
        ["Until:-0:2E1", "Until:1.0:1.5D1", "Until:2.4E+1:0E"],
        ["Until:1e-9999:20", "Until:1:15", "Until:24:-1D-9999"],
        ["Until:-1D-9999:20", "Until:1:15", "Until:24:1e-9999"],
    ] {
        let mut raw = authored_raw(fixture, "Average", false);
        for (at, text) in [3, 5, 7].into_iter().zip(untils) {
            raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][at]["field"] =
                serde_json::json!(text);
        }
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual integral Until schedule")?;
        let day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual integral Until Day owner")?;
        assert_eq!(day.timestep_values.len(), 96);
        for step in 0..96 {
            let expected = match step {
                0 => 10.0_f64,
                1 => 120.0,
                _ => 175.0,
            };
            assert_eq!(day.timestep_values[step].to_bits(), expected.to_bits());
        }
        for step in 0..96 {
            let expected = match step {
                0 => 10.0_f64,
                1 => 120.0,
                _ => 175.0,
            };
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits()
            );
        }
    }
    // Keep the real nonIntegral Warning/Continue before the unsupported native
    // string-floating-format operation; no successful population/fatal is made up.
    for (until, label) in [("Until:0.5:20", "HH"), ("Until:00:20.5", "MM")] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][3]["field"] =
            serde_json::json!(until);
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
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (1, 1)
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        let text = std::str::from_utf8(&errors)?;
        let warning = text
            .find(&format!("non-integer numeric in {label}"))
            .ok_or("real nonIntegral warning")?;
        let continuation = text
            .find("Other errors may result.")
            .ok_or("real nonIntegral continuation")?;
        assert!(warning < continuation);
        assert!(!text.contains("Until value to be used"));
        let day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual pre-population Day owner")?;
        assert!(
            day.timestep_values
                .iter()
                .all(|value| value.to_bits() == 0.0_f64.to_bits())
        );
    }
    // Both empty events are actual authored intervals, not omitted fields.
    // The real minute child writes1440bins; the real manager writes96samples.
    for (mode, interpolation, source_mode) in [
        (
            "No",
            ep_model::ScheduleInterpolation::No,
            super::minute::SourceScheduleInterpolation::No,
        ),
        (
            "Average",
            ep_model::ScheduleInterpolation::Average,
            super::minute::SourceScheduleInterpolation::Average,
        ),
        (
            "Linear",
            ep_model::ScheduleInterpolation::Linear,
            super::minute::SourceScheduleInterpolation::Linear,
        ),
    ] {
        let untils = ["UNTIL:0:0", "UNTIL:03:00", "UNTIL:12:60", "UNTIL:24:00"].map(str::to_owned);
        let values = [2.0_f64, 0.0, 2.0, 1.0];
        let (mut minutes, mut assigned, mut found_errors) = ([0.0; 1440], [false; 1440], false);
        let mut diagnostics = Vec::new();
        super::minute::process_interval_fields_with(
            &untils,
            &values,
            4,
            4,
            &mut minutes,
            &mut assigned,
            &mut found_errors,
            "ACTUAL EMPTY INTERVALS",
            "Schedule:Compact",
            source_mode,
            15,
            &mut |kind, message| {
                diagnostics.push((kind, message.to_owned()));
                Ok(())
            },
        )?;
        assert!(!found_errors);
        assert!(assigned.iter().all(|value| *value));
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics.iter().all(|(kind, message)| *kind
            == super::minute::SourceScheduleMinuteDiagnostic::Warning
            && message.contains("zero time interval detected")));
        if mode != "Linear" {
            assert!(minutes[..180].iter().all(|value| *value == 0.0));
            assert!(minutes[180..].iter().all(|value| *value == 1.0));
        } else {
            assert!(minutes[0] > 1.0 && minutes[180] > 1.0);
        }
        let canonical = [0, 180, 180, 1440];
        let segments = canonical
            .into_iter()
            .zip(values)
            .map(
                |(until_minute_of_day, value)| ep_model::ScheduleCompactSegment {
                    until_minute_of_day,
                    value,
                },
            )
            .collect::<Vec<_>>();
        let (_, expected_steps) =
            ep_model::populate_schedule_day_values(interpolation, &segments, Some(15));
        let mut raw = authored_raw(fixture, mode, false);
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(2);
        let mut data = vec![
            serde_json::json!({"field":"Through:12/31"}),
            serde_json::json!({"field":"For:AllDays"}),
            serde_json::json!({"field":format!("Interpolate:{mode}")}),
        ];
        for (until, value) in untils.iter().zip(values) {
            data.extend([
                serde_json::json!({"field":until}),
                serde_json::json!({"field":value}),
            ]);
        }
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(data);
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
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup
            ),
            (2, 2)
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual empty interval graph")?;
        let day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual empty interval Day owner")?;
        assert_eq!(day.timestep_values.len(), 96);
        for (step, value) in day.timestep_values.iter().enumerate() {
            assert_eq!(
                value.to_bits(),
                expected_steps[step].to_bits(),
                "{mode} step{step}"
            );
            let raw_expected = if mode == "Average" {
                minutes[step * 15..(step + 1) * 15].iter().sum::<f64>() / 15.0
            } else {
                minutes[(step + 1) * 15 - 1]
            };
            assert_eq!(value.to_bits(), raw_expected.to_bits());
        }
        let maximum = manager.source_max(index)?;
        if mode == "Linear" {
            assert!(maximum > 1.0);
        } else {
            assert_eq!(maximum, 1.0);
            assert_eq!(manager.source_min(index)?, 0.0);
        }
        for step in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected_steps[step as usize].to_bits()
            );
        }
    }
    // Actual IP distinguishes a present empty item from a missing whole entry.
    // Read the real getter count/alpha/blank, then use the genuine minute graph
    // and live clock for every reported timestep; no profile is seeded here.
    for (entry, expected_alpha, expected) in [
        (serde_json::json!({"field":"1D0"}), "1D0", 1.0_f64),
        (serde_json::json!({"field":"-5D-1"}), "-5D-1", -0.5),
        (serde_json::json!({"field":"0E"}), "0E", 0.0),
        (serde_json::json!({"field":"1e-9999"}), "1E-9999", 0.0),
        (serde_json::json!({"field":"-1e-9999"}), "-1E-9999", -0.0),
        (serde_json::json!({"field":"1D-9999"}), "1D-9999", 0.0),
        (serde_json::json!({"field":"-1D-9999"}), "-1D-9999", -0.0),
        (serde_json::json!({"field":""}), "", 0.0),
        (serde_json::json!({"field":"   "}), "   ", 0.0),
        (serde_json::json!({}), "", 0.0),
        (serde_json::json!({"ignored":{"field":null}}), "", 0.0),
    ] {
        let mut raw = authored_raw(fixture, "No", false);
        raw["ScheduleTypeLimits"]["Fraction"]["lower_limit_value"] = serde_json::json!(-1);
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(1);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = serde_json::json!([
            {"field":"Through:12/31"},{"field":"For:AllDays"},
            {"field":"Until:24:00"},entry,
        ]);
        let mut input = processor(fixture, &raw)?;
        let ordinal = input
            .source_raw_object_names("Schedule:Compact")?
            .iter()
            .position(|name| name.eq_ignore_ascii_case("INTERNAL GAIN SCHEDULE"))
            .ok_or("actual Compact object ordinal")?;
        let args = input.get_object_def_max_args("Schedule:Compact")?;
        let mut locals: super::SourceScheduleGetterLocals = (
            vec![String::new(); args.alphas + 1],
            vec![String::new(); args.alphas + 1],
            vec![String::new(); args.numerics],
            vec![0.0; args.numerics],
            vec![true; args.alphas + 1],
            vec![true; args.numerics],
        );
        let count = SelectedScheduleManager::get_source_compact_object_item(
            &mut input,
            ordinal,
            &mut locals,
            true,
        )?;
        assert_eq!(count, 6); // Name + Type + four real extensible items.
        assert_eq!(locals.0[5], expected_alpha);
        assert_eq!(locals.4[5], expected_alpha.is_empty());
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual scalar schedule")?;
        let actual_day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == "INTERNAL GAIN SCHEDULE_dy_1")
            .ok_or("actual scalar Day owner")?;
        assert_eq!(actual_day.timestep_values.len(), 96);
        assert!(
            actual_day
                .timestep_values
                .iter()
                .all(|value| value.to_bits() == expected.to_bits())
        );
        assert_eq!(manager.source_min(index)?.to_bits(), expected.to_bits());
        assert_eq!(manager.source_max(index)?.to_bits(), expected.to_bits());
        for step in 0..96 {
            let clock = WeatherGlobalState {
                hour_of_day: step / 4 + 1,
                time_step: step % 4 + 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits()
            );
        }
    }
    // These stop at existing unavailable/schema boundaries; no Native severe
    // text, callback or counter is fabricated by the backend-free caller.
    for entry in [
        Some(serde_json::json!({"field":null})),
        Some(serde_json::json!(null)),
        Some(serde_json::json!({"field":"junk"})),
        Some(serde_json::json!({"field":"+1"})),
        Some(serde_json::json!({"field":"nan"})),
        Some(serde_json::json!({"field":"nanE"})),
        Some(serde_json::json!({"field":"1e9999E"})),
        Some(serde_json::json!({"field":"\u{00a0}1"})),
        Some(serde_json::json!({"field":"Through:12/31"})),
        None,
    ] {
        let mut raw = authored_raw(fixture, "No", false);
        let mut data = vec![
            serde_json::json!({"field":"Through:12/31"}),
            serde_json::json!({"field":"For:AllDays"}),
            serde_json::json!({"field":"Until:24:00"}),
        ];
        if let Some(entry) = entry {
            data.push(entry);
        }
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(data);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        assert!(
            manager
                .process_schedule_input(&mut input, &declarations, 4, &mut MemoryAudit::default())
                .is_err()
        );
    }
    // Compiler-accepted General MonthDay endpoints reach the actual child,
    // including noncanonical plain3 folds. Genuine Day/Week owners are retained.
    for (through, ordinal) in [
        ("Through: DECEMBER 31", 366_i32),
        ("Through: 31 DEC", 366),
        ("Through: 12-31", 366),
        ("Through: 12 31", 366),
        ("Through 2013/12/31", 366),
        ("Through: 12/31/2013", 366),
        ("Through2013-12-31", 366),
        ("Through 2013:12:31", 366),
        ("Through 2013.5/12.9/31.9", 366),
        ("Through 2.013D+3/1.2E+1/3.1D1", 366),
        ("Through 12.9/31.9/2013.5", 366),
        ("Through -2013.5/-12/-31", 366),
        ("Through 2013.5/2.9/29.9", 60),
        ("Through 2013D-1/12", 12),
        ("Through 12.9/31.9", 366),
        ("Through: DEC 31.9", 366),
        ("Through 31.9 DECADE", 366),
        ("Through DECOR 31.9", 366),
        ("Through 31ST DEC", 366),
        ("Through 31.9 OF DECEMBER", 366),
        ("Through DEC IN 31.9", 366),
        ("Through 31STST DEC", 366),
        ("Through 2.9/29.9", 60),
        ("Through 29TH FEBRUARY", 60),
        ("Through12D0:31E0", 366),
        ("Through 12E/31D", 366),
        ("Through -12.9/-31.9", 366),
        ("Through :12.9/31.9", 366),
        ("Through12+E-1", 336),
        ("Through 2013/2/30", 61),
        ("Through 2013/2/BAD", 31),
        ("Through 2013/3/0", 60),
        ("Through 2013/1/366", 366),
        ("Through 2013/2/335", 366),
        ("Through 2013/11/61", 366),
    ] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][0]["field"] =
            serde_json::json!(through);
        // Actual Raw Source getters already accept these no-colon spellings;
        // exercise the real minute producer, not a typed preview value.
        for (at, text) in [
            (1, "For AllDays"),
            (2, "Interpolate Average"),
            (3, "Until 00:20"),
            (5, "Until 01:15"),
            (7, "Until 24:00"),
        ] {
            raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][at]["field"] =
                serde_json::json!(text);
        }
        if ordinal < 366 {
            let data = raw["Schedule:Compact"]["Internal Gain Schedule"]["data"]
                .as_array_mut()
                .ok_or("actual authored fixed-date fields")?;
            data.extend([
                serde_json::json!({"field":"Through:12/31"}),
                serde_json::json!({"field":"For:AllDays"}),
                serde_json::json!({"field":"Until:24:00"}),
                serde_json::json!({"field":5}),
            ]);
        }
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual named-month Compact pointer")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual named-month Detailed owner".into());
        };
        let week = detail.week_schedules[1].ok_or("actual first-day Week owner")?;
        let endpoint = usize::try_from(ordinal)?;
        assert!(
            detail.week_schedules[1..=endpoint]
                .iter()
                .all(|slot| *slot == Some(week)),
            "{through}"
        );
        if endpoint < 366 {
            let next_week =
                detail.week_schedules[endpoint + 1].ok_or("actual next-period Week owner")?;
            assert_ne!(next_week, week);
            assert!(
                detail.week_schedules[endpoint + 1..]
                    .iter()
                    .all(|slot| *slot == Some(next_week))
            );
        }
        assert!(
            manager.week_schedules[week].day_schedules[1..]
                .iter()
                .all(Option::is_some)
        );
        for (time_step, expected) in [(1, 10.0_f64), (2, 120.0_f64)] {
            let clock = WeatherGlobalState {
                hour_of_day: 1,
                time_step,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: ordinal,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits(),
                "{through}"
            );
        }
        if ordinal < 366 {
            let clock = WeatherGlobalState {
                hour_of_day: 1,
                time_step: 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            let environment = WeatherEnvironmentState {
                day_of_year_schedule: ordinal + 1,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                5.0_f64.to_bits()
            );
        }
    }
    // The real Through caller stops defined-int/checked-ordinal/array failures
    // before any out-of-range schedule pointer. No Native diagnostic is invented.
    for through in [
        "2013/13/2147483648",
        "2013/2147483648/31",
        "2013/2/2147483647",
        "2013/2/336",
        "2013/13/NANE",
        "2013ST/12/31",
    ] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"][0]["field"] =
            serde_json::json!(format!("Through:{through}"));
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        assert!(
            manager
                .process_schedule_input(&mut input, &declarations, 4, &mut MemoryAudit::default())
                .is_err(),
            "{through}"
        );
    }
    // Source already parses every authored empty period into real Week/Day
    // owners. Only the actual annual pointer graph contributes to min/max.
    for (periods, inactive_number, expected_min, expected_max) in [
        (
            vec![("6/30", 0.25_f64), ("6/30", 2.0), ("12/31", 0.75)],
            2,
            0.25,
            0.75,
        ),
        (vec![("12/31", 0.25_f64), ("6/30", 2.0)], 2, 0.25, 0.25),
    ]
    .into_iter()
    .chain(
        [
            "0",
            "0.0",
            "-0",
            ".5",
            "-5D-1",
            "0E",
            "0D",
            "",
            "   ",
            "1e-9999",
            "-1e-9999",
            "1D-9999",
            "-1D-9999",
            "2013/13/31",
            "13/31/2013",
            "BAD/2/2013",
            "BAD/BAD/BAD",
            "12/31/100",
            "+2013/12/31",
            "2013/+12/31",
            "1e9999/12/31",
            "2013/13/2147483647.9",
            "2013/nan/31",
        ]
        .into_iter()
        .map(|zero| (vec![(zero, 2.0), ("12/31", 0.75)], 1, 0.75, 0.75)),
    ) {
        let mut raw = authored_raw(fixture, "No", false);
        raw["ScheduleTypeLimits"]["Fraction"]["upper_limit_value"] = serde_json::json!(1);
        let mut data = Vec::new();
        for (through, value) in periods {
            data.extend([
                serde_json::json!({"field":format!("Through: {through}")}),
                serde_json::json!({"field":"For: AllDays"}),
                serde_json::json!({"field":"Until: 24:00"}),
                serde_json::json!({"field":value}),
            ]);
        }
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = Value::Array(data);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        manager.process_schedule_input(
            &mut input,
            &declarations,
            4,
            &mut MemoryAudit::default(),
        )?;
        let index = manager
            .get_source_schedule("INTERNAL GAIN SCHEDULE")?
            .ok_or("actual empty-period Detailed pointer")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual empty-period Detailed owner".into());
        };
        let inactive_week = manager
            .week_schedules
            .iter()
            .find(|week| week.name == format!("INTERNAL GAIN SCHEDULE_wk_{inactive_number}"))
            .ok_or("actual inactive Week owner")?;
        assert!(inactive_week.is_used);
        assert!(!inactive_week.is_min_max_set);
        assert!(
            detail.week_schedules[1..]
                .iter()
                .all(|slot| slot.is_some() && *slot != Some(inactive_week.number))
        );
        let inactive_day = manager
            .day_schedules
            .iter()
            .find(|day| day.name == format!("INTERNAL GAIN SCHEDULE_dy_{inactive_number}"))
            .ok_or("actual inactive Day owner")?;
        assert!(inactive_day.is_used);
        assert!(!inactive_day.is_min_max_set);
        assert_eq!(inactive_day.timestep_values.len(), 96);
        assert!(
            inactive_day
                .timestep_values
                .iter()
                .all(|value| *value == 2.0)
        );
        assert_eq!(manager.source_min(index)?, expected_min);
        assert_eq!(manager.source_max(index)?, expected_max);
        let clock = WeatherGlobalState {
            hour_of_day: 1,
            time_step: 1,
            time_steps_in_hour: 4,
            ..Default::default()
        };
        for (day_of_year_schedule, expected) in [(1, expected_min), (183, expected_max)] {
            let environment = WeatherEnvironmentState {
                day_of_year_schedule,
                day_of_week: 3,
                ..Default::default()
            };
            manager.update_source_schedule_values(&clock, &environment)?;
            assert_eq!(
                manager.source_current_schedule_value(index)?.to_bits(),
                expected.to_bits()
            );
        }
    }
    // Blank, unresolved and named-unlimited types leave this Detailed
    // schedule uncached. Its real downstream getter must use the live clock,
    // not currentVal, an inferred minutes clock, or an eager input reduction.
    for type_case in ["missing", "blank", "unknown", "unlimited"] {
        let mut raw = authored_raw(fixture, "Average", false);
        let gain = raw["Schedule:Compact"]["Internal Gain Schedule"]
            .as_object_mut()
            .ok_or("actual Compact fields")?;
        match type_case {
            "missing" => {
                gain.remove("schedule_type_limits_name");
            }
            "blank" => {
                gain.insert("schedule_type_limits_name".into(), serde_json::json!(""));
            }
            "unknown" => {
                gain.insert(
                    "schedule_type_limits_name".into(),
                    serde_json::json!("UNKNOWN LIMIT"),
                );
            }
            "unlimited" => {
                let limits = raw["ScheduleTypeLimits"]["Fraction"]
                    .as_object_mut()
                    .ok_or("actual type fields")?;
                limits.remove("lower_limit_value");
                limits.remove("upper_limit_value");
            }
            _ => return Err("unknown authored type case".into()),
        }
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
            .ok_or("actual Compact pointer")?;
        assert_eq!(
            manager.schedules[index.0].schedule_type_number.is_some(),
            type_case == "unlimited"
        );
        assert!(!manager.schedules[index.0].is_min_max_set);
        assert_eq!(manager.source_current_schedule_value(index)?, 0.0);
        let actual_clock = WeatherGlobalState {
            time_steps_in_hour: 4,
            ..Default::default()
        };
        assert!(manager.source_has_value(index, 10.0, &actual_clock)?);
        assert!(!manager.schedules[index.0].is_min_max_set);
        assert!(manager.source_min(index).is_err()); // legacy owner still missing
        assert!(!manager.schedules[index.0].is_min_max_set);
        assert_eq!(manager.source_min_with_clock(index, &actual_clock)?, 10.0);
        assert_eq!(manager.source_max_with_clock(index, &actual_clock)?, 175.0);
        assert!(manager.source_check_min_with_clock(index, 0.0, &actual_clock)?);
        assert!(!manager.source_check_min_max_inclusive(index, 0.0, 4.0, &actual_clock)?);
        for group in 0..4 {
            assert_eq!(
                manager.source_day_group_min_max_with_clock(index, group, &actual_clock)?,
                (10.0, 175.0)
            );
        }
        let expected_warning = i32::from(type_case != "unlimited");
        assert_eq!(state.tracking.total_warning_errors, expected_warning);
        assert_eq!(
            state.tracking.total_warning_errors_during_warmup,
            expected_warning
        );
        assert_eq!(state.tracking.total_severe_errors, 0);
        assert!(!state.warmup_flag);
        let text = std::str::from_utf8(&errors)?;
        if expected_warning == 1 {
            let header = text
                .find("ProcessScheduleInput: Schedule:Compact = INTERNAL GAIN SCHEDULE")
                .ok_or("actual warning header")?;
            let field = text
                .find(if type_case == "unknown" {
                    "Schedule Type Limits Name = UNKNOWN LIMIT, item not found."
                } else {
                    "Schedule Type Limits Name is empty."
                })
                .ok_or("actual field title")?;
            let last = text
                .find("Schedule will not be validated.")
                .ok_or("actual final Continue")?;
            assert!(header < field && field < last);
        } else {
            assert!(text.is_empty());
        }
    }
    // Incomplete but valid masks keep the genuine MissingDaySchedule0
    // pointers. A failed final Continue retains the already populated first
    // Through period and prevents creation of the later period.
    struct MissingDaySink {
        bytes: Vec<u8>,
        fail_final_continue: bool,
    }
    impl io::Write for MissingDaySink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_final_continue
                && String::from_utf8_lossy(bytes)
                    .contains("Missing day types will have 0.0 as Schedule Values")
            {
                return Err(io::Error::other("actual missing-day final Continue sink"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    for fail_final_continue in [false, true] {
        let mut raw = authored_raw(fixture, "Average", false);
        raw["Schedule:Compact"]["Internal Gain Schedule"]["data"] = serde_json::json!([
            {"field":"Through: 6/30"}, {"field":"For: Tuesday"},
            {"field":"Interpolate: Average"}, {"field":"Until: 24:00"}, {"field":2},
            {"field":"Through: 12/31"}, {"field":"For: AllDays"},
            {"field":"Interpolate: Average"}, {"field":"Until: 24:00"}, {"field":3}
        ]);
        let mut input = processor(fixture, &raw)?;
        let declarations = schedule_declarations(fixture, &raw)?;
        let mut manager = SelectedScheduleManager::default();
        manager.init_constant_state(0)?;
        let (mut state, mut errors, mut console, mut audit) = (
            interpolation_error_state(),
            MissingDaySink {
                bytes: Vec::new(),
                fail_final_continue,
            },
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
        if fail_final_continue {
            assert!(result.is_err());
        } else {
            result?;
        }
        assert_eq!(
            (
                state.tracking.total_warning_errors,
                state.tracking.total_warning_errors_during_warmup,
                state.tracking.total_severe_errors
            ),
            (1, 1, 0)
        );
        assert!(!state.warmup_flag); // Actual borrowed warmup=true was read.
        let index = *manager
            .schedule_map
            .get("INTERNAL GAIN SCHEDULE")
            .ok_or("actual incomplete-mask Detailed pointer")?;
        let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
            return Err("actual incomplete-mask Detailed owner".into());
        };
        let first_week = detail.week_schedules[1].ok_or("actual first Through Week")?;
        assert!(
            detail.week_schedules[1..=182]
                .iter()
                .all(|slot| *slot == Some(first_week))
        );
        let week = &manager.week_schedules[first_week];
        let tuesday = week.day_schedules[3].ok_or("actual Tuesday Day")?;
        assert_ne!(tuesday, 0);
        for day_type in 1..13 {
            assert_eq!(
                week.day_schedules[day_type],
                Some(if day_type == 3 { tuesday } else { 0 })
            );
        }
        assert_eq!(manager.day_schedules[0].name, "MissingDaySchedule-0.0");
        assert_eq!(manager.day_schedules[0].timestep_values.len(), 96);
        assert!(
            manager.day_schedules[0]
                .timestep_values
                .iter()
                .all(|v| v.to_bits() == 0.0_f64.to_bits())
        );
        assert!(
            manager.day_schedules[tuesday]
                .timestep_values
                .iter()
                .all(|v| *v == 2.0)
        );
        let diagnostic_rows = std::str::from_utf8(&errors.bytes)?
            .lines()
            .filter(|line| line.contains("**"))
            .collect::<Vec<_>>();
        let expected = [
            "   ** Warning ** ProcessScheduleInput: Schedule:Compact = INTERNAL GAIN SCHEDULE",
            "   **   ~~~   ** has missing day types in Through=6/30",
            "   **   ~~~   ** Last \"For\" field=FOR: TUESDAY",
            "   **   ~~~   ** Missing day types= \"Sunday\", \"Monday\", \"Wednesday\", \"Thursday\", \"Friday\", \"Saturday\", \"Holiday\", \"SummerDesignDay\", \"WinterDesignDay\", \"CustomDay1\", \"CustomDay2\"",
            "   **   ~~~   ** Missing day types will have 0.0 as Schedule Values",
        ];
        assert_eq!(
            diagnostic_rows,
            expected[..if fail_final_continue { 4 } else { 5 }]
        );
        if fail_final_continue {
            assert!(detail.week_schedules[183..].iter().all(Option::is_none));
            assert!(
                !std::str::from_utf8(&audit.bytes)?
                    .contains("Processing Schedule Input -- Complete")
            );
        } else {
            let second =
                detail.week_schedules[183].ok_or("actual continued second Through Week")?;
            assert_ne!(second, first_week);
            assert!(
                detail.week_schedules[183..]
                    .iter()
                    .all(|slot| *slot == Some(second))
            );
            let clock = WeatherGlobalState {
                hour_of_day: 1,
                time_step: 1,
                time_steps_in_hour: 4,
                ..Default::default()
            };
            for (day_of_year_schedule, day_of_week, holiday_index, expected) in [
                (1, 3, 0, 2.0_f64),
                (1, 1, 0, 0.0),
                (1, 3, 8, 0.0),
                (1, 3, 12, 0.0),
                (183, 1, 0, 3.0),
            ] {
                let environment = WeatherEnvironmentState {
                    day_of_year_schedule,
                    day_of_week,
                    holiday_index,
                    ..Default::default()
                };
                manager.update_source_schedule_values(&clock, &environment)?;
                assert_eq!(
                    manager.source_current_schedule_value(index)?.to_bits(),
                    expected.to_bits()
                );
            }
            assert!(
                std::str::from_utf8(&audit.bytes)?
                    .ends_with("  Processing Schedule Input -- Complete\n")
            );
        }
    }
    // Retain AddDetailed/type and the successful Warning/first Continue when
    // the final Continue sink fails; week/day population has not been reached.
    struct FailTypeContinue {
        bytes: Vec<u8>,
    }
    impl io::Write for FailTypeContinue {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if String::from_utf8_lossy(bytes).contains("Schedule will not be validated.") {
                return Err(io::Error::other("actual Compact type Continue sink"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut raw = authored_raw(fixture, "Average", false);
    raw["Schedule:Compact"]["Internal Gain Schedule"]
        .as_object_mut()
        .ok_or("actual Compact fields")?
        .remove("schedule_type_limits_name");
    let mut input = processor(fixture, &raw)?;
    let declarations = schedule_declarations(fixture, &raw)?;
    let mut manager = SelectedScheduleManager::default();
    manager.init_constant_state(0)?;
    let (mut state, mut errors, mut console, mut audit) = (
        interpolation_error_state(),
        FailTypeContinue { bytes: Vec::new() },
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
    assert_eq!(state.tracking.total_warning_errors, 1);
    assert_eq!(state.tracking.total_warning_errors_during_warmup, 1);
    let index = manager
        .schedule_map
        .get("INTERNAL GAIN SCHEDULE")
        .ok_or("retained Detailed name")?;
    let super::SelectedScheduleData::Detailed(detail) = &manager.schedules[index.0].data else {
        return Err("retained Detailed dynamic type".into());
    };
    assert_eq!(detail.schedule_kind, 1);
    assert!(manager.schedules[index.0].schedule_type_number.is_none());
    assert!(manager.week_schedules.is_empty());
    assert_eq!(manager.day_schedules.len(), 1); // genuine built-in MissingDay only
    let text = std::str::from_utf8(&errors.bytes)?;
    assert!(text.contains("Schedule Type Limits Name is empty."));
    assert!(!text.contains("Schedule will not be validated."));
    assert!(!std::str::from_utf8(&audit.bytes)?.contains("Processing Schedule Input -- Complete"));

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
