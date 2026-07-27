//! No-OA humidity-control helper calculations.

use crate::{energyplus_psychrometric_humidity_ratio_from_rh, zone_equipment::ZoneSysEnergyDemand};
use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::limits::{
    IdealLoadsSensibleLimitContext, cooling_capacity_limit_is_zero, heating_capacity_limit_is_zero,
};
use super::psychrometrics::{
    MINIMUM_HUMIDITY_RATIO, humidity_ratio_from_enthalpy_and_dry_bulb, moist_air_enthalpy_j_per_kg,
};
use super::types::IdealLoadsZoneState;

const SMALL_HUMIDITY_RATIO_DIFFERENCE: f64 = 0.00025;

pub(super) fn humidistat_dehumidification_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    if system.dehumidification_control_type != DehumidificationControlType::Humidistat
        || cooling_capacity_limit_is_zero(system, context)
    {
        return 0.0;
    }

    let moisture_demand_kg_per_s = demand.remaining_output_req_to_dehumid_sp_kg_per_s;
    let delta_humidity_ratio =
        system.minimum_cooling_supply_air_humidity_ratio - zone_state.air_humidity_ratio;
    if delta_humidity_ratio < -SMALL_HUMIDITY_RATIO_DIFFERENCE && moisture_demand_kg_per_s < 0.0 {
        (moisture_demand_kg_per_s / delta_humidity_ratio).max(0.0)
    } else {
        0.0
    }
}

pub(super) fn humidistat_humidification_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    if system.humidification_control_type != HumidificationControlType::Humidistat
        || heating_capacity_limit_is_zero(system, context)
    {
        return 0.0;
    }

    let moisture_demand_kg_per_s = demand.remaining_output_req_to_humid_sp_kg_per_s;
    let delta_humidity_ratio =
        system.maximum_heating_supply_air_humidity_ratio - zone_state.air_humidity_ratio;
    if delta_humidity_ratio > SMALL_HUMIDITY_RATIO_DIFFERENCE && moisture_demand_kg_per_s > 0.0 {
        (moisture_demand_kg_per_s / delta_humidity_ratio).max(0.0)
    } else {
        0.0
    }
}

pub(super) fn heating_supply_humidity_ratio(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    supply_temperature_c: f64,
    mixed_supply_humidity_ratio: f64,
    supply_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    let mut supply_humidity_ratio = match system.humidification_control_type {
        HumidificationControlType::Humidistat if supply_mass_flow_rate_kg_per_s > 0.0 => {
            let supply_humidity_ratio_for_humidification =
                (demand.remaining_output_req_to_humid_sp_kg_per_s / supply_mass_flow_rate_kg_per_s
                    + zone_state.air_humidity_ratio)
                    .min(system.maximum_heating_supply_air_humidity_ratio);
            mixed_supply_humidity_ratio.max(supply_humidity_ratio_for_humidification)
        }
        HumidificationControlType::ConstantSupplyHumidityRatio
            if supply_mass_flow_rate_kg_per_s > 0.0 =>
        {
            system.maximum_heating_supply_air_humidity_ratio
        }
        _ => mixed_supply_humidity_ratio,
    };
    if system.dehumidification_control_type == DehumidificationControlType::Humidistat
        && supply_mass_flow_rate_kg_per_s > 0.0
        && matches!(
            system.humidification_control_type,
            HumidificationControlType::Humidistat | HumidificationControlType::None
        )
    {
        let supply_humidity_ratio_for_dehumidification =
            (demand.remaining_output_req_to_dehumid_sp_kg_per_s / supply_mass_flow_rate_kg_per_s
                + zone_state.air_humidity_ratio)
                .max(system.minimum_cooling_supply_air_humidity_ratio);
        supply_humidity_ratio =
            supply_humidity_ratio.min(supply_humidity_ratio_for_dehumidification);
    }
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        supply_temperature_c,
        1.0,
        context.barometric_pressure_pa,
    )
    .unwrap_or(f64::INFINITY);
    supply_humidity_ratio.min(saturation_humidity_ratio)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn cooling_supply_humidity_ratio(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    mixed_air_state: IdealLoadsZoneState,
    supply_temperature_c: f64,
    mixed_supply_humidity_ratio: f64,
    supply_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    supply_air_sensible_cooling_rate_w: f64,
    mixed_air_enthalpy_j_per_kg: f64,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    let supply_humidity_ratio = match system.dehumidification_control_type {
        DehumidificationControlType::ConstantSensibleHeatRatio
            if supply_mass_flow_rate_kg_per_s > 0.0 && system.cooling_sensible_heat_ratio > 0.0 =>
        {
            let cooling_total_output_w =
                supply_air_sensible_cooling_rate_w / system.cooling_sensible_heat_ratio;
            let supply_enthalpy_j_per_kg = (mixed_air_enthalpy_j_per_kg
                - cooling_total_output_w / supply_mass_flow_rate_kg_per_s)
                .max(moist_air_enthalpy_j_per_kg(
                    supply_temperature_c,
                    MINIMUM_HUMIDITY_RATIO,
                ));
            let humidity_from_enthalpy = humidity_ratio_from_enthalpy_and_dry_bulb(
                supply_enthalpy_j_per_kg,
                supply_temperature_c,
            )
            .max(MINIMUM_HUMIDITY_RATIO);
            mixed_supply_humidity_ratio
                .min(humidity_from_enthalpy)
                .max(system.minimum_cooling_supply_air_humidity_ratio)
                .min(mixed_air_state.air_humidity_ratio)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if supply_mass_flow_rate_kg_per_s > 0.0 =>
        {
            system
                .minimum_cooling_supply_air_humidity_ratio
                .max(MINIMUM_HUMIDITY_RATIO)
        }
        DehumidificationControlType::Humidistat if supply_mass_flow_rate_kg_per_s > 0.0 => {
            let supply_humidity_ratio_for_dehumidification = (demand
                .remaining_output_req_to_dehumid_sp_kg_per_s
                / supply_mass_flow_rate_kg_per_s
                + zone_state.air_humidity_ratio)
                .max(system.minimum_cooling_supply_air_humidity_ratio);
            mixed_supply_humidity_ratio.min(supply_humidity_ratio_for_dehumidification)
        }
        _ => mixed_supply_humidity_ratio,
    };
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        supply_temperature_c,
        1.0,
        context.barometric_pressure_pa,
    )
    .unwrap_or(f64::INFINITY);
    supply_humidity_ratio.min(saturation_humidity_ratio)
}
