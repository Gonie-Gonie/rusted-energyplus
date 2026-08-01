//! CP373 source provenance and eight-route structural validation.

use super::{Route, Snapshot};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment::transition::predecessor_route;

pub(super) fn structural_route(snapshot: Snapshot) -> Option<Route> {
    if snapshot.source
        != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE
        || snapshot.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
        || snapshot.source_order
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER
    {
        return None;
    }
    let route = predecessor_route(super::predecessor_snapshot(snapshot))?;
    let h = snapshot
        .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed;
    let n = snapshot
        .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed;
    let local_route_matches = match route {
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationAssignmentExecuted => h && !n,
        Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationAssignmentExecuted => !h && n,
        _ => !h && !n,
    };
    local_route_matches.then_some(route)
}
