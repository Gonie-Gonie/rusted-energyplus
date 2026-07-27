//! Exact direct-lane shape checks for one CP322 source-site snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
};

type Winner = (Operand, f64);

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> bool {
    if !snapshot.cooling_body_entered {
        return usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
            == 1
            && snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
            && skipped_shape(snapshot);
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || !snapshot.unit_body_entered
        || !snapshot.predecessor_cooling_body_entered
        || !all_active_sites_executed(snapshot)
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
        (Operand::PositiveZeroFloor, 0.0_f64),
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
        && same_value(
            snapshot.positive_zero_outdoor_air_maximum_kg_per_s,
            first.1.1,
        )
        && snapshot.cooling_less_than_dehumidification == Some(second.0)
        && snapshot.cooling_dehumidification_winner == Some(second.1.0)
        && same_value(
            snapshot.cooling_dehumidification_maximum_kg_per_s,
            second.1.1,
        )
        && snapshot.leading_less_than_candidate_pair == Some(third.0)
        && snapshot.leading_candidate_pair_winner == Some(third.1.0)
        && same_value(snapshot.leading_candidate_pair_maximum_kg_per_s, third.1.1)
        && snapshot.leading_less_than_humidification == Some(fourth.0)
        && snapshot.final_winner == Some(fourth.1.0)
        && same_value(snapshot.maximum_supply_mass_flow_rate_kg_per_s, fourth.1.1)
        && same_value(snapshot.assigned_supply_mass_flow_rate_kg_per_s, fourth.1.1)
        && same_value(
            snapshot.resulting_supply_mass_flow_rate_kg_per_s,
            fourth.1.1,
        )
}

fn all_active_sites_executed(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> bool {
    snapshot.outdoor_air_mass_flow_rate_read
        && snapshot.supply_mass_flow_rate_for_cool_read
        && snapshot.supply_mass_flow_rate_for_dehumidification_read
        && snapshot.supply_mass_flow_rate_for_humidification_read
        && snapshot.positive_zero_vs_outdoor_air_comparison_evaluated
        && snapshot.cooling_vs_dehumidification_comparison_evaluated
        && snapshot.leading_vs_candidate_pair_comparison_evaluated
        && snapshot.leading_vs_humidification_comparison_evaluated
        && snapshot.supply_mass_flow_rate_assigned
}

fn skipped_shape(snapshot: &PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot) -> bool {
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

fn same_value(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

pub(in crate::pipeline) fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
