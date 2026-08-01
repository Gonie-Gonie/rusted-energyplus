//! CP376 raw IEEE copy tests.

use super::super::{
    advance_cooling_supply_humidity_ratio_pre_saturation_original_assignment_state as advance,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact,
    private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentOwner as Owner,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState as State,
};
use super::release::completed_cp375_case;
use super::routes::predecessor_for_route;

#[test]
fn cp376_copy_preserves_signed_zero_and_quiet_nan_payloads() {
    let (_, _, direct) = completed_cp375_case();
    for bits in [
        (-0.0f64).to_bits(),
        0.0f64.to_bits(),
        0x0000_0000_0000_0001,
        f64::INFINITY.to_bits(),
        0x7ff8_0000_0000_0376,
    ] {
        let value = f64::from_bits(bits);
        let predecessor = predecessor_for_route(direct, 4, value);
        let snapshot = private_cooling_supply_humidity_ratio_pre_saturation_original_assignment_characterization(
            predecessor,
            Some(value),
            Some(Owner::Cp347NoneCase),
        )
        .expect("pure CP376 copy");
        assert_eq!(
            snapshot
                .purchased_air_supply_humidity_ratio_before_saturation_check
                .map(f64::to_bits),
            Some(bits),
        );
        assert_eq!(
            snapshot
                .assigned_supply_humidity_ratio_original
                .map(f64::to_bits),
            Some(bits),
        );
        assert!(cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact(
            snapshot, snapshot,
        ));
    }
}

#[test]
fn cp376_cp375_owned_copy_rejects_even_one_bit_of_operand_drift() {
    let (_, _, direct) = completed_cp375_case();
    let payload = f64::from_bits(0x7ff8_0000_0000_0042);
    let predecessor = predecessor_for_route(direct, 6, payload);
    let exact = ActiveInput {
        purchased_air_supply_humidity_ratio: payload,
        owner: Owner::Cp375MaximumAssignment,
    };
    let mut state = State::new(predecessor.system);
    assert!(advance(&mut state, predecessor, Some(exact)).is_some());

    let drift = ActiveInput {
        purchased_air_supply_humidity_ratio: f64::from_bits(payload.to_bits() ^ 1),
        ..exact
    };
    let mut state = State::new(predecessor.system);
    assert!(advance(&mut state, predecessor, Some(drift)).is_none());
    assert_eq!(state, State::new(predecessor.system));
}
