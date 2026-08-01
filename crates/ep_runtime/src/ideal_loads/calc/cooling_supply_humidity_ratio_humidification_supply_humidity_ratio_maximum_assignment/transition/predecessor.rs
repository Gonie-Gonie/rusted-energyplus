//! CP374 predecessor provenance, route, and binary64 validation.

use super::Route;
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRetainedRoute as Cp374Route,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_route,
};

pub(in crate::ideal_loads::calc) fn predecessor_route(predecessor: Predecessor) -> Option<Route> {
    Some(match cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_snapshot_route(predecessor)? {
        Cp374Route::UnitOff => Route::UnitOff,
        Cp374Route::NonCooling => Route::NonCooling,
        Cp374Route::PositiveGuardFalseFallthrough => Route::PositiveGuardFalseFallthrough,
        Cp374Route::HeatingAvailabilityGuardFalseFallthrough => Route::HeatingAvailabilityGuardFalseFallthrough,
        Cp374Route::HumidificationControlGuardFalseFallthrough => Route::HumidificationControlGuardFalseFallthrough,
        Cp374Route::DehumidificationControlHumidistatSupplyHumidityRatioForHumidificationMaximumLimitExecuted => Route::DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
        Cp374Route::DehumidificationControlNoneSupplyHumidityRatioForHumidificationMaximumLimitExecuted => Route::DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
        Cp374Route::DehumidificationControlGuardFalseFallthrough => Route::DehumidificationControlGuardFalseFallthrough,
    })
}
