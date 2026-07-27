//! JSON serialization for one CP322 source-site snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
};
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "outdoor_air_mass_flow_rate_read": snapshot.outdoor_air_mass_flow_rate_read,
        "outdoor_air_mass_flow_rate_kg_per_s":
            snapshot.outdoor_air_mass_flow_rate_kg_per_s,
        "outdoor_air_mass_flow_rate_kg_per_s_ieee_bits":
            ieee_bits(snapshot.outdoor_air_mass_flow_rate_kg_per_s),
        "supply_mass_flow_rate_for_cool_read":
            snapshot.supply_mass_flow_rate_for_cool_read,
        "supply_mass_flow_rate_for_cool_kg_per_s":
            snapshot.supply_mass_flow_rate_for_cool_kg_per_s,
        "supply_mass_flow_rate_for_cool_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_for_cool_kg_per_s),
        "supply_mass_flow_rate_for_dehumidification_read":
            snapshot.supply_mass_flow_rate_for_dehumidification_read,
        "supply_mass_flow_rate_for_dehumidification_kg_per_s":
            snapshot.supply_mass_flow_rate_for_dehumidification_kg_per_s,
        "supply_mass_flow_rate_for_dehumidification_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_for_dehumidification_kg_per_s),
        "supply_mass_flow_rate_for_humidification_read":
            snapshot.supply_mass_flow_rate_for_humidification_read,
        "supply_mass_flow_rate_for_humidification_kg_per_s":
            snapshot.supply_mass_flow_rate_for_humidification_kg_per_s,
        "supply_mass_flow_rate_for_humidification_kg_per_s_ieee_bits":
            ieee_bits(snapshot.supply_mass_flow_rate_for_humidification_kg_per_s),
    });
    extend_object(
        &mut value,
        json!({
            "positive_zero_vs_outdoor_air_comparison_evaluated":
                snapshot.positive_zero_vs_outdoor_air_comparison_evaluated,
            "positive_zero_less_than_outdoor_air":
                snapshot.positive_zero_less_than_outdoor_air,
            "positive_zero_outdoor_air_winner":
                snapshot.positive_zero_outdoor_air_winner.map(operand_name),
            "positive_zero_outdoor_air_maximum_kg_per_s":
                snapshot.positive_zero_outdoor_air_maximum_kg_per_s,
            "positive_zero_outdoor_air_maximum_kg_per_s_ieee_bits":
                ieee_bits(snapshot.positive_zero_outdoor_air_maximum_kg_per_s),
            "cooling_vs_dehumidification_comparison_evaluated":
                snapshot.cooling_vs_dehumidification_comparison_evaluated,
            "cooling_less_than_dehumidification":
                snapshot.cooling_less_than_dehumidification,
            "cooling_dehumidification_winner":
                snapshot.cooling_dehumidification_winner.map(operand_name),
            "cooling_dehumidification_maximum_kg_per_s":
                snapshot.cooling_dehumidification_maximum_kg_per_s,
            "cooling_dehumidification_maximum_kg_per_s_ieee_bits":
                ieee_bits(snapshot.cooling_dehumidification_maximum_kg_per_s),
            "leading_vs_candidate_pair_comparison_evaluated":
                snapshot.leading_vs_candidate_pair_comparison_evaluated,
            "leading_less_than_candidate_pair":
                snapshot.leading_less_than_candidate_pair,
            "leading_candidate_pair_winner":
                snapshot.leading_candidate_pair_winner.map(operand_name),
            "leading_candidate_pair_maximum_kg_per_s":
                snapshot.leading_candidate_pair_maximum_kg_per_s,
            "leading_candidate_pair_maximum_kg_per_s_ieee_bits":
                ieee_bits(snapshot.leading_candidate_pair_maximum_kg_per_s),
            "leading_vs_humidification_comparison_evaluated":
                snapshot.leading_vs_humidification_comparison_evaluated,
            "leading_less_than_humidification":
                snapshot.leading_less_than_humidification,
            "final_winner": snapshot.final_winner.map(operand_name),
            "maximum_supply_mass_flow_rate_kg_per_s":
                snapshot.maximum_supply_mass_flow_rate_kg_per_s,
            "maximum_supply_mass_flow_rate_kg_per_s_ieee_bits":
                ieee_bits(snapshot.maximum_supply_mass_flow_rate_kg_per_s),
            "supply_mass_flow_rate_assigned":
                snapshot.supply_mass_flow_rate_assigned,
            "assigned_supply_mass_flow_rate_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_kg_per_s,
            "assigned_supply_mass_flow_rate_kg_per_s_ieee_bits":
                ieee_bits(snapshot.assigned_supply_mass_flow_rate_kg_per_s),
            "resulting_supply_mass_flow_rate_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_kg_per_s,
            "resulting_supply_mass_flow_rate_kg_per_s_ieee_bits":
                ieee_bits(snapshot.resulting_supply_mass_flow_rate_kg_per_s),
        }),
    );
    value
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

fn operand_name(operand: Operand) -> &'static str {
    match operand {
        Operand::PositiveZeroFloor => "PositiveZeroFloor",
        Operand::OutdoorAir => "OutdoorAir",
        Operand::Cooling => "Cooling",
        Operand::Dehumidification => "Dehumidification",
        Operand::Humidification => "Humidification",
    }
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
