//! CP375 branch-specific result-store owner tests.

use super::*;
use super::super::active_humidistat_operands_from_cp362_counterfactual;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;

#[test]
fn cp375_humidistat_left_is_the_validated_same_call_cp362_result() {
    let (runtime, system, _) = completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let unit = runtime.units.get(&system.id).expect("known system");
    let mut predecessor =
        active_cp374(DehumidificationControlType::Humidistat, 0.009, 0.008);
    align_with_retained_cp362(unit, &mut predecessor);
    let operands = active_humidistat_operands_from_cp362_counterfactual(
        &runtime,
        unit,
        &system,
        predecessor,
        -0.001,
        0.008,
    )
    .expect("validated CP362 Humidistat owner");
    let right = predecessor
        .resulting_supply_humidity_ratio_for_humidification
        .expect("CP374 right owner");
    let expected = if operands.purchased_air_supply_humidity_ratio < right {
        right
    } else {
        operands.purchased_air_supply_humidity_ratio
    };
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, Some(operands)).expect("CP375 H route");

    assert!(
        snapshot.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed
    );
    assert_eq!(
        snapshot
            .purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
            .map(f64::to_bits),
        Some(operands.purchased_air_supply_humidity_ratio.to_bits())
    );
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
        Some(expected.to_bits())
    );
}

#[test]
fn cp375_humidistat_owner_rejects_wrong_call_identity() {
    let (runtime, system, _) = completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let unit = runtime.units.get(&system.id).expect("known system");
    let mut predecessor =
        active_cp374(DehumidificationControlType::Humidistat, 0.009, 0.008);
    align_with_retained_cp362(unit, &mut predecessor);
    predecessor.parent_call_ordinal = predecessor.parent_call_ordinal.saturating_add(1);
    assert!(
        active_humidistat_operands_from_cp362_counterfactual(
            &runtime,
            unit,
            &system,
            predecessor,
            -0.001,
            0.008,
        )
        .is_none()
    );
}

fn align_with_retained_cp362(
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    predecessor: &mut Predecessor,
) {
    let owner = unit
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest
        .expect("retained direct CP362");
    predecessor.system = owner.system;
    predecessor.parent_call_ordinal = owner.parent_call_ordinal;
    predecessor.controlled_zone = owner.controlled_zone;
}
