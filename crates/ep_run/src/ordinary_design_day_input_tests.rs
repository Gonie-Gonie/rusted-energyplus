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
            "/../../energyplus_porting_plan/cases/A-24H/input.idf"
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
