use super::*;
use crate::CompileResult;
use ep_model::ScheduleUnitType;

fn compact_result(
    interpolation: &str,
    fields: &str,
    lower: f64,
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let input = format!(
        r#"{{
        "Timestep": {{"Context": {{"number_of_timesteps_per_hour": 4}}}},
        "ScheduleTypeLimits": {{"T": {{"lower_limit_value": {lower},
            "upper_limit_value": 1, "numeric_type": "Continuous"}}}},
        "Schedule:Compact": {{"S": {{"schedule_type_limits_name": "T", "data": [
            {{"field": "Through: 12/31"}}, {{"field": "For: AllDays"}},
            {{"field": "Interpolate: {interpolation}"}}, {fields}
        ]}}}}
    }}"#
    );
    Ok(compile_raw_model(&parse_epjson_str(&input).map_err(
        |error| std::io::Error::other(format!("literal schedule input should parse: {error}")),
    )?))
}

fn has_compact_range_error(result: &CompileResult) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "ScheduleValueOutsideTypeLimits"
            && diagnostic.object_type == "Schedule:Compact"
    })
}

#[test]
fn rejects_populated_compact_values_outside_active_limits() -> Result<(), Box<dyn std::error::Error>>
{
    let result = compact_result("No", r#"{"field":"Until: 24:00"},{"field":2}"#, 0.0)?;
    assert!(result.model.is_none());
    assert!(has_compact_range_error(&result));
    Ok(())
}

#[test]
fn validates_actual_average_or_endpoint_instead_of_segment_extrema()
-> Result<(), Box<dyn std::error::Error>> {
    let brief_peak = r#"{"field":"Until: 00:01"},{"field":2},
        {"field":"Until: 24:00"},{"field":0}"#;
    for mode in ["Average", "No"] {
        let result = compact_result(mode, brief_peak, 0.0)?;
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert!(result.model.is_some());
    }
    let excessive_average = compact_result(
        "Average",
        r#"
        {"field":"Until: 00:01"},{"field":30},
        {"field":"Until: 24:00"},{"field":0}"#,
        0.0,
    )?;
    assert!(has_compact_range_error(&excessive_average));
    Ok(())
}

#[test]
fn unreferenced_all_other_days_profile_does_not_change_annual_extrema()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compact_result(
        "No",
        r#"
        {"field":"Until: 24:00"},{"field":0},
        {"field":"For: AllOtherDays"},
        {"field":"Until: 24:00"},{"field":2}"#,
        0.0,
    )?;
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    Ok(())
}

#[test]
fn missing_day_owned_zero_participates_in_positive_lower_limit_validation()
-> Result<(), Box<dyn std::error::Error>> {
    // The parser's source-defined missing-day profile owns the other eleven real day types.
    let input = r#"{
        "Timestep":{"Context":{"number_of_timesteps_per_hour":4}},
        "ScheduleTypeLimits":{"T":{"lower_limit_value":0.5,"upper_limit_value":2,
            "numeric_type":"Continuous"}},
        "Schedule:Compact":{"S":{"schedule_type_limits_name":"T","data":[
            {"field":"Through: 12/31"},{"field":"For: Monday"},
            {"field":"Until: 24:00"},{"field":1}
        ]}}
    }"#;
    let result =
        compile_raw_model(&parse_epjson_str(input).map_err(|error| {
            std::io::Error::other(format!("literal input should parse: {error}"))
        })?);
    assert!(result.model.is_none());
    assert!(has_compact_range_error(&result));
    Ok(())
}

#[test]
fn compact_limits_use_difference_first_f32_epsilon() -> Result<(), Box<dyn std::error::Error>> {
    let epsilon = f64::from(f32::EPSILON);
    let accepted = [-epsilon, 1.0 + epsilon];
    let rejected = [
        f64::from_bits((-epsilon).to_bits() + 1),
        f64::from_bits((1.0 + epsilon).to_bits() + 1),
    ];
    for (values, should_error) in [(&accepted, false), (&rejected, true)] {
        for value in values {
            let fields = format!(r#"{{"field":"Until: 24:00"}},{{"field":{value}}}"#);
            let result = compact_result("No", &fields, 0.0)?;
            assert_eq!(has_compact_range_error(&result), should_error);
            assert_eq!(result.model.is_none(), should_error);
        }
    }
    Ok(())
}

#[test]
fn all_unit_names_have_real_typed_owners_and_blank_unit_stays_invalid()
-> Result<(), Box<dyn std::error::Error>> {
    let units = [
        ("Dimensionless", ScheduleUnitType::Dimensionless),
        ("Temperature", ScheduleUnitType::Temperature),
        ("DeltaTemperature", ScheduleUnitType::DeltaTemperature),
        ("PrecipitationRate", ScheduleUnitType::PrecipitationRate),
        ("Angle", ScheduleUnitType::Angle),
        (
            "ConvectionCoefficient",
            ScheduleUnitType::ConvectionCoefficient,
        ),
        ("ActivityLevel", ScheduleUnitType::ActivityLevel),
        ("Velocity", ScheduleUnitType::Velocity),
        ("Capacity", ScheduleUnitType::Capacity),
        ("Power", ScheduleUnitType::Power),
        ("Availability", ScheduleUnitType::Availability),
        ("Percent", ScheduleUnitType::Percent),
        ("Control", ScheduleUnitType::Control),
        ("Mode", ScheduleUnitType::Mode),
    ];
    for (name, expected) in units {
        let input = format!(
            r#"{{"ScheduleTypeLimits":{{"T":{{"unit_type":"{}"}}}}}}"#,
            name.to_ascii_lowercase()
        );
        let result =
            compile_raw_model(&parse_epjson_str(&input).map_err(|error| {
                std::io::Error::other(format!("unit input should parse: {error}"))
            })?);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .model
                .ok_or_else(|| std::io::Error::other("typed unit owner"))?
                .schedule_type_limits[0]
                .unit_type,
            expected
        );
    }
    for declaration in ["", r#""unit_type":"""#] {
        let input = format!(r#"{{"ScheduleTypeLimits":{{"T":{{{declaration}}}}}}}"#);
        let result =
            compile_raw_model(&parse_epjson_str(&input).map_err(|error| {
                std::io::Error::other(format!("blank unit should parse: {error}"))
            })?);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .model
                .ok_or_else(|| std::io::Error::other("typed default owner"))?
                .schedule_type_limits[0]
                .unit_type,
            ScheduleUnitType::Invalid
        );
    }
    Ok(())
}

#[test]
fn invalid_unit_rejects_model_without_new_discrete_value_rejection()
-> Result<(), Box<dyn std::error::Error>> {
    let invalid = parse_epjson_str(r#"{"ScheduleTypeLimits":{"T":{"unit_type":"NotAUnit"}}}"#)
        .map_err(|error| {
            std::io::Error::other(format!(
                "invalid enum input should still parse JSON: {error}"
            ))
        })?;
    let result = compile_raw_model(&invalid);
    assert!(result.has_errors());
    assert!(result.model.is_none());
    let discrete = parse_epjson_str(
        r#"{
        "ScheduleTypeLimits":{"T":{"lower_limit_value":0,"upper_limit_value":1,
            "numeric_type":"Discrete"}},
        "Schedule:Compact":{"S":{"schedule_type_limits_name":"T","data":[
            {"field":"Through: 12/31"},{"field":"For: AllDays"},
            {"field":"Until: 24:00"},{"field":0.5}
        ]}}
    }"#,
    )
    .map_err(|error| {
        std::io::Error::other(format!("literal discrete input should parse: {error}"))
    })?;
    let result = compile_raw_model(&discrete);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.model.is_some());
    Ok(())
}
