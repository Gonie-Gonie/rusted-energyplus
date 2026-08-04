//! CP413 public-release shape and fail-closed snapshot tests.

use super::{all_routes, predecessor_for_outcome, predecessor_for_route, predecessor_with_operands, predecessor_with_original};
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization,
};

#[test]
fn direct_active_snapshot_requires_finite_saturation_and_original_operands() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let finite_predecessor = predecessor_for_outcome(route, 1, false);
    let finite = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        finite_predecessor,
    )
    .expect("finite direct characterization");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(finite));

    let original_nan = predecessor_with_original(
        route,
        1,
        f64::from_bits(0x7ff8_0000_0000_4133),
    );
    let raw_original = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        original_nan,
    )
    .expect("raw original characterization");
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(raw_original));

    let saturation_nan = predecessor_with_operands(
        route,
        1,
        0.01,
        f64::from_bits(0x7ff8_0000_0000_4134),
        101_325.0,
    );
    let raw_saturation = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        saturation_nan,
    )
    .expect("raw saturation characterization");
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(raw_saturation));
}

#[test]
fn direct_complete_null_skip_and_private_route_are_distinct() {
    let inactive = all_routes()[0];
    let predecessor = predecessor_for_route(inactive, 1);
    let skip = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        predecessor,
    )
    .expect("complete-null skip");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(skip));
    assert!(skip.saturation_supply_humidity_ratio_for_guard.is_none());
    assert!(skip.original_supply_humidity_ratio_for_guard.is_none());

    let private_route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 18)
        .expect("active private route");
    let predecessor = predecessor_for_outcome(private_route, 1, false);
    let private = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        predecessor,
    )
    .expect("private characterization");
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(private));
}

#[test]
fn original_owner_corroboration_and_local_bits_are_fail_closed() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.active)
        .expect("active route");
    let mut predecessor = predecessor_for_outcome(route, 1, false);
    let original = predecessor
        .predecessor_cp411_resulting_supply_humidity_ratio
        .expect("corroborated original");
    predecessor.predecessor_cp411_resulting_supply_humidity_ratio =
        Some(f64::from_bits(original.to_bits() ^ 1));
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        predecessor,
    )
    .is_none());
}
