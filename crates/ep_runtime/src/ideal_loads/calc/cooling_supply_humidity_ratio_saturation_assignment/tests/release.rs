//! CP377 release-shape and fail-closed validation tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_saturation_assignment_characterization,
};
use super::predecessor_for_route;

#[test]
fn cp377_direct_snapshot_requires_finite_temperature_positive_pressure_and_finite_result() {
    let predecessor = predecessor_for_route(4, 0.008);
    let finite = private_cooling_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        Some(18.0),
        Some(Owner::Cp334MixedAirLimit),
        Some(101_325.0),
    )
    .expect("finite direct characterization");
    assert!(
        cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            finite,
        )
    );

    for (temperature, pressure) in [
        (f64::NAN, 101_325.0),
        (18.0, f64::NAN),
        (18.0, -0.0),
        (18.0, -1.0),
        (18.0, f64::INFINITY),
    ] {
        let raw = private_cooling_supply_humidity_ratio_saturation_assignment_characterization(
            predecessor,
            Some(temperature),
            Some(Owner::Cp334MixedAirLimit),
            Some(pressure),
        )
        .expect("pure raw characterization");
        assert!(
            !cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
                raw,
            )
        );
    }
}

#[test]
fn cp377_direct_snapshot_validation_recomputes_the_canonical_result() {
    let predecessor = predecessor_for_route(4, 0.008);
    let exact = private_cooling_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        Some(18.0),
        Some(Owner::Cp344CapacityMixedAirLimit),
        Some(101_325.0),
    )
    .expect("exact characterization");

    let mut corrupted = exact;
    corrupted.saturation_supply_humidity_ratio = corrupted
        .saturation_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            corrupted,
        )
    );
}

#[test]
fn cp377_complete_null_direct_skips_do_not_admit_operands() {
    for route in 0..3 {
        let predecessor = predecessor_for_route(route, 0.0);
        let skip = private_cooling_supply_humidity_ratio_saturation_assignment_characterization(
            predecessor,
            None,
            None,
            None,
        )
        .expect("complete-null skip");
        assert!(
            cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
                skip,
            )
        );
        assert!(
            private_cooling_supply_humidity_ratio_saturation_assignment_characterization(
                predecessor,
                Some(18.0),
                Some(Owner::Cp334MixedAirLimit),
                Some(101_325.0),
            )
            .is_none()
        );
    }
}
