//! CP392 canonical `PsyWFnTdbH` binary64 tests.

use super::*;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

#[test]
fn canonical_inverse_uses_strict_negative_floor_without_extra_normalization() {
    let negative = energyplus_psy_w_fn_tdb_h(0.0, -1.0);
    let below_floor_but_positive = energyplus_psy_w_fn_tdb_h(0.0, 1.0);
    let negative_zero = energyplus_psy_w_fn_tdb_h(0.0, -0.0);
    let nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let propagated_nan = energyplus_psy_w_fn_tdb_h(0.0, nan);
    let negative_infinity = energyplus_psy_w_fn_tdb_h(20.0, f64::NEG_INFINITY);
    let pole_c = -2_500_940.0 / 1_858.95;

    assert_eq!(negative.to_bits(), 1.0e-5f64.to_bits());
    assert!(below_floor_but_positive > 0.0);
    assert!(below_floor_but_positive < 1.0e-5);
    assert_eq!(negative_zero.to_bits(), (-0.0f64).to_bits());
    assert_eq!(propagated_nan.to_bits(), nan.to_bits());
    assert_eq!(negative_infinity.to_bits(), 1.0e-5f64.to_bits());
    assert_eq!(energyplus_psy_w_fn_tdb_h(0.0, f64::INFINITY), f64::INFINITY);
    assert_eq!(energyplus_psy_w_fn_tdb_h(pole_c, 0.0), f64::INFINITY);
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(pole_c.next_down(), 0.0).to_bits(),
        1.0e-5f64.to_bits(),
    );
    assert!(energyplus_psy_w_fn_tdb_h(pole_c.next_up(), 0.0).is_finite());
}

#[test]
fn active_routes_assign_the_canonical_inverse_and_preserve_carrier_bits() {
    for inherited in [3, 4, 7] {
        let chain = fixtures::chain(
            inherited,
            1,
            true,
            Some(D::ConstantSensibleHeatRatio),
            1,
            0.7,
            18.0,
            1.0,
        );
        let mut state = State::new(chain.cp391.system);
        let snapshot = advance(&mut state, chain.cp391).expect("active CP392");
        let temperature = chain
            .cp391
            .resulting_supply_temperature_c
            .expect("temperature");
        let enthalpy = chain
            .cp391
            .resulting_supply_enthalpy_j_per_kg
            .expect("enthalpy");
        let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);

        assert_eq!(
            snapshot.supply_temperature_c.map(f64::to_bits),
            Some(temperature.to_bits())
        );
        assert_eq!(
            snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(enthalpy.to_bits())
        );
        for value in [
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
        }
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            chain.cp391.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            chain
                .cp391
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
    }
}

#[test]
fn source_decimal_floor_literal_has_locked_binary64_bits() {
    assert_eq!((1.0e-5f64).to_bits(), 0x3ee4_f8b5_88e3_68f1);
}

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState;

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot>{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state(state, predecessor)
}
