//! Consumed by the existing raw-admission table; no additional test or weather math.

use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
    SiteLocation,
};
use ep_raw_model::{IdfOrderTarget, ObjectType, apply_idf_declaration_order, parse_epjson_str};
use ep_runtime::time_axis::build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps;
use ep_runtime::weather::{
    EpwCalendarMetadata,
    day::{
        DesignDayHumidity, DesignDayInput, DesignDayParameters, WeatherEnvironmentConfiguration,
    },
};

type TestResult = Result<(), Box<dyn std::error::Error>>;
const HEATING: &str = "Denver Centennial  Golden   N Ann Htg 99% Condns DB";
const COOLING: &str = "Denver Centennial  Golden   N Ann Clg 1% Condns DB=>MWB";

pub(super) fn verify_admitted_design_day_projection() -> TestResult {
    // Literal A-24H getter inputs. No calculated Native weather values enter this table.
    let document = serde_json::json!({
        "Version":{"v":{"version_identifier":"26.1"}},
        "Building":{"b":{"solar_distribution":"MinimalShadowing"}},
        "Timestep":{"t":{"number_of_timesteps_per_hour":4}},
        "Zone":{"Zone One":{}},
        "SimulationControl":{"s":{
            "do_zone_sizing_calculation":"No", "do_system_sizing_calculation":"No",
            "do_plant_sizing_calculation":"No", "run_simulation_for_sizing_periods":"No",
            "run_simulation_for_weather_file_run_periods":"Yes",
            "do_hvac_sizing_simulation_for_sizing_periods":"No"
        }},
        "SizingPeriod:DesignDay":{
            "Denver Centennial  Golden   N Ann Htg 99% Condns DB":{
                "month":12,"day_of_month":21,"day_type":"WinterDesignDay",
                "maximum_dry_bulb_temperature":-15.5,"daily_dry_bulb_temperature_range":0.0,
                "humidity_condition_type":"WetBulb","wetbulb_or_dewpoint_at_maximum_dry_bulb":-15.5,
                "barometric_pressure":81198.0,"wind_speed":3.0,"wind_direction":340.0,
                "rain_indicator":"No","snow_indicator":"No","daylight_saving_time_indicator":"No",
                "solar_model_indicator":"ASHRAEClearSky","sky_clearness":0.0
            },
            "Denver Centennial  Golden   N Ann Clg 1% Condns DB=>MWB":{
                "month":7,"day_of_month":21,"day_type":"SummerDesignDay",
                "maximum_dry_bulb_temperature":32.0,"daily_dry_bulb_temperature_range":15.2,
                "humidity_condition_type":"WetBulb","wetbulb_or_dewpoint_at_maximum_dry_bulb":15.5,
                "barometric_pressure":81198.0,"wind_speed":4.9,"wind_direction":0.0,
                "rain_indicator":"No","snow_indicator":"No","daylight_saving_time_indicator":"No",
                "solar_model_indicator":"ASHRAEClearSky","sky_clearness":1.0
            }
        }
    });
    let lexical = parse_epjson_str(&serde_json::to_string(&document)?)?;
    crate::ordinary_equipment_input::resolve(&lexical)?;
    let expected = expected_days()?;
    let mut config = configuration(2)?;
    crate::ordinary_design_day_input::project(&lexical, &mut config, false)?;
    assert_eq!(
        config.design_days,
        [expected[1].clone(), expected[0].clone()]
    );
    assert!(!config.reverse_design_days);
    assert_controls(&config, [false, false, false, false, false, true])?;

    let mut ordered = lexical.clone();
    apply_idf_declaration_order(
        &mut ordered,
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/validation/cases/A-24H/input.idf"
        )),
        &[IdfOrderTarget {
            object_type: "SizingPeriod:DesignDay",
            name_field_index: 0,
        }],
    )?;
    crate::ordinary_equipment_input::resolve(&ordered)?;
    crate::ordinary_design_day_input::project(&ordered, &mut config, false)?;
    assert_eq!(config.design_days, expected);
    crate::ordinary_design_day_input::project(&ordered, &mut config, true)?;
    assert_eq!(
        config.design_days,
        [expected[1].clone(), expected[0].clone()]
    );
    assert!(config.reverse_design_days);
    crate::ordinary_design_day_input::project(&lexical, &mut config, true)?;
    assert_eq!(config.design_days, expected);

    // Missing SimulationControl follows its real six defaults; true inputs are
    // retained for the actual runtime's sizing admission, not silently cleared.
    let mut absent_controls = lexical.clone();
    absent_controls
        .objects
        .remove(&ObjectType("SimulationControl".into()));
    crate::ordinary_equipment_input::resolve(&absent_controls)?;
    crate::ordinary_design_day_input::project(&absent_controls, &mut config, false)?;
    assert_controls(&config, [false, false, false, false, true, true])?;
    let mut sizing_document = document.clone();
    sizing_document["SimulationControl"]["s"]["do_zone_sizing_calculation"] = "Yes".into();
    let sizing = parse_epjson_str(&serde_json::to_string(&sizing_document)?)?;
    crate::ordinary_equipment_input::resolve(&sizing)?;
    crate::ordinary_design_day_input::project(&sizing, &mut config, false)?;
    assert_controls(&config, [true, false, false, false, false, true])?;

    // Failed projection cannot replace prior successful input owners.
    let mut mismatched = configuration(1)?;
    let before_days = mismatched.design_days.clone();
    assert!(crate::ordinary_design_day_input::project(&lexical, &mut mismatched, false).is_err());
    assert_eq!(mismatched.design_days, before_days);
    assert!(mismatched.setup_simulation_controls.is_none());
    let mut one = lexical.clone();
    one.objects
        .get_mut(&ObjectType("SizingPeriod:DesignDay".into()))
        .ok_or("DD family")?
        .retain(|name, _| name.0 == HEATING);
    crate::ordinary_equipment_input::resolve(&one)?;
    assert!(crate::ordinary_design_day_input::project(&one, &mut mismatched, true).is_err());
    assert_eq!(mismatched.design_days, before_days);
    assert!(mismatched.setup_simulation_controls.is_none());
    crate::ordinary_design_day_input::project(&one, &mut mismatched, false)?;
    assert_eq!(mismatched.design_days, [expected[0].clone()]);

    // Both explicit Source lanes hand original scalar/default-profile inputs
    // to the same genuine getter. Unscoped legacy retains its DTO boundary.
    for (mode, operand, value) in [
        ("WetBulb", "wetbulb_or_dewpoint_at_maximum_dry_bulb", 34.0),
        ("DewPoint", "wetbulb_or_dewpoint_at_maximum_dry_bulb", 34.0),
        ("HumidityRatio", "humidity_ratio_at_maximum_dry_bulb", 0.009),
        ("Enthalpy", "enthalpy_at_maximum_dry_bulb", 50000.0),
        (
            "WetBulbProfileDefaultMultipliers",
            "wetbulb_or_dewpoint_at_maximum_dry_bulb",
            15.5,
        ),
    ] {
        let mut candidate = document.clone();
        let object = candidate["SizingPeriod:DesignDay"][COOLING]
            .as_object_mut()
            .ok_or("DD object")?;
        object.insert("humidity_condition_type".into(), mode.into());
        object.remove("wetbulb_or_dewpoint_at_maximum_dry_bulb");
        object.insert(operand.into(), value.into());
        object.insert("daily_wet_bulb_temperature_range".into(), 6.5.into());
        object.remove("sky_clearness"); // Genuine ClearSky warning/default, not eager error.
        for key in [
            "dry_bulb_temperature_range_modifier_day_schedule_name",
            "humidity_condition_day_schedule_name",
            "beam_solar_day_schedule_name",
            "diffuse_solar_day_schedule_name",
        ] {
            object.insert(key.into(), " Unused unbound day name ".into());
        }
        let raw = parse_epjson_str(&candidate.to_string())?;
        crate::ordinary_equipment_input::resolve_source_a(&raw)?;
        let mut prepared = configuration(2)?;
        prepared.design_days = expected.to_vec();
        crate::ordinary_design_day_input::prepare_source_a(&raw, &mut prepared, true)?;
        assert!(prepared.design_days.is_empty());
        assert!(prepared.setup_simulation_controls.is_none());
        assert!(prepared.reverse_design_days);
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        let fields = ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?;
        use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as DDValue;
        assert_eq!(
            fields["humidity_condition_type"],
            DDValue::String(mode.into())
        );
        assert!(matches!(&fields[operand], DDValue::Number(_)));
        assert!(!fields.contains_key("sky_clearness"));
        assert_eq!(
            fields["humidity_condition_day_schedule_name"],
            DDValue::String(" Unused unbound day name ".into())
        );
        assert!(ip.is_object_unused("SizingPeriod:DesignDay", COOLING));
        crate::ordinary_equipment_input::resolve_source_b(&raw)?;
        let b_ip = crate::ordinary_input_processor::resolve_source_b(&raw)?;
        assert_eq!(
            b_ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?,
            fields
        );
        let mut b_prepared = configuration(2)?;
        b_prepared.design_days = expected.to_vec();
        crate::ordinary_design_day_input::project_source(&raw, &mut b_prepared, true)?;
        assert!(
            b_prepared.design_days.is_empty()
                && b_prepared.setup_simulation_controls.is_none()
                && b_prepared.reverse_design_days
        );
        if !matches!(mode, "WetBulb" | "DewPoint") {
            assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        }
    }
    let mut opaque = document.clone();
    let object = opaque["SizingPeriod:DesignDay"][COOLING]
        .as_object_mut()
        .ok_or("DD metadata object")?;
    object.insert("humidity_condition_type".into(), "HumidityRatio".into());
    object.remove("wetbulb_or_dewpoint_at_maximum_dry_bulb");
    object.remove("humidity_ratio_at_maximum_dry_bulb");
    // Unknown aliases cannot fill a selected operand; the real getter diagnoses
    // its absence. The six JSON kinds remain original metadata without coercion.
    object.insert("humidity_ratio".into(), 0.009.into());
    for (key, value) in [
        ("opaque_null", serde_json::json!(null)),
        ("opaque_bool", serde_json::json!(true)),
        ("opaque_number", serde_json::json!(7)),
        ("opaque_string", serde_json::json!(" kept ")),
        (
            "opaque_array",
            serde_json::json!([null, true, 7, " text ", {}, []]),
        ),
        (
            "opaque_object",
            serde_json::json!({"nested":[null,false,8]}),
        ),
    ] {
        object.insert(key.into(), value);
    }
    let raw = parse_epjson_str(&opaque.to_string())?;
    crate::ordinary_equipment_input::resolve_source_a(&raw)?;
    crate::ordinary_equipment_input::resolve_source_b(&raw)?;
    let before = raw.clone();
    let mut prepared = configuration(2)?;
    crate::ordinary_design_day_input::prepare_source_a(&raw, &mut prepared, false)?;
    assert_eq!(raw, before);
    let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
    let fields = ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?;
    let b_ip = crate::ordinary_input_processor::resolve_source_b(&raw)?;
    assert_eq!(
        b_ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?,
        fields
    );
    use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as DDValue;
    assert!(!fields.contains_key("humidity_ratio_at_maximum_dry_bulb"));
    assert_eq!(fields["humidity_ratio"], DDValue::Number("0.009".into()));
    assert_eq!(fields["opaque_null"], DDValue::Null);
    assert_eq!(fields["opaque_bool"], DDValue::Bool(true));
    assert_eq!(fields["opaque_number"], DDValue::Number("7".into()));
    assert_eq!(fields["opaque_string"], DDValue::String(" kept ".into()));
    assert!(matches!(&fields["opaque_array"], DDValue::Array(items) if items.len()==6));
    assert!(
        matches!(&fields["opaque_object"], DDValue::Object(items) if items.contains_key("nested"))
    );
    let hb = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)?;
    let actual = hb.objects["SizingPeriod:DesignDay"]
        .iter()
        .find(|row| row.object_name == COOLING)
        .ok_or("actual DD raw getter row")?;
    assert!(
        !actual
            .fields
            .contains_key("humidity_ratio_at_maximum_dry_bulb")
    );
    assert!(!actual.fields.contains_key("humidity_ratio"));
    assert!(!actual.fields.keys().any(|key| key.starts_with("opaque_")));

    // Native JSON integer-kind retrieval wraps u64 to i64 before Real64.
    // The floating-kind control is intentionally outside the real DD range;
    // this checks getter classification only, without running either engine.
    for (token, expected) in [
        ("18446744073709551615", -1.0),
        ("18446744073709551615.0", 1.844_674_407_370_955_2e19),
    ] {
        let mut candidate = document.clone();
        let authored: serde_json::Value = serde_json::from_str(token)?;
        let authored_text = authored.to_string();
        candidate["SizingPeriod:DesignDay"][COOLING]["wetbulb_or_dewpoint_at_maximum_dry_bulb"] =
            authored;
        let raw = parse_epjson_str(&candidate.to_string())?;
        crate::ordinary_equipment_input::resolve_source_a(&raw)?;
        let original = raw.clone();
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        assert_eq!(
            ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?["wetbulb_or_dewpoint_at_maximum_dry_bulb"],
            DDValue::Number(authored_text)
        );
        let hb = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)?;
        let row = hb.objects["SizingPeriod:DesignDay"]
            .iter()
            .find(|row| row.object_name == COOLING)
            .ok_or("source DD numeric row")?;
        assert_eq!(
            row.fields["wetbulb_or_dewpoint_at_maximum_dry_bulb"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(expected)
        );
        let legacy = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
        let row = legacy.objects["SizingPeriod:DesignDay"]
            .iter()
            .find(|row| row.object_name == COOLING)
            .ok_or("legacy DD numeric row")?;
        assert_eq!(
            row.fields["wetbulb_or_dewpoint_at_maximum_dry_bulb"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                1.844_674_407_370_955_2e19
            )
        );
        assert_eq!(raw, original);
    }
    for (field, value) in [
        ("humidity_condition_type", serde_json::json!(true)),
        ("humidity_ratio_at_maximum_dry_bulb", serde_json::json!({})),
        ("maximum_dry_bulb_temperature", serde_json::json!(71)),
        ("idf_order", serde_json::json!(0.0)),
        ("idf_max_fields", serde_json::json!(-1)),
        ("idf_max_extensible_fields", serde_json::json!("0")),
    ] {
        let mut invalid = opaque.clone();
        invalid["SizingPeriod:DesignDay"][COOLING][field] = value;
        assert!(
            crate::ordinary_equipment_input::resolve_source_a(&parse_epjson_str(
                &invalid.to_string()
            )?)
            .is_err(),
            "{field}"
        );
        assert!(
            crate::ordinary_equipment_input::resolve_source_b(&parse_epjson_str(
                &invalid.to_string()
            )?)
            .is_err(),
            "{field}"
        );
    }
    // Distinct selected roles reach the same original raw/IP/HB owners. This
    // is admission/custody evidence; actual schedule lookup and physics belong
    // to their separate genuine-owner and physical tests.
    const DAY_NAME: &str = " 104 actual day ";
    for (dry, humidity, solar, reset) in [
        (
            "MultiplierSchedule",
            "WetBulb",
            "ASHRAEClearSky",
            "FullResetAtBeginEnvironment",
        ),
        (
            "DifferenceSchedule",
            "DewPoint",
            "ZhangHuang",
            "FullResetAtBeginEnvironment",
        ),
        (
            "TemperatureProfileSchedule",
            "RelativeHumiditySchedule",
            "Schedule",
            "SuppressAllBeginEnvironmentResets",
        ),
        (
            "DefaultMultipliers",
            "WetBulbProfileDifferenceSchedule",
            "ASHRAETau",
            "FullResetAtBeginEnvironment",
        ),
        (
            "DefaultMultipliers",
            "WetBulbProfileMultiplierSchedule",
            "ASHRAETau2017",
            "SuppressAllBeginEnvironmentResets",
        ),
    ] {
        let mut candidate = document.clone();
        let mut day = serde_json::Map::new();
        for hour in 1..=24 {
            day.insert(format!("hour_{hour}"), 0.5.into());
        }
        candidate["Schedule:Day:Hourly"] = serde_json::json!({});
        candidate["Schedule:Day:Hourly"][DAY_NAME] = day.into();
        let object = candidate["SizingPeriod:DesignDay"][COOLING]
            .as_object_mut()
            .ok_or("selected DD object")?;
        for (field, value) in [
            ("dry_bulb_temperature_range_modifier_type", dry),
            ("humidity_condition_type", humidity),
            ("solar_model_indicator", solar),
            ("begin_environment_reset_mode", reset),
        ] {
            object.insert(field.into(), value.into());
        }
        for field in [
            "dry_bulb_temperature_range_modifier_day_schedule_name",
            "humidity_condition_day_schedule_name",
            "beam_solar_day_schedule_name",
            "diffuse_solar_day_schedule_name",
        ] {
            object.insert(field.into(), DAY_NAME.into());
        }
        if dry == "TemperatureProfileSchedule" {
            object.remove("maximum_dry_bulb_temperature");
        }
        let raw = parse_epjson_str(&candidate.to_string())?;
        crate::ordinary_equipment_input::resolve_source_a(&raw)?;
        crate::ordinary_equipment_input::resolve_source_b(&raw)?;
        assert!(crate::ordinary_equipment_input::resolve(&raw).is_err());
        let original = raw.clone();
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        let b_ip = crate::ordinary_input_processor::resolve_source_b(&raw)?;
        assert_eq!(
            b_ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?,
            ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?
        );
        assert_eq!(b_ip.get_num_objects_found("Schedule:Day:Hourly"), 1);
        assert_eq!(ip.get_num_objects_found("Schedule:Day:Hourly"), 1);
        let fields = ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?;
        for (field, value) in [
            ("dry_bulb_temperature_range_modifier_type", dry),
            ("humidity_condition_type", humidity),
            ("solar_model_indicator", solar),
            ("begin_environment_reset_mode", reset),
        ] {
            assert_eq!(fields[field], DDValue::String(value.into()));
        }
        assert_eq!(
            fields.contains_key("maximum_dry_bulb_temperature"),
            dry != "TemperatureProfileSchedule"
        );
        let hb = crate::ordinary_heat_balance_input::resolve_source(&raw, &ip)?;
        let actual = hb.objects["SizingPeriod:DesignDay"]
            .iter()
            .find(|row| row.object_name == COOLING)
            .ok_or("selected DD HB row")?;
        for field in [
            "dry_bulb_temperature_range_modifier_day_schedule_name",
            "humidity_condition_day_schedule_name",
            "beam_solar_day_schedule_name",
            "diffuse_solar_day_schedule_name",
        ] {
            assert_eq!(fields[field], DDValue::String(DAY_NAME.into()));
            assert_eq!(
                actual.fields[field],
                ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                    DAY_NAME.into()
                )
            );
        }
        let mut prepared = configuration(2)?;
        prepared.design_days = expected.to_vec();
        crate::ordinary_design_day_input::prepare_source_a(&raw, &mut prepared, true)?;
        assert!(
            prepared.design_days.is_empty()
                && prepared.setup_simulation_controls.is_none()
                && prepared.reverse_design_days
        );
        assert_eq!(raw, original);

        // Missing lookup tables/names are not repaired or resolved by admission.
        // The original selected getter must own its warning/severe ordering.
        candidate
            .as_object_mut()
            .ok_or("DD root")?
            .remove("Schedule:Day:Hourly");
        candidate["SizingPeriod:DesignDay"][COOLING]
            .as_object_mut()
            .ok_or("missing named-day DD")?
            .remove("humidity_condition_day_schedule_name");
        let absent = parse_epjson_str(&candidate.to_string())?;
        crate::ordinary_equipment_input::resolve_source_a(&absent)?;
        crate::ordinary_equipment_input::resolve_source_b(&absent)?;
        assert!(crate::ordinary_equipment_input::resolve(&absent).is_err());
        let ip = crate::ordinary_input_processor::resolve_source_a(&absent)?;
        let b_ip = crate::ordinary_input_processor::resolve_source_b(&absent)?;
        assert_eq!(
            b_ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?,
            ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?
        );
        assert_eq!(b_ip.get_num_objects_found("Schedule:Day:Hourly"), 0);
        assert_eq!(ip.get_num_objects_found("Schedule:Day:Hourly"), 0);
        assert!(
            !ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?
                .contains_key("humidity_condition_day_schedule_name")
        );
        let mut prepared = configuration(2)?;
        crate::ordinary_design_day_input::prepare_source_a(&absent, &mut prepared, false)?;
        assert!(prepared.design_days.is_empty());
    }
    // Known optional-name nulls retain their original representation at the
    // raw/IP boundary and fail HB scalar projection without defaulting.
    for field in [
        "dry_bulb_temperature_range_modifier_day_schedule_name",
        "humidity_condition_day_schedule_name",
        "beam_solar_day_schedule_name",
        "diffuse_solar_day_schedule_name",
    ] {
        let mut candidate = document.clone();
        candidate["SizingPeriod:DesignDay"][COOLING][field] = serde_json::Value::Null;
        let raw = parse_epjson_str(&candidate.to_string())?;
        crate::ordinary_equipment_input::resolve_source_a(&raw)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        assert_eq!(
            ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?[field],
            DDValue::Null
        );
        assert!(crate::ordinary_heat_balance_input::resolve_source(&raw, &ip).is_err());
        let b_ip = crate::ordinary_input_processor::resolve_source_b(&raw)?;
        assert_eq!(
            b_ip.source_raw_object_fields("SizingPeriod:DesignDay", COOLING)?[field],
            DDValue::Null
        );
        assert!(crate::ordinary_heat_balance_input::resolve_source(&raw, &b_ip).is_err());
    }
    for (field, value) in [
        (
            "dry_bulb_temperature_range_modifier_day_schedule_name",
            serde_json::json!(true),
        ),
        (
            "humidity_condition_day_schedule_name",
            serde_json::json!([]),
        ),
        ("beam_solar_day_schedule_name", serde_json::json!({})),
        ("diffuse_solar_day_schedule_name", serde_json::json!(17)),
    ] {
        let mut invalid = document.clone();
        invalid["SizingPeriod:DesignDay"][COOLING][field] = value;
        assert!(
            crate::ordinary_equipment_input::resolve_source_a(&parse_epjson_str(
                &invalid.to_string()
            )?)
            .is_err(),
            "{field}"
        );
        assert!(
            crate::ordinary_equipment_input::resolve_source_b(&parse_epjson_str(
                &invalid.to_string()
            )?)
            .is_err(),
            "{field}"
        );
    }
    for (field, value) in [
        ("dry_bulb_temperature_range_modifier_type", "UnknownDryBulb"),
        ("humidity_condition_type", "UnknownHumidity"),
        ("solar_model_indicator", "UnknownSolar"),
        // Preserve the official SuppressAll key without renaming it to the
        // different Native getter spelling. Runtime tests own the actual flag.
        (
            "begin_environment_reset_mode",
            "SuppressThermalResetAtBeginEnvironment",
        ),
    ] {
        let mut invalid = document.clone();
        invalid["SizingPeriod:DesignDay"][COOLING][field] = value.into();
        assert!(
            crate::ordinary_equipment_input::resolve_source_a(&parse_epjson_str(
                &invalid.to_string()
            )?)
            .is_err(),
            "{field}"
        );
        assert!(
            crate::ordinary_equipment_input::resolve_source_b(&parse_epjson_str(
                &invalid.to_string()
            )?)
            .is_err(),
            "{field}"
        );
    }
    let mut empty_table = document.clone();
    empty_table["SizingPeriod:DesignDay"] = serde_json::json!({});
    let empty = parse_epjson_str(&empty_table.to_string())?;
    crate::ordinary_equipment_input::resolve_source_a(&empty)?;
    let ip = crate::ordinary_input_processor::resolve_source_a(&empty)?;
    assert_eq!(ip.get_num_objects_found("SizingPeriod:DesignDay"), 0);
    let mut prepared = configuration(0)?;
    crate::ordinary_design_day_input::prepare_source_a(&empty, &mut prepared, false)?;
    assert!(prepared.design_days.is_empty());
    empty_table["SizingPeriod:DesignDay"] = serde_json::json!({"present but empty":{}});
    assert!(
        crate::ordinary_equipment_input::resolve_source_a(&parse_epjson_str(
            &empty_table.to_string()
        )?)
        .is_err()
    );
    let mut one_source = configuration(1)?;
    crate::ordinary_design_day_input::prepare_source_a(&one, &mut one_source, true)?;
    assert!(one_source.reverse_design_days && one_source.design_days.is_empty());
    let mut mismatch = configuration(1)?;
    mismatch.design_days = expected.to_vec();
    let prior = mismatch.design_days.clone();
    assert!(
        crate::ordinary_design_day_input::prepare_source_a(&lexical, &mut mismatch, true).is_err()
    );
    assert_eq!(mismatch.design_days, prior);
    Ok(())
}

fn expected_days() -> Result<[DesignDayInput; 2], Box<dyn std::error::Error>> {
    let make = |(
        name,
        month,
        day_type,
        maximum_dry_bulb,
        daily_dry_bulb_range,
        humidity_indicator,
        wind_speed,
        wind_direction,
        sky_clearness,
    ): (&str, i32, i32, f64, f64, f64, f64, f64, f64)| {
        DesignDayInput::from_selected_parameters(
            DesignDayParameters {
                name: name.to_ascii_uppercase(),
                month,
                day_of_month: 21,
                day_type,
                maximum_dry_bulb,
                daily_dry_bulb_range,
                humidity_type: DesignDayHumidity::WetBulb,
                humidity_indicator,
                barometric_pressure: Some(81198.0),
                wind_speed,
                wind_direction,
                rain: false,
                snow: false,
                daylight_saving: false,
                sky_clearness,
                maximum_warmup_days: -1,
            },
            4,
        )
    };
    Ok([
        make((HEATING, 12, 10, -15.5, 0.0, -15.5, 3.0, 340.0, 0.0))?,
        make((COOLING, 7, 9, 32.0, 15.2, 15.5, 4.9, 0.0, 1.0))?,
    ])
}

fn assert_controls(config: &WeatherEnvironmentConfiguration, expected: [bool; 6]) -> TestResult {
    let actual = config
        .setup_simulation_controls
        .ok_or("projected controls")?;
    assert_eq!(
        [
            actual.zone_sizing(),
            actual.system_sizing(),
            actual.plant_sizing(),
            actual.hvac_sizing_simulation(),
            actual.run_sizing_periods(),
            actual.run_weather_periods()
        ],
        expected
    );
    Ok(())
}

fn configuration(
    design_day_count: i32,
) -> Result<WeatherEnvironmentConfiguration, Box<dyn std::error::Error>> {
    let period = RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new("Raw DD projection"),
        begin_month: 1,
        begin_day_of_month: 1,
        begin_year: Some(2013),
        end_month: 1,
        end_day_of_month: 1,
        end_year: Some(2013),
        day_of_week_for_start_day: Some(DayOfWeek::Tuesday),
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        use_weather_file_holidays_and_special_days: false,
        use_weather_file_daylight_saving_period: false,
        apply_weekend_holiday_rule: false,
        use_weather_file_rain_indicators: false,
        use_weather_file_snow_indicators: false,
        treat_weather_as_actual: false,
    };
    let site = SiteLocation {
        name: NormalizedName::new("Denver"),
        latitude_deg: 39.74,
        longitude_deg: -105.18,
        time_zone_hours: -7.0,
        elevation_m: 1829.0,
    };
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        &period,
        &EpwCalendarMetadata::default(),
        4,
    )?;
    Ok(WeatherEnvironmentConfiguration::new(
        &axis,
        &period,
        &site,
        design_day_count,
        0,
    )?)
}
