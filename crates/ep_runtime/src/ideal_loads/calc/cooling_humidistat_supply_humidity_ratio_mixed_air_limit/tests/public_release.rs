use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit as advance,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
};

pub(super) fn completed_cp361_case() -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    completed_cp361_case_for(-1000.0, 1.0, false)
}

fn completed_cp361_case_for(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    let (mut runtime, system, cp355) =
        completed_cp355_case(cooling_demand_w, overall_availability, capacity_limit)?;
    let cp356 =
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            cp355,
        )
        .ok()?;
    let cp357 =
        advance_direct_no_oa_calc_cooling_constant_shr_case_break(&mut runtime, &system, cp356)
            .ok()?;
    let cp358 =
        advance_direct_no_oa_calc_cooling_humidistat_case_entry(&mut runtime, &system, cp357)
            .ok()?;
    let cp359 = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp358,
    )
    .ok()?;
    let cp360 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            cp359,
        )
        .ok()?;
    let cp361 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            cp360,
        )
        .ok()?;
    Some((runtime, system, cp361))
}

pub(super) fn completed_cp362_case() -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Snapshot,
)> {
    let (mut runtime, system, predecessor) = completed_cp361_case()?;
    let snapshot = advance(&mut runtime, &system, predecessor).ok()?;
    Some((runtime, system, snapshot))
}

#[test]
fn public_release_retains_exact_cp361_link_and_complete_null_snapshot() {
    let (mut runtime, system, predecessor) = completed_cp361_case().unwrap();
    let snapshot = advance(&mut runtime, &system, predecessor).unwrap();
    assert!(
        cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(
        cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
            snapshot,
            predecessor,
        )
    );
    let state = &runtime
        .units
        .get(&system.id)
        .unwrap()
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn public_wrapper_u_n_p_c0_routes_are_complete_null_and_do_not_read_operands() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
        (-1_000.0, 1.0, false, (false, false, false, true)),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp361_case_for(demand, availability, capacity).unwrap();
        let snapshot = advance(&mut runtime, &system, predecessor).unwrap();
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
                snapshot,
                predecessor,
            )
        );
        for flag in [
            snapshot.mixed_air_humidity_ratio_for_minimum_read,
            snapshot.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read,
            snapshot.source_shaped_two_argument_minimum_evaluated,
            snapshot.supply_humidity_ratio_assignment_performed,
        ] {
            assert!(!flag);
        }
        assert_eq!(
            [
                snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
                snapshot.mixed_air_humidity_ratio,
                snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
                snapshot.minimum_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ],
            [None; 6]
        );
    }
}

#[test]
fn owner_predecessor_and_witness_corruption_is_rejected_transactionally() {
    let (runtime, system, predecessor) = completed_cp361_case().unwrap();

    let mut corrupt_latest = runtime.clone();
    corrupt_latest
        .units
        .get_mut(&system.id)
        .unwrap()
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
        .latest
        .as_mut()
        .unwrap()
        .parent_call_ordinal += 1;
    let before = corrupt_latest.clone();
    assert!(advance(&mut corrupt_latest, &system, predecessor).is_err());
    assert_eq!(corrupt_latest, before);

    let mut corrupt_witness = runtime.clone();
    let mut forged_witness = predecessor;
    forged_witness.controlled_zone = ep_model::ZoneId(999);
    corrupt_witness
        .set_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
            system.id,
            forged_witness,
        );
    let before = corrupt_witness.clone();
    assert!(advance(&mut corrupt_witness, &system, predecessor).is_err());
    assert_eq!(corrupt_witness, before);

    let mut corrupt_owner_state = runtime;
    corrupt_owner_state
        .units
        .get_mut(&system.id)
        .unwrap()
        .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .source_site_execution_count = 1;
    let before = corrupt_owner_state.clone();
    assert!(advance(&mut corrupt_owner_state, &system, predecessor).is_err());
    assert_eq!(corrupt_owner_state, before);
}

#[test]
fn replay_is_rejected_transactionally_without_redistributing_counts() {
    let (mut runtime, system, predecessor) = completed_cp361_case().unwrap();
    advance(&mut runtime, &system, predecessor).unwrap();
    let before = runtime.clone();
    assert!(advance(&mut runtime, &system, predecessor).is_err());
    assert_eq!(runtime, before);
}
