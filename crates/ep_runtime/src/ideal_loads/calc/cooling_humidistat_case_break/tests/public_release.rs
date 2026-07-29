use super::*;

#[test]
fn public_direct_routes_skip_break_and_private_h_uses_only_cp362_bridge() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
        (-1_000.0, 1.0, false, (false, false, false, true)),
    ] {
        let fixture = completed_cp362_case_for(demand, availability, capacity);
        assert!(fixture.is_some(), "CP362 direct route fixture must succeed");
        let Some((mut runtime, system, predecessor)) = fixture else {
            return;
        };
        let transition = advance_direct_no_oa_calc_cooling_humidistat_case_break(
            &mut runtime,
            &system,
            predecessor,
        );
        assert!(transition.is_ok(), "CP363 direct transition must succeed");
        let Ok(snapshot) = transition else {
            return;
        };
        assert!(cooling_humidistat_case_break_snapshot_is_exact_direct_release(snapshot));
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert!(!snapshot.dehumidification_control_humidistat_case_exited_via_break);
        let summary =
            purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary(&runtime, system.id);
        assert!(summary.is_ok(), "CP363 lifecycle summary must succeed");
        let Ok(summary) = summary else {
            return;
        };
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }

    let fixture = completed_cp362_case_for(-100_000.0, 1.0, true);
    assert!(fixture.is_some(), "CP362 Humidistat fixture must succeed");
    let Some((mut runtime, system, predecessor)) = fixture else {
        return;
    };
    let direct =
        advance_direct_no_oa_calc_cooling_humidistat_case_break(&mut runtime, &system, predecessor);
    assert!(direct.is_ok(), "CP363 direct bridge seed must succeed");
    let Ok(direct) = direct else {
        return;
    };
    let unit = runtime.units.get(&system.id);
    assert!(unit.is_some(), "CP363 Humidistat runtime unit must exist");
    let Some(unit) = unit else {
        return;
    };
    let private_h = private_humidistat_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        PRIVATE_DEMAND,
        PRIVATE_ZONE_HUMIDITY,
    );
    assert!(
        private_h.is_some(),
        "CP363 private Humidistat bridge must succeed"
    );
    let Some(private_h) = private_h else {
        return;
    };
    assert!(private_h.dehumidification_control_humidistat_case_exited_via_break);
    assert!(!private_h.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip);
    assert!(private_humidistat_counterfactual_links_to_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        private_h,
        PRIVATE_DEMAND,
        PRIVATE_ZONE_HUMIDITY,
    ));
}

#[test]
fn private_constant_supply_bridge_is_canonical_for_cp364() {
    let fixture = completed_cp362_case_for(-1_000.0, 1.0, false);
    assert!(fixture.is_some(), "CP362 CSH fixture must succeed");
    let Some((mut runtime, system, predecessor)) = fixture else {
        return;
    };
    let direct =
        advance_direct_no_oa_calc_cooling_humidistat_case_break(&mut runtime, &system, predecessor);
    assert!(direct.is_ok(), "CP363 direct CSH bridge seed must succeed");
    let Ok(direct) = direct else {
        return;
    };
    let unit = runtime.units.get(&system.id);
    assert!(unit.is_some(), "CP363 CSH runtime unit must exist");
    let Some(unit) = unit else {
        return;
    };
    let private_csh = private_constant_supply_humidity_ratio_counterfactual_from_direct_release(
        &runtime, unit, &system, direct,
    );
    assert!(
        private_csh.is_some(),
        "CP363 private CSH bridge must succeed"
    );
    let Some(private_csh) = private_csh else {
        return;
    };
    assert_eq!(
        private_csh.predecessor_dehumidification_control_type,
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
    );
    assert!(private_csh.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip);
    assert!(!private_csh.dehumidification_control_humidistat_case_exited_via_break);
    assert!(
        private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release(
            &runtime,
            unit,
            &system,
            direct,
            private_csh,
        )
    );
    let mut forged = private_csh;
    forged.parent_call_ordinal = forged.parent_call_ordinal.wrapping_add(1);
    assert!(
        !private_constant_supply_humidity_ratio_counterfactual_links_to_direct_release(
            &runtime, unit, &system, direct, forged,
        )
    );
}

#[test]
fn corruption_identity_replay_and_runtime_forge_reject_without_mutation() {
    let corrupt_fixture = completed_cp362_case_for(-1_000.0, 1.0, false);
    assert!(
        corrupt_fixture.is_some(),
        "CP362 corruption fixture must succeed"
    );
    let Some((mut corrupt, system, predecessor)) = corrupt_fixture else {
        return;
    };
    let mut forged_predecessor = predecessor;
    forged_predecessor.source_order = &[];
    assert_rejected_unchanged(&mut corrupt, &system, forged_predecessor);

    let coordinated_fixture = completed_cp362_case_for(-1_000.0, 1.0, false);
    assert!(
        coordinated_fixture.is_some(),
        "CP362 coordinated-forge fixture must succeed"
    );
    let Some((mut coordinated, system, predecessor)) = coordinated_fixture else {
        return;
    };
    let mut forged = predecessor;
    forged.mixed_air_humidity_ratio = Some(0.009);
    let unit = coordinated.units.get_mut(&system.id);
    assert!(unit.is_some(), "CP362 coordinated-forge unit must exist");
    let Some(unit) = unit else {
        return;
    };
    unit.calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
        .latest = Some(forged);
    coordinated.set_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(
        system.id, forged,
    );
    assert_rejected_unchanged(&mut coordinated, &system, forged);

    let replay_fixture = completed_cp362_case_for(-1_000.0, 1.0, false);
    assert!(
        replay_fixture.is_some(),
        "CP362 replay fixture must succeed"
    );
    let Some((mut replay, system, predecessor)) = replay_fixture else {
        return;
    };
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_case_break(&mut replay, &system, predecessor,)
            .is_ok()
    );
    assert_rejected_unchanged(&mut replay, &system, predecessor);

    let redistributed_fixture = completed_cp362_case_for(-1_000.0, 1.0, false);
    assert!(
        redistributed_fixture.is_some(),
        "CP362 redistributed-counter fixture must succeed"
    );
    let Some((mut redistributed, system, predecessor)) = redistributed_fixture else {
        return;
    };
    let transition = advance_direct_no_oa_calc_cooling_humidistat_case_break(
        &mut redistributed,
        &system,
        predecessor,
    );
    assert!(
        transition.is_ok(),
        "CP363 redistributed-counter seed must succeed"
    );
    let Ok(snapshot) = transition else {
        return;
    };
    let witness = redistributed.cooling_humidistat_case_break_latest_witness(system.id);
    let unit = redistributed.units.get_mut(&system.id);
    assert!(unit.is_some(), "CP363 mutable runtime unit must exist");
    let Some(unit) = unit else {
        return;
    };
    unit.calc_cooling_humidistat_case_break
        .witnessed_dehumidification_control_none_case_completed_skip_count = 0;
    unit.calc_cooling_humidistat_case_break
        .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count =
        1;
    let unit = redistributed.units.get(&system.id);
    assert!(
        unit.is_some(),
        "CP363 redistributed runtime unit must exist"
    );
    let Some(unit) = unit else {
        return;
    };
    assert!(
        !completed_direct_cooling_humidistat_case_break_is_consistent(
            &redistributed,
            unit,
            &system,
            snapshot,
            witness,
        )
    );
}

fn completed_cp362_case_for(
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
    let cp362 = advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit(
        &mut runtime,
        &system,
        cp361,
    )
    .ok()?;
    Some((runtime, system, cp362))
}

fn assert_rejected_unchanged(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: Predecessor,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_case_break(runtime, system, predecessor,)
            .is_err()
    );
    assert_eq!(*runtime, before);
}
