//! CP380 post-saturation capacity-limit guard validation and JSON serialization.

mod serialization;
mod validation;

pub(in crate::pipeline) use serialization::lifecycle_json;
pub(in crate::pipeline) use validation::validate_direct_lifecycle;

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
    };

    use super::*;

    #[test]
    fn serialization_locks_all_four_selector_names_and_control_only_shapes() {
        for (limit, name, expected_second, expected_sites) in [
            (IdealLoadsLimit::LimitCapacity, "LimitCapacity", None, 3),
            (
                IdealLoadsLimit::LimitFlowRateAndCapacity,
                "LimitFlowRateAndCapacity",
                Some("LimitFlowRateAndCapacity"),
                5,
            ),
            (IdealLoadsLimit::NoLimit, "NoLimit", Some("NoLimit"), 4),
            (
                IdealLoadsLimit::LimitFlowRate,
                "LimitFlowRate",
                Some("LimitFlowRate"),
                4,
            ),
        ] {
            let value = lifecycle_json(&active_lifecycle(limit));
            let latest = &value["latest"];
            assert_eq!(value["source_site_execution_count"], expected_sites);
            assert_eq!(latest["first_cooling_limit"], name);
            assert_eq!(latest["second_cooling_limit"].as_str(), expected_second);
            let keys: BTreeSet<_> = latest
                .as_object()
                .expect("CP380 latest object")
                .keys()
                .map(String::as_str)
                .collect();
            assert!(keys.iter().all(|key| {
                !key.contains("ieee")
                    && !key.contains("enthalpy_j_per_kg")
                    && !key.contains("capacity_w")
                    && !key.contains("node")
                    && !key.contains("report")
            }));
        }
    }

    fn active_lifecycle(
        limit: IdealLoadsLimit,
    ) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary {
        let capacity = limit == IdealLoadsLimit::LimitCapacity;
        let second = !capacity;
        let combined = second && limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let selected = capacity || combined;
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        state.transition_count = 1;
        state.heating_availability_guard_false_fallthrough_count = 1;
        state.capacity_limit_guard_evaluation_count = 1;
        state.source_site_execution_count = 2 + 2 * usize::from(second) + usize::from(selected);
        state.configured_cooling_limit_owned_read_count = 1;
        state.cp337_same_call_selector_lineage_corroboration_count = 1;
        state.first_cooling_limit_read_count = 1;
        state.cooling_limit_capacity_comparison_count = 1;
        state.cooling_limit_capacity_match_count = usize::from(capacity);
        state.second_cooling_limit_read_count = usize::from(second);
        state.cooling_limit_flow_rate_and_capacity_comparison_count = usize::from(second);
        state.cooling_limit_flow_rate_and_capacity_match_count = usize::from(combined);
        state.cooling_limit_rejected_count = usize::from(!selected);
        state.capacity_limit_body_entry_count = usize::from(selected);
        state.active_guard_false_fallthrough_count = usize::from(!selected);
        state.heating_availability_guard_false_fallthrough_body_entry_count = usize::from(selected);
        state.heating_availability_guard_false_fallthrough_capacity_guard_false_count =
            usize::from(!selected);
        state.latest = Some(PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            source_order:
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE_ORDER,
            system: IdealLoadsAirSystemId(0),
            parent_call_ordinal: 1,
            controlled_zone: ZoneId(0),
            unit_off_skipped: false,
            non_cooling_skipped: false,
            positive_guard_false_fallthrough_skipped: false,
            heating_availability_guard_false_fallthrough: true,
            humidification_control_guard_false_fallthrough: false,
            dehumidification_control_humidistat_maximum_assignment_executed: false,
            dehumidification_control_none_maximum_assignment_executed: false,
            dehumidification_control_guard_false_fallthrough: false,
            predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed: true,
            capacity_limit_guard_evaluated: true,
            configured_cooling_limit_owned_read: true,
            cp337_same_call_selector_lineage_corroborated: true,
            first_cooling_limit_read: true,
            first_cooling_limit: Some(limit),
            cooling_limit_capacity_comparison_evaluated: true,
            cooling_limit_capacity: Some(capacity),
            second_cooling_limit_read: second,
            second_cooling_limit: second.then_some(limit),
            cooling_limit_flow_rate_and_capacity_comparison_evaluated: second,
            cooling_limit_flow_rate_and_capacity: second.then_some(combined),
            cooling_limit_condition_satisfied: Some(selected),
            cooling_limit_rejected: !selected,
            capacity_limit_body_entered: selected,
            active_guard_false_fallthrough: !selected,
        });
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
            state,
        }
    }
}
