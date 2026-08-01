use ep_model::{DehumidificationControlType, HumidificationControlType};

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_case_break,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard,
};

#[test]
fn direct_release_skips_sites_and_canonical_private_bridge_uses_five() {
    let (mut runtime, system, predecessor) =
        completed_cp370_case().expect("CP370 fixture must complete");

    let direct =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP371 direct release");
    assert!(
        cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_snapshot_is_exact_direct_release(
            direct,
        )
    );
    assert!(direct.predecessor_humidification_control_guard_false_fallthrough);
    assert!(!direct.dehumidification_control_type_first_read);
    assert!(!direct.dehumidification_control_type_second_read);
    assert!(!direct.dehumidification_control_body_entered);
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.humidification_control_guard_false_fallthrough_count,
        1
    );
    assert_eq!(state.dehumidification_control_type_first_read_count, 0);
    assert_eq!(state.source_site_execution_count, 0);

    let unit = runtime.units.get(&system.id).expect("known unit");
    let private =
        private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_from_direct_release(
            &runtime,
            unit,
            &system,
            direct,
        )
        .expect("canonical CP370 Humidistat bridge");
    assert_eq!(
        private.predecessor_humidification_control_type,
        Some(HumidificationControlType::Humidistat),
    );
    assert_eq!(
        private.first_dehumidification_control_type,
        Some(DehumidificationControlType::None),
    );
    assert_eq!(
        private.dehumidification_control_type_humidistat,
        Some(false)
    );
    assert!(private.dehumidification_control_type_second_read);
    assert_eq!(
        private.second_dehumidification_control_type,
        Some(DehumidificationControlType::None),
    );
    assert_eq!(private.dehumidification_control_type_none, Some(true));
    assert!(private.dehumidification_control_body_entered);
    assert!(!private.dehumidification_control_guard_false_fallthrough);
    assert_eq!(private.source_order.len(), 5);
    assert!(
        private_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_counterfactual_links_to_direct_release(
            &runtime,
            unit,
            &system,
            direct,
            private,
        )
    );
}

#[test]
fn supplied_cp370_drift_is_rejected_transactionally() {
    let (mut runtime, system, mut predecessor) =
        completed_cp370_case().expect("CP370 fixture must complete");
    predecessor.humidification_control_type = Some(HumidificationControlType::Humidistat);
    predecessor.humidification_control_type_humidistat = Some(true);
    predecessor.humidification_control_body_entered = true;
    predecessor.humidification_control_guard_false_fallthrough = false;

    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(Error::CoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp370_private_witness_drift_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) =
        completed_cp370_case().expect("CP370 fixture must complete");
    let mut witness = runtime
        .cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(
            system.id,
        )
        .expect("CP370 witness");
    witness.humidification_control_type_read = false;
    runtime
        .set_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witness(
            system.id, witness,
        );

    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(Error::CoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp346_selected_control_witness_drift_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) =
        completed_cp370_case().expect("CP370 fixture must complete");
    let mut witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
        )
        .expect("CP346 witness");
    witness.dehumidification_control_switch_dispatched =
        !witness.dehumidification_control_switch_dispatched;
    runtime.set_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
        system.id,
        witness,
    );

    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(Error::DehumidificationControlTypeProvenanceMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn pending_counter_overflow_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) =
        completed_cp370_case().expect("CP370 fixture must complete");
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard
        .dehumidification_control_type_first_read_count = usize::MAX;

    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

pub(in crate::ideal_loads::calc) fn completed_cp370_case() -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
)> {
    completed_cp370_case_with_capacity_limit(false)
}

pub(in crate::ideal_loads::calc) fn completed_cp370_case_with_capacity_limit(
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
)> {
    let (mut runtime, mut system, cp355) = completed_cp355_case(-1_000.0, 1.0, capacity_limit)?;
    system.minimum_cooling_supply_air_humidity_ratio = 0.0077;
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
    let cp362 = advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit(
        &mut runtime,
        &system,
        cp361,
    )
    .ok()?;
    let cp363 =
        advance_direct_no_oa_calc_cooling_humidistat_case_break(&mut runtime, &system, cp362)
            .ok()?;
    let cp364 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry(
        &mut runtime,
        &system,
        cp363,
    )
    .ok()?;
    let cp365 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        cp364,
    )
    .ok()?;
    let cp366 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        cp365,
    )
    .ok()?;
    let cp367 =
        advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            cp366,
        )
        .ok()?;
    let cp368 = advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        cp367,
    )
    .ok()?;
    let cp369 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard(
            &mut runtime,
            &system,
            cp368,
        )
        .ok()?;
    let cp370 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard(
            &mut runtime,
            &system,
            cp369,
        )
        .ok()?;
    Some((runtime, system, cp370))
}
