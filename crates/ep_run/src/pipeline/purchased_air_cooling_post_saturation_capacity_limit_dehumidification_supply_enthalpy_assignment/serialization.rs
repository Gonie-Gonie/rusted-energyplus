//! JSON serialization for CP417 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentLifecycleSummary;
use serde_json::{Value, json};

pub(in crate::pipeline) mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "predecessor_supply_temperature_saturation_assignment_count": state.predecessor_supply_temperature_saturation_assignment_count,
        "predecessor_supply_temperature_saturation_mixed_air_limit_count": state.predecessor_supply_temperature_saturation_mixed_air_limit_count,
        "predecessor_supply_humidity_ratio_assignment_count": state.predecessor_supply_humidity_ratio_assignment_count,
        "supply_enthalpy_assignment_count": state.supply_enthalpy_assignment_count,
        "predecessor_route_counts": state.predecessor_route_counts.as_slice(),
        "predecessor_guard_false_fallthrough_route_counts": state.predecessor_guard_false_fallthrough_route_counts.as_slice(),
        "predecessor_guard_body_entry_route_counts": state.predecessor_guard_body_entry_route_counts.as_slice(),
        "predecessor_supply_temperature_saturation_assignment_route_counts": state.predecessor_supply_temperature_saturation_assignment_route_counts.as_slice(),
        "predecessor_supply_temperature_mixed_air_limit_route_counts": state.predecessor_supply_temperature_mixed_air_limit_route_counts.as_slice(),
        "predecessor_supply_humidity_ratio_assignment_route_counts": state.predecessor_supply_humidity_ratio_assignment_route_counts.as_slice(),
        "supply_enthalpy_assignment_route_counts": state.supply_enthalpy_assignment_route_counts.as_slice(),
        "source_site_execution_count": state.source_site_execution_count,
        "cp416_supply_humidity_ratio_state_owner_count": state.cp416_supply_humidity_ratio_state_owner_count,
        "unchanged_supply_humidity_ratio_preservation_count": state.unchanged_supply_humidity_ratio_preservation_count,
        "cp416_supply_enthalpy_state_owner_count": state.cp416_supply_enthalpy_state_owner_count,
        "unchanged_supply_enthalpy_preservation_count": state.unchanged_supply_enthalpy_preservation_count,
        "cp416_supply_temperature_state_owner_count": state.cp416_supply_temperature_state_owner_count,
        "unchanged_supply_temperature_preservation_count": state.unchanged_supply_temperature_preservation_count,
        "cp417_psychrometric_supply_enthalpy_state_owner_count": state.cp417_psychrometric_supply_enthalpy_state_owner_count,
        "cp416_retained_supply_temperature_owned_read_count": state.cp416_retained_supply_temperature_owned_read_count,
        "supply_temperature_for_enthalpy_read_count": state.supply_temperature_for_enthalpy_read_count,
        "cp416_retained_supply_humidity_ratio_owned_read_count": state.cp416_retained_supply_humidity_ratio_owned_read_count,
        "supply_humidity_ratio_for_enthalpy_read_count": state.supply_humidity_ratio_for_enthalpy_read_count,
        "psychrometric_supply_enthalpy_evaluation_count": state.psychrometric_supply_enthalpy_evaluation_count,
        "supply_enthalpy_assignment_write_count": state.supply_enthalpy_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}

#[cfg(test)]
mod tests {
    use ep_model::IdealLoadsAirSystemId;
    use ep_runtime::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState,
    };

    use super::*;

    #[test]
    fn lifecycle_json_retains_seven_width_36_route_arrays_and_cp417_counters() {
        let lifecycle = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
            first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState::new(IdealLoadsAirSystemId(0)),
        };
        let value = lifecycle_json(&lifecycle);
        for field in [
            "predecessor_route_counts",
            "predecessor_guard_false_fallthrough_route_counts",
            "predecessor_guard_body_entry_route_counts",
            "predecessor_supply_temperature_saturation_assignment_route_counts",
            "predecessor_supply_temperature_mixed_air_limit_route_counts",
            "predecessor_supply_humidity_ratio_assignment_route_counts",
            "supply_enthalpy_assignment_route_counts",
        ] {
            assert_eq!(value[field].as_array().map(Vec::len), Some(36), "{field}");
        }
        for field in [
            "predecessor_supply_humidity_ratio_assignment_count",
            "supply_enthalpy_assignment_count",
            "source_site_execution_count",
            "cp417_psychrometric_supply_enthalpy_state_owner_count",
            "cp416_retained_supply_temperature_owned_read_count",
            "cp416_retained_supply_humidity_ratio_owned_read_count",
            "psychrometric_supply_enthalpy_evaluation_count",
            "supply_enthalpy_assignment_write_count",
        ] {
            assert_eq!(value[field], 0, "{field}");
        }
        assert!(value["latest"].is_null());
    }
}
