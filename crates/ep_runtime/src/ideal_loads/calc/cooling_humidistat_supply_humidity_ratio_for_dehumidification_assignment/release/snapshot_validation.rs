//! Exact CP360 snapshot and binary64 validation.

use super::super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Snapshot,
};

mod route;

pub(in crate::ideal_loads) fn cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
    snapshot: Snapshot,
) -> bool {
    matches!(
        snapshot_route(snapshot),
        Some(
            Route::UnitOff
                | Route::NonCooling
                | Route::PositiveGuardFalseFallthrough
                | Route::DehumidificationControlNoneCaseCompletedSkip
        )
    )
}

pub(super) fn snapshot_route(snapshot: Snapshot) -> Option<Route> {
    let route = route::structural_route(snapshot)?;
    values_fit_route(snapshot, route).then_some(route)
}

pub(in crate::ideal_loads::calc) fn snapshots_match_bit_exact(
    mut left: Snapshot,
    mut right: Snapshot,
) -> bool {
    let values_match = [
        (
            left.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            right.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        ),
        (
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        (
            left.moisture_demand_derived_supply_humidity_ratio,
            right.moisture_demand_derived_supply_humidity_ratio,
        ),
        (
            left.zone_node_humidity_ratio,
            right.zone_node_humidity_ratio,
        ),
        (
            left.calculated_supply_humidity_ratio_for_dehumidification,
            right.calculated_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.assigned_supply_humidity_ratio_for_dehumidification,
            right.assigned_supply_humidity_ratio_for_dehumidification,
        ),
        (
            left.resulting_supply_humidity_ratio_for_dehumidification,
            right.resulting_supply_humidity_ratio_for_dehumidification,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_bits_match(left, right));
    for snapshot in [&mut left, &mut right] {
        snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.moisture_demand_derived_supply_humidity_ratio = None;
        snapshot.zone_node_humidity_ratio = None;
        snapshot.calculated_supply_humidity_ratio_for_dehumidification = None;
        snapshot.assigned_supply_humidity_ratio_for_dehumidification = None;
        snapshot.resulting_supply_humidity_ratio_for_dehumidification = None;
    }
    values_match && left == right
}

fn values_fit_route(snapshot: Snapshot, route: Route) -> bool {
    let active = route
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted;
    if !active {
        return [
            snapshot.zone_dehumidifying_setpoint_moisture_demand_read,
            snapshot.supply_mass_flow_rate_read,
            snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
            snapshot.zone_node_humidity_ratio_read,
            snapshot.supply_humidity_ratio_for_dehumidification_calculated,
            snapshot.supply_humidity_ratio_for_dehumidification_assigned,
        ]
        .into_iter()
        .all(|flag| !flag)
            && [
                snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
                snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
                snapshot.supply_mass_flow_rate_kg_per_s,
                snapshot.moisture_demand_derived_supply_humidity_ratio,
                snapshot.zone_node_humidity_ratio,
                snapshot.calculated_supply_humidity_ratio_for_dehumidification,
                snapshot.assigned_supply_humidity_ratio_for_dehumidification,
                snapshot.resulting_supply_humidity_ratio_for_dehumidification,
            ]
            .into_iter()
            .all(|value| value.is_none());
    }
    if ![
        snapshot.zone_dehumidifying_setpoint_moisture_demand_read,
        snapshot.supply_mass_flow_rate_read,
        snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
        snapshot.zone_node_humidity_ratio_read,
        snapshot.supply_humidity_ratio_for_dehumidification_calculated,
        snapshot.supply_humidity_ratio_for_dehumidification_assigned,
    ]
    .into_iter()
    .all(|flag| flag)
    {
        return false;
    }
    let (
        Some(predecessor_demand),
        Some(demand),
        Some(flow),
        Some(quotient),
        Some(zone_humidity),
        Some(calculated),
        Some(assigned),
        Some(resulting),
    ) = (
        snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.moisture_demand_derived_supply_humidity_ratio,
        snapshot.zone_node_humidity_ratio,
        snapshot.calculated_supply_humidity_ratio_for_dehumidification,
        snapshot.assigned_supply_humidity_ratio_for_dehumidification,
        snapshot.resulting_supply_humidity_ratio_for_dehumidification,
    )
    else {
        return false;
    };
    let expected_quotient = demand / flow;
    let expected_calculated = expected_quotient + zone_humidity;
    predecessor_demand.to_bits() == demand.to_bits()
        && quotient.to_bits() == expected_quotient.to_bits()
        && calculated.to_bits() == expected_calculated.to_bits()
        && assigned.to_bits() == calculated.to_bits()
        && resulting.to_bits() == calculated.to_bits()
}

pub(super) fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
