//! Selected weather controls from captured environment inputs and raw diagnostics.
//!
//! This resolver implements the four selected assignments, not the other
//! effects or warning text of the complete native initialization routines.

use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use ep_runtime::weather::day::WeatherSolarControls;

/// Literal environment inputs captured once before preparing a live session.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WeatherSolarEnvironmentInputs {
    pub(crate) ignore_solar_radiation: Option<String>,
    pub(crate) ignore_beam_radiation: Option<String>,
    pub(crate) ignore_diffuse_radiation: Option<String>,
}

/// Capture the three source-named variables without changing the process.
pub(crate) fn capture_environment_inputs() -> Result<WeatherSolarEnvironmentInputs, String> {
    fn capture(name: &str) -> Result<Option<String>, String> {
        match std::env::var(name) {
            Ok(value) => Ok(Some(value)),
            Err(std::env::VarError::NotPresent) => Ok(None),
            Err(std::env::VarError::NotUnicode(_)) => Err(format!(
                "weather control {name} is outside the UTF-8 input domain"
            )),
        }
    }
    Ok(WeatherSolarEnvironmentInputs {
        ignore_solar_radiation: capture("IgnoreSolarRadiation")?,
        ignore_beam_radiation: capture("IgnoreBeamRadiation")?,
        ignore_diffuse_radiation: capture("IgnoreDiffuseRadiation")?,
    })
}

/// Resolve selected controls in environment-then-diagnostics source order.
///
/// The supplied model is the actual canonical epJSON raw owner. The native
/// epJSON instance loop uses the first name-ordered Output:Diagnostics object;
/// it explicitly breaks before processing another instance (cc:1052-1053).
pub(crate) fn resolve_controls(
    raw: &RawModel,
    environment: &WeatherSolarEnvironmentInputs,
) -> Result<WeatherSolarControls, String> {
    // DataEnvironment.hh:200-203 initializes these flags false. The three
    // nonempty environment assignments precede GetProjectData diagnostics.
    let mut controls = WeatherSolarControls {
        ignore_solar_radiation: environment_on(environment.ignore_solar_radiation.as_deref()),
        ignore_beam_radiation: environment_on(environment.ignore_beam_radiation.as_deref()),
        ignore_diffuse_radiation: environment_on(environment.ignore_diffuse_radiation.as_deref()),
        ..WeatherSolarControls::default()
    };
    let Some(instances) = raw.objects.get(&ObjectType("Output:Diagnostics".into())) else {
        return Ok(controls);
    };
    let Some((_, instance)) = instances.first_key_value() else {
        return Ok(controls);
    };
    let Some(diagnostics) = instance.fields.get(&FieldName("diagnostics".into())) else {
        return Ok(controls);
    };
    let RawValue::Array(diagnostics) = diagnostics else {
        return Err("Output:Diagnostics diagnostics is outside the array input domain".into());
    };
    for diagnostic in diagnostics {
        let RawValue::Object(fields) = diagnostic else {
            return Err("Output:Diagnostics entry is outside the object input domain".into());
        };
        // Native GetProjectData warns and skips an entry without its key.
        let Some(key) = fields.get(&FieldName("key".into())) else {
            continue;
        };
        let RawValue::String(key) = key else {
            return Err("Output:Diagnostics key is outside the string input domain".into());
        };
        if key.eq_ignore_ascii_case("DisplayWeatherMissingDataWarnings") {
            controls.display_weather_missing_data_warnings = true;
        } else if key.eq_ignore_ascii_case("IgnoreSolarRadiation") {
            controls.ignore_solar_radiation = true;
        } else if key.eq_ignore_ascii_case("IgnoreBeamRadiation") {
            controls.ignore_beam_radiation = true;
        } else if key.eq_ignore_ascii_case("IgnoreDiffuseRadiation") {
            controls.ignore_diffuse_radiation = true;
        }
    }
    Ok(controls)
}

fn environment_on(value: Option<&str>) -> bool {
    // UtilityRoutines.cc:691-702 examines only the first character, untrimmed.
    matches!(
        value.and_then(|value| value.as_bytes().first()),
        Some(b'Y' | b'y' | b'T' | b't')
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_raw_model::{ObjectName, RawObject};
    use std::collections::BTreeMap;

    fn diagnostics(keys: &[&str]) -> RawObject {
        let values = keys
            .iter()
            .map(|key| {
                RawValue::Object(BTreeMap::from([(
                    FieldName("key".into()),
                    RawValue::String((*key).into()),
                )]))
            })
            .collect();
        RawObject {
            fields: BTreeMap::from([(FieldName("diagnostics".into()), RawValue::Array(values))]),
            source_span: None,
        }
    }

    fn model(instances: &[(&str, RawObject)]) -> RawModel {
        RawModel::new(
            None,
            BTreeMap::from([(
                ObjectType("Output:Diagnostics".into()),
                instances
                    .iter()
                    .map(|(name, object)| (ObjectName((*name).into()), object.clone()))
                    .collect(),
            )]),
        )
    }

    #[test]
    fn source_environment_checks_the_untrimmed_first_character_only() {
        for value in ["Y", "yes", "True", "t-anything"] {
            assert!(environment_on(Some(value)), "{value}");
        }
        for value in ["", " Y", " true", "N", "false", "1", "αY"] {
            assert!(!environment_on(Some(value)), "{value}");
        }
        assert!(!environment_on(None));
    }

    #[test]
    fn supplied_environment_inputs_remain_effective_without_diagnostics() {
        let environment = WeatherSolarEnvironmentInputs {
            ignore_solar_radiation: Some("Yes".into()),
            ignore_beam_radiation: Some("false".into()),
            ignore_diffuse_radiation: Some("true".into()),
        };
        assert_eq!(
            resolve_controls(&RawModel::default(), &environment),
            Ok(WeatherSolarControls {
                display_weather_missing_data_warnings: false,
                ignore_solar_radiation: true,
                ignore_beam_radiation: false,
                ignore_diffuse_radiation: true,
            })
        );
    }

    #[test]
    fn diagnostics_assign_true_after_environment_values() {
        let raw = model(&[(
            "A",
            diagnostics(&[
                "displayweathermissingdatawarnings",
                "IgnoreSolarRadiation",
                "IgnoreBeamRadiation",
                "IgnoreDiffuseRadiation",
            ]),
        )]);
        let environment = WeatherSolarEnvironmentInputs {
            ignore_solar_radiation: Some("No".into()),
            ignore_beam_radiation: Some("No".into()),
            ignore_diffuse_radiation: Some("No".into()),
        };
        assert_eq!(
            resolve_controls(&raw, &environment),
            Ok(WeatherSolarControls {
                display_weather_missing_data_warnings: true,
                ignore_solar_radiation: true,
                ignore_beam_radiation: true,
                ignore_diffuse_radiation: true,
            })
        );
    }

    #[test]
    fn duplicate_instances_use_only_the_first_epjson_name() {
        let raw = model(&[
            ("Z", diagnostics(&["IgnoreSolarRadiation"])),
            ("A", diagnostics(&["IgnoreBeamRadiation"])),
        ]);
        assert_eq!(
            resolve_controls(&raw, &WeatherSolarEnvironmentInputs::default()),
            Ok(WeatherSolarControls {
                ignore_beam_radiation: true,
                ..WeatherSolarControls::default()
            })
        );
    }

    #[test]
    fn unrelated_con_diagnostic_does_not_assign_weather_controls() {
        let raw = model(&[("A", diagnostics(&["DisplayAdvancedReportVariables"]))]);
        assert_eq!(
            resolve_controls(&raw, &WeatherSolarEnvironmentInputs::default()),
            Ok(WeatherSolarControls::default())
        );
    }

    #[test]
    fn missing_extensible_key_is_skipped_but_malformed_types_are_rejected() {
        let mut object = diagnostics(&[]);
        object.fields.insert(
            FieldName("diagnostics".into()),
            RawValue::Array(vec![RawValue::Object(BTreeMap::new())]),
        );
        assert_eq!(
            resolve_controls(
                &model(&[("A", object)]),
                &WeatherSolarEnvironmentInputs::default()
            ),
            Ok(WeatherSolarControls::default())
        );
        let mut object = diagnostics(&[]);
        object
            .fields
            .insert(FieldName("diagnostics".into()), RawValue::Bool(true));
        assert!(
            resolve_controls(
                &model(&[("A", object)]),
                &WeatherSolarEnvironmentInputs::default()
            )
            .is_err()
        );
    }
}
