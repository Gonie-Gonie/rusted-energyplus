use super::{all_routes, predecessor_with_operands, predecessor_with_original};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::transition::source_strict_less_than;

#[test]
fn raw_strict_less_than_covers_ieee_edges() {
    let nan = f64::from_bits(0x7ff8_0000_0000_4130);
    let subnormal = f64::from_bits(1);
    assert!(!source_strict_less_than(nan, 1.0));
    assert!(!source_strict_less_than(1.0, nan));
    assert!(source_strict_less_than(f64::NEG_INFINITY, f64::INFINITY));
    assert!(!source_strict_less_than(f64::INFINITY, f64::NEG_INFINITY));
    assert!(!source_strict_less_than(-0.0, 0.0));
    assert!(!source_strict_less_than(0.0, -0.0));
    assert!(source_strict_less_than(0.0, subnormal));
    assert!(!source_strict_less_than(subnormal, 0.0));
    assert!(!source_strict_less_than(subnormal, subnormal));
}

#[test]
fn private_transition_preserves_nan_payload_and_unordered_false() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.active)
        .expect("active route");
    let original = f64::from_bits(0x7ff8_0000_0000_4131);
    let predecessor = predecessor_with_original(route, 1, original);
    let snapshot = advance(&mut State::new(predecessor.system), predecessor)
        .expect("private NaN characterization");
    assert_eq!(
        snapshot.original_supply_humidity_ratio_for_guard.map(f64::to_bits),
        Some(original.to_bits()),
    );
    assert_eq!(
        snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        Some(false),
    );
    assert!(snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough);
}

#[test]
fn private_transition_handles_nan_saturation_without_coercion() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.active)
        .expect("active route");
    let pressure_nan = f64::from_bits(0x7ff8_0000_0000_4132);
    let predecessor = predecessor_with_operands(route, 1, 0.01, 18.0, pressure_nan);
    let saturation = predecessor
        .resulting_saturation_supply_humidity_ratio
        .expect("CP412 saturation result");
    assert!(saturation.is_nan());
    let snapshot = advance(&mut State::new(predecessor.system), predecessor)
        .expect("private NaN saturation characterization");
    assert_eq!(
        snapshot.saturation_supply_humidity_ratio_for_guard.map(f64::to_bits),
        Some(saturation.to_bits()),
    );
    assert_eq!(
        snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        Some(false),
    );
}

#[test]
fn equality_is_false_and_one_bit_drift_is_rejected() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.active)
        .expect("active route");
    let seed = predecessor_with_original(route, 1, 0.001);
    let saturation = seed
        .resulting_saturation_supply_humidity_ratio
        .expect("saturation");
    let predecessor = predecessor_with_original(route, 1, saturation);
    let mut snapshot = advance(&mut State::new(predecessor.system), predecessor)
        .expect("equal operands");
    assert_eq!(
        snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
        Some(false),
    );
    let value = snapshot
        .original_supply_humidity_ratio_for_guard
        .expect("original operand");
    snapshot.original_supply_humidity_ratio_for_guard =
        Some(f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact(snapshot));
}
