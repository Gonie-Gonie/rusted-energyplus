//! One finite Native comparison consumed by the existing lifecycle regression.

use super::{ProductionWeatherTimestepSeries, SetupSimulationControls, SetupSimulationRole};
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::weather::day::{DesignDayHumidity, DesignDayInput, DesignDayParameters};
use chrono::{Datelike, Local};
use ep_model::FirstHourInterpolationStartingValues;
use serde_json::Value;

#[path = "design_day_native_state_tests.rs"]
mod state;

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[path = "design_day_native_tests/passive_psy.rs"]
mod passive_psy;

pub(crate) fn verify_actual_design_day_weather() -> TestResult {
    passive_psy::verify_if_selected()?;
    NUMERICAL_ASSESSMENT.with_borrow_mut(|values| values.clear());
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/ordinary_design_day_native.json"))?;
    assert_eq!(
        fixture["schema"],
        "src06-actual-design-day-weather-fixture.v1"
    );
    let mut config = super::super::lifecycle_tests::configuration(
        6,
        30,
        7,
        2,
        FirstHourInterpolationStartingValues::Hour24,
    )?;
    // These are the pinned A-24H input operands, not Native calculated outputs.
    // The decoy EPW supplies header transport only; DD must consume no EPW rows.
    config.site.elevation_m = 1829.0;
    config.design_day_count = 2;
    config.setup_simulation_controls = Some(SetupSimulationControls::from_admitted_input_flags(
        false, false, false, false, false, true,
    ));
    for (name, month, day_type, db, range, wb, speed, direction, clearness) in [
        (
            "DENVER CENTENNIAL  GOLDEN   N ANN HTG 99% CONDNS DB",
            12,
            10,
            -15.5,
            0.0,
            -15.5,
            3.0,
            340.0,
            0.0,
        ),
        (
            "DENVER CENTENNIAL  GOLDEN   N ANN CLG 1% CONDNS DB=>MWB",
            7,
            9,
            32.0,
            15.2,
            15.5,
            4.9,
            0.0,
            1.0,
        ),
    ] {
        config
            .design_days
            .push(DesignDayInput::from_selected_parameters(
                DesignDayParameters {
                    name: name.into(),
                    month,
                    day_of_month: 21,
                    day_type,
                    maximum_dry_bulb: db,
                    daily_dry_bulb_range: range,
                    humidity_type: DesignDayHumidity::WetBulb,
                    humidity_indicator: wb,
                    barometric_pressure: Some(81198.0),
                    wind_speed: speed,
                    wind_direction: direction,
                    rain: false,
                    snow: false,
                    daylight_saving: false,
                    sky_clearness: clearness,
                    maximum_warmup_days: -1,
                },
                4,
            )?);
    }
    let series = ProductionWeatherTimestepSeries::from_bytes(
        super::super::lifecycle_tests::DECOY_BYTES.to_vec(),
        config,
    )?;
    let mut reports = CtfInitializationReports::default();
    reports
        .registry
        .set_predefined_tables(false, false)
        .map_err(|error| format!("real early registry: {error:?}"))?;
    series.begin_setup_simulation()?;
    series.reset_setup_environment_counter();
    let events = fixture["design_day_manage_weather_returns"]
        .as_array()
        .ok_or("DD calls")?;
    assert_eq!(events.len(), 6);
    let initial = series.snapshot();
    let mut ordinal = 0;
    let mut initialized_years = [0; 2];
    let mut same_input_sky_slots = 0;
    let mut same_input_sky_differences = Vec::new();
    for environment in 1..=2 {
        assert_eq!(series.next_setup_environment(&mut reports)?, (true, false));
        if environment == 1 {
            verify_reports(&reports, &fixture)?;
        }
        for role in [
            SetupSimulationRole::FirstHour,
            SetupSimulationRole::RepeatedFirstHour,
            SetupSimulationRole::LastHour,
        ] {
            ordinal += 1;
            let native = &events[ordinal - 1];
            assert_eq!(native["actual_setup_call_ordinal"], ordinal);
            assert_eq!(native["setup_role"], format!("{role:?}"));
            series.prepare_setup_call(role);
            let year_before = Local::now().year();
            let call = series.manage_setup_weather(ordinal as u64, role, true, false, false)?;
            let year_after = Local::now().year();
            // date_and_time is a live external input. Future runs must obtain the
            // actual local year, never replay the captured year into production.
            if role == SetupSimulationRole::FirstHour {
                assert!([year_before, year_after].contains(&call.daily.year));
                initialized_years[environment - 1] = call.daily.year;
            }
            assert_eq!(call.daily.year, initialized_years[environment - 1]);
            assert_eq!(call.ordinal, ordinal as u64);
            assert_eq!(call.environment_number, environment as i32);
            assert_eq!(call.environment_name, native["clock"]["environment_name"]);
            assert_eq!(series.setup_environment_name(), call.environment_name);
            assert_eq!(
                Some(call.kickoff_simulation),
                native["clock"]["KickOffSimulation"].as_bool()
            );
            assert_eq!(
                Some(call.kickoff_sizing),
                native["weather"]["KickOffSizing"].as_bool()
            );
            assert_eq!(
                Some(call.do_weather_init_reporting),
                native["weather"]["DoWeatherInitReporting"].as_bool()
            );
            let snapshot = series.snapshot();
            // UpdateWeatherData copies this selected DesignDay's full Tomorrow
            // carrier to Today. Test that transport against Rust's own produced
            // carrier; computed Native values have the separately declared scope.
            assert_eq!(
                snapshot.state.today_values.is_allocated(),
                snapshot.state.tomorrow_values.is_allocated()
            );
            assert_eq!(
                snapshot.state.today_values.time_steps(),
                snapshot.state.tomorrow_values.time_steps()
            );
            assert_eq!(
                snapshot.state.today_values.hours(),
                snapshot.state.tomorrow_values.hours()
            );
            assert_eq!(
                snapshot.state.today_values.slots().len(),
                snapshot.state.tomorrow_values.slots().len()
            );
            for (today, tomorrow) in snapshot
                .state
                .today_values
                .slots()
                .iter()
                .zip(snapshot.state.tomorrow_values.slots())
            {
                assert_eq!(today.boolean_values(), tomorrow.boolean_values());
                assert_eq!(
                    today.real_values().map(f64::to_bits),
                    tomorrow.real_values().map(f64::to_bits),
                    "literal full DesignDay weather carrier copy"
                );
            }
            state::verify(&snapshot.state, native, ordinal)?;
            state::site_and_solar(&snapshot.state, native, -7.0)?;
            // Isolate the actual DD sky producer from upstream PSY differences.
            // These are complete captured Native slot operands, never expected
            // values seeded into production. All six actual repeated carriers
            // remain separate; neither the fixture nor Native output is changed.
            let native_weather = &native["weather"];
            assert_eq!(
                native_weather["Environment"]["rows"][environment - 1]["value"]["skyTempModel"],
                0,
                "actual ClarkAllen owner"
            );
            assert_eq!(
                native_weather["DesDayInput"]["rows"][environment - 1]["value"]["RainInd"],
                0,
                "actual OSky=0 Source branch"
            );
            for row in native_weather["wvarsHrTsToday"]["rows"]
                .as_array()
                .ok_or("actual DD sky operand rows")?
            {
                let value = &row["value"];
                let captured = |field: &str| -> Result<f64, Box<dyn std::error::Error>> {
                    Ok(f64::from_bits(u64::from_str_radix(
                        value[field]["value_bits"]
                            .as_str()
                            .ok_or("actual DD sky operand bits")?,
                        16,
                    )?))
                };
                let mut slot = super::WeatherVars {
                    out_dry_bulb_temp: captured("OutDryBulbTemp")?,
                    out_dew_point_temp: captured("OutDewPointTemp")?,
                    out_rel_hum: captured("OutRelHum")?,
                    ..Default::default()
                };
                let opaque_sky_cover = captured("OpaqueSkyCover")?;
                assert_eq!(opaque_sky_cover.to_bits(), 0.0_f64.to_bits());
                super::super::sky::initialize_design_day_sky(&mut slot, opaque_sky_cover);
                same_input_sky_slots += 1;
                for (field, actual) in [
                    ("HorizIRSky", slot.horiz_ir_sky),
                    ("SkyTemp", slot.sky_temp),
                ] {
                    let expected = captured(field)?;
                    assert!(actual.is_finite() && expected.is_finite());
                    if actual.to_bits() != expected.to_bits() {
                        same_input_sky_differences.push((
                            ordinal,
                            row["hour"].as_u64().ok_or("actual sky hour")?,
                            row["timestep"].as_u64().ok_or("actual sky timestep")?,
                            field,
                            actual.to_bits(),
                            expected.to_bits(),
                        ));
                    }
                }
            }
            verify_design_day_owners(&series, &native["weather"], &initialized_years)?;
            assert_eq!(snapshot.interpret_count, initial.interpret_count);
            assert_eq!(snapshot.line_read_count, initial.line_read_count);
            if role == SetupSimulationRole::FirstHour {
                series.finish_first_setup_call();
            }
        }
    }
    // Do not enter the real RunPeriod with the unrelated decoy weather file.
    // Exterior/HB/full Setup are separate consumers and are not claimed here.
    let assessments = NUMERICAL_ASSESSMENT.with_borrow(|values| values.clone());
    for field in PSY_CALLER_FIELDS.into_iter().chain(UNPAIRED_FIELDS) {
        assert!(
            assessments.contains_key(field),
            "declared numerical field was not observed: {field}"
        );
    }
    for (name, (observations, nonidentical_bits, max_absolute)) in assessments {
        let role = match real_scope(&name)? {
            NativeRealScope::PsyCaller { .. } => {
                "inherited-PSY-caller-output-assessment; PSY-input/cache-pairing-unobserved"
            }
            NativeRealScope::UnpairedComputed => "UNPAIRED-computed-output; no-numerical-PASS",
            NativeRealScope::ExactBits => {
                return Err("exact field incorrectly entered numerical accounting".into());
            }
        };
        eprintln!(
            "DD scoped assessment {name}: {role}; {observations} observations; {nonidentical_bits} nonidentical bits; maximum absolute {max_absolute:e}"
        );
    }
    assert_eq!(same_input_sky_slots, 6 * 96);
    for (call, hour, timestep, field, actual, expected) in &same_input_sky_differences {
        eprintln!(
            "DD same-input sky call{call} hour{hour} ts{timestep} {field}: Rust {actual:016x}, Native {expected:016x}"
        );
    }
    assert!(
        same_input_sky_differences.is_empty(),
        "DD same-input sky strict bits: {} differences across {} outputs; no numerical PASS or cause attribution on failure",
        same_input_sky_differences.len(),
        same_input_sky_slots * 2
    );
    // The immutable strict-bit diagnostic remains FAIL. Passing this regression
    // certifies its declared transport/owner scope, never all Native weather,
    // PSY cache/input pairing, full Setup, or a libm attribution.
    Ok(())
}

fn verify_design_day_owners(
    series: &ProductionWeatherTimestepSeries,
    native: &Value,
    initialized_years: &[i32; 2],
) -> TestResult {
    let session = series.session.borrow();
    for (index, input) in session.configuration.design_days.iter().enumerate() {
        let raw = &native["DesDayInput"]["rows"][index]["value"];
        assert_eq!(input.name(), raw["Title"]);
        real(
            input.resolved_pressure(),
            &raw["PressBarom"],
            "DesDayInput.PressBarom",
        )?;
        state::daily(
            input.daily_variables(),
            &native["DesignDay"]["rows"][index]["value"],
            initialized_years[index],
        )?;
        let modifiers = native["desDayMods"]["rows"][index]["value"]["rows"]
            .as_array()
            .ok_or("mods")?;
        assert_eq!(modifiers.len(), input.dry_bulb_modifiers().len());
        for (actual, expected) in input.dry_bulb_modifiers().iter().zip(modifiers) {
            real(
                *actual,
                &expected["value"]["OutDryBulbTemp"],
                "desDayMods.OutDryBulbTemp",
            )?;
        }
        let schedules = &native["spSiteSchedules"]["rows"][index]["value"];
        for (actual, name) in input.site_schedule_values().iter().zip([
            "OutDryBulbTemp",
            "OutRelHum",
            "BeamSolarRad",
            "DifSolarRad",
            "SkyTemp",
        ]) {
            real(*actual, &schedules[name], "spSiteSchedules")?;
        }
    }
    Ok(())
}

fn verify_reports(reports: &CtfInitializationReports, fixture: &Value) -> TestResult {
    let native = &fixture["get_design_day_data"][1]["predefined_reports"];
    for (key, value) in fixture["six_predefined_column_ids"]
        .as_object()
        .ok_or("columns")?
    {
        assert_eq!(
            reports
                .registry
                .named_indices
                .get(key)
                .copied()
                .map(i64::from),
            value.as_i64()
        );
    }
    assert_eq!(
        Some(i64::from(reports.table.num_table_entry)),
        native["numTableEntry"].as_i64()
    );
    assert_eq!(
        Some(i64::from(reports.table.size_table_entry)),
        native["sizeTableEntry"].as_i64()
    );
    let entries = reports
        .table
        .entries
        .as_ref()
        .ok_or("real table allocation")?;
    let rows = native["tableEntry"]["rows"]
        .as_array()
        .ok_or("Native table rows")?;
    assert_eq!(entries.len(), rows.len());
    for (actual, row) in entries.iter().zip(rows) {
        let expected = &row["value"];
        assert_eq!(actual.char_entry, expected["charEntry"]);
        assert_eq!(actual.object_name, expected["objectName"]);
        assert_eq!(
            Some(i64::from(actual.index_column)),
            expected["indexColumn"].as_i64()
        );
        assert_eq!(
            Some(i64::from(actual.sub_table_index)),
            expected["subTableIndex"].as_i64()
        );
        assert_eq!(
            Some(i64::from(actual.unique_object_name)),
            expected["uniqueObjName"].as_i64()
        );
        assert_eq!(
            Some(i64::from(actual.significant_digits)),
            expected["significantDigits"].as_i64()
        );
        assert_eq!(
            Some(actual.original_entry_is_real),
            expected["origEntryIsReal"].as_bool()
        );
        real(
            actual.original_real_entry,
            &expected["origRealEntry"],
            "table.origRealEntry",
        )?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum NativeRealScope {
    ExactBits,
    PsyCaller { atol: f64, rtol: f64 },
    UnpairedComputed,
}

const PSY_CALLER_FIELDS: [&str; 5] = [
    "OutDewPointTemp",
    "OutWetBulbTemp",
    "OutHumRat",
    "OutEnthalpy",
    "OutAirDensity",
];
const UNPAIRED_FIELDS: [&str; 8] = [
    "OutRelHum",
    "OutRelHumValue",
    "SkyTemp",
    "SkyTempKelvin",
    "HorizIRSky",
    "BeamSolarRad",
    "DifSolarRad",
    "GndSolarRad",
];

fn real_scope(name: &str) -> Result<NativeRealScope, Box<dyn std::error::Error>> {
    let field = name.rsplit('.').next().ok_or("missing field name")?;
    let scope = match field {
        // These constants already existed in frozen PSY-01/02 contracts. Here
        // they assess caller outputs only: intermediate/cache inputs were not
        // captured, so this cannot certify paired PSY calls/cache parity.
        "OutDewPointTemp" | "OutWetBulbTemp" => NativeRealScope::PsyCaller {
            atol: 1.0e-10,
            rtol: 1.0e-13,
        },
        "OutHumRat" | "OutAirDensity" => NativeRealScope::PsyCaller {
            atol: 1.0e-12,
            rtol: 1.0e-13,
        },
        "OutEnthalpy" => NativeRealScope::PsyCaller {
            atol: 1.0e-7,
            rtol: 1.0e-13,
        },
        "OutRelHum" | "OutRelHumValue" | "SkyTemp" | "SkyTempKelvin" | "HorizIRSky"
        | "BeamSolarRad" | "DifSolarRad" | "GndSolarRad" => NativeRealScope::UnpairedComputed,
        // No fallback classification: every remaining currently compared real
        // carrier is named explicitly and retains strict signed-zero bit checks.
        "PressBarom"
        | "OutDryBulbTemp"
        | "spSiteSchedules"
        | "origRealEntry"
        | "Interpolation"
        | "SolarInterpolation"
        | "TimeStepFraction"
        | "CurrentTime"
        | "TimeStepZone"
        | "TimeStepZoneSec"
        | "SinSolarDeclinAngle"
        | "CosSolarDeclinAngle"
        | "EquationOfTime"
        | "OutBaroPress"
        | "WindSpeed"
        | "WindDir"
        | "Albedo"
        | "WaterPrecip"
        | "LiquidPrecip"
        | "TotalSkyCover"
        | "OpaqueSkyCover"
        | "LiquidPrecipitation"
        | "GndReflectance"
        | "GndReflectanceForDayltg"
        | "TotalCloudCover"
        | "OpaqueCloudCover"
        | "SOLCOS"
        | "CosLatitude"
        | "Elevation"
        | "Latitude"
        | "Longitude"
        | "SinLatitude"
        | "StdBaroPress"
        | "TS1TimeOffset"
        | "TimeZoneMeridian"
        | "TimeZoneNumber"
        | "HrAngle"
        | "SnowGndRefModifier"
        | "SnowGndRefModifierForDayltg"
        | "SolarAltitudeAngle"
        | "SolarAzimuthAngle"
        | "GroundReflectances" => NativeRealScope::ExactBits,
        _ => return Err(format!("undeclared Native real comparison field: {name}").into()),
    };
    Ok(scope)
}

fn real(actual: f64, native: &Value, name: &str) -> TestResult {
    let bits = u64::from_str_radix(
        native["value_bits"]
            .as_str()
            .ok_or_else(|| format!("missing bits {name}: {native}"))?,
        16,
    )?;
    let expected = f64::from_bits(bits);
    // This selected fixture was independently audited as finite/+zero only.
    // No nonfinite value or signed-zero change can pass a numerical exemption.
    assert!(
        actual.is_finite() && expected.is_finite(),
        "{name} selected finite domain"
    );
    assert_eq!(actual.classify(), expected.classify(), "{name} IEEE class");
    if actual == 0.0 && expected == 0.0 {
        assert_eq!(actual.to_bits(), bits, "{name} signed zero");
    }
    match real_scope(name)? {
        NativeRealScope::ExactBits => {
            assert_eq!(actual.to_bits(), bits, "{name} exact carrier bits")
        }
        scope => {
            let difference = (actual - expected).abs();
            if let NativeRealScope::PsyCaller { atol, rtol } = scope {
                assert!(
                    difference <= atol + rtol * expected.abs(),
                    "{name} inherited PSY caller-output bound: Rust {actual:?}, Native {expected:?}, difference {difference:e}"
                );
            }
            // UNPAIRED deliberately has no magnitude acceptance threshold. It
            // is retained/class-checked, counted and reported, never a PASS row.
            let field = name
                .rsplit('.')
                .next()
                .ok_or("missing field name")?
                .to_owned();
            NUMERICAL_ASSESSMENT.with_borrow_mut(|values| {
                let entry = values.entry(field).or_insert((0, 0, 0.0));
                entry.0 += 1;
                entry.1 += usize::from(actual.to_bits() != bits);
                entry.2 = entry.2.max(difference);
            });
        }
    }
    Ok(())
}

thread_local! {
    static NUMERICAL_ASSESSMENT: std::cell::RefCell<std::collections::BTreeMap<String, (usize, usize, f64)>> = std::cell::RefCell::default();
}
