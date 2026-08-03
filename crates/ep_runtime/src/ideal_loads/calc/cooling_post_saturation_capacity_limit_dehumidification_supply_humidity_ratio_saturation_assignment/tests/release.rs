//! CP412 public-release shape and fail-closed snapshot tests.

use super::{all_routes, predecessor_for_route, predecessor_with_temperature};
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization,
};

#[test]
fn direct_active_snapshot_requires_finite_temperature_positive_pressure_and_finite_result() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let finite_predecessor = predecessor_with_temperature(route, 1, 18.0);
    let finite = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
        finite_predecessor,
        Some(101_325.0),
    )
    .expect("finite direct characterization");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(finite));

    for (temperature, pressure) in [
        (f64::NAN, 101_325.0),
        (18.0, f64::NAN),
        (18.0, -0.0),
        (18.0, -1.0),
        (18.0, f64::INFINITY),
    ] {
        let predecessor = predecessor_with_temperature(route, 1, temperature);
        let raw = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
            predecessor,
            Some(pressure),
        )
        .expect("pure raw characterization");
        assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(raw));
    }
}

#[test]
fn direct_snapshot_recomputes_the_canonical_result() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let predecessor = predecessor_with_temperature(route, 1, 18.0);
    let exact = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        Some(101_325.0),
    )
    .expect("exact characterization");
    let mut corrupted = exact;
    corrupted.saturation_supply_humidity_ratio = corrupted
        .saturation_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(corrupted));
}

#[test]
fn direct_complete_null_skip_ignores_pressure_and_private_route_is_not_public() {
    let inactive = all_routes()[0];
    let predecessor = predecessor_for_route(inactive, 1);
    let skip = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        None,
    )
    .expect("complete-null skip");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(skip));
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        Some(101_325.0),
    )
    .is_none());

    let private_route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 18)
        .expect("active private route");
    let predecessor = predecessor_with_temperature(private_route, 1, 18.0);
    let private = private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        Some(101_325.0),
    )
    .expect("private characterization");
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(private));
}
