use super::*;
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorState, SourceErrorTracking,
};
use crate::weather::raw::{AllocatedValues, DateType, DstPeriod, SpecialDay};
use std::io::{self, Write};
// Authored unit live-owner operands deliberately differ from backend ctor phase.
fn phases() -> SourceDiagnosticFlags<'static> {
    SourceDiagnosticFlags {
        kickoff: &false,
        developer: &false,
        warmup: &true,
        doing_sizing: &false,
    }
}
fn state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: false,
        version_string: "authored version".into(),
        idd_version_string: "authored IDD".into(),
        doing_input_processing: false,
        print_console_output: false,
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
            last_severe_error: "prior severe".into(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

fn backend<'a>(
    state: &'a mut SourceErrorState,
    error: &'a mut dyn Write,
    console: &'a mut dyn Write,
) -> SourceErrorBackend<'a> {
    SourceErrorBackend {
        state,
        error_stream: Some(error),
        console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    }
}

#[test]
fn source_dst_fixed_nth_last_wrap_and_definedness_tables() {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let mut backend = backend(&mut state, &mut error, &mut console);
    let weekdays = [3, 6, 6, 2, 4, 7, 2, 5, 1, 3, 6, 1]; // Authored test operands, not constructors.
    let ends = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (period, expected, count) in [
        (
            DstPeriod {
                start_date_type: DateType::MonthDay,
                start_month: 3,
                start_day: 1,
                start_weekday: None,
                end_date_type: DateType::MonthDay,
                end_month: 3,
                end_day: 1,
                end_weekday: None,
            },
            (3, 1, 3, 1),
            1,
        ),
        (
            DstPeriod {
                start_date_type: DateType::MonthDay,
                start_month: 11,
                start_day: 1,
                start_weekday: None,
                end_date_type: DateType::MonthDay,
                end_month: 2,
                end_day: 1,
                end_weekday: None,
            },
            (11, 1, 2, 1),
            94,
        ),
        (
            DstPeriod {
                start_date_type: DateType::NthDayInMonth,
                start_month: 3,
                start_day: 2,
                start_weekday: Some(1),
                end_date_type: DateType::LastDayInMonth,
                end_month: 3,
                end_day: 0,
                end_weekday: Some(1),
            },
            (3, 10, 3, 31),
            22,
        ),
        // Literal Original end-Nth shift permits February32 instead of rejecting
        // it through the older preview's nonexistent-fifth-weekday predicate.
        (
            DstPeriod {
                start_date_type: DateType::MonthDay,
                start_month: 2,
                start_day: 1,
                start_weekday: None,
                end_date_type: DateType::NthDayInMonth,
                end_month: 2,
                end_day: 5,
                end_weekday: Some(2),
            },
            (2, 1, 2, 32),
            32,
        ),
    ] {
        let header = RawEpwHeaderState {
            dst: period,
            ..Default::default()
        };
        let mut index = [9; 366];
        assert_eq!(
            set_dst_dates(
                &weekdays,
                &ends,
                &mut index,
                &header,
                &mut backend,
                &phases()
            )
            .unwrap(),
            expected
        );
        assert_eq!(index.iter().filter(|&&v| v == 1).count(), count);
        assert!(index.iter().all(|&v| v == 0 || v == 1));
    }
    let header = RawEpwHeaderState {
        dst: DstPeriod {
            start_date_type: DateType::NthDayInMonth,
            start_month: 3,
            start_day: 1,
            start_weekday: None,
            ..Default::default()
        },
        ..Default::default()
    };
    let mut index = [9; 366];
    assert!(
        set_dst_dates(
            &weekdays,
            &ends,
            &mut index,
            &header,
            &mut backend,
            &phases()
        )
        .is_err()
    );
    assert_eq!(index, [9; 366]);
}

fn special(
    month: i32,
    day: i32,
    kind: DateType,
    weekday: i32,
    duration: i32,
    day_type: i32,
) -> SpecialDay {
    SpecialDay {
        name: format!("D{month}-{day}"),
        month,
        day,
        date_type: kind,
        weekday,
        duration,
        day_type,
        ..Default::default()
    }
}
#[test]
fn source_special_date_tables_and_overlap_failure_keep_actual_prefix() {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let weekdays = [3, 6, 6, 2, 4, 7, 2, 5, 1, 3, 6, 1];
    let ends = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (day, leap, weekend, expected, filled) in [
        (
            special(1, 1, DateType::MonthDay, 0, 1, 8),
            0,
            true,
            (1, 2),
            vec![2],
        ),
        (
            special(12, 31, DateType::MonthDay, 0, 1, 8),
            0,
            true,
            (1, 2),
            vec![2],
        ),
        (
            special(12, 31, DateType::MonthDay, 0, 2, 12),
            0,
            false,
            (12, 31),
            vec![365, 1],
        ),
        (
            special(2, 28, DateType::MonthDay, 0, 1, 8),
            1,
            false,
            (2, 27),
            vec![59],
        ),
        (
            special(3, 2, DateType::NthDayInMonth, 1, 1, 11),
            0,
            false,
            (3, 10),
            vec![69],
        ),
        (
            special(3, 0, DateType::LastDayInMonth, 1, 1, 9),
            0,
            false,
            (3, 31),
            vec![90],
        ),
    ] {
        let mut header = RawEpwHeaderState {
            leap_year_add: leap,
            number_special_days: 1,
            special_days: AllocatedValues {
                allocated: true,
                values: vec![day],
            },
            ..Default::default()
        };
        let mut week = [2; 366];
        week[0] = 1;
        week[364] = 7;
        let mut types = [99; 366];
        set_special_days(
            &weekdays,
            &ends,
            &week,
            &mut types,
            &true,
            &weekend,
            &mut header,
            &mut backend(&mut state, &mut error, &mut console),
            &phases(),
        )
        .unwrap();
        assert_eq!(
            (
                header.special_days.values[0].actual_start_month,
                header.special_days.values[0].actual_start_day
            ),
            expected
        );
        let actual = types
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v != 0).then_some((i + 1) as i32))
            .collect::<Vec<_>>();
        let mut sorted = filled;
        sorted.sort_unstable();
        assert_eq!(actual, sorted);
    }
    struct FailContinue {
        bytes: Vec<u8>,
    }
    impl Write for FailContinue {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if String::from_utf8_lossy(buf).contains("...This could be caused") {
                return Err(io::Error::other("authored continue write failure"));
            }
            self.bytes.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut header = RawEpwHeaderState {
        number_special_days: 2,
        special_days: AllocatedValues {
            allocated: true,
            values: vec![
                special(1, 1, DateType::MonthDay, 0, 1, 8),
                special(1, 1, DateType::MonthDay, 0, 1, 12),
            ],
        },
        ..Default::default()
    };
    let mut types = [99; 366];
    let mut failed = FailContinue { bytes: Vec::new() };
    assert!(
        set_special_days(
            &weekdays,
            &ends,
            &[2; 366],
            &mut types,
            &true,
            &false,
            &mut header,
            &mut backend(&mut state, &mut failed, &mut console),
            &phases(),
        )
        .is_err()
    );
    assert_eq!(state.tracking.total_warning_errors, 1);
    assert_eq!(state.tracking.total_warning_errors_during_warmup, 1);
    assert_eq!(types[0], 8);
    assert_eq!(types[1..], [0; 365]);
    assert!(
        String::from_utf8(failed.bytes)
            .unwrap()
            .contains("overwriting previously entered special day period")
    );
    assert_eq!(header.special_days.values[1].actual_start_day, 1);
}

#[test]
fn source_calendar_nth_errors_are_deferred_and_preserve_real_writes() {
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let weekdays = [3, 6, 6, 2, 4, 7, 2, 5, 1, 3, 6, 1];
    let ends = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut header = RawEpwHeaderState {
        number_special_days: 2,
        special_days: AllocatedValues {
            allocated: true,
            values: vec![
                special(2, 5, DateType::NthDayInMonth, 2, 1, 8),
                special(1, 1, DateType::MonthDay, 0, 1, 12),
            ],
        },
        ..Default::default()
    };
    let mut types = [99; 366];
    assert!(
        set_special_days(
            &weekdays,
            &ends,
            &[2; 366],
            &mut types,
            &true,
            &false,
            &mut header,
            &mut backend(&mut state, &mut error, &mut console),
            &phases(),
        )
        .is_err()
    );
    assert_eq!(types[0], 12);
    assert_eq!(types[1..], [0; 365]);
    assert_eq!(state.tracking.total_severe_errors, 1);
    assert_eq!(state.tracking.total_severe_errors_during_warmup, 1);
    assert!(
        String::from_utf8(error)
            .unwrap()
            .contains("Program terminates due to preceding condition(s).")
    );
}

#[test]
fn source_calendar_input_getters_keep_raw_fraction_and_retained_dst_refs() {
    use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput;
    use crate::heat_balance::selected_input_processor::{
        SelectedInputProcessor, SourceInputFamily, SourceInputProcessorShortCut,
        SourceObjectDefinition, SourceObjectShape,
    };
    use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
    use crate::weather::raw::{HeaderCallerPreparation, RawEpwCursorOwner};
    use std::collections::BTreeMap;
    let mut raw = SelectedHeatBalanceInput {
        source_is_epjson: true,
        ..Default::default()
    };
    let rows: [(&str, &str, BTreeMap<String, ProjectInputValue>); 2] = [
        (
            "RunPeriodControl:SpecialDays",
            "Holiday",
            BTreeMap::from([
                ("start_date".into(), ProjectInputValue::String("1/1".into())),
                ("duration".into(), ProjectInputValue::Number(1.9)),
            ]),
        ),
        (
            "RunPeriodControl:DaylightSavingTime",
            " ",
            BTreeMap::from([
                ("start_date".into(), ProjectInputValue::String("1/1".into())),
                (
                    "end_date".into(),
                    ProjectInputValue::String("LAST SUNDAY IN OCTOBER".into()),
                ),
            ]),
        ),
    ];
    let mut families = BTreeMap::new();
    let mut definitions = Vec::new();
    for (family, name, fields) in rows {
        families.insert(
            family.into(),
            SourceInputFamily::Objects(vec![SourceObjectShape {
                object_name: name.into(),
                field_sizes: fields.keys().map(|k| (k.clone(), 1)).collect(),
            }]),
        );
        definitions.push(SourceObjectDefinition {
            object_type: family.into(),
            alpha_fields: if family.ends_with("SpecialDays") {
                3
            } else {
                2
            },
            numeric_fields: usize::from(family.ends_with("SpecialDays")),
            alpha_extensions: 0,
            numeric_extensions: 0,
            extension: None,
        });
        raw.objects.insert(
            family.into(),
            vec![ProjectInputObject {
                object_name: name.into(),
                fields,
                idf_max_fields: None,
            }],
        );
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families).unwrap();
    let mut shortcut = SourceInputProcessorShortCut::default();
    shortcut.allocate_from_input(&input).unwrap();
    let header = "LOCATION,Fixture,CO,USA,Test,000000,39.74,-105.18,-7,1600\nDESIGN CONDITIONS,0\nTYPICAL/EXTREME PERIODS,0\nGROUND TEMPERATURES,0\nHOLIDAYS/DAYLIGHT SAVINGS,No,3/12,11/5,0\nCOMMENTS 1,\nCOMMENTS 2,\nDATA PERIODS,1,1,Data,Sunday,1/1,12/31\n";
    let mut cursor = RawEpwCursorOwner::open_from_bytes(
        header.as_bytes().to_vec(),
        RawEpwHeaderState::default(),
        HeaderCallerPreparation {
            run_period_control_special_days: 1,
        },
    )
    .unwrap();
    assert_eq!(cursor.header().epw_dst.start_weekday, None);
    cursor.source_header_mut().idf_dst.start_weekday = Some(6);
    let mut state = state();
    let mut error = Vec::new();
    let mut console = Vec::new();
    let mut errors = false;
    let old_shared = shortcut.alpha_args.clone();
    cursor
        .get_source_special_day_period_data(
            &raw,
            &mut input,
            &mut shortcut,
            &mut backend(&mut state, &mut error, &mut console),
            &mut errors,
        )
        .unwrap();
    assert_eq!(shortcut.alpha_args, old_shared); // Native local/no-optional getter.
    assert_eq!(cursor.header().special_days.values[0].duration, 1);
    cursor
        .get_source_dst_data(
            &raw,
            &mut input,
            &mut shortcut,
            &mut backend(&mut state, &mut error, &mut console),
            &mut errors,
        )
        .unwrap();
    cursor.apply_source_idf_dst_override();
    assert!(!errors);
    assert!(cursor.header().idf_daylight_saving);
    assert_eq!(cursor.header().dst.start_weekday, Some(6));
    assert_eq!(cursor.header().dst.end_weekday, Some(1));
    assert!(!input.is_object_unused("RunPeriodControl:SpecialDays", "Holiday"));
    assert!(!input.is_object_unused("RunPeriodControl:DaylightSavingTime", " "));
}
