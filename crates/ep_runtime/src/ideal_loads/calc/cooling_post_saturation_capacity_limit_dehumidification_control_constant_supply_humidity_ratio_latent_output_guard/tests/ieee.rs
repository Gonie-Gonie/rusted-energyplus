//! Raw IEEE-754 `>=` and bit-exact CP402 characterization.

use super::fixtures::{active_input, all_predecessors};
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshots_match_bit_exact,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization,
};

#[test]
fn raw_greater_than_or_equal_truth_table_includes_nan_infinities_and_signed_zero() {
    let compare = super::super::transition::source_greater_than_or_equal;
    assert!(compare(7.0, 7.0));
    assert!(compare(0.0, -0.0));
    assert!(compare(-0.0, 0.0));
    assert!(!compare(f64::NAN, 1.0));
    assert!(!compare(1.0, f64::NAN));
    assert!(compare(f64::INFINITY, 1.0));
    assert!(compare(f64::INFINITY, f64::INFINITY));
    assert!(!compare(f64::NEG_INFINITY, 1.0));
    assert!(compare(f64::NEG_INFINITY, f64::NEG_INFINITY));
    assert!(!compare(1.0, f64::INFINITY));
    assert!(compare(1.0, f64::NEG_INFINITY));
}

#[test]
fn private_characterization_preserves_nonfinite_capacity_bits_and_unordered_false() {
    let predecessor = all_predecessors()[20];
    let latent = predecessor.cooling_latent_output_w.expect("active latent");
    let nan = f64::from_bits(0x7ff8_1234_5678_9abc);
    for (capacity, expected) in [
        (nan, false),
        (f64::INFINITY, false),
        (f64::NEG_INFINITY, true),
        (latent, true),
    ] {
        let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization(
            predecessor,
            active_input(predecessor, capacity),
        )
        .expect("private active CP402 characterization");
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact(snapshot));
        assert_eq!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release(snapshot),
            capacity.is_finite() && capacity >= 0.0,
        );
        assert_eq!(
            snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits),
            Some(capacity.to_bits()),
        );
        assert_eq!(
            snapshot.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
            Some(expected),
        );
    }
}

#[test]
fn bit_exact_snapshot_matching_handles_nan_payloads_without_partial_eq() {
    let predecessor = all_predecessors()[20];
    let nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let input = ActiveInput {
        maximum_total_cooling_capacity_w: nan,
        ..active_input(predecessor, nan).expect("active input")
    };
    let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization(
        predecessor,
        Some(input),
    )
    .expect("NaN CP402 snapshot");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshots_match_bit_exact(
        snapshot,
        snapshot,
    ));
    let mut different_payload = snapshot;
    different_payload.maximum_total_cooling_capacity_w =
        Some(f64::from_bits(nan.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshots_match_bit_exact(
        snapshot,
        different_payload,
    ));
}
