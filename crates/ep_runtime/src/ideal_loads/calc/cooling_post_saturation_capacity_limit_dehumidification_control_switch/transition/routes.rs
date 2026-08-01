//! CP385 route classification and CP386 selector refinement.

use ep_model::DehumidificationControlType;

use super::Predecessor;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Cp384Snapshot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PredecessorRoute {
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
    HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
    DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
    DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
    DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
    DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
    DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
    DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
}

impl PredecessorRoute {
    pub(super) const fn index(self) -> usize {
        use PredecessorRoute as P;
        match self {
            P::UnitOff => 0,
            P::NonCooling => 1,
            P::PositiveGuardFalseFallthrough => 2,
            P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 3,
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 4,
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => 5,
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 6,
            P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 7,
            P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 8,
            P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => 9,
            P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 10,
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 11,
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 12,
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => 13,
            P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => 14,
            P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough => 15,
            P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough => 16,
            P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough => 17,
            P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => 18,
            P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough => 19,
            P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough => 20,
            P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough => 21,
            P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => 22,
        }
    }

    pub(in crate::ideal_loads::calc) const fn is_assignment(self) -> bool {
        use PredecessorRoute as P;
        matches!(
            self,
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
                | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
                | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
                | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned
                | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
        )
    }

    const fn preserves_enthalpy(self) -> bool {
        use PredecessorRoute as P;
        matches!(
            self,
            P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough
                | P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor: PredecessorRoute,
    pub selected_case: Option<DehumidificationControlType>,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<PredecessorRoute> {
    use PredecessorRoute as P;
    if predecessor.source
        != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
        || predecessor.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || predecessor.source_order
            != PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    if !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact(
        predecessor_control_shape(predecessor),
    ) {
        return None;
    }
    let base = [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
    ];
    if base.into_iter().filter(|flag| *flag).count() != 1 {
        return None;
    }
    let route = if predecessor.unit_off_skipped {
        P::UnitOff
    } else if predecessor.non_cooling_skipped {
        P::NonCooling
    } else if predecessor.positive_guard_false_fallthrough_skipped {
        P::PositiveGuardFalseFallthrough
    } else {
        let lineage = if predecessor.heating_availability_guard_false_fallthrough {
            0
        } else if predecessor.humidification_control_guard_false_fallthrough {
            1
        } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
            2
        } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
            3
        } else {
            4
        };
        let stages = [
            predecessor.predecessor_active_capacity_limit_guard_false_fallthrough,
            predecessor.predecessor_dehumidification_guard_false_fallthrough,
            predecessor.dehumidification_total_output_capacity_guard_false_fallthrough,
            predecessor.dehumidification_total_output_maximum_capacity_assignment_executed,
        ];
        if stages.into_iter().filter(|flag| *flag).count() != 1 {
            return None;
        }
        match (lineage, stages) {
            (0, [true, false, false, false]) => P::HeatingAvailabilityGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
            (0, [false, true, false, false]) => P::HeatingAvailabilityGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
            (0, [false, false, true, false]) => P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
            (0, [false, false, false, true]) => P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
            (1, [true, false, false, false]) => P::HumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
            (1, [false, true, false, false]) => P::HumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
            (1, [false, false, true, false]) => P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
            (1, [false, false, false, true]) => P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
            (2, [true, false, false, false]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
            (2, [false, true, false, false]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
            (2, [false, false, true, false]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
            (2, [false, false, false, true]) => P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
            (3, [true, false, false, false]) => P::DehumidificationControlNoneMaximumAssignmentExecutedCapacityLimitGuardFalseFallthrough,
            (3, [false, true, false, false]) => P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationGuardFalseFallthrough,
            (3, [false, false, true, false]) => P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputCapacityGuardFalseFallthrough,
            (3, [false, false, false, true]) => P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned,
            (4, [true, false, false, false]) => P::DehumidificationControlGuardFalseFallthroughCapacityLimitGuardFalseFallthrough,
            (4, [false, true, false, false]) => P::DehumidificationControlGuardFalseFallthroughDehumidificationGuardFalseFallthrough,
            (4, [false, false, true, false]) => P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputCapacityGuardFalseFallthrough,
            (4, [false, false, false, true]) => P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned,
            _ => return None,
        }
    };
    predecessor_payload_is_exact(predecessor, route).then_some(route)
}

fn predecessor_control_shape(predecessor: Predecessor) -> Cp384Snapshot {
    Cp384Snapshot {
        source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        first_excluded_source: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: crate::ideal_loads::PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        preexisting_cooling_total_output_w: None,
        cp383_retained_maximum_total_cooling_capacity_owned_read: false,
        maximum_total_cooling_capacity_read: false,
        maximum_total_cooling_capacity_w: None,
        cooling_total_output_assigned: false,
        assigned_cooling_total_output_w: None,
        resulting_cooling_total_output_w: None,
    }
}

pub(in crate::ideal_loads::calc) const fn selector_is_allowed(
    route: PredecessorRoute,
    selector: Option<DehumidificationControlType>,
) -> bool {
    use DehumidificationControlType as D;
    use PredecessorRoute as P;
    if !route.is_assignment() {
        return selector.is_none();
    }
    match route {
        P::HeatingAvailabilityGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned
        | P::HumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => selector.is_some(),
        P::DehumidificationControlHumidistatMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => matches!(selector, Some(D::Humidistat)),
        P::DehumidificationControlNoneMaximumAssignmentExecutedDehumidificationTotalOutputMaximumCapacityAssigned => matches!(selector, Some(D::None)),
        P::DehumidificationControlGuardFalseFallthroughDehumidificationTotalOutputMaximumCapacityAssigned => matches!(selector, Some(D::ConstantSensibleHeatRatio | D::ConstantSupplyHumidityRatio)),
        _ => false,
    }
}

fn predecessor_payload_is_exact(predecessor: Predecessor, route: PredecessorRoute) -> bool {
    if route.is_assignment() {
        let (
            Some(_preexisting),
            Some(mixed_air),
            Some(output),
            Some(flow),
            Some(specific),
            Some(calculated),
            Some(assigned),
            Some(resulting),
        ) = (
            predecessor.preexisting_supply_enthalpy_j_per_kg,
            predecessor.mixed_air_enthalpy_j_per_kg,
            predecessor.cooling_total_output_w,
            predecessor.supply_mass_flow_rate_kg_per_s,
            predecessor.specific_cooling_output_j_per_kg,
            predecessor.calculated_supply_enthalpy_j_per_kg,
            predecessor.assigned_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ) else {
            return false;
        };
        let expected_specific = output / flow;
        let expected_enthalpy = mixed_air - expected_specific;
        predecessor.supply_enthalpy_assignment_executed
            && predecessor.cp379_retained_supply_enthalpy_owned_read
            && predecessor.cp329_retained_mixed_air_enthalpy_owned_read
            && predecessor.mixed_air_enthalpy_read
            && predecessor.cp384_retained_cooling_total_output_owned_read
            && predecessor.cooling_total_output_read
            && predecessor.cp330_retained_supply_mass_flow_rate_owned_read
            && predecessor.supply_mass_flow_rate_read
            && predecessor.specific_cooling_output_calculated
            && predecessor.supply_enthalpy_difference_calculated
            && predecessor.supply_enthalpy_assigned
            && specific.to_bits() == expected_specific.to_bits()
            && calculated.to_bits() == expected_enthalpy.to_bits()
            && assigned.to_bits() == calculated.to_bits()
            && resulting.to_bits() == assigned.to_bits()
    } else if route.preserves_enthalpy() {
        let (Some(preexisting), Some(resulting)) = (
            predecessor.preexisting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ) else {
            return false;
        };
        !predecessor.supply_enthalpy_assignment_executed
            && predecessor.cp379_retained_supply_enthalpy_owned_read
            && active_payload_is_absent(predecessor)
            && preexisting.to_bits() == resulting.to_bits()
    } else {
        !predecessor.supply_enthalpy_assignment_executed
            && predecessor.preexisting_supply_enthalpy_j_per_kg.is_none()
            && !predecessor.cp379_retained_supply_enthalpy_owned_read
            && active_payload_is_absent(predecessor)
            && predecessor.resulting_supply_enthalpy_j_per_kg.is_none()
    }
}

fn active_payload_is_absent(predecessor: Predecessor) -> bool {
    !predecessor.cp329_retained_mixed_air_enthalpy_owned_read
        && !predecessor.mixed_air_enthalpy_read
        && predecessor.mixed_air_enthalpy_j_per_kg.is_none()
        && !predecessor.cp384_retained_cooling_total_output_owned_read
        && !predecessor.cooling_total_output_read
        && predecessor.cooling_total_output_w.is_none()
        && !predecessor.cp330_retained_supply_mass_flow_rate_owned_read
        && !predecessor.supply_mass_flow_rate_read
        && predecessor.supply_mass_flow_rate_kg_per_s.is_none()
        && !predecessor.specific_cooling_output_calculated
        && predecessor.specific_cooling_output_j_per_kg.is_none()
        && !predecessor.supply_enthalpy_difference_calculated
        && predecessor.calculated_supply_enthalpy_j_per_kg.is_none()
        && !predecessor.supply_enthalpy_assigned
        && predecessor.assigned_supply_enthalpy_j_per_kg.is_none()
}
