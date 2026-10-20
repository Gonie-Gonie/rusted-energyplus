use super::super::compile_raw_model;
use ep_raw_model::{FieldName, ObjectName, ObjectType, RawValue, parse_epjson_str};

#[test]
fn run_period_year_numbers_follow_original_truncation_and_zero_sentinel() {
    for (begin, end, expected_begin, expected_end) in [
        (None, None, None, None),
        (Some("0"), Some("0"), None, None),
        (Some("0.9"), Some("-0.9"), None, None),
        (Some("2013.9"), Some("2013.9"), Some(2013), Some(2013)),
        (Some("2013"), Some("0"), Some(2013), None),
        (Some("1583.99"), Some("1583"), Some(1583), Some(1583)),
    ] {
        let mut fields = vec![
            "\"begin_month\":1".to_owned(),
            "\"begin_day_of_month\":1".to_owned(),
            "\"end_month\":1".to_owned(),
            "\"end_day_of_month\":1".to_owned(),
        ];
        for (field, value) in [("begin_year", begin), ("end_year", end)] {
            if let Some(value) = value {
                fields.push(format!("\"{field}\":{value}"));
            }
        }
        let text = format!("{{\"RunPeriod\":{{\"p\":{{{}}}}}}}", fields.join(","));
        let raw = parse_epjson_str(&text).expect("raw numeric RunPeriod years");
        let compiled = compile_raw_model(&raw);
        assert!(
            !compiled.has_errors(),
            "{text}: {:?}",
            compiled.report.diagnostics
        );
        let model = compiled.model.expect("typed RunPeriod years");
        assert_eq!(model.run_periods[0].begin_year, expected_begin, "{text}");
        assert_eq!(model.run_periods[0].end_year, expected_end, "{text}");
        let object = &raw.objects[&ObjectType("RunPeriod".into())][&ObjectName("p".into())];
        for (field, value) in [("begin_year", begin), ("end_year", end)] {
            assert_eq!(
                object.fields.get(&FieldName(field.into())),
                value.map(|v| RawValue::Number(v.into())).as_ref(),
                "raw authored year must remain unchanged"
            );
        }
    }
}

#[test]
fn run_period_year_conversion_rejects_wrong_types_and_undefined_integer_casts() {
    for field in ["begin_year", "end_year"] {
        for value in [
            "-1",
            "-2013.5",
            "2147483648",
            "-2147483649",
            "\"2013\"",
            "\"\"",
            "null",
            "true",
        ] {
            let text = format!("{{\"RunPeriod\":{{\"p\":{{\"{field}\":{value}}}}}}}");
            let compiled =
                compile_raw_model(&parse_epjson_str(&text).expect("raw invalid RunPeriod year"));
            assert!(
                compiled.has_errors() && compiled.model.is_none(),
                "{field}: {value}"
            );
        }
    }
}
