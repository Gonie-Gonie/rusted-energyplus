//! Finite raw field grammar for the pinned Source A/B families.
//! Retains input operands for their actual Source getters and owners.
use super::{blank, number, text_is};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
pub(super) fn is_family(family: &str) -> bool {
    matches!(
        family,
        "NodeList"
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
            | "ZoneHVAC:EquipmentConnections"
            | "ZoneHVAC:EquipmentList"
            | "ZoneHVAC:IdealLoadsAirSystem"
    )
}
// Retain the ordered raw input-bound predicates without changing admission logic.
#[allow(clippy::manual_range_contains)]
pub(super) fn admitted_field(family: &str, name: &str, value: &RawValue) -> bool {
    if matches!(
        name,
        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
    ) {
        return matches!(value, RawValue::Number(text) if text.parse::<usize>().is_ok());
    }
    match (family, name) {
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
        ("NodeList", "nodes") => {
            blank(value)
                || matches!(value, RawValue::Array(items) if items.iter().all(|item| matches!(item, RawValue::Object(fields) if fields.iter().all(|(name, value)| match name.0.as_str() { "node_name" => blank(value) || matches!(value, RawValue::String(_)), _ => false, }) && fields.get(&FieldName("node_name".into())).is_some_and(|value| !blank(value)))))
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
            blank(value)
                || text_is(
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
            blank(value)
                || text_is(
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
    match family {
        "Output:Schedules" => &["key_field"],
        "Schedule:Week:Daily" => &REGULAR_DAY_FIELDS,
        "Schedule:Day:Interval" | "Schedule:Week:Compact" => &["data"],
        "Schedule:Day:List" => &["minutes_per_item", "extensions"],
        "Schedule:Year" => &["schedule_weeks"],
        "NodeList" => &[],
        "Schedule:Compact" => &[],
        "Schedule:File" => &["file_name", "column_number", "rows_to_skip_at_top"],
        "ThermostatSetpoint:DualSetpoint" => &[],
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
