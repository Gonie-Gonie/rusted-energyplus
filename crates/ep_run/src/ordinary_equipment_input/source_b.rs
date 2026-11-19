//! Finite raw field grammar for the pinned Source A/B families.
//! Retains input operands for their actual Source getters and owners.
use super::{blank, number, text_is};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
/// Native getter order; this is schema/Raw custody, not a second Curve registry.
pub(crate) const ANALYTIC_CURVE_FAMILIES: [&str; 5] = [
    "Curve:Biquadratic",
    "Curve:QuadraticLinear",
    "Curve:CubicLinear",
    "Curve:Bicubic",
    "Curve:FanPressureRise",
];
pub(crate) fn is_analytic_curve_family(family: &str) -> bool {
    ANALYTIC_CURVE_FAMILIES.contains(&family)
}
fn analytic_curve_numeric_fields(family: &str) -> &'static [&'static str] {
    match family {
        "Curve:Biquadratic" => &[
            "coefficient1_constant",
            "coefficient2_x",
            "coefficient3_x_2",
            "coefficient4_y",
            "coefficient5_y_2",
            "coefficient6_x_y",
            "minimum_value_of_x",
            "maximum_value_of_x",
            "minimum_value_of_y",
            "maximum_value_of_y",
            "minimum_curve_output",
            "maximum_curve_output",
        ],
        "Curve:QuadraticLinear" => &[
            "coefficient1_constant",
            "coefficient2_x",
            "coefficient3_x_2",
            "coefficient4_y",
            "coefficient5_x_y",
            "coefficient6_x_2_y",
            "minimum_value_of_x",
            "maximum_value_of_x",
            "minimum_value_of_y",
            "maximum_value_of_y",
            "minimum_curve_output",
            "maximum_curve_output",
        ],
        "Curve:CubicLinear" => &[
            "coefficient1_constant",
            "coefficient2_x",
            "coefficient3_x_2",
            "coefficient4_x_3",
            "coefficient5_y",
            "coefficient6_x_y",
            "minimum_value_of_x",
            "maximum_value_of_x",
            "minimum_value_of_y",
            "maximum_value_of_y",
            "minimum_curve_output",
            "maximum_curve_output",
        ],
        "Curve:Bicubic" => &[
            "coefficient1_constant",
            "coefficient2_x",
            "coefficient3_x_2",
            "coefficient4_y",
            "coefficient5_y_2",
            "coefficient6_x_y",
            "coefficient7_x_3",
            "coefficient8_y_3",
            "coefficient9_x_2_y",
            "coefficient10_x_y_2",
            "minimum_value_of_x",
            "maximum_value_of_x",
            "minimum_value_of_y",
            "maximum_value_of_y",
            "minimum_curve_output",
            "maximum_curve_output",
        ],
        "Curve:FanPressureRise" => &[
            "coefficient1_c1",
            "coefficient2_c2",
            "coefficient3_c3",
            "coefficient4_c4",
            "minimum_value_of_qfan",
            "maximum_value_of_qfan",
            "minimum_value_of_psm",
            "maximum_value_of_psm",
            "minimum_curve_output",
            "maximum_curve_output",
        ],
        _ => &[],
    }
}
fn analytic_curve_required_fields(family: &str) -> &'static [&'static str] {
    let fields = analytic_curve_numeric_fields(family);
    &fields[..fields.len().saturating_sub(2)]
}
fn analytic_curve_admitted_field(family: &str, name: &str, value: &RawValue) -> bool {
    if analytic_curve_numeric_fields(family).contains(&name) {
        return matches!(value, RawValue::Number(token)
            if ep_model::classify_source_json_number(token).is_ok());
    }
    if family != "Curve:FanPressureRise" {
        let choices: Option<&[&str]> = match name {
            "input_unit_type_for_x" | "input_unit_type_for_y" if family == "Curve:CubicLinear" => {
                Some(&["", "Dimensionless", "Temperature"])
            }
            "input_unit_type_for_x" | "input_unit_type_for_y" => Some(&[
                "",
                "Dimensionless",
                "Distance",
                "MassFlow",
                "Power",
                "Temperature",
                "VolumetricFlow",
            ]),
            "output_unit_type" => Some(&[
                "",
                "Capacity",
                "Dimensionless",
                "Power",
                "Pressure",
                "Temperature",
            ]),
            _ => None,
        };
        if let Some(choices) = choices {
            return matches!(value, RawValue::String(text) if choices.contains(&text.as_str()));
        }
    }
    // Official direct-epJSON additional members,including name/idf_*,are not operands.
    true
}
/// Original schema plus actual unused/no-AFN composition only; never derives Curve state.
pub(crate) fn validate_unused_analytic_curves(raw: &RawModel) -> Result<(), String> {
    let selected = raw
        .objects
        .iter()
        .any(|(family, rows)| is_analytic_curve_family(&family.0) && !rows.is_empty());
    if !selected {
        return Ok(());
    }
    for family in ANALYTIC_CURVE_FAMILIES {
        raw.validate_source_idf_object_custody(family)?;
    }
    if raw.objects.iter().any(|(family, rows)| {
        !rows.is_empty()
            && (family.0.starts_with("AirflowNetwork:")
                || family.0.starts_with("Duct:Loss:")
                || family.0.starts_with("EnergyManagementSystem:")
                || family.0.starts_with("Table:")
                || (family.0.starts_with("Curve:") && !is_analytic_curve_family(&family.0)))
    }) {
        return Err(
            "Source unused analytic curves exclude AFN,duct loss,EMS,Table and other Curve owners"
                .into(),
        );
    }
    for family in ANALYTIC_CURVE_FAMILIES {
        let Some(rows) = raw.objects.get(&ObjectType(family.into())) else {
            continue;
        };
        for (name, object) in rows {
            if !super::window_gas::valid_name(&name.0) {
                return Err(format!("Source {family} requires its original schema name"));
            }
            for required in analytic_curve_required_fields(family) {
                if object
                    .fields
                    .get(&FieldName((*required).into()))
                    .is_none_or(|value| !analytic_curve_admitted_field(family, required, value))
                {
                    return Err(format!("Source {family} requires schema-valid {required}"));
                }
            }
            if object
                .fields
                .iter()
                .any(|(name, value)| !analytic_curve_admitted_field(family, &name.0, value))
            {
                return Err(format!(
                    "Source {family} requires valid original known optional fields"
                ));
            }
        }
    }
    Ok(())
}
pub(super) fn is_family(family: &str) -> bool {
    is_analytic_curve_family(family)
        || matches!(
            family,
            "SetpointManager:Scheduled"
                | "Output:Schedules"
                | "Schedule:Compact"
                | "Schedule:File"
                | "Schedule:Day:Hourly"
                | "Schedule:Day:Interval"
                | "Schedule:Day:List"
                | "Schedule:Week:Compact"
                | "Schedule:Week:Daily"
                | "Schedule:Year"
                | "ThermostatSetpoint:DualSetpoint"
                | "ZoneControl:Thermostat"
                | "ZoneControl:Humidistat"
                | "DesignSpecification:OutdoorAir"
                | "OutdoorAir:Node"
                | "OutdoorAir:NodeList"
                | "ZoneHVAC:EquipmentConnections"
                | "ZoneHVAC:EquipmentList"
                | "ZoneHVAC:IdealLoadsAirSystem"
        )
}
// Retain the ordered raw input-bound predicates without changing admission logic.
#[allow(clippy::manual_range_contains)]
pub(super) fn admitted_field(
    family: &str,
    name: &str,
    value: &RawValue,
    source_is_epjson: bool,
) -> bool {
    if is_analytic_curve_family(family) {
        return analytic_curve_admitted_field(family, name, value);
    }
    // Direct JSON keeps its existing unconsumed-metadata boundary. Original
    // IDF bookkeeping is admitted separately only with real raw custody.
    if family == "OutdoorAir:NodeList" {
        return if name == "nodes" {
            matches!(value, RawValue::Array(members) if members.iter().all(|member| {
                matches!(member, RawValue::Object(fields)
                    if (source_is_epjson || fields.keys().all(|field| field.0 == "node_or_nodelist_name"))
                        && match fields.get(&FieldName("node_or_nodelist_name".into())) {
                            Some(RawValue::String(_)) => true,
                            None => !source_is_epjson,
                            _ => false,
                        })
            }))
        } else {
            source_is_epjson
        };
    }
    if matches!(
        name,
        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
    ) {
        // Original OA/DSOA metadata is accepted only by the caller's genuine
        // recovered-custody branch, never this generic three-key fallback.
        return (source_is_epjson
            || !matches!(family, "OutdoorAir:Node" | "DesignSpecification:OutdoorAir"))
            && matches!(value, RawValue::Number(text) if text.parse::<usize>().is_ok());
    }
    match (family, name) {
        (
            "OutdoorAir:Node",
            "drybulb_temperature_schedule_name"
            | "wetbulb_temperature_schedule_name"
            | "wind_speed_schedule_name"
            | "wind_direction_schedule_name"
            | "wind_pressure_coefficient_curve_name",
        ) => {
            matches!(value, RawValue::String(_))
        }
        ("OutdoorAir:Node", "symmetric_wind_pressure_coefficient_curve") => {
            matches!(value, RawValue::String(value) if matches!(value.as_str(), "" | "No" | "Yes"))
        }
        ("OutdoorAir:Node", "wind_angle_type") => {
            matches!(value, RawValue::String(value) if matches!(value.as_str(), "" | "Absolute" | "Relative"))
        }
        ("OutdoorAir:Node", "height_above_ground") => {
            matches!(value, RawValue::Number(token) if ep_model::classify_source_json_number(token)
                .map(ep_model::SourceJsonNumber::as_f64)
                .is_ok_and(|height| height.is_finite()))
        }
        ("DesignSpecification:OutdoorAir", "outdoor_air_method") => text_is(
            value,
            &[
                "",
                "Flow/Person",
                "Flow/Area",
                "Flow/Zone",
                "AirChanges/Hour",
                "Sum",
                "Maximum",
            ],
        ),
        (
            "DesignSpecification:OutdoorAir",
            "outdoor_air_flow_per_person"
            | "outdoor_air_flow_per_zone_floor_area"
            | "outdoor_air_flow_per_zone"
            | "outdoor_air_flow_air_changes_per_hour",
        ) => number(value).is_some_and(|value| value >= 0.0),
        (
            "DesignSpecification:OutdoorAir",
            "outdoor_air_schedule_name"
            | "proportional_control_minimum_outdoor_air_flow_rate_schedule_name",
        ) => {
            matches!(value, RawValue::String(_))
        }
        // Humidistat has four alpha fields including its map-key Name.
        (
            "ZoneControl:Humidistat",
            "zone_name"
            | "humidifying_relative_humidity_setpoint_schedule_name"
            | "dehumidifying_relative_humidity_setpoint_schedule_name",
        ) => {
            matches!(value, RawValue::String(_))
        }
        // The instance schema permits extra metadata, with reserved IDF
        // bookkeeping still checked above. No metadata becomes a getter field.
        ("ZoneControl:Humidistat", _) => true,
        // The Native object has four alpha fields (Name is the map key), no

        // numeric/extensible fields. All nine Native selectors reach real Node fields.
        ("SetpointManager:Scheduled", "control_variable") => text_is(
            value,
            &[
                "Temperature",
                "MaximumTemperature",
                "MinimumTemperature",
                "HumidityRatio",
                "MaximumHumidityRatio",
                "MinimumHumidityRatio",
                "MassFlowRate",
                "MaximumMassFlowRate",
                "MinimumMassFlowRate",
            ],
        ),
        ("SetpointManager:Scheduled", "schedule_name" | "setpoint_node_or_nodelist_name") => {
            matches!(value, RawValue::String(text) if !text.is_empty())
        }
        // Official instance schema permits extra metadata. Reserved IDF
        // bookkeeping was checked above; no extra field becomes a getter operand.
        ("SetpointManager:Scheduled", _) => true,
        ("Schedule:File", "schedule_type_limits_name" | "file_name") => {
            matches!(value, RawValue::String(_))
        }
        ("Schedule:File", "column_number") => {
            number(value).is_some_and(|n| n.fract() == 0.0 && n >= 1.0)
        }
        ("Schedule:File", "rows_to_skip_at_top") => {
            number(value).is_some_and(|n| n.fract() == 0.0 && n >= 0.0)
        }
        ("Schedule:File", "number_of_hours_of_data") => {
            number(value).is_some_and(|n| (8760.0..=8784.0).contains(&n))
        }
        ("Schedule:File", "column_separator") => {
            text_is(value, &["", "Comma", "Semicolon", "Tab", "Space"])
        }
        ("Schedule:File", "interpolate_to_timestep" | "adjust_schedule_for_daylight_savings") => {
            text_is(value, &["", "No", "Yes"])
        }
        ("Schedule:File", "minutes_per_item") => {
            number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=60.0).contains(&n))
        }
        ("Output:Schedules", "key_field") => text_is(value, &["Hourly", "Timestep"]),
        // The unnamed report schema does not prohibit additional members.
        // They remain raw metadata, never schedule values or field counts.
        ("Output:Schedules", _) => true,
        (
            "Schedule:Day:Hourly" | "Schedule:Day:Interval" | "Schedule:Day:List" | "Schedule:Year",
            "schedule_type_limits_name",
        ) => {
            matches!(value, RawValue::String(_))
        }
        ("Schedule:Day:Hourly", name)
            if name
                .strip_prefix("hour_")
                .and_then(|s| s.parse::<u32>().ok())
                .is_some_and(|n| (1..=24).contains(&n) && name == format!("hour_{n}")) =>
        {
            number(value).is_some()
        }
        ("Schedule:Day:Interval" | "Schedule:Day:List", "interpolate_to_timestep") => {
            text_is(value, &["", "No", "Average", "Linear"])
        }
        ("Schedule:Day:Interval", "data") => {
            // Schema items have no required fields or additionalProperties
            // restriction. The actual IP supplies missing numeric fields as
            // blank0 and ignores unrelated retained item metadata.
            matches!(value, RawValue::Array(items) if items.iter().all(|item| {
                let RawValue::Object(fields) = item else {return false;};
                fields.get(&FieldName("time".into())).is_none_or(|value| matches!(value, RawValue::String(_)))
                    && fields.get(&FieldName("value_until_time".into())).is_none_or(|value| number(value).is_some())
            }))
        }
        ("Schedule:Day:List", "minutes_per_item") => {
            number(value).is_some_and(|n| n.fract() == 0.0 && n >= 1.0 && n <= 60.0)
        }
        ("Schedule:Day:List", "extensions") => {
            matches!(value, RawValue::Array(items) if items.iter().all(|item| {
                let RawValue::Object(fields) = item else {return false;};
                fields.get(&FieldName("value".into())).is_none_or(|value| number(value).is_some())
            }))
        }
        ("Schedule:Week:Compact", "data") => {
            matches!(value, RawValue::Array(items) if items.iter().all(|item| {
                let RawValue::Object(fields) = item else {return false;};
                ["daytype_list", "schedule_day_name"].iter().all(|field| {
                    matches!(fields.get(&FieldName((*field).into())), Some(RawValue::String(_)))
                })
            }))
        }
        ("Schedule:Week:Daily", name) if REGULAR_DAY_FIELDS.contains(&name) => {
            matches!(value, RawValue::String(_))
        }
        ("Schedule:Year", "schedule_weeks") => {
            matches!(value, RawValue::Array(items) if !items.is_empty() && items.len() <= 53 && items.iter().all(|item| {
                let RawValue::Object(fields) = item else {return false;};
                matches!(fields.get(&FieldName("schedule_week_name".into())), Some(RawValue::String(_))) && ["start_month", "start_day", "end_month", "end_day"].iter().all(|name| {
                    fields.get(&FieldName((*name).into())).and_then(number).is_some_and(|n| n.fract() == 0.0 && n >= 1.0 && n <= if name.ends_with("month") {12.0} else {31.0})
                })
            }))
        }
        ("Schedule:Compact", "schedule_type_limits_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Schedule:Compact", "data") => {
            blank(value)
                // Native items.field is optional number|string, with no
                // additionalProperties restriction. Other item members remain
                // raw metadata; only the actual field is read as an alpha.
                || matches!(value, RawValue::Array(items) if items.iter().all(|item| matches!(item, RawValue::Object(fields) if fields.get(&FieldName("field".into())).is_none_or(|value| number(value).is_some() || matches!(value, RawValue::String(_))))))
        }
        ("ThermostatSetpoint:DualSetpoint", "heating_setpoint_temperature_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ThermostatSetpoint:DualSetpoint", "cooling_setpoint_temperature_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "zone_or_zonelist_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "control_type_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "control_1_object_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "ThermostatSetpoint:DualSetpoint",
                        "ThermostatSetpoint:SingleCooling",
                        "ThermostatSetpoint:SingleHeating",
                        "ThermostatSetpoint:SingleHeatingOrCooling",
                    ],
                )
        }
        ("ZoneControl:Thermostat", "control_1_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "control_2_object_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "ThermostatSetpoint:DualSetpoint",
                        "ThermostatSetpoint:SingleCooling",
                        "ThermostatSetpoint:SingleHeating",
                        "ThermostatSetpoint:SingleHeatingOrCooling",
                    ],
                )
        }
        ("ZoneControl:Thermostat", "control_2_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "control_3_object_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "ThermostatSetpoint:DualSetpoint",
                        "ThermostatSetpoint:SingleCooling",
                        "ThermostatSetpoint:SingleHeating",
                        "ThermostatSetpoint:SingleHeatingOrCooling",
                    ],
                )
        }
        ("ZoneControl:Thermostat", "control_3_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "control_4_object_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "ThermostatSetpoint:DualSetpoint",
                        "ThermostatSetpoint:SingleCooling",
                        "ThermostatSetpoint:SingleHeating",
                        "ThermostatSetpoint:SingleHeatingOrCooling",
                    ],
                )
        }
        ("ZoneControl:Thermostat", "control_4_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneControl:Thermostat", "temperature_difference_between_cutout_and_setpoint") => {
            blank(value) || number(value).is_some_and(|n| n >= 0.0)
        }
        ("ZoneHVAC:EquipmentConnections", "zone_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:EquipmentConnections", "zone_conditioning_equipment_list_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:EquipmentConnections", "zone_air_inlet_node_or_nodelist_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:EquipmentConnections", "zone_air_exhaust_node_or_nodelist_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:EquipmentConnections", "zone_air_node_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:EquipmentConnections", "zone_return_air_node_or_nodelist_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        (
            "ZoneHVAC:EquipmentConnections",
            "zone_return_air_node_1_flow_rate_fraction_schedule_name",
        ) => blank(value) || matches!(value, RawValue::String(_)),
        (
            "ZoneHVAC:EquipmentConnections",
            "zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name",
        ) => blank(value) || matches!(value, RawValue::String(_)),
        ("ZoneHVAC:EquipmentList", "load_distribution_scheme") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "SequentialLoad",
                        "SequentialUniformPLR",
                        "UniformLoad",
                        "UniformPLR",
                    ],
                )
        }
        ("ZoneHVAC:EquipmentList", "equipment") => {
            blank(value)
                || matches!(value, RawValue::Array(items) if items.iter().all(|item| matches!(item, RawValue::Object(fields) if fields.iter().all(|(name, value)| match name.0.as_str() { "zone_equipment_object_type" => blank(value) || text_is(value, &["AirLoopHVAC:UnitarySystem", "Fan:ZoneExhaust", "HeatExchanger:AirToAir:FlatPlate", "WaterHeater:HeatPump:PumpedCondenser", "WaterHeater:HeatPump:WrappedCondenser", "ZoneHVAC:AirDistributionUnit", "ZoneHVAC:Baseboard:Convective:Electric", "ZoneHVAC:Baseboard:Convective:Water", "ZoneHVAC:Baseboard:RadiantConvective:Electric", "ZoneHVAC:Baseboard:RadiantConvective:Steam", "ZoneHVAC:Baseboard:RadiantConvective:Water", "ZoneHVAC:CoolingPanel:RadiantConvective:Water", "ZoneHVAC:Dehumidifier:DX", "ZoneHVAC:EnergyRecoveryVentilator", "ZoneHVAC:EvaporativeCoolerUnit", "ZoneHVAC:ForcedAir:UserDefined", "ZoneHVAC:FourPipeFanCoil", "ZoneHVAC:HighTemperatureRadiant", "ZoneHVAC:HybridUnitaryHVAC", "ZoneHVAC:IdealLoadsAirSystem", "ZoneHVAC:LowTemperatureRadiant:ConstantFlow", "ZoneHVAC:LowTemperatureRadiant:Electric", "ZoneHVAC:LowTemperatureRadiant:VariableFlow", "ZoneHVAC:OutdoorAirUnit", "ZoneHVAC:PackagedTerminalAirConditioner", "ZoneHVAC:PackagedTerminalHeatPump", "ZoneHVAC:RefrigerationChillerSet", "ZoneHVAC:TerminalUnit:VariableRefrigerantFlow", "ZoneHVAC:UnitHeater", "ZoneHVAC:UnitVentilator", "ZoneHVAC:VentilatedSlab", "ZoneHVAC:WaterToAirHeatPump", "ZoneHVAC:WindowAirConditioner"]), "zone_equipment_name" => blank(value) || matches!(value, RawValue::String(_)), "zone_equipment_cooling_sequence" => blank(value) || number(value).is_some_and(|n| n.fract() == 0.0 && n >= 0.0), "zone_equipment_heating_or_no_load_sequence" => blank(value) || number(value).is_some_and(|n| n.fract() == 0.0 && n >= 0.0), "zone_equipment_sequential_cooling_fraction_schedule_name" => blank(value) || matches!(value, RawValue::String(_)), "zone_equipment_sequential_heating_fraction_schedule_name" => blank(value) || matches!(value, RawValue::String(_)), _ => false, }) && fields.get(&FieldName("zone_equipment_cooling_sequence".into())).is_some_and(|value| !blank(value)) && fields.get(&FieldName("zone_equipment_heating_or_no_load_sequence".into())).is_some_and(|value| !blank(value)) && fields.get(&FieldName("zone_equipment_name".into())).is_some_and(|value| !blank(value)) && fields.get(&FieldName("zone_equipment_object_type".into())).is_some_and(|value| !blank(value)))))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "availability_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "zone_supply_air_node_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "zone_exhaust_air_node_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "system_inlet_air_node_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "maximum_heating_supply_air_temperature") => {
            blank(value) || number(value).is_some_and(|n| n > 0.0 && n < 100.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "minimum_cooling_supply_air_temperature") => {
            blank(value) || number(value).is_some_and(|n| n > -100.0 && n < 50.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "maximum_heating_supply_air_humidity_ratio") => {
            blank(value) || number(value).is_some_and(|n| n > 0.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "minimum_cooling_supply_air_humidity_ratio") => {
            blank(value) || number(value).is_some_and(|n| n > 0.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "heating_limit") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "LimitCapacity",
                        "LimitFlowRate",
                        "LimitFlowRateAndCapacity",
                        "NoLimit",
                    ],
                )
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "maximum_heating_air_flow_rate") => {
            blank(value)
                || (number(value).is_some_and(|n| n >= 0.0) || text_is(value, &["Autosize"]))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "maximum_sensible_heating_capacity") => {
            blank(value)
                || (number(value).is_some_and(|n| n >= 0.0) || text_is(value, &["Autosize"]))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "cooling_limit") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "LimitCapacity",
                        "LimitFlowRate",
                        "LimitFlowRateAndCapacity",
                        "NoLimit",
                    ],
                )
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "maximum_cooling_air_flow_rate") => {
            blank(value)
                || (number(value).is_some_and(|n| n >= 0.0) || text_is(value, &["Autosize"]))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "maximum_total_cooling_capacity") => {
            blank(value)
                || (number(value).is_some_and(|n| n >= 0.0) || text_is(value, &["Autosize"]))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "heating_availability_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "cooling_availability_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "dehumidification_control_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "ConstantSensibleHeatRatio",
                        "ConstantSupplyHumidityRatio",
                        "Humidistat",
                        "None",
                    ],
                )
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "cooling_sensible_heat_ratio") => {
            blank(value) || number(value).is_some_and(|n| n <= 1.0 && n > 0.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "humidification_control_type") => {
            blank(value)
                || text_is(
                    value,
                    &["", "ConstantSupplyHumidityRatio", "Humidistat", "None"],
                )
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "design_specification_outdoor_air_object_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "outdoor_air_inlet_node_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "demand_controlled_ventilation_type") => {
            blank(value) || text_is(value, &["", "CO2Setpoint", "None", "OccupancySchedule"])
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "outdoor_air_economizer_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "DifferentialDryBulb",
                        "DifferentialEnthalpy",
                        "NoEconomizer",
                    ],
                )
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "heat_recovery_type") => {
            blank(value) || text_is(value, &["", "Enthalpy", "None", "Sensible"])
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "sensible_heat_recovery_effectiveness") => {
            blank(value) || number(value).is_some_and(|n| n >= 0.0 && n <= 1.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "latent_heat_recovery_effectiveness") => {
            blank(value) || number(value).is_some_and(|n| n >= 0.0 && n <= 1.0)
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "design_specification_zonehvac_sizing_object_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "heating_fuel_efficiency_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "heating_fuel_type") => {
            // Exact empty is legal; null and whitespace-only are not fuel enums.
            text_is(
                value,
                &[
                    "",
                    "Coal",
                    "Diesel",
                    "DistrictCooling",
                    "DistrictHeatingSteam",
                    "DistrictHeatingWater",
                    "Electricity",
                    "FuelOilNo1",
                    "FuelOilNo2",
                    "Gasoline",
                    "NaturalGas",
                    "OtherFuel1",
                    "OtherFuel2",
                    "Propane",
                ],
            )
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "cooling_fuel_efficiency_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("ZoneHVAC:IdealLoadsAirSystem", "cooling_fuel_type") => {
            // Exact empty is legal; null and whitespace-only are not fuel enums.
            text_is(
                value,
                &[
                    "",
                    "Coal",
                    "Diesel",
                    "DistrictCooling",
                    "DistrictHeatingSteam",
                    "DistrictHeatingWater",
                    "Electricity",
                    "FuelOilNo1",
                    "FuelOilNo2",
                    "Gasoline",
                    "NaturalGas",
                    "OtherFuel1",
                    "OtherFuel2",
                    "Propane",
                ],
            )
        }
        _ => false,
    }
}
pub(super) fn required_fields(family: &str) -> &'static [&'static str] {
    if is_analytic_curve_family(family) {
        return analytic_curve_required_fields(family);
    }
    match family {
        "SetpointManager:Scheduled" => &[
            "control_variable",
            "schedule_name",
            "setpoint_node_or_nodelist_name",
        ],
        "Output:Schedules" => &["key_field"],
        "Schedule:Week:Daily" => &REGULAR_DAY_FIELDS,
        "Schedule:Day:Interval" | "Schedule:Week:Compact" => &["data"],
        "Schedule:Day:List" => &["minutes_per_item", "extensions"],
        "Schedule:Year" => &["schedule_weeks"],
        "Schedule:Compact" => &[],
        "Schedule:File" => &["file_name", "column_number", "rows_to_skip_at_top"],
        "ThermostatSetpoint:DualSetpoint" => &[],
        "ZoneControl:Humidistat" => &[
            "zone_name",
            "humidifying_relative_humidity_setpoint_schedule_name",
        ],
        "ZoneControl:Thermostat" => &[
            "zone_or_zonelist_name",
            "control_type_schedule_name",
            "control_1_object_type",
            "control_1_name",
        ],
        "ZoneHVAC:EquipmentConnections" => &[
            "zone_name",
            "zone_conditioning_equipment_list_name",
            "zone_air_node_name",
        ],
        "ZoneHVAC:EquipmentList" => &[],
        "ZoneHVAC:IdealLoadsAirSystem" => &["zone_supply_air_node_name"],
        _ => &[],
    }
}
pub(super) fn has_source_detailed_reference(raw: &RawModel, reference: &str) -> bool {
    ["Schedule:Compact", "Schedule:Year", "Schedule:File"]
        .iter()
        .any(|family| {
            raw.objects
                .get(&ObjectType((*family).into()))
                .is_some_and(|objects| {
                    objects
                        .keys()
                        .any(|name| name.0.eq_ignore_ascii_case(reference))
                })
        })
}

const REGULAR_DAY_FIELDS: [&str; 12] = [
    "sunday_schedule_day_name",
    "monday_schedule_day_name",
    "tuesday_schedule_day_name",
    "wednesday_schedule_day_name",
    "thursday_schedule_day_name",
    "friday_schedule_day_name",
    "saturday_schedule_day_name",
    "holiday_schedule_day_name",
    "summerdesignday_schedule_day_name",
    "winterdesignday_schedule_day_name",
    "customday1_schedule_day_name",
    "customday2_schedule_day_name",
];

/// Admission validates actual raw schedule identity, not a typed empty graph.
/// Resolution/current values and node connections remain the production getters.
pub(super) fn validate_scheduled_setpoint_references(raw: &RawModel) -> Result<(), String> {
    let Some(objects) = raw
        .objects
        .get(&ObjectType("SetpointManager:Scheduled".into()))
    else {
        return Ok(());
    };
    // Original IDF reaches the same getter only through the actual staged
    // declaration-order producer, never a lexical-order or typed-ID guess.
    if !objects.is_empty()
        && !raw.is_epjson()
        && !raw.has_idf_declaration_order("SetpointManager:Scheduled")
    {
        return Err("Scheduled IDF requires its actual recovered declaration order".into());
    }
    for object in objects.values() {
        let Some(RawValue::String(schedule)) =
            object.fields.get(&FieldName("schedule_name".into()))
        else {
            return Err("Scheduled setpoint requires actual schedule_name string".into());
        };
        // Native GetSchedule includes these two genuine built-in Constant owners.
        let builtin = ["Constant-0.0", "Constant-1.0"]
            .iter()
            .any(|name| schedule.eq_ignore_ascii_case(name));
        let constant = raw
            .objects
            .get(&ObjectType("Schedule:Constant".into()))
            .is_some_and(|rows| {
                rows.keys()
                    .any(|name| name.0.eq_ignore_ascii_case(schedule))
            });
        if !builtin && !constant && !has_source_detailed_reference(raw, schedule) {
            return Err("Scheduled setpoint schedule reference is outside the actual supported schedule owners".into());
        }
        // No gain-style nonnegative constraint: temperature schedules may be negative.
        // No preview value or guessed NodeId is stored by admission.
    }
    Ok(())
}
