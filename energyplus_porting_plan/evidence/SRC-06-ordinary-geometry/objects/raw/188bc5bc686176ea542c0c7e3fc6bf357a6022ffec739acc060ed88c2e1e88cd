//! Compiled project controls and branch settings for bounded admission traces.

use ep_model::{
    AutosizeOrNumber, InsideSurfaceConvectionAlgorithm, OutsideSurfaceConvectionAlgorithm, Terrain,
    TypedModel,
};
use ep_raw_model::{FieldName, RawModel, RawObject, RawValue};
use serde_json::{Value, json};

fn object<'a>(raw: &'a RawModel, kind: &str) -> Option<&'a RawObject> {
    raw.objects
        .iter()
        .find(|(key, _)| key.0 == kind)
        .and_then(|(_, values)| values.values().next())
}

fn text(
    raw: &RawModel,
    kind: &str,
    field: &str,
    default: &str,
    errors: &mut Vec<String>,
) -> String {
    match object(raw, kind).and_then(|value| value.fields.get(&FieldName(field.into()))) {
        None | Some(RawValue::Null) => default.into(),
        Some(RawValue::String(value)) if value.trim().is_empty() => default.into(),
        Some(RawValue::String(value)) => value.trim().into(),
        _ => {
            errors.push(format!("{kind}.{field} must be a string"));
            default.into()
        }
    }
}

fn number(raw: &RawModel, kind: &str, field: &str, default: f64, errors: &mut Vec<String>) -> f64 {
    match object(raw, kind).and_then(|value| value.fields.get(&FieldName(field.into()))) {
        None | Some(RawValue::Null) => default,
        Some(RawValue::String(value)) if value.trim().is_empty() => default,
        Some(RawValue::Number(value)) => match value.parse::<f64>() {
            Ok(value) if value.is_finite() => value,
            _ => {
                errors.push(format!("{kind}.{field} must be a finite number"));
                default
            }
        },
        _ => {
            errors.push(format!("{kind}.{field} must be a number"));
            default
        }
    }
}

fn yes_no(
    raw: &RawModel,
    kind: &str,
    field: &str,
    default: &str,
    errors: &mut Vec<String>,
) -> bool {
    let value = text(raw, kind, field, default, errors);
    if value.eq_ignore_ascii_case("Yes") {
        true
    } else if value.eq_ignore_ascii_case("No") {
        false
    } else {
        errors.push(format!("{kind}.{field} must be Yes or No"));
        default == "Yes"
    }
}

fn limit(value: Option<AutosizeOrNumber>) -> Value {
    match value {
        Some(AutosizeOrNumber::Value(value)) => json!(value),
        Some(AutosizeOrNumber::Autosize) => json!("Autosize"),
        None => Value::Null,
    }
}

pub(super) fn project_settings(
    raw: &RawModel,
    model: Option<&TypedModel>,
    errors: &mut Vec<String>,
) -> Value {
    let mut simulation = serde_json::Map::new();
    for (field, default) in [
        ("do_zone_sizing_calculation", "No"),
        ("do_system_sizing_calculation", "No"),
        ("do_plant_sizing_calculation", "No"),
        ("run_simulation_for_sizing_periods", "Yes"),
        ("run_simulation_for_weather_file_run_periods", "Yes"),
        ("do_hvac_sizing_simulation_for_sizing_periods", "No"),
    ] {
        simulation.insert(
            field.into(),
            json!(yes_no(raw, "SimulationControl", field, default, errors)),
        );
    }
    let passes = number(
        raw,
        "SimulationControl",
        "maximum_number_of_hvac_sizing_simulation_passes",
        1.0,
        errors,
    );
    if passes < 1.0 || passes.fract() != 0.0 {
        errors.push("SimulationControl.maximum_number_of_hvac_sizing_simulation_passes must be a positive integer".into());
    }
    simulation.insert(
        "maximum_number_of_hvac_sizing_simulation_passes".into(),
        json!(passes),
    );
    let heat = text(
        raw,
        "HeatBalanceAlgorithm",
        "algorithm",
        "ConductionTransferFunction",
        errors,
    );
    let heat = if heat.eq_ignore_ascii_case("CTF")
        || heat.eq_ignore_ascii_case("ConductionTransferFunction")
    {
        "ConductionTransferFunction".into()
    } else {
        heat
    };
    let air = text(
        raw,
        "ZoneAirHeatBalanceAlgorithm",
        "algorithm",
        "ThirdOrderBackwardDifference",
        errors,
    );
    let air = if air.eq_ignore_ascii_case("ThirdOrderBackwardDifference") {
        "ThirdOrderBackwardDifference".into()
    } else {
        air
    };
    let room = text(
        raw,
        "RoomAirModelType",
        "room_air_modeling_type",
        "Mixing",
        errors,
    );
    let room = if room.eq_ignore_ascii_case("Mixing") {
        "Mixing".into()
    } else {
        room
    };
    let mut heat_controls = serde_json::Map::new();
    for (field, default) in [
        ("surface_temperature_upper_limit", 200.0),
        (
            "minimum_surface_convection_heat_transfer_coefficient_value",
            0.1,
        ),
        (
            "maximum_surface_convection_heat_transfer_coefficient_value",
            1000.0,
        ),
    ] {
        heat_controls.insert(
            field.into(),
            json!(number(raw, "HeatBalanceAlgorithm", field, default, errors)),
        );
    }
    let sizing = yes_no(
        raw,
        "ZoneAirHeatBalanceAlgorithm",
        "do_space_heat_balance_for_sizing",
        "No",
        errors,
    );
    let simulation_spaces = yes_no(
        raw,
        "ZoneAirHeatBalanceAlgorithm",
        "do_space_heat_balance_for_simulation",
        "No",
        errors,
    );
    let mut settings = json!({
        "simulation_control": simulation, "heat_balance_algorithm": heat,
        "zone_air_heat_balance_algorithm": air, "room_air_model_type": room,
        "room_air_controls": {
            "zone_name": text(raw, "RoomAirModelType", "zone_name", "", errors),
            "air_temperature_coupling_strategy": text(raw, "RoomAirModelType", "air_temperature_coupling_strategy", "Direct", errors),
        },
        "heat_balance_controls": heat_controls,
        "zone_air_controls": {"do_space_heat_balance_for_sizing": sizing, "do_space_heat_balance_for_simulation": simulation_spaces},
    });
    let Some(model) = model else {
        return settings;
    };
    for (field, default) in [
        ("maximum_number_of_warmup_days", 25.0),
        ("minimum_number_of_warmup_days", 1.0),
    ] {
        if number(raw, "Building", field, default, errors) <= 0.0 {
            errors.push(format!("Building.{field} must be positive; source error fallback is not an admissible input"));
        }
    }
    settings["timesteps_per_hour"] = json!(model.timestep.number_of_timesteps_per_hour);
    settings["building"] = model.building.as_ref().map_or(Value::Null, |building| {
        json!({
            "north_axis_deg": number(raw, "Building", "north_axis", 0.0, errors),
            "effective_north_axis_deg": building.north_axis_deg,
            "terrain": format!("{:?}", building.terrain),
            "loads_convergence_tolerance": building.loads_convergence_tolerance_w,
            "temperature_convergence_tolerance": building.temperature_convergence_tolerance_delta_c,
            "solar_distribution": format!("{:?}", building.solar_distribution),
            "maximum_warmup_days": building.maximum_number_of_warmup_days,
            "minimum_warmup_days": building.minimum_number_of_warmup_days,
        })
    });
    settings["site_atmosphere"] = model.building.as_ref().map_or(Value::Null, |building| {
        // GetProjectControlData:566-585 derives these values from Terrain.
        // GetSiteAtmosphereData:1302-1308 retains them and sets the gradient
        // when Site:HeightVariation is absent, as required by this scope.
        let (exponent, height) = match building.terrain {
            Terrain::Country => (0.14, 270.0),
            Terrain::Suburbs | Terrain::Urban => (0.22, 370.0),
            Terrain::City => (0.33, 460.0),
            Terrain::Ocean => (0.10, 210.0),
        };
        json!({
            "wind_speed_profile_exponent": exponent,
            "wind_speed_profile_boundary_layer_thickness_m": height,
            "air_temperature_gradient_k_per_m": 0.0065,
        })
    });
    if let Some(zone) = model.zones.first() {
        settings["inside_convection"] =
            json!(inside_name(zone.inside_convection_algorithm.effective()));
        settings["outside_convection"] =
            json!(outside_name(zone.outside_convection_algorithm.effective()));
    }
    settings["run_periods"] = json!(model.run_periods.iter().map(|period| json!({
        "name": period.name.0, "begin_month": period.begin_month, "begin_day_of_month": period.begin_day_of_month,
        "begin_year": period.begin_year, "end_month": period.end_month, "end_day_of_month": period.end_day_of_month,
        "end_year": period.end_year,
        "day_of_week_for_start_day": period.day_of_week_for_start_day.map(|day| format!("{day:?}")),
        "first_hour_interpolation_starting_values": format!("{:?}", period.first_hour_interpolation_starting_values),
        "use_weather_file_holidays_and_special_days": period.use_weather_file_holidays_and_special_days,
        "use_weather_file_daylight_saving_period": period.use_weather_file_daylight_saving_period,
        "apply_weekend_holiday_rule": period.apply_weekend_holiday_rule,
        "use_weather_file_rain_indicators": period.use_weather_file_rain_indicators,
        "use_weather_file_snow_indicators": period.use_weather_file_snow_indicators,
        "treat_weather_as_actual": period.treat_weather_as_actual,
    })).collect::<Vec<_>>());
    settings["geometry"] = json!(model.global_geometry_rules.map(|rules| json!({
        "coordinate_system": format!("{:?}", rules.coordinate_system),
        "vertex_entry_direction": format!("{:?}", rules.vertex_entry_direction),
        "starting_vertex_position": format!("{:?}", rules.starting_vertex_position),
    })));
    settings["schedules"] = json!({
        "constant": model.schedules.iter().map(|schedule| json!({"id":schedule.id.0,"name":schedule.name.0,"value":schedule.hourly_value})).collect::<Vec<_>>(),
        "compact": model.compact_schedules.iter().map(|schedule| json!({"id":schedule.id.0,"name":schedule.name.0,"period_count":schedule.periods.len()})).collect::<Vec<_>>(),
    });
    settings["ideal_loads"] = json!(model.ideal_loads_air_systems.iter().map(|system| json!({
        "name": system.name.0, "heating_limit": format!("{:?}",system.heating_limit),
        "cooling_limit": format!("{:?}",system.cooling_limit),
        "maximum_heating_air_flow_rate_m3_per_s": limit(system.maximum_heating_air_flow_rate_m3_per_s),
        "maximum_cooling_air_flow_rate_m3_per_s": limit(system.maximum_cooling_air_flow_rate_m3_per_s),
        "maximum_sensible_heating_capacity_w": limit(system.maximum_sensible_heating_capacity_w),
        "maximum_total_cooling_capacity_w": limit(system.maximum_total_cooling_capacity_w),
        "maximum_heating_supply_air_temperature_c": system.maximum_heating_supply_air_temperature_c,
        "minimum_cooling_supply_air_temperature_c": system.minimum_cooling_supply_air_temperature_c,
        "availability_schedule_id": system.availability_schedule.map(|id| id.0),
        "heating_availability_schedule_id": system.heating_availability_schedule.map(|id| id.0),
        "cooling_availability_schedule_id": system.cooling_availability_schedule.map(|id| id.0),
        "supply_air_node": system.zone_supply_air_node_name.0,
        "system_inlet_air_node": system.system_inlet_air_node_name.as_ref().map(|name| &name.0),
        "dehumidification_control_type": format!("{:?}", system.dehumidification_control_type),
        "humidification_control_type": format!("{:?}", system.humidification_control_type),
        "outdoor_air_specification": system.design_specification_outdoor_air_object_name.as_ref().map(|name| &name.0),
        "branch": ep_runtime::select_purchased_air_branch(system).label(),
    })).collect::<Vec<_>>());
    settings
}

fn inside_name(value: InsideSurfaceConvectionAlgorithm) -> &'static str {
    match value {
        InsideSurfaceConvectionAlgorithm::Tarp => "TARP",
        InsideSurfaceConvectionAlgorithm::Simple => "Simple",
        InsideSurfaceConvectionAlgorithm::CeilingDiffuser => "CeilingDiffuser",
        InsideSurfaceConvectionAlgorithm::TrombeWall => "TrombeWall",
        InsideSurfaceConvectionAlgorithm::AdaptiveConvectionAlgorithm => {
            "AdaptiveConvectionAlgorithm"
        }
        InsideSurfaceConvectionAlgorithm::AstmC1340 => "ASTMC1340",
    }
}

fn outside_name(value: OutsideSurfaceConvectionAlgorithm) -> &'static str {
    match value {
        OutsideSurfaceConvectionAlgorithm::Doe2 => "DOE-2",
        OutsideSurfaceConvectionAlgorithm::Tarp => "TARP",
        OutsideSurfaceConvectionAlgorithm::SimpleCombined => "SimpleCombined",
        OutsideSurfaceConvectionAlgorithm::MoWitt => "MoWiTT",
        OutsideSurfaceConvectionAlgorithm::AdaptiveConvectionAlgorithm => {
            "AdaptiveConvectionAlgorithm"
        }
    }
}
