use super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};

mod public_release;
mod release_corruption;

fn completed_cp346_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
)> {
    let (mut runtime, system, predecessor) =
        completed_cp344_case(cooling_demand_w, overall_availability, capacity_limit);
    let cp345 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .ok()?;
    let cp346 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            cp345,
        )
        .ok()?;
    Some((runtime, system, cp346))
}

#[test]
fn pure_transition_rejects_active_input_mismatch_without_mutation() {
    let completed = completed_cp346_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((_runtime, system, predecessor)) = completed else {
        return;
    };
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
            system.id,
        );
    let before = state.clone();
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
            &mut state,
            predecessor,
            None,
        )
        .is_none()
    );
    assert_eq!(state, before);

    let input_value = predecessor
        .predecessor_assigned_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(input_value.is_some());
    let Some(input_value) = input_value else {
        return;
    };
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
            &mut state,
            predecessor,
            Some(
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput {
                    mixed_air_humidity_ratio: input_value,
                },
            ),
        )
        .is_none()
    );
    assert_eq!(state, before);

    let skipped = completed_cp346_case(1.0, 1.0, false);
    assert!(skipped.is_some());
    let Some((_, _, skipped)) = skipped else {
        return;
    };
    let before = state.clone();
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
            &mut state,
            skipped,
            Some(
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput {
                    mixed_air_humidity_ratio: 0.008,
                },
            ),
        )
        .is_none()
    );
    assert_eq!(state, before);
}

#[test]
fn pure_transition_retains_each_deferred_non_none_selection_without_none_sites() {
    for selector in [
        ep_model::DehumidificationControlType::ConstantSensibleHeatRatio,
        ep_model::DehumidificationControlType::Humidistat,
        ep_model::DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let completed = completed_cp346_case(-1_000.0, 1.0, false);
        assert!(completed.is_some());
        let Some((_, system, mut predecessor)) = completed else {
            return;
        };
        predecessor.dehumidification_control_type = Some(selector);
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
                system.id,
            );
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
                &mut state,
                predecessor,
                None,
            );
        assert!(snapshot.is_some());
        let Some(snapshot) = snapshot else {
            return;
        };
        assert!(!snapshot.dehumidification_control_none_case_entered);
        assert_eq!(state.source_site_execution_count, 0);
        assert!(snapshot.resulting_supply_humidity_ratio.is_none());
    }
}
