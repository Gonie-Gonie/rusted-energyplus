use super::{
    moisture_demand::{
        NoOaThirdOrderHumidityCorrectorInput, NoOaThirdOrderMoistureDemandInput,
        calc_no_oa_third_order_moisture_demand_compat,
        correct_no_oa_third_order_humidity_ratio_compat, third_order_humidity_history_term,
    },
    types::IdealLoadsZoneState,
};
use crate::{
    energyplus_moist_air_density_kg_per_m3, energyplus_psychrometric_humidity_ratio_from_rh,
    energyplus_water_vapor_gas_enthalpy_j_per_kg,
};

#[test]
fn third_order_humidity_history_term_matches_energyplus_coefficients() {
    let history = [0.0085, 0.0082, 0.0081];
    let expected = 3.0 * history[0] - 1.5 * history[1] + (1.0 / 3.0) * history[2];

    assert_close(
        third_order_humidity_history_term(history),
        expected,
        1.0e-15,
    );
}

#[test]
fn no_oa_third_order_moisture_demand_matches_energyplus_predictor_formula() {
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 24.0,
        air_humidity_ratio: 0.008,
    };
    let input = NoOaThirdOrderMoistureDemandInput {
        zone_state,
        previous_zone_timestep_humidity_ratios: [0.0085, 0.0082, 0.0081],
        zone_volume_m3: 1.0,
        zone_moisture_capacity_multiplier: 1.0,
        timestep_seconds: 900.0,
        barometric_pressure_pa: 83_411.819_665_895_82,
        latent_gain_w: 600.0,
        humidifying_relative_humidity_percent: 10.0,
        dehumidifying_relative_humidity_percent: 45.0,
        zone_multiplier: 2.0,
    };

    let result = calc_no_oa_third_order_moisture_demand_compat(input)
        .expect("predictor input should be valid");

    let density = energyplus_moist_air_density_kg_per_m3(
        input.barometric_pressure_pa,
        zone_state.air_temperature_c,
        zone_state.air_humidity_ratio,
    )
    .expect("density should be valid");
    let c = density * input.zone_volume_m3 * input.zone_moisture_capacity_multiplier / 900.0;
    let b = input.latent_gain_w
        / energyplus_water_vapor_gas_enthalpy_j_per_kg(zone_state.air_temperature_c);
    let history = 3.0 * 0.0085 - 1.5 * 0.0082 + (1.0 / 3.0) * 0.0081;
    let expected_humidifying = ((11.0 / 6.0) * c * result.humidifying_setpoint_humidity_ratio
        - (b + c * history))
        * input.zone_multiplier;
    let expected_dehumidifying = ((11.0 / 6.0) * c * result.dehumidifying_setpoint_humidity_ratio
        - (b + c * history))
        * input.zone_multiplier;

    assert_close(
        result.humidifying_setpoint_load_kg_per_s,
        expected_humidifying,
        1.0e-15,
    );
    assert_close(
        result.dehumidifying_setpoint_load_kg_per_s,
        expected_dehumidifying,
        1.0e-15,
    );
    assert_close(
        result.total_output_required_kg_per_s,
        expected_dehumidifying,
        1.0e-15,
    );
}

#[test]
fn no_oa_third_order_moisture_demand_clamps_invalid_setpoint_order() {
    let input = NoOaThirdOrderMoistureDemandInput {
        zone_state: IdealLoadsZoneState {
            air_temperature_c: 22.0,
            air_humidity_ratio: 0.007,
        },
        previous_zone_timestep_humidity_ratios: [0.007, 0.007, 0.007],
        zone_volume_m3: 10.0,
        zone_moisture_capacity_multiplier: 1.0,
        timestep_seconds: 900.0,
        barometric_pressure_pa: 101_325.0,
        latent_gain_w: 0.0,
        humidifying_relative_humidity_percent: 60.0,
        dehumidifying_relative_humidity_percent: 45.0,
        zone_multiplier: 1.0,
    };

    let result = calc_no_oa_third_order_moisture_demand_compat(input)
        .expect("predictor input should be valid");

    assert_close(
        result.humidifying_setpoint_humidity_ratio,
        result.dehumidifying_setpoint_humidity_ratio,
        1.0e-15,
    );
    assert_close(
        result.total_output_required_kg_per_s,
        result.humidifying_setpoint_load_kg_per_s,
        1.0e-15,
    );
}

#[test]
fn no_oa_third_order_humidity_corrector_matches_energyplus_formula() {
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 24.0,
        air_humidity_ratio: 0.008,
    };
    let input = NoOaThirdOrderHumidityCorrectorInput {
        zone_state,
        previous_zone_timestep_humidity_ratios: [0.0085, 0.0082, 0.0081],
        zone_volume_m3: 1.0,
        zone_moisture_capacity_multiplier: 1.0,
        timestep_seconds: 900.0,
        barometric_pressure_pa: 83_411.819_665_895_82,
        latent_gain_w: 600.0,
        supply_mass_flow_rate_kg_per_s: 0.05,
        supply_humidity_ratio: 0.007,
    };

    let result = correct_no_oa_third_order_humidity_ratio_compat(input)
        .expect("corrector input should be valid");

    let density = energyplus_moist_air_density_kg_per_m3(
        input.barometric_pressure_pa,
        zone_state.air_temperature_c,
        zone_state.air_humidity_ratio,
    )
    .expect("density should be valid");
    let c = density * input.zone_volume_m3 * input.zone_moisture_capacity_multiplier
        / input.timestep_seconds;
    let b = input.latent_gain_w
        / energyplus_water_vapor_gas_enthalpy_j_per_kg(zone_state.air_temperature_c)
        + input.supply_mass_flow_rate_kg_per_s * input.supply_humidity_ratio;
    let a = input.supply_mass_flow_rate_kg_per_s;
    let history = 3.0 * 0.0085 - 1.5 * 0.0082 + (1.0 / 3.0) * 0.0081;
    let expected = (b + c * history) / ((11.0 / 6.0) * c + a);

    assert_close(result.zone_air_humidity_ratio, expected, 1.0e-15);
    assert_close(result.b_kg_water_per_s, b, 1.0e-15);
    assert_close(result.a_kg_dry_air_per_s, a, 1.0e-15);
}

#[test]
fn no_oa_third_order_humidity_corrector_clamps_to_saturation() {
    let input = NoOaThirdOrderHumidityCorrectorInput {
        zone_state: IdealLoadsZoneState {
            air_temperature_c: 10.0,
            air_humidity_ratio: 0.006,
        },
        previous_zone_timestep_humidity_ratios: [0.006, 0.006, 0.006],
        zone_volume_m3: 1.0,
        zone_moisture_capacity_multiplier: 1.0,
        timestep_seconds: 900.0,
        barometric_pressure_pa: 101_325.0,
        latent_gain_w: 0.0,
        supply_mass_flow_rate_kg_per_s: 1.0,
        supply_humidity_ratio: 0.5,
    };

    let result = correct_no_oa_third_order_humidity_ratio_compat(input)
        .expect("corrector input should be valid");
    let saturation = energyplus_psychrometric_humidity_ratio_from_rh(
        input.zone_state.air_temperature_c,
        1.0,
        input.barometric_pressure_pa,
    )
    .expect("saturation humidity should be valid");

    assert_close(result.zone_air_humidity_ratio, saturation, 1.0e-15);
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let delta = (actual - expected).abs();
    assert!(
        delta <= tolerance,
        "actual {actual} expected {expected} delta {delta} > {tolerance}"
    );
}
