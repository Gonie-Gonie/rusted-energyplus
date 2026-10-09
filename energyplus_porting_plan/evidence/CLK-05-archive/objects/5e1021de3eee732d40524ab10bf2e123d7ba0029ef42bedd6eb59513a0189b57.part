use super::produce_day;
use crate::weather::day::hourly::process_hour;
use crate::weather::day::{WeatherDayState, WeatherDayValues, WeatherVars};
use crate::weather::raw::{
    RawEpwInput, RawEpwOutputs, RawReadProvenance, RawWeatherDay, RawWeatherSlot, project_record,
};
use crate::weather::{WeatherTimestepSample, WeatherTimestepSeries};
use ep_model::FirstHourInterpolationStartingValues;

fn raw_day(day: i32) -> RawWeatherDay {
    RawWeatherDay {
        hours: (1..=24)
            .map(|hour| {
                let mut raw = RawEpwOutputs {
                    dates: [2026, 1, day, hour, 60],
                    ..RawEpwOutputs::default()
                };
                raw.mandatory_reals[0] = 10.0 + f64::from(day) + f64::from(hour) * 0.1;
                raw.mandatory_reals[1] = 5.0;
                raw.mandatory_reals[2] = 50.0;
                raw.mandatory_reals[3] = 101325.0;
                raw.mandatory_reals[6] = 300.0;
                raw.mandatory_reals[7] = 100.0 + f64::from(hour);
                raw.mandatory_reals[8] = 200.0 + f64::from(hour);
                raw.mandatory_reals[9] = 30.0 + f64::from(hour);
                raw.mandatory_reals[14] = if hour == 24 { 350.0 } else { 10.0 };
                raw.mandatory_reals[15] = 3.0;
                raw.mandatory_reals[16] = 7.0;
                raw.mandatory_reals[17] = 5.0;
                raw.optional_reals = [11.0, 0.1, 0.0, 3.0, 0.2, 0.5];
                RawWeatherSlot {
                    raw,
                    provenance: RawReadProvenance {
                        start_byte: (hour as usize - 1) * 100,
                        end_byte: hour as usize * 100,
                        line_read_attempt: hour as usize + 8,
                    },
                }
            })
            .collect(),
        final_stream: RawEpwInput::new_unopened(Vec::new()).snapshot(),
    }
}

fn state(steps: usize) -> WeatherDayState {
    let mut owner = WeatherDayState {
        tomorrow_values: WeatherDayValues::allocated(steps).unwrap(),
        ..WeatherDayState::default()
    };
    owner.global.time_steps_in_hour = steps as i32;
    owner.setup_interpolation_values().unwrap();
    owner.weather.time_step_fraction = 1.0 / steps as f64;
    owner.weather.is_rain_threshold = 0.8 / steps as f64;
    owner
}

fn prepare_hourly(raw: &RawWeatherDay, owner: &mut WeatherDayState) {
    for (index, slot) in raw.hours.iter().enumerate() {
        let record = project_record(&slot.raw, slot.provenance.line_read_attempt).unwrap();
        let hourly = process_hour(&slot.raw, record, owner);
        owner.tomorrow_values.hour_mut(index + 1).unwrap()[0] = hourly;
    }
}

// Compatibility scalar previews preserve their explicit raw-record predecessor.
// Their separate PSY cache lifetime is not a cross-day physical parity claim.
fn sample_bits(sample: &WeatherTimestepSample) -> [u64; 10] {
    [
        sample.dry_bulb_c,
        sample.relative_humidity_percent,
        sample.atmospheric_pressure_pa,
        sample.horizontal_infrared_radiation_w_per_m2,
        sample.global_horizontal_radiation_w_per_m2,
        sample.direct_normal_radiation_w_per_m2,
        sample.diffuse_horizontal_radiation_w_per_m2,
        sample.wind_speed_m_per_s,
        sample.wind_direction_deg,
        sample.liquid_precipitation_depth_mm,
    ]
    .map(f64::to_bits)
}

#[test]
fn explicit_predecessor_retains_existing_samples_across_day_boundary() {
    for policy in [
        FirstHourInterpolationStartingValues::Hour1,
        FirstHourInterpolationStartingValues::Hour24,
    ] {
        let mut owner = state(4);
        let first_raw = raw_day(1);
        prepare_hourly(&first_raw, &mut owner);
        let first = produce_day(&first_raw, None, 4, policy, &mut owner).unwrap();
        let second_raw = raw_day(2);
        prepare_hourly(&second_raw, &mut owner);
        let before_counts = owner.missed_counts;
        let before_history = owner.missing_values;
        let second =
            produce_day(&second_raw, Some(first.records[23]), 4, policy, &mut owner).unwrap();
        assert_eq!(owner.missed_counts, before_counts);
        assert_eq!(owner.missing_values, before_history);
        let records = first
            .records
            .iter()
            .chain(&second.records)
            .copied()
            .collect::<Vec<_>>();
        let prior = WeatherTimestepSeries::from_records(&records, 4, policy);
        let current = first
            .samples
            .iter()
            .chain(&second.samples)
            .collect::<Vec<_>>();
        assert_eq!(current.len(), 192);
        for (actual, previous) in current.iter().zip(prior.timestep_samples()) {
            assert_eq!(actual.timestep, previous.timestep);
            assert_eq!(sample_bits(actual), sample_bits(previous));
        }
        assert_eq!(second.records.len(), 24);
        assert_eq!(second.samples[0].record_index, 0);
        assert_eq!(
            owner.last_hour.real_values().map(f64::to_bits),
            second.hourly_values[23].real_values().map(f64::to_bits)
        );
    }
}

#[test]
fn solar_next_hour_wraps_within_the_real_day_and_preserves_cache_canaries() {
    let mut raw = raw_day(1);
    for slot in &mut raw.hours {
        slot.raw.mandatory_reals[8] = 0.0;
    }
    raw.hours[0].raw.mandatory_reals[8] = 1000.0;
    let mut owner = state(4);
    owner.next_hour = WeatherVars {
        is_rain: true,
        is_snow: true,
        out_dry_bulb_temp: 91.0,
        out_dew_point_temp: 92.0,
        out_baro_press: 93.0,
        out_rel_hum: 94.0,
        wind_speed: 95.0,
        wind_dir: 96.0,
        sky_temp: -0.0,
        horiz_ir_sky: 98.0,
        beam_solar_rad: 99.0,
        dif_solar_rad: 100.0,
        albedo: 101.0,
        water_precip: 102.0,
        liquid_precip: 103.0,
        total_sky_cover: 104.0,
        opaque_sky_cover: 105.0,
    };
    let before = owner.next_hour;
    prepare_hourly(&raw, &mut owner);
    let result = produce_day(
        &raw,
        None,
        4,
        FirstHourInterpolationStartingValues::Hour24,
        &mut owner,
    )
    .unwrap();
    assert_eq!(result.records.len(), 24);
    assert_eq!(
        owner.tomorrow_values.hour(24).unwrap()[3]
            .beam_solar_rad
            .to_bits(),
        500.0_f64.to_bits()
    );
    let mut retained = owner.next_hour;
    retained.beam_solar_rad = before.beam_solar_rad;
    retained.dif_solar_rad = before.dif_solar_rad;
    retained.liquid_precip = before.liquid_precip;
    assert_eq!(retained.boolean_values(), before.boolean_values());
    assert_eq!(
        retained.real_values().map(f64::to_bits),
        before.real_values().map(f64::to_bits)
    );
    assert_eq!(owner.next_hour.beam_solar_rad, 1000.0);
    assert!(owner.weather.last_hour_set);
}

#[test]
fn producer_admission_preserves_preexisting_owners_and_one_step_caches() {
    let raw = raw_day(1);
    let mut owner = state(4);
    owner.last_hour = WeatherVars {
        sky_temp: -0.0,
        ..WeatherVars::default()
    };
    let before = owner.clone();
    assert!(
        produce_day(
            &raw,
            None,
            1,
            FirstHourInterpolationStartingValues::Hour24,
            &mut owner
        )
        .is_err()
    );
    assert_eq!(owner, before);
    assert_eq!(
        owner.last_hour.sky_temp.to_bits(),
        before.last_hour.sky_temp.to_bits()
    );
    let mut owner = state(1);
    owner.last_hour = before.last_hour;
    prepare_hourly(&raw, &mut owner);
    let previous = owner.clone();
    produce_day(
        &raw,
        None,
        1,
        FirstHourInterpolationStartingValues::Hour24,
        &mut owner,
    )
    .unwrap();
    assert_eq!(
        owner.last_hour.real_values().map(f64::to_bits),
        previous.last_hour.real_values().map(f64::to_bits)
    );
    assert_eq!(owner.next_hour, previous.next_hour);
    assert_eq!(owner.weather.last_hour_set, previous.weather.last_hour_set);
}

#[test]
fn producer_retains_processed_hourly_values_without_replaying_missing_counts() {
    let mut raw = raw_day(1);
    raw.hours[3].raw.mandatory_reals[0] = 99.9;
    raw.hours[3].raw.optional_reals[5] = 999.0;
    let mut owner = state(4);
    owner.missing_values.base.liquid_precip = 0.75;
    prepare_hourly(&raw, &mut owner);
    assert_eq!(owner.missed_counts.out_dry_bulb_temp, 1);
    assert_eq!(owner.missed_counts.liquid_precip, 1);
    let counts = owner.missed_counts;
    let history = owner.missing_values;
    let processed = owner.tomorrow_values.hour(4).unwrap()[0];
    let produced = produce_day(
        &raw,
        None,
        4,
        FirstHourInterpolationStartingValues::Hour24,
        &mut owner,
    )
    .unwrap();
    assert_eq!(produced.records[3].dry_bulb_c, 99.9);
    assert_eq!(produced.hourly_values[3], processed);
    assert_eq!(produced.hourly_values[3].out_dry_bulb_temp, 11.3);
    assert_eq!(produced.hourly_values[3].liquid_precip, 0.75);
    assert_eq!(
        owner.tomorrow_values.hour(4).unwrap()[3].out_dry_bulb_temp,
        11.3
    );
    assert_eq!(owner.missed_counts, counts);
    assert_eq!(owner.missing_values, history);
}

#[test]
fn missing_setup_is_an_admission_failure_without_cache_or_grid_writes() {
    let raw = raw_day(1);
    let mut owner = state(4);
    prepare_hourly(&raw, &mut owner);
    owner.weather.interpolation = None;
    owner.last_hour.wind_dir = -0.0;
    let before = owner.clone();
    assert!(
        produce_day(
            &raw,
            None,
            4,
            FirstHourInterpolationStartingValues::Hour24,
            &mut owner,
        )
        .is_err()
    );
    assert_eq!(owner, before);
    assert_eq!(owner.last_hour.wind_dir.to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn producer_consumes_stored_solar_weights_and_actual_hour_caches() {
    let raw = raw_day(1);
    let mut owner = state(4);
    prepare_hourly(&raw, &mut owner);
    owner.weather.solar_interpolation = Some(vec![0.0, 1.0, 0.0, 0.0]);
    let produced = produce_day(
        &raw,
        None,
        4,
        FirstHourInterpolationStartingValues::Hour24,
        &mut owner,
    )
    .unwrap();
    let first = owner.tomorrow_values.hour(1).unwrap();
    assert_eq!(
        first[0].beam_solar_rad.to_bits(),
        produced.hourly_values[23].beam_solar_rad.to_bits()
    );
    assert_eq!(
        first[1].beam_solar_rad.to_bits(),
        produced.hourly_values[0].beam_solar_rad.to_bits()
    );
    assert_eq!(
        first[2].beam_solar_rad.to_bits(),
        produced.hourly_values[1].beam_solar_rad.to_bits()
    );
    assert_eq!(
        first[3].dif_solar_rad.to_bits(),
        produced.hourly_values[1].dif_solar_rad.to_bits()
    );
}

#[test]
fn solar_array_shape_is_admitted_before_writes_and_one_step_bypasses_use() {
    let raw = raw_day(1);
    for array in [None, Some(vec![0.5; 3])] {
        let mut owner = state(4);
        prepare_hourly(&raw, &mut owner);
        owner.weather.solar_interpolation = array;
        let before = owner.clone();
        assert!(
            produce_day(
                &raw,
                None,
                4,
                FirstHourInterpolationStartingValues::Hour24,
                &mut owner,
            )
            .is_err()
        );
        assert_eq!(owner, before);
    }
    let mut owner = state(1);
    prepare_hourly(&raw, &mut owner);
    owner.weather.solar_interpolation = None;
    let before = owner.clone();
    let produced = produce_day(
        &raw,
        None,
        1,
        FirstHourInterpolationStartingValues::Hour24,
        &mut owner,
    )
    .unwrap();
    assert_eq!(
        owner.tomorrow_values.hour(1).unwrap()[0],
        produced.hourly_values[0]
    );
    assert_eq!(owner.last_hour, before.last_hour);
    assert_eq!(owner.next_hour, before.next_hour);
    assert_eq!(owner.weather.last_hour_set, before.weather.last_hour_set);
}

#[test]
fn raw_solar_locals_reach_the_producer_despite_a_different_record_projection() {
    let mut raw = raw_day(1);
    for slot in &mut raw.hours {
        slot.raw.mandatory_reals[8] = 0.0;
        slot.raw.mandatory_reals[9] = 0.0;
    }
    raw.hours[0].raw.mandatory_reals[8] = -8.0;
    raw.hours[0].raw.mandatory_reals[9] = -3.0;
    raw.hours[1].raw.mandatory_reals[8] = 9999.0;
    raw.hours[1].raw.mandatory_reals[9] = 9999.0;
    let before_raw = raw.hours.iter().map(|slot| slot.raw).collect::<Vec<_>>();
    for warnings in [false, true] {
        let mut owner = state(4);
        owner.environment.display_weather_missing_data_warnings = warnings;
        for (index, slot) in raw.hours.iter().enumerate() {
            let mut record = project_record(&slot.raw, slot.provenance.line_read_attempt).unwrap();
            // The selected source locals must come from raw references rather
            // than these deliberately different compatibility record fields.
            record.direct_normal_radiation_wh_per_m2 = 1234.0;
            record.diffuse_horizontal_radiation_wh_per_m2 = 5678.0;
            let hourly = process_hour(&slot.raw, record, &mut owner);
            owner.tomorrow_values.hour_mut(index + 1).unwrap()[0] = hourly;
        }
        let first_hour = owner.tomorrow_values.hour(1).unwrap()[0];
        let second_hour = owner.tomorrow_values.hour(2).unwrap()[0];
        assert_eq!(
            first_hour.beam_solar_rad.to_bits(),
            (if warnings { 0.0_f64 } else { -8.0_f64 }).to_bits()
        );
        assert_eq!(
            first_hour.dif_solar_rad.to_bits(),
            (if warnings { 0.0_f64 } else { -3.0_f64 }).to_bits()
        );
        assert_eq!(second_hour.beam_solar_rad, 0.0);
        assert_eq!(second_hour.dif_solar_rad, 0.0);
        assert_eq!(
            owner.out_of_range_counts.beam_solar_rad,
            i32::from(warnings)
        );
        assert_eq!(owner.out_of_range_counts.dif_solar_rad, i32::from(warnings));
        assert_eq!(owner.missed_counts.beam_solar_rad, i32::from(warnings));
        assert_eq!(
            owner.missed_counts.dif_solar_rad,
            if warnings { 2 } else { 0 }
        );
        let counts = (owner.missed_counts, owner.out_of_range_counts);
        let produced = produce_day(
            &raw,
            None,
            4,
            FirstHourInterpolationStartingValues::Hour1,
            &mut owner,
        )
        .unwrap();
        assert_eq!(produced.hourly_values[0], first_hour);
        assert_eq!(
            owner.tomorrow_values.hour(1).unwrap()[1]
                .beam_solar_rad
                .to_bits(),
            first_hour.beam_solar_rad.to_bits()
        );
        assert_eq!(
            owner.tomorrow_values.hour(1).unwrap()[1]
                .dif_solar_rad
                .to_bits(),
            first_hour.dif_solar_rad.to_bits()
        );
        assert_eq!((owner.missed_counts, owner.out_of_range_counts), counts);
    }
    assert_eq!(
        raw.hours.iter().map(|slot| slot.raw).collect::<Vec<_>>(),
        before_raw
    );
}
