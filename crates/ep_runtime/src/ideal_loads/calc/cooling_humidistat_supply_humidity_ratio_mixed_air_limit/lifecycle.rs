use ep_model::IdealLoadsAirSystemId;

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState,
};
use crate::ideal_loads::PurchasedAirRuntimeState;

/// Final selected-unit CP362 lifecycle summary.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary {
    pub source: &'static str,
    pub first_excluded_source: &'static str,
    pub state: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState,
}

/// Returns the bounded selected-unit CP362 lifecycle summary.
pub fn purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError,
> {
    let unit = runtime.units.get(&system).ok_or(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError::UnknownSystem {
            system,
        },
    )?;
    Ok(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
            state: unit
                .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
                .clone(),
        },
    )
}
