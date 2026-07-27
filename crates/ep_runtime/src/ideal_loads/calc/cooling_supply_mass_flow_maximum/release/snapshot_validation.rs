use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
};

type Winner = (Operand, f64);

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER;
    let unit_off =
        snapshot.unit_off_skipped && !snapshot.unit_body_entered && !snapshot.cooling_body_entered;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.cooling_body_entered;
    provenance
        && snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            cooling_sites_are_exact(snapshot)
        } else {
            skipped_sites_are_exact(snapshot)
        }
}

pub(super) fn cooling_supply_mass_flow_maximum_snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute> {
    if !cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute::NonCooling)
    } else {
        Some(PurchasedAirCalcCoolingSupplyMassFlowMaximumRetainedRoute::CoolingMaximumAssigned)
    }
}

fn cooling_sites_are_exact(snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot) -> bool {
    if !snapshot.outdoor_air_mass_flow_rate_read
        || !snapshot.supply_mass_flow_rate_for_cool_read
        || !snapshot.supply_mass_flow_rate_for_dehumidification_read
        || !snapshot.supply_mass_flow_rate_for_humidification_read
        || !snapshot.positive_zero_vs_outdoor_air_comparison_evaluated
        || !snapshot.cooling_vs_dehumidification_comparison_evaluated
        || !snapshot.leading_vs_candidate_pair_comparison_evaluated
        || !snapshot.leading_vs_humidification_comparison_evaluated
        || !snapshot.supply_mass_flow_rate_assigned
    {
        return false;
    }
    let Some(outdoor_air) = snapshot.outdoor_air_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(cool) = snapshot.supply_mass_flow_rate_for_cool_kg_per_s else {
        return false;
    };
    let Some(dehumidification) = snapshot.supply_mass_flow_rate_for_dehumidification_kg_per_s
    else {
        return false;
    };
    let Some(humidification) = snapshot.supply_mass_flow_rate_for_humidification_kg_per_s else {
        return false;
    };
    if outdoor_air.to_bits() != 0.0_f64.to_bits() {
        return false;
    }

    let first = source_pair(
        (Operand::PositiveZeroFloor, 0.0),
        (Operand::OutdoorAir, outdoor_air),
    );
    let second = source_pair(
        (Operand::Cooling, cool),
        (Operand::Dehumidification, dehumidification),
    );
    let third = source_pair(first.1, second.1);
    let fourth = source_pair(third.1, (Operand::Humidification, humidification));
    snapshot.positive_zero_less_than_outdoor_air == Some(first.0)
        && snapshot.positive_zero_outdoor_air_winner == Some(first.1.0)
        && has_bits(
            snapshot.positive_zero_outdoor_air_maximum_kg_per_s,
            first.1.1,
        )
        && snapshot.cooling_less_than_dehumidification == Some(second.0)
        && snapshot.cooling_dehumidification_winner == Some(second.1.0)
        && has_bits(
            snapshot.cooling_dehumidification_maximum_kg_per_s,
            second.1.1,
        )
        && snapshot.leading_less_than_candidate_pair == Some(third.0)
        && snapshot.leading_candidate_pair_winner == Some(third.1.0)
        && has_bits(snapshot.leading_candidate_pair_maximum_kg_per_s, third.1.1)
        && snapshot.leading_less_than_humidification == Some(fourth.0)
        && snapshot.final_winner == Some(fourth.1.0)
        && has_bits(snapshot.maximum_supply_mass_flow_rate_kg_per_s, fourth.1.1)
        && has_bits(snapshot.assigned_supply_mass_flow_rate_kg_per_s, fourth.1.1)
        && has_bits(
            snapshot.resulting_supply_mass_flow_rate_kg_per_s,
            fourth.1.1,
        )
}

fn skipped_sites_are_exact(snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot) -> bool {
    !snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_for_cool_read
        && snapshot.supply_mass_flow_rate_for_cool_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_for_dehumidification_read
        && snapshot
            .supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_humidification_read
        && snapshot
            .supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
        && !snapshot.positive_zero_vs_outdoor_air_comparison_evaluated
        && snapshot.positive_zero_less_than_outdoor_air.is_none()
        && snapshot.positive_zero_outdoor_air_winner.is_none()
        && snapshot
            .positive_zero_outdoor_air_maximum_kg_per_s
            .is_none()
        && !snapshot.cooling_vs_dehumidification_comparison_evaluated
        && snapshot.cooling_less_than_dehumidification.is_none()
        && snapshot.cooling_dehumidification_winner.is_none()
        && snapshot.cooling_dehumidification_maximum_kg_per_s.is_none()
        && !snapshot.leading_vs_candidate_pair_comparison_evaluated
        && snapshot.leading_less_than_candidate_pair.is_none()
        && snapshot.leading_candidate_pair_winner.is_none()
        && snapshot.leading_candidate_pair_maximum_kg_per_s.is_none()
        && !snapshot.leading_vs_humidification_comparison_evaluated
        && snapshot.leading_less_than_humidification.is_none()
        && snapshot.final_winner.is_none()
        && snapshot.maximum_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_assigned
        && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
        && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none()
}

fn source_pair(left: Winner, right: Winner) -> (bool, Winner) {
    let right_wins = left.1 < right.1;
    (right_wins, if right_wins { right } else { left })
}

fn has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
