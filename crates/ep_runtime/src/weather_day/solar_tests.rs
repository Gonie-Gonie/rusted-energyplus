use super::{interpolation_weights, process_hourly_solar};
use crate::weather::day::{WeatherDayState, WeatherVars};

#[test]
fn false_warning_control_retains_negative_and_signed_zero_solar() {
    let mut state = WeatherDayState::default();
    let before_counts = state.missed_counts;
    let mut value = WeatherVars {
        beam_solar_rad: -4.0,
        dif_solar_rad: -0.0,
        ..WeatherVars::default()
    };
    process_hourly_solar(&mut value, &mut state);
    assert_eq!(value.beam_solar_rad.to_bits(), (-4.0_f64).to_bits());
    assert_eq!(value.dif_solar_rad.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(state.missed_counts, before_counts);
    assert_eq!(state.out_of_range_counts, Default::default());
}

#[test]
fn warning_counts_preserve_diffuse_assignment_and_negative_range_order() {
    let mut state = WeatherDayState::default();
    state.environment.display_weather_missing_data_warnings = true;
    state.missed_counts.beam_solar_rad = 5;
    state.missed_counts.dif_solar_rad = 91;
    let mut value = WeatherVars {
        beam_solar_rad: 9999.0,
        dif_solar_rad: 9999.0,
        ..WeatherVars::default()
    };
    process_hourly_solar(&mut value, &mut state);
    assert_eq!(state.missed_counts.beam_solar_rad, 6);
    assert_eq!(state.missed_counts.dif_solar_rad, 7);
    value.beam_solar_rad = -1.0;
    value.dif_solar_rad = -2.0;
    process_hourly_solar(&mut value, &mut state);
    assert_eq!(state.missed_counts.beam_solar_rad, 6);
    assert_eq!(state.missed_counts.dif_solar_rad, 7);
    assert_eq!(state.out_of_range_counts.beam_solar_rad, 1);
    assert_eq!(state.out_of_range_counts.dif_solar_rad, 1);
    assert_eq!([value.beam_solar_rad, value.dif_solar_rad], [0.0, 0.0]);
}

#[test]
fn beam_diffuse_and_all_ignore_controls_are_independent() {
    for (all, beam, diffuse) in [
        (false, true, false),
        (false, false, true),
        (true, false, false),
    ] {
        let mut state = WeatherDayState::default();
        state.environment.ignore_solar_radiation = all;
        state.environment.ignore_beam_radiation = beam;
        state.environment.ignore_diffuse_radiation = diffuse;
        let mut value = WeatherVars {
            beam_solar_rad: 24.0,
            dif_solar_rad: 13.0,
            ..WeatherVars::default()
        };
        process_hourly_solar(&mut value, &mut state);
        assert_eq!(value.beam_solar_rad, if all || beam { 0.0 } else { 24.0 });
        assert_eq!(value.dif_solar_rad, if all || diffuse { 0.0 } else { 13.0 });
    }
}

#[test]
fn exact_midpoint_does_not_absorb_near_one_stored_weight() {
    let below_one = f64::from_bits(1.0_f64.to_bits() - 1);
    assert_eq!(interpolation_weights(1.0, 2, 0.25), (0.0, 0.0));
    let before = interpolation_weights(below_one, 1, 0.25);
    let after = interpolation_weights(below_one, 2, 0.25);
    assert!(before.0 > 0.0);
    assert_eq!(before.1.to_bits(), 0.0_f64.to_bits());
    assert_eq!(after.0.to_bits(), 0.0_f64.to_bits());
    assert!(after.1 > 0.0);
}
