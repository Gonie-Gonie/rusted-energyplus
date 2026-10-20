use super::*;
use crate::{
    PartialRunPolicy, RunMode, RunOutputFormat, SupportStatus, TraceLevel, assess_support,
};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

fn input(scope: PortingScope) -> Value {
    let mut value = json!({
        "Version":{"Version":{"version_identifier":"26.1"}},
        "Building":{"Building":{}},
        "SimulationControl":{"Control":{"run_simulation_for_sizing_periods":"No"}},
        "GlobalGeometryRules":{"Geometry":{"starting_vertex_position":"UpperLeftCorner","vertex_entry_direction":"Counterclockwise","coordinate_system":"World"}},
        "RunPeriod":{"Period":{"begin_month":1,"begin_day_of_month":1,"begin_year":2013,"end_month":1,"end_day_of_month":1,"end_year":2013}},
        "Zone":{"Zone":{"volume":1,"floor_area":1}},
        "Material:NoMass":{"Opaque":{"roughness":"MediumRough","thermal_resistance":2}},
        "Construction":{"Opaque":{"outside_layer":"Opaque"}},
        "Schedule:Constant":{"On":{"hourly_value":1},"Type":{"hourly_value":4},"Heat":{"hourly_value":19},"Cool":{"hourly_value":25}},
    });
    let cube = [
        (
            "Floor",
            "Floor",
            [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]],
        ),
        ("Roof", "Roof", [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]]),
        ("West", "Wall", [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]]),
        ("East", "Wall", [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]]),
        (
            "South",
            "Wall",
            [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]],
        ),
        (
            "North",
            "Wall",
            [[0, 1, 0], [0, 1, 1], [1, 1, 1], [1, 1, 0]],
        ),
    ];
    value["BuildingSurface:Detailed"] = json!(cube.into_iter().map(|(name, kind, points)| (name.to_string(),json!({
        "surface_type":kind,"construction_name":"Opaque","zone_name":"Zone",
        "outside_boundary_condition":"Outdoors","sun_exposure":"NoSun","wind_exposure":"NoWind",
        "vertices":points.into_iter().map(|point|json!({"vertex_x_coordinate":point[0],"vertex_y_coordinate":point[1],"vertex_z_coordinate":point[2]})).collect::<Vec<_>>(),
    }))).collect::<BTreeMap<_,_>>());
    if scope == PortingScope::B {
        value["ThermostatSetpoint:DualSetpoint"] = json!({"Dual":{"heating_setpoint_temperature_schedule_name":"Heat","cooling_setpoint_temperature_schedule_name":"Cool"}});
        value["ZoneControl:Thermostat"] = json!({"Thermostat":{"zone_or_zonelist_name":"Zone","control_type_schedule_name":"Type","control_1_object_type":"ThermostatSetpoint:DualSetpoint","control_1_name":"Dual"}});
        value["ZoneHVAC:IdealLoadsAirSystem"] = json!({"Ideal":{"zone_supply_air_node_name":"Supply","availability_schedule_name":"On","dehumidification_control_type":"None","humidification_control_type":"None","heating_limit":"NoLimit","cooling_limit":"NoLimit"}});
        value["ZoneHVAC:EquipmentList"] = json!({"Equipment":{"load_distribution_scheme":"SequentialLoad","equipment":[{"zone_equipment_object_type":"ZoneHVAC:IdealLoadsAirSystem","zone_equipment_name":"Ideal","zone_equipment_cooling_sequence":1,"zone_equipment_heating_or_no_load_sequence":1}]}});
        value["ZoneHVAC:EquipmentConnections"] = json!({"Connection":{"zone_name":"Zone","zone_conditioning_equipment_list_name":"Equipment","zone_air_inlet_node_or_nodelist_name":"Supply","zone_air_node_name":"Air","zone_return_air_node_or_nodelist_name":"Return"}});
    }
    value
}

fn trace(value: &Value, scope: PortingScope) -> PortingScopeTrace {
    let raw = parse_epjson_str(&value.to_string()).expect("valid epJSON");
    let compiled = compile_raw_model(&raw);
    inspect_porting_scope(&raw, &compiled.report, compiled.model.as_ref(), scope)
}

#[test]
fn timestep_admission_compilation_and_axis_agree_for_the_full_schema_domain() {
    for requested in 1..=60 {
        let mut value = input(PortingScope::A);
        value["Timestep"] = json!({"t":{"number_of_timesteps_per_hour":requested}});
        let raw = parse_epjson_str(&value.to_string()).expect("raw clock input");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("compiled valid clock");
        let trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(trace.admissible, "{requested}: {:?}", trace.violations);
        let actual = crate::ordinary_equipment_input::resolve(&raw).expect("Source raw admission");
        let steps = model.timestep.number_of_timesteps_per_hour;
        assert_eq!(actual, ep_runtime::heat_balance::ordinary_equipment::OrdinaryEquipmentInput::from_family_inventory([], steps as usize).expect("clock inventory"));
        let axes = ep_runtime::time_axis::build_environment_time_axes(model)
            .expect("effective clock axis");
        assert_eq!(axes[0].zone_timestep.timesteps_per_hour, steps);
        assert_eq!(axes[0].sample_count(), 24 * steps as usize);
    }
}

#[test]
fn run_period_year_normalization_reaches_calendar_and_axis_before_source_weather() {
    for (begin, end, weekday, year) in [
        (json!(0), json!(0), Some("Tuesday"), 2013),
        (json!(0.9), json!(-0.9), Some("Tuesday"), 2013),
        (json!(2013.9), json!(2013.9), None, 2013),
        (json!(2013), json!(0), None, 2013),
    ] {
        let mut value = input(PortingScope::A);
        value["RunPeriod"]["Period"]["begin_year"] = begin;
        value["RunPeriod"]["Period"]["end_year"] = end;
        if let Some(weekday) = weekday {
            value["RunPeriod"]["Period"]["day_of_week_for_start_day"] = json!(weekday);
        }
        let raw = parse_epjson_str(&value.to_string()).expect("raw RunPeriod year variants");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("normalized typed RunPeriod");
        let trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(trace.admissible, "{value}: {:?}", trace.violations);
        assert!(
            crate::ordinary_input_processor::resolve(&raw).is_ok(),
            "raw Source admission"
        );
        let axes = ep_runtime::time_axis::build_environment_time_axes(model)
            .expect("normalized date calendar");
        assert_eq!(axes[0].calendar.start_year, year);
        assert_eq!(axes[0].calendar.end_year, year);
        assert_eq!(axes[0].calendar.total_days, 1);
        assert_eq!(
            axes[0].sample_count(),
            24 * model.timestep.number_of_timesteps_per_hour as usize
        );
    }
    for (name, period, begin_year, end_year, days) in [
        (
            "December rollover with fractional start and zero end year",
            json!({"begin_month":12,"begin_day_of_month":31,"begin_year":2013.9,"end_month":1,"end_day_of_month":1,"end_year":0}),
            2013,
            2014,
            2,
        ),
        (
            "February29 zero years select the Original leap default",
            json!({"begin_month":2,"begin_day_of_month":29,"begin_year":0,"end_month":2,"end_day_of_month":29,"end_year":0}),
            2012,
            2012,
            1,
        ),
    ] {
        let mut value = input(PortingScope::A);
        value["RunPeriod"]["Period"] = period;
        let raw = parse_epjson_str(&value.to_string()).expect("raw year/date normalization");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("normalized year/date model");
        let trace = inspect_porting_scope(&raw, &compiled.report, Some(model), PortingScope::A);
        assert!(trace.admissible, "{name}: {:?}", trace.violations);
        assert!(
            crate::ordinary_input_processor::resolve(&raw).is_ok(),
            "{name}"
        );
        let axes =
            ep_runtime::time_axis::build_environment_time_axes(model).expect("year/date calendar");
        assert_eq!(axes[0].calendar.start_year, begin_year, "{name}");
        assert_eq!(axes[0].calendar.end_year, end_year, "{name}");
        assert_eq!(axes[0].calendar.total_days, days, "{name}");
        assert_eq!(
            axes[0].sample_count(),
            days * 24 * model.timestep.number_of_timesteps_per_hour as usize,
            "{name}"
        );
    }
    for (begin, end) in [(1582, 1582), (0, 2013), (2013, 2012)] {
        let mut value = input(PortingScope::A);
        value["RunPeriod"]["Period"]["begin_year"] = json!(begin);
        value["RunPeriod"]["Period"]["end_year"] = json!(end);
        let raw =
            parse_epjson_str(&value.to_string()).expect("raw Source-invalid year relationship");
        let compiled = compile_raw_model(&raw);
        let model = compiled.model.as_ref().expect("typed numeric fields");
        assert!(
            ep_runtime::time_axis::build_environment_time_axes(model).is_err(),
            "Source-invalid calendar must stay blocked"
        );
    }
}

#[test]
fn default_controls_and_source_adjustments_are_observable() {
    let mut value = input(PortingScope::A);
    value["Building"]["Building"]["north_axis"] = json!(-450);
    value["Building"]["Building"]["minimum_number_of_warmup_days"] = json!(30);
    let actual = trace(&value, PortingScope::A);
    assert!(actual.admissible, "{:?}", actual.violations);
    assert_eq!(actual.settings["building"]["north_axis_deg"], -450.0);
    assert_eq!(
        actual.settings["building"]["effective_north_axis_deg"],
        -90.0
    );
    assert_eq!(actual.settings["building"]["maximum_warmup_days"], 30);
    assert_eq!(actual.settings["building"]["minimum_warmup_days"], 30);
    assert_eq!(
        actual.settings["heat_balance_algorithm"],
        "ConductionTransferFunction"
    );
    assert_eq!(
        actual.settings["zone_air_heat_balance_algorithm"],
        "ThirdOrderBackwardDifference"
    );
    assert_eq!(actual.settings["room_air_model_type"], "Mixing");
    assert_eq!(actual.settings["timesteps_per_hour"], 4);
    assert_eq!(actual.production["conformance_claim"], false);
    value["Building"]["Building"]["maximum_number_of_warmup_days"] = json!(0);
    assert!(!trace(&value, PortingScope::A).admissible);
}

#[test]
fn effective_site_atmosphere_uses_the_compiled_terrain_and_no_override() {
    // Pinned GetProjectControlData terrain branches and the absent-object
    // GetSiteAtmosphereData default; these are settings, not height physics.
    for (terrain, exponent, height) in [
        ("Country", 0.14, 270.0),
        ("Suburbs", 0.22, 370.0),
        ("Urban", 0.22, 370.0),
        ("City", 0.33, 460.0),
        ("Ocean", 0.10, 210.0),
    ] {
        let mut value = input(PortingScope::A);
        value["Building"]["Building"]["terrain"] = json!(terrain);
        let actual = trace(&value, PortingScope::A);
        assert!(actual.admissible, "{terrain}: {:?}", actual.violations);
        assert_eq!(
            actual.settings["site_atmosphere"],
            json!({
                "wind_speed_profile_exponent": exponent,
                "wind_speed_profile_boundary_layer_thickness_m": height,
                "air_temperature_gradient_k_per_m": 0.0065,
            })
        );
    }
    let value = input(PortingScope::A);
    assert_eq!(
        trace(&value, PortingScope::A).settings["site_atmosphere"],
        json!({
            "wind_speed_profile_exponent": 0.22,
            "wind_speed_profile_boundary_layer_thickness_m": 370.0,
            "air_temperature_gradient_k_per_m": 0.0065,
        })
    );
    let mut value = value;
    value["Site:HeightVariation"] = json!({"Override":{
        "wind_speed_profile_exponent":0.22,
        "wind_speed_profile_boundary_layer_thickness":370.0,
        "air_temperature_gradient_coefficient":0.0065,
    }});
    let actual = trace(&value, PortingScope::A);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|v| v.contains("Site:HeightVariation"))
    );
}

#[test]
fn scientific_algorithms_are_rejected_before_bounded_execution() {
    for (kind, field, setting, expected) in [
        (
            "HeatBalanceAlgorithm",
            "algorithm",
            "ConductionFiniteDifference",
            "heat_balance_algorithm",
        ),
        (
            "ZoneAirHeatBalanceAlgorithm",
            "algorithm",
            "EulerMethod",
            "zone_air_heat_balance_algorithm",
        ),
        (
            "RoomAirModelType",
            "room_air_modeling_type",
            "UserDefined",
            "room_air_model_type",
        ),
        (
            "SurfaceConvectionAlgorithm:Inside",
            "algorithm",
            "Simple",
            "convection",
        ),
    ] {
        let mut value = input(PortingScope::A);
        value[kind] = json!({"Control":{field:setting}});
        let actual = trace(&value, PortingScope::A);
        assert!(!actual.admissible, "{kind} incorrectly admitted");
        assert!(
            actual
                .violations
                .iter()
                .any(|message| message.contains(expected)),
            "{:?}",
            actual.violations
        );
    }
}

#[test]
fn new_features_and_active_design_days_are_not_silently_ignored() {
    let mut value = input(PortingScope::A);
    value["ZoneInfiltration:DesignFlowRate"] = json!({"Infiltration":{"design_flow_rate":0.01}});
    assert!(!trace(&value, PortingScope::A).admissible);
    let mut value = input(PortingScope::A);
    value["SizingPeriod:DesignDay"] = json!({"Day":{}});
    assert!(trace(&value, PortingScope::A).admissible);
    value["SimulationControl"]["Control"]["run_simulation_for_sizing_periods"] = json!("Yes");
    let actual = trace(&value, PortingScope::A);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|message| message.contains("active sizing-period"))
    );
}

#[test]
fn direct_sensible_limits_are_explicit_and_scope_a_rejects_conditioning() {
    let mut value = input(PortingScope::B);
    assert!(
        trace(&value, PortingScope::B).admissible,
        "{:?}",
        trace(&value, PortingScope::B).violations
    );
    assert!(!trace(&value, PortingScope::A).admissible);
    for side in ["heating", "cooling"] {
        value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][format!("{side}_limit")] =
            json!("LimitFlowRate");
        value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"][format!("maximum_{side}_air_flow_rate")] =
            json!(0.02);
    }
    let actual = trace(&value, PortingScope::B);
    assert!(actual.admissible, "{:?}", actual.violations);
    assert_eq!(
        actual.settings["ideal_loads"][0]["maximum_heating_air_flow_rate_m3_per_s"],
        0.02
    );
    value["ZoneHVAC:IdealLoadsAirSystem"]["Ideal"]["maximum_heating_air_flow_rate"] =
        json!("Autosize");
    let actual = trace(&value, PortingScope::B);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|message| message.contains("Autosize"))
    );
}

#[test]
fn bounded_controls_do_not_change_ordinary_support_assessment() {
    let mut value = input(PortingScope::A);
    value["ZoneAirHeatBalanceAlgorithm"] =
        json!({"Algorithm":{"algorithm":"ThirdOrderBackwardDifference"}});
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    assert!(
        inspect_porting_scope(
            &raw,
            &compiled.report,
            compiled.model.as_ref(),
            PortingScope::A
        )
        .admissible
    );
    let ordinary = assess_support(
        &raw,
        &compiled.report,
        compiled.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Summary,
    );
    assert_eq!(ordinary.status, SupportStatus::Unsupported);
    assert!(
        ordinary
            .unsupported_objects
            .iter()
            .any(|entry| entry.object_type == "ZoneAirHeatBalanceAlgorithm")
    );
}

#[test]
fn latent_sources_are_blocked_inside_the_sensible_only_contract() {
    let mut value = input(PortingScope::A);
    value["OtherEquipment"] = json!({"Gain":{
        "zone_or_zonelist_or_space_or_spacelist_name":"Zone", "schedule_name":"On",
        "design_level_calculation_method":"EquipmentLevel", "design_level":100,
        "fraction_latent":0.0, "fraction_radiant":0.0, "fraction_lost":0.0,
    }});
    assert!(trace(&value, PortingScope::A).admissible);
    value["OtherEquipment"]["Gain"]["fraction_latent"] = json!(0.25);
    let actual = trace(&value, PortingScope::A);
    assert!(!actual.admissible);
    assert!(
        actual
            .violations
            .iter()
            .any(|message| message.contains("latent gains"))
    );
}

#[test]
fn effective_calendar_branches_are_checked_after_weather_resolution() {
    let mut value = input(PortingScope::A);
    value["RunPeriodControl:SpecialDays"] = json!({"Holiday":{
        "start_date":"1/1", "duration":1, "special_day_type":"Holiday",
    }});
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    let mut weather_text = String::from(
        "LOCATION,Example\nDESIGN CONDITIONS\nTYPICAL/EXTREME PERIODS\nGROUND TEMPERATURES\nHOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\nCOMMENTS 1\nCOMMENTS 2\nDATA PERIODS,1,1,Data,Friday,1/1,1/1\n",
    );
    for hour in 1..=24 {
        weather_text.push_str(&format!(
            "1999,1,1,{hour},60,Source,1.0,0.0,50,101325,0,0,300,10,20,30,0,0,0,0,180,2.5\n"
        ));
    }
    let weather =
        ep_runtime::weather::parse_epw_weather_file(&weather_text).expect("weather fixture");
    let actual = super::environment::environment_from_weather(
        compiled.model.as_ref().expect("compiled model"),
        &weather,
        "embedded unit fixture",
    )
    .expect("weather inputs");
    assert_eq!(actual["active_special_day_count"], 1);
    assert!(
        validate_porting_environment(&actual)
            .iter()
            .any(|message| message.contains("special-day"))
    );
    let mut value = input(PortingScope::A);
    value["RunPeriodControl:DaylightSavingTime"] =
        json!({"DST":{"start_date":"1/1","end_date":"1/1"}});
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    let actual = super::environment::environment_from_weather(
        compiled.model.as_ref().expect("compiled model"),
        &weather,
        "embedded unit fixture",
    )
    .expect("weather inputs");
    assert_eq!(actual["daylight_saving"]["active"], true);
    assert!(
        validate_porting_environment(&actual)
            .iter()
            .any(|message| message.contains("daylight-saving"))
    );
}

#[test]
fn weather_trace_uses_real_calendar_selection_and_zone_schedule_values() {
    let value = input(PortingScope::B);
    let raw = parse_epjson_str(&value.to_string()).expect("epJSON");
    let compiled = compile_raw_model(&raw);
    let mut weather_text = String::from(
        "LOCATION,Example\nDESIGN CONDITIONS\nTYPICAL/EXTREME PERIODS\nGROUND TEMPERATURES\nHOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\nCOMMENTS 1\nCOMMENTS 2\nDATA PERIODS,1,1,Data,Friday,1/1,1/1\n",
    );
    for hour in 1..=24 {
        weather_text.push_str(&format!(
            "1999,1,1,{hour},60,Source,1.0,0.0,50,101325,0,0,300,10,20,30,0,0,0,0,180,2.5\n"
        ));
    }
    let weather =
        ep_runtime::weather::parse_epw_weather_file(&weather_text).expect("weather fixture");
    let actual = super::environment::environment_from_weather(
        compiled.model.as_ref().expect("compiled model"),
        &weather,
        "embedded unit fixture",
    )
    .expect("weather input trace");
    assert_eq!(actual["resolved_start_year"], 2013);
    assert_eq!(actual["resolved_start_day_of_week"], "Tuesday");
    assert_eq!(actual["calendar_year"], 2013);
    assert_eq!(actual["weather_record_year"], 1999);
    assert_eq!(actual["hourly_samples"], 24);
    assert_eq!(actual["zone_timestep_samples"], 96);
    assert_eq!(actual["daylight_saving"]["active"], false);
    let schedules = actual["schedule_values"].as_array().expect("schedules");
    let thermostat = schedules
        .iter()
        .find(|schedule| schedule["name"] == "TYPE")
        .expect("thermostat schedule");
    assert!(
        thermostat["values"]
            .as_array()
            .expect("values")
            .iter()
            .all(|value| value == &json!(4.0))
    );
}
