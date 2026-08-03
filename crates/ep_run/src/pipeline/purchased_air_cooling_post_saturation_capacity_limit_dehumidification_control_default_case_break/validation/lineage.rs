//! Exact CP409-to-CP410 compact pipeline lineage validation.

use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Snapshot,
};

pub(super) fn lineage_is_exact(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    snapshot.system == predecessor.system
        && snapshot.parent_call_ordinal == predecessor.parent_call_ordinal
        && snapshot.controlled_zone == predecessor.controlled_zone
        && inherited_flags(snapshot) == predecessor_flags(predecessor)
        && snapshot.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && shared_case_flags(snapshot) == predecessor_shared_case_flags(predecessor)
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
            == predecessor
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
        && option_bits_equal(
            snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        )
        && option_bits_equal(
            snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        )
        && option_bits_equal(
            snapshot.predecessor_cp409_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        )
        && !snapshot.dehumidification_control_default_case_exited_via_break
        && carriers_are_preserved(snapshot, predecessor)
}

fn carriers_are_preserved(snapshot: Snapshot, predecessor: Predecessor) -> bool {
    option_bits_equal(
        snapshot.resulting_supply_humidity_ratio,
        predecessor.resulting_supply_humidity_ratio,
    ) && option_bits_equal(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    ) && option_bits_equal(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    )
}

fn inherited_flags(snapshot: Snapshot) -> [bool; 28] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
    ]
}

fn predecessor_flags(snapshot: Predecessor) -> [bool; 28] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
        snapshot.predecessor_capacity_limit_guard_evaluated,
        snapshot.predecessor_capacity_limit_body_entered,
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_evaluated,
        snapshot.predecessor_dehumidification_body_entered,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_total_output_assignment_executed,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        snapshot.predecessor_supply_enthalpy_assignment_executed,
        snapshot.predecessor_dehumidification_control_type_read,
        snapshot.predecessor_dehumidification_control_switch_dispatched,
        snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        snapshot.predecessor_dehumidification_control_humidistat_case_entered,
        snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
    ]
}

fn shared_case_flags(snapshot: Snapshot) -> [bool; 3] {
    [
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
    ]
}

fn predecessor_shared_case_flags(snapshot: Predecessor) -> [bool; 3] {
    [
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_bits_distinguish_signed_zero() {
        assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
        assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
    }
}
