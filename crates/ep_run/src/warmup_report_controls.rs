//! Input-only selected GetProjectData warmup report facts; no output/result inputs.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use ep_runtime::HeatBalanceOrdinaryWarmupReportInputs;
use ep_runtime::heat_balance::inside_convection_dispatcher::NoUserConvectionInput;

pub(crate) fn resolve(raw: &RawModel) -> Result<HeatBalanceOrdinaryWarmupReportInputs, String> {
    let control = raw.objects.get(&ObjectType("SimulationControl".into()));
    if control.is_some_and(|instances| instances.len() > 1) {
        return Err("ordinary A reporting requires the admitted unique SimulationControl".into());
    }
    let first = control
        .and_then(|instances| instances.first_key_value())
        .map(|(_, instance)| instance);
    let yes_no = |field: &str, default: bool| -> Result<bool, String> {
        match first.and_then(|instance| instance.fields.get(&FieldName(field.into()))) {
            None => Ok(default),
            Some(RawValue::String(text)) if text.is_empty() => Ok(default),
            Some(RawValue::String(text)) if text.eq_ignore_ascii_case("Yes") => Ok(true),
            Some(RawValue::String(text)) if text.eq_ignore_ascii_case("No") => Ok(false),
            Some(_) => Err(format!(
                "ordinary A SimulationControl.{field} is outside actual Yes/No input"
            )),
        }
    };
    let do_zone_sizing = yes_no("do_zone_sizing_calculation", false)?;
    let do_system_sizing = yes_no("do_system_sizing_calculation", false)?;
    let do_plant_sizing = yes_no("do_plant_sizing_calculation", false)?;
    let do_des_day_sim = yes_no("run_simulation_for_sizing_periods", true)?;
    let do_weather_sim = yes_no("run_simulation_for_weather_file_run_periods", true)?;
    let requested_hvac_sizing = yes_no("do_hvac_sizing_simulation_for_sizing_periods", false)?;
    // Admission follows the actual GetProjectData1146-1158 reset. Raw Yes is
    // retained for the real getter, which emits the mismatch warning itself.
    let do_hvac_sizing = requested_hvac_sizing && do_plant_sizing;
    let do_pure_load_calc = !do_des_day_sim
        && !do_weather_sim
        && !do_hvac_sizing
        && (do_zone_sizing || do_system_sizing);
    let design_days = raw
        .objects
        .get(&ObjectType("SizingPeriod:DesignDay".into()))
        .is_some_and(|instances| !instances.is_empty());
    if do_zone_sizing
        || do_system_sizing
        || do_plant_sizing
        || do_hvac_sizing
        || !do_weather_sim
        || (do_des_day_sim && design_days)
    {
        return Err(
            "ordinary A warmup reporting excludes active sizing/pure-load environments".into(),
        );
    }
    // DataSystemVariables constructor false; GetProjectData processes only the
    // first name-ordered instance, retaining every actual selected key in order.
    let mut detailed = false;
    let mut advanced_report_variables = false;
    if let Some((_, first)) = raw
        .objects
        .get(&ObjectType("Output:Diagnostics".into()))
        .and_then(|instances| instances.first_key_value())
        && let Some(value) = first.fields.get(&FieldName("diagnostics".into()))
    {
        let RawValue::Array(entries) = value else {
            return Err("Output:Diagnostics diagnostics is outside array input".into());
        };
        for entry in entries {
            let RawValue::Object(fields) = entry else {
                return Err("Output:Diagnostics entry is outside object input".into());
            };
            let Some(value) = fields.get(&FieldName("key".into())) else {
                continue;
            };
            let RawValue::String(key) = value else {
                return Err("Output:Diagnostics key is outside string input".into());
            };
            if key.eq_ignore_ascii_case("ReportDetailedWarmupConvergence") {
                detailed = true;
            }
            if key.eq_ignore_ascii_case("DisplayAdvancedReportVariables") {
                advanced_report_variables = true;
            }
        }
    }
    let air = raw
        .objects
        .get(&ObjectType("ZoneAirHeatBalanceAlgorithm".into()));
    if air.is_some_and(|instances| instances.len() > 1) {
        return Err("ordinary A requires unique ZoneAirHeatBalanceAlgorithm".into());
    }
    let first_air = air
        .and_then(|instances| instances.first_key_value())
        .map(|(_, instance)| instance);
    let space_control = |field: &str| -> Result<bool, String> {
        match first_air.and_then(|instance| instance.fields.get(&FieldName(field.into()))) {
            None | Some(RawValue::Null) => Ok(false),
            Some(RawValue::String(text))
                if text.trim().is_empty() || text.eq_ignore_ascii_case("No") =>
            {
                Ok(false)
            }
            Some(RawValue::String(text)) if text.eq_ignore_ascii_case("Yes") => Ok(true),
            Some(_) => Err(format!(
                "ordinary A ZoneAirHeatBalanceAlgorithm.{field} is outside actual Yes/No input"
            )),
        }
    };
    let do_space_heat_balance_for_sizing = space_control("do_space_heat_balance_for_sizing")?;
    let do_space_heat_balance_for_simulation =
        space_control("do_space_heat_balance_for_simulation")?;
    if do_space_heat_balance_for_sizing || do_space_heat_balance_for_simulation {
        return Err("ordinary A temperature owner excludes active space heat balance".into());
    }
    // GetUserConvCoeffs queries these six actual raw families in Source order.
    // A missing typed representation does not establish their absence.
    let counts = NoUserConvectionInput::FAMILIES.map(|family| {
        raw.objects
            .get(&ObjectType(family.into()))
            .map_or(0, |objects| objects.len())
    });
    let no_user_convection_input = NoUserConvectionInput::new(counts).map_err(|error| {
        format!(
            "ordinary A no-user convection input unavailable: {error:?}; actual counts={counts:?}"
        )
    })?;
    let heat = raw.objects.get(&ObjectType("HeatBalanceAlgorithm".into()));
    if heat.is_some_and(|instances| instances.len() > 1) {
        return Err("ordinary A requires unique HeatBalanceAlgorithm".into());
    }
    let field = "minimum_surface_convection_heat_transfer_coefficient_value";
    let value = heat
        .and_then(|instances| instances.first_key_value())
        .and_then(|(_, instance)| instance.fields.get(&FieldName(field.into())));
    // DataHeatBalance.hh1790 constructor; GetProjectData879-882 only writes
    // the supplied nonblank number. IDD N2 admits finite values strictly >0.
    let low_hconv_limit_w_per_m2_k = match value {
        None | Some(RawValue::Null) => 0.1,
        Some(RawValue::String(text)) if text.trim().is_empty() => 0.1,
        Some(RawValue::Number(text)) => text
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| {
                format!("ordinary A HeatBalanceAlgorithm.{field} requires a finite positive number")
            })?,
        Some(_) => {
            return Err(format!(
                "ordinary A HeatBalanceAlgorithm.{field} requires a number or blank"
            ));
        }
    };
    Ok(HeatBalanceOrdinaryWarmupReportInputs {
        detailed,
        advanced_report_variables,
        radiation_input: None,
        do_pure_load_calc,
        do_space_heat_balance_for_sizing,
        do_space_heat_balance_for_simulation,
        no_user_convection_input,
        low_hconv_limit_w_per_m2_k,
        equipment_input: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_raw_model::parse_epjson_str;

    #[test]
    fn absent_controls_use_source_defaults_and_real_selected_key_is_retained()
    -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            resolve(&parse_epjson_str("{}")?)?,
            HeatBalanceOrdinaryWarmupReportInputs {
                detailed: false,
                advanced_report_variables: false,
                radiation_input: None,
                do_pure_load_calc: false,
                do_space_heat_balance_for_sizing: false,
                do_space_heat_balance_for_simulation: false,
                no_user_convection_input: NoUserConvectionInput::new([0; 6])?,
                low_hconv_limit_w_per_m2_k: 0.1,
                equipment_input: None,
            }
        );
        let raw = parse_epjson_str(
            r#"{"Output:Diagnostics":{"first":{"diagnostics":[{}, {"key":"reportdetailedwarmupconvergence"}]}}}"#,
        )?;
        assert!(resolve(&raw)?.detailed);
        let raw = parse_epjson_str(
            r#"{"Output:Diagnostics":{"first":{"diagnostics":[{"key":"displayadvancedreportvariables"}]}}}"#,
        )?;
        let actual = resolve(&raw)?;
        assert!(actual.advanced_report_variables);
        assert!(!actual.detailed);
        assert!(actual.radiation_input.is_none());
        Ok(())
    }

    #[test]
    fn first_name_ordered_diagnostics_only_and_active_sizing_is_unavailable()
    -> Result<(), Box<dyn std::error::Error>> {
        let raw = parse_epjson_str(
            r#"{"Output:Diagnostics":{"Z":{"diagnostics":[{"key":"ReportDetailedWarmupConvergence"}]},"A":{"diagnostics":[{"key":"DisplayExtraWarnings"}]} }}"#,
        )?;
        assert!(!resolve(&raw)?.detailed);
        let raw = parse_epjson_str(
            r#"{"SimulationControl":{"control":{"do_zone_sizing_calculation":"Yes"}}}"#,
        )?;
        assert!(resolve(&raw).is_err());
        let raw = parse_epjson_str(
            r#"{"SimulationControl":{"control":{"run_simulation_for_weather_file_run_periods":"No"}}}"#,
        )?;
        assert!(resolve(&raw).is_err());
        for field in [
            "do_space_heat_balance_for_sizing",
            "do_space_heat_balance_for_simulation",
        ] {
            let raw = parse_epjson_str(&format!(
                r#"{{"ZoneAirHeatBalanceAlgorithm":{{"air":{{"{field}":"Yes"}}}}}}"#
            ))?;
            assert!(resolve(&raw).is_err());
            let raw = parse_epjson_str(&format!(
                r#"{{"ZoneAirHeatBalanceAlgorithm":{{"air":{{"{field}":null}}}}}}"#
            ))?;
            assert!(!resolve(&raw)?.do_space_heat_balance_for_simulation);
        }
        Ok(())
    }

    #[test]
    fn simulation_control_inactive_hvac_request_keeps_effective_sizing_blocked()
    -> Result<(), Box<dyn std::error::Error>> {
        for (plant, hvac, accepted) in [
            ("No", "Yes", true),
            ("", "Yes", true),
            ("No", "No", true),
            ("Yes", "No", false),
            ("Yes", "Yes", false),
        ] {
            let raw = parse_epjson_str(&format!(
                r#"{{"SimulationControl":{{"control":{{"do_plant_sizing_calculation":"{plant}","do_hvac_sizing_simulation_for_sizing_periods":"{hvac}","run_simulation_for_sizing_periods":"No","run_simulation_for_weather_file_run_periods":"Yes"}}}}}}"#
            ))?;
            let actual = resolve(&raw);
            assert_eq!(actual.is_ok(), accepted, "plant={plant}, HVAC={hvac}");
            if let Ok(actual) = actual {
                assert!(!actual.do_pure_load_calc);
            }
        }
        Ok(())
    }

    #[test]
    fn ordinary_convection_raw_inputs_preserve_defaults_values_and_reject_unsupported()
    -> Result<(), Box<dyn std::error::Error>> {
        let field = "minimum_surface_convection_heat_transfer_coefficient_value";
        for (name, input, expected) in [
            ("absent object", "{}", Some(0.1_f64)),
            (
                "absent field",
                r#"{"HeatBalanceAlgorithm":{"heat":{}}}"#,
                Some(0.1),
            ),
            (
                "null blank",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":null}}}"#,
                Some(0.1),
            ),
            (
                "text blank",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":" "}}}"#,
                Some(0.1),
            ),
            (
                "explicit default",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":0.1}}}"#,
                Some(0.1),
            ),
            (
                "explicit positive",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":0.375}}}"#,
                Some(0.375),
            ),
            (
                "zero",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":0}}}"#,
                None,
            ),
            (
                "negative",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":-0.1}}}"#,
                None,
            ),
            (
                "numeric text",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":"0.1"}}}"#,
                None,
            ),
            (
                "boolean",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":true}}}"#,
                None,
            ),
            (
                "structured",
                r#"{"HeatBalanceAlgorithm":{"heat":{"minimum_surface_convection_heat_transfer_coefficient_value":[]}}}"#,
                None,
            ),
            (
                "duplicate object",
                r#"{"HeatBalanceAlgorithm":{"first":{},"second":{}}}"#,
                None,
            ),
        ] {
            let result = resolve(&parse_epjson_str(input)?);
            if let Some(expected) = expected {
                let actual = result.map_err(|error| format!("{name}: {error}"))?;
                assert_eq!(actual.no_user_convection_input.counts(), [0; 6], "{name}");
                assert_eq!(
                    actual.low_hconv_limit_w_per_m2_k.to_bits(),
                    expected.to_bits(),
                    "{name}"
                );
            } else {
                assert!(result.is_err(), "{name} must be unavailable");
            }
        }
        for family in NoUserConvectionInput::FAMILIES {
            let empty = format!(r#"{{"{family}":{{}}}}"#);
            assert_eq!(
                resolve(&parse_epjson_str(&empty)?)?
                    .no_user_convection_input
                    .counts(),
                [0; 6]
            );
            let input = format!(r#"{{"{family}":{{"first":{{}},"second":{{}}}}}}"#);
            let result = resolve(&parse_epjson_str(&input)?);
            assert!(result.is_err(), "nonzero actual raw count for {family}");
        }
        // Raw numeric lexemes can also be supplied by a non-JSON adapter. A
        // nonfinite/unparseable operand cannot be retained as valid input.
        for number in ["NaN", "inf", "1e400", "not-a-number"] {
            let mut raw = parse_epjson_str(r#"{"HeatBalanceAlgorithm":{"heat":{}}}"#)?;
            raw.objects
                .get_mut(&ObjectType("HeatBalanceAlgorithm".into()))
                .and_then(|objects| objects.values_mut().next())
                .ok_or("missing heat input")?
                .fields
                .insert(FieldName(field.into()), RawValue::Number(number.into()));
            assert!(
                resolve(&raw).is_err(),
                "unsupported numeric operand {number}"
            );
        }
        Ok(())
    }
}
