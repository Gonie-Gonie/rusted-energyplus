use super::super::compile_raw_model;
use ep_model::Version;
use ep_raw_model::parse_epjson_str;

#[test]
fn all_schema_timestep_integers_use_original_nearest_divisors() {
    // Original SimulationManager::GetProjectData, including upward ties. Native
    // CLI independently calibrates 7/8/9/14/45 as 6/10/10/15/60.
    let expected: [u32; 60] = [
        1, 2, 3, 4, 5, 6, 6, 10, 10, 10, 12, 12, 12, 15, 15, 15, 15, 20, 20, 20, 20, 20, 20, 20,
        30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 60, 60, 60,
        60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    ];
    for (index, expected) in expected.into_iter().enumerate() {
        let requested = index + 1;
        let raw = parse_epjson_str(&format!(
            r#"{{"Timestep":{{"t":{{"number_of_timesteps_per_hour":{requested}}}}}}}"#
        ))
        .expect("raw timestep");
        let compiled = compile_raw_model(&raw);
        assert!(
            !compiled.has_errors(),
            "{requested}: {:?}",
            compiled.report.diagnostics
        );
        let model = compiled.model.expect("typed timestep");
        assert_eq!(
            model.timestep.number_of_timesteps_per_hour, expected,
            "requested {requested}"
        );
    }
}

#[test]
fn timestep_presence_defaults_and_schema_boundaries_remain_distinct() {
    for (text, expected) in [
        (r#"{}"#, 4),
        (r#"{"Timestep":{}}"#, 4),
        (r#"{"Timestep":{"t":{}}}"#, 6),
        (
            r#"{"HeatBalanceAlgorithm":{"h":{"algorithm":"ConductionFiniteDifference"}}}"#,
            20,
        ),
        (
            r#"{"HeatBalanceAlgorithm":{"h":{"algorithm":"ConductionFiniteDifference"}},"Timestep":{"t":{"number_of_timesteps_per_hour":30}}}"#,
            30,
        ),
    ] {
        let compiled = compile_raw_model(&parse_epjson_str(text).expect("raw input"));
        assert!(
            !compiled.has_errors(),
            "{text}: {:?}",
            compiled.report.diagnostics
        );
        assert_eq!(
            compiled
                .model
                .expect("typed input")
                .timestep
                .number_of_timesteps_per_hour,
            expected,
            "{text}"
        );
    }
    // An empty IDF field is omitted/defaulted by the converter. An explicit
    // epJSON string remains the wrong type for this integer field.
    let idf_blank = ep_raw_model::parse_epjson_str_with_idf_order(
        r#"{"Timestep":{"Timestep 1":{}}}"#,
        "Timestep,;",
    )
    .expect("converted blank IDF");
    assert_eq!(
        compile_raw_model(&idf_blank)
            .model
            .expect("typed blank IDF")
            .timestep
            .number_of_timesteps_per_hour,
        6
    );
    for value in [
        "0", "61", "-1", "4.5", "4.0", "4e0", "\"7\"", "\"\"", "null",
    ] {
        let text = format!(r#"{{"Timestep":{{"t":{{"number_of_timesteps_per_hour":{value}}}}}}}"#);
        let compiled = compile_raw_model(&parse_epjson_str(&text).expect("raw invalid input"));
        assert!(compiled.has_errors() && compiled.model.is_none(), "{text}");
    }
    let duplicate = parse_epjson_str(r#"{"Timestep":{"a":{},"b":{}}}"#).expect("raw duplicates");
    assert!(compile_raw_model(&duplicate).has_errors());
}

#[test]
fn version_text_is_advisory_and_does_not_select_another_engine() {
    for authored in ["26.1", "26.1.0", "26.1.99", "25.2", "not-a-version", ""] {
        let text = format!(r#"{{"Version":{{"v":{{"version_identifier":"{authored}"}}}}}}"#);
        let compiled = compile_raw_model(&parse_epjson_str(&text).expect("raw Version"));
        assert!(
            !compiled.has_errors(),
            "{authored:?}: {:?}",
            compiled.report.diagnostics
        );
        let model = compiled.model.expect("typed Version");
        assert_eq!(model.version, Version::oracle_26_1_0());
        assert_eq!(model.authored_version_identifier.as_deref(), Some(authored));
    }
    let absent = compile_raw_model(&parse_epjson_str("{}").expect("absent Version"));
    let absent = absent.model.expect("typed absent Version");
    assert_eq!(absent.version, Version::oracle_26_1_0());
    assert_eq!(absent.authored_version_identifier, None);
    // Native direct epJSON accepts an explicitly blank string, but a present
    // Version object without its raw string fails before getter defaults.
    for text in [
        r#"{"Version":{"v":{}}}"#,
        r#"{"Version":{"v":{"version_identifier":26.1}}}"#,
        r#"{"Version":{"v":{"version_identifier":null}}}"#,
        r#"{"Version":{"a":{"version_identifier":"26.1"},"b":{"version_identifier":"26.1"}}}"#,
    ] {
        let compiled = compile_raw_model(&parse_epjson_str(text).expect("raw Version boundary"));
        assert!(compiled.has_errors() && compiled.model.is_none(), "{text}");
    }
}
