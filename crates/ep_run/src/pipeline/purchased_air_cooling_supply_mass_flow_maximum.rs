//! Run-summary evidence for the bounded PurchasedAir cooling supply-flow maximum.

use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary, PurchasedAirCalcMinimumOaPrefixSnapshot,
    PurchasedAirInitLifecycleSummary,
};

mod serialization;
mod validation;

pub(super) use serialization::lifecycle_json;
use validation::{same_option, snapshot_shape, validate_source_counters};

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary>,
    predecessor_cp321: Option<&PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary>,
    minimum_oa: Option<&PurchasedAirCalcMinimumOaPrefixLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling supply-flow maximum evidence"
            .to_string()
    })?;
    let predecessor_cp321 = predecessor_cp321.ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no CP321 evidence".to_string()
    })?;
    let minimum_oa = minimum_oa.ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no minimum-OA evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no initialization evidence"
            .to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no coupling call count".to_string()
    })?;

    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        || predecessor_cp321.source != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        || predecessor_cp321.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        || minimum_oa.source != PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
    {
        return Err(
            "direct-zone IdealLoads cooling supply-flow maximum provenance is invalid".to_string(),
        );
    }

    let state = &lifecycle.state;
    let predecessor_state = &predecessor_cp321.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip partition",
    )?;
    let partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition partition",
    )?;
    for (field, expected, actual) in [
        (
            "transition_count",
            coupling_call_count,
            state.transition_count,
        ),
        (
            "predecessor_transition_count",
            predecessor_state.transition_count,
            state.transition_count,
        ),
        (
            "minimum_oa_transition_count",
            minimum_oa.state.transition_count,
            state.transition_count,
        ),
        ("transition_partition", state.transition_count, partition),
        (
            "unit_off_skip_count",
            predecessor_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor_state.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "cooling_body_entry_count",
            predecessor_state.cooling_body_entry_count,
            state.cooling_body_entry_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling supply-flow maximum invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    validate_source_counters(state)?;

    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no latest snapshot".to_string()
    })?;
    let predecessor = predecessor_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no latest CP321 snapshot"
            .to_string()
    })?;
    let minimum_oa_latest = minimum_oa.state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no latest minimum-OA snapshot"
            .to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling supply-flow maximum has no declared system".to_string()
        })?;
    let expected_zone = init_lifecycle.controlled_zone.ok_or_else(|| {
        "direct-zone IdealLoads cooling supply-flow maximum has no controlled Zone".to_string()
    })?;
    if state.system != expected_system
        || predecessor_state.system != expected_system
        || minimum_oa.state.system != expected_system
        || !latest_matches_release(
            latest,
            predecessor,
            minimum_oa_latest,
            expected_system,
            expected_zone,
            coupling_call_count,
        )
    {
        return Err(
            "direct-zone IdealLoads cooling supply-flow maximum latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn latest_matches_release(
    maximum: &PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    predecessor: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    minimum_oa: &PurchasedAirCalcMinimumOaPrefixSnapshot,
    expected_system: ep_model::IdealLoadsAirSystemId,
    expected_zone: ep_model::ZoneId,
    call_count: usize,
) -> bool {
    let expected_outdoor_air = maximum
        .cooling_body_entered
        .then_some(minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s)
        .flatten();
    maximum.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        && maximum.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        && maximum.source_order == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER
        && predecessor.source == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        && predecessor.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        && predecessor.source_order
            == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER
        && minimum_oa.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && minimum_oa.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER
        && [maximum.system, predecessor.system, minimum_oa.system]
            .into_iter()
            .all(|system| system == expected_system)
        && [
            maximum.parent_call_ordinal,
            predecessor.parent_call_ordinal,
            minimum_oa.parent_call_ordinal,
        ]
        .into_iter()
        .all(|ordinal| ordinal == call_count)
        && [
            maximum.controlled_zone,
            predecessor.controlled_zone,
            minimum_oa.controlled_zone,
        ]
        .into_iter()
        .all(|zone| zone == expected_zone)
        && maximum.unit_body_entered == predecessor.unit_body_entered
        && maximum.unit_body_entered == minimum_oa.unit_body_entered
        && maximum.predecessor_cooling_body_entered == predecessor.cooling_body_entered
        && maximum.unit_off_skipped == predecessor.unit_off_skipped
        && maximum.non_cooling_skipped == predecessor.non_cooling_skipped
        && maximum.cooling_body_entered == predecessor.cooling_body_entered
        && same_option(
            maximum.outdoor_air_mass_flow_rate_kg_per_s,
            expected_outdoor_air,
        )
        && same_option(
            maximum.supply_mass_flow_rate_for_cool_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        )
        && same_option(
            maximum.supply_mass_flow_rate_for_dehumidification_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        )
        && same_option(
            maximum.supply_mass_flow_rate_for_humidification_kg_per_s,
            predecessor.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        )
        && snapshot_shape(maximum)
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling supply-flow maximum {label} overflowed")
    })
}

#[cfg(test)]
mod tests {
    use ep_model::{IdealLoadsAirSystemId, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    };

    use super::{lifecycle_json, snapshot_shape};

    fn active_snapshot() -> PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
        PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
            source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_body_entered: true,
            predecessor_cooling_body_entered: true,
            unit_off_skipped: false,
            non_cooling_skipped: false,
            cooling_body_entered: true,
            outdoor_air_mass_flow_rate_read: true,
            outdoor_air_mass_flow_rate_kg_per_s: Some(0.0),
            supply_mass_flow_rate_for_cool_read: true,
            supply_mass_flow_rate_for_cool_kg_per_s: Some(1.0),
            supply_mass_flow_rate_for_dehumidification_read: true,
            supply_mass_flow_rate_for_dehumidification_kg_per_s: Some(2.0),
            supply_mass_flow_rate_for_humidification_read: true,
            supply_mass_flow_rate_for_humidification_kg_per_s: Some(3.0),
            positive_zero_vs_outdoor_air_comparison_evaluated: true,
            positive_zero_less_than_outdoor_air: Some(false),
            positive_zero_outdoor_air_winner: Some(Operand::PositiveZeroFloor),
            positive_zero_outdoor_air_maximum_kg_per_s: Some(0.0),
            cooling_vs_dehumidification_comparison_evaluated: true,
            cooling_less_than_dehumidification: Some(true),
            cooling_dehumidification_winner: Some(Operand::Dehumidification),
            cooling_dehumidification_maximum_kg_per_s: Some(2.0),
            leading_vs_candidate_pair_comparison_evaluated: true,
            leading_less_than_candidate_pair: Some(true),
            leading_candidate_pair_winner: Some(Operand::Dehumidification),
            leading_candidate_pair_maximum_kg_per_s: Some(2.0),
            leading_vs_humidification_comparison_evaluated: true,
            leading_less_than_humidification: Some(true),
            final_winner: Some(Operand::Humidification),
            maximum_supply_mass_flow_rate_kg_per_s: Some(3.0),
            supply_mass_flow_rate_assigned: true,
            assigned_supply_mass_flow_rate_kg_per_s: Some(3.0),
            resulting_supply_mass_flow_rate_kg_per_s: Some(3.0),
        }
    }

    fn lifecycle_with_latest(
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    ) -> PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary {
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState::new(snapshot.system);
        state.transition_count = 1;
        state.cooling_body_entry_count = 1;
        state.outdoor_air_mass_flow_rate_read_count = 1;
        state.supply_mass_flow_rate_for_cool_read_count = 1;
        state.supply_mass_flow_rate_for_dehumidification_read_count = 1;
        state.supply_mass_flow_rate_for_humidification_read_count = 1;
        state.positive_zero_vs_outdoor_air_comparison_count = 1;
        state.cooling_vs_dehumidification_comparison_count = 1;
        state.leading_vs_candidate_pair_comparison_count = 1;
        state.leading_vs_humidification_comparison_count = 1;
        state.maximum_evaluation_count = 1;
        state.supply_mass_flow_rate_assignment_count = 1;
        state.latest = Some(snapshot);
        PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }

    fn set_candidates(
        snapshot: &mut PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
        cooling: f64,
        dehumidification: f64,
        humidification: f64,
    ) {
        fn pair(left: (Operand, f64), right: (Operand, f64)) -> (bool, (Operand, f64)) {
            let right_wins = left.1 < right.1;
            (right_wins, if right_wins { right } else { left })
        }

        snapshot.supply_mass_flow_rate_for_cool_kg_per_s = Some(cooling);
        snapshot.supply_mass_flow_rate_for_dehumidification_kg_per_s = Some(dehumidification);
        snapshot.supply_mass_flow_rate_for_humidification_kg_per_s = Some(humidification);
        let first = pair(
            (Operand::PositiveZeroFloor, 0.0),
            (Operand::OutdoorAir, 0.0),
        );
        let second = pair(
            (Operand::Cooling, cooling),
            (Operand::Dehumidification, dehumidification),
        );
        let third = pair(first.1, second.1);
        let fourth = pair(third.1, (Operand::Humidification, humidification));
        snapshot.positive_zero_less_than_outdoor_air = Some(first.0);
        snapshot.positive_zero_outdoor_air_winner = Some(first.1.0);
        snapshot.positive_zero_outdoor_air_maximum_kg_per_s = Some(first.1.1);
        snapshot.cooling_less_than_dehumidification = Some(second.0);
        snapshot.cooling_dehumidification_winner = Some(second.1.0);
        snapshot.cooling_dehumidification_maximum_kg_per_s = Some(second.1.1);
        snapshot.leading_less_than_candidate_pair = Some(third.0);
        snapshot.leading_candidate_pair_winner = Some(third.1.0);
        snapshot.leading_candidate_pair_maximum_kg_per_s = Some(third.1.1);
        snapshot.leading_less_than_humidification = Some(fourth.0);
        snapshot.final_winner = Some(fourth.1.0);
        snapshot.maximum_supply_mass_flow_rate_kg_per_s = Some(fourth.1.1);
        snapshot.assigned_supply_mass_flow_rate_kg_per_s = Some(fourth.1.1);
        snapshot.resulting_supply_mass_flow_rate_kg_per_s = Some(fourth.1.1);
    }

    #[test]
    fn source_tree_and_operand_labels_are_validated_and_serialized() {
        let snapshot = active_snapshot();
        assert!(snapshot_shape(&snapshot));

        let value = lifecycle_json(&lifecycle_with_latest(snapshot));
        assert_eq!(
            value["latest"]["positive_zero_outdoor_air_winner"],
            "PositiveZeroFloor"
        );
        assert_eq!(value["latest"]["final_winner"], "Humidification");
        assert_eq!(
            value["latest"]["source_order"].as_array().map(Vec::len),
            Some(6)
        );
    }

    #[test]
    fn non_finite_operands_remain_valid_and_retain_ieee_bits_in_json() {
        for (cooling, dehumidification, humidification) in [
            (f64::NAN, 2.0, 3.0),
            (f64::INFINITY, 2.0, 3.0),
            (f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        ] {
            let mut snapshot = active_snapshot();
            set_candidates(&mut snapshot, cooling, dehumidification, humidification);
            assert!(snapshot_shape(&snapshot));

            let result = snapshot.resulting_supply_mass_flow_rate_kg_per_s.unwrap();
            let value = lifecycle_json(&lifecycle_with_latest(snapshot));
            let latest = &value["latest"];
            for (field, expected) in [
                ("supply_mass_flow_rate_for_cool_kg_per_s", cooling),
                (
                    "supply_mass_flow_rate_for_dehumidification_kg_per_s",
                    dehumidification,
                ),
                (
                    "supply_mass_flow_rate_for_humidification_kg_per_s",
                    humidification,
                ),
            ] {
                assert_eq!(
                    latest[format!("{field}_ieee_bits")],
                    format!("0x{:016x}", expected.to_bits())
                );
                if !expected.is_finite() {
                    assert!(latest[field].is_null());
                }
            }
            assert_eq!(
                latest["resulting_supply_mass_flow_rate_kg_per_s_ieee_bits"],
                format!("0x{:016x}", result.to_bits())
            );
            if !result.is_finite() {
                assert!(latest["resulting_supply_mass_flow_rate_kg_per_s"].is_null());
            }
        }
    }

    #[test]
    fn corrupted_intermediate_winner_is_rejected() {
        let mut snapshot = active_snapshot();
        snapshot.leading_candidate_pair_winner = Some(Operand::Cooling);
        assert!(!snapshot_shape(&snapshot));
    }
}
