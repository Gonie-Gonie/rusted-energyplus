//! Zone moisture demand predictor helpers.

use crate::{
    energyplus_moist_air_density_kg_per_m3, energyplus_psychrometric_humidity_ratio_from_rh,
    energyplus_water_vapor_gas_enthalpy_j_per_kg,
};

use super::types::IdealLoadsZoneState;

const THIRD_ORDER_CURRENT_WEIGHT: f64 = 11.0 / 6.0;

/// Evaluates the EnergyPlus ThirdOrder humidity-ratio history term.
#[must_use]
pub fn third_order_humidity_history_term(previous_zone_timestep_humidity_ratios: [f64; 3]) -> f64 {
    let [w_prev_0, w_prev_1, w_prev_2] = previous_zone_timestep_humidity_ratios;
    3.0 * w_prev_0 - 1.5 * w_prev_1 + (1.0 / 3.0) * w_prev_2
}

/// Inputs for the no-OA ThirdOrder `calcPredictedHumidityRatio` subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoOaThirdOrderMoistureDemandInput {
    /// Zone state used by EnergyPlus as `ZT` and `airHumRat`.
    pub zone_state: IdealLoadsZoneState,
    /// `WPrevZoneTSTemp[0..2]` in EnergyPlus source order.
    pub previous_zone_timestep_humidity_ratios: [f64; 3],
    /// Zone volume in m3.
    pub zone_volume_m3: f64,
    /// `ZoneVolCapMultpMoist`; EnergyPlus default is 1.0.
    pub zone_moisture_capacity_multiplier: f64,
    /// System timestep seconds.
    pub timestep_seconds: f64,
    /// Barometric pressure in Pa.
    pub barometric_pressure_pa: f64,
    /// Internal latent gain in W.
    pub latent_gain_w: f64,
    /// Humidifying RH schedule value in percent.
    pub humidifying_relative_humidity_percent: f64,
    /// Dehumidifying RH schedule value in percent.
    pub dehumidifying_relative_humidity_percent: f64,
    /// Zone multiplier times list multiplier.
    pub zone_multiplier: f64,
}

/// No-OA ThirdOrder moisture demand prediction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoOaThirdOrderMoistureDemand {
    /// EnergyPlus `OutputRequiredToHumidifyingSP` in kgWater/s.
    pub humidifying_setpoint_load_kg_per_s: f64,
    /// EnergyPlus `OutputRequiredToDehumidifyingSP` in kgWater/s.
    pub dehumidifying_setpoint_load_kg_per_s: f64,
    /// EnergyPlus `TotalOutputRequired` in kgWater/s.
    pub total_output_required_kg_per_s: f64,
    /// Humidifying RH setpoint converted to humidity ratio.
    pub humidifying_setpoint_humidity_ratio: f64,
    /// Dehumidifying RH setpoint converted to humidity ratio.
    pub dehumidifying_setpoint_humidity_ratio: f64,
}

/// Inputs for the no-OA ThirdOrder `correctHumRat` subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoOaThirdOrderHumidityCorrectorInput {
    /// Zone state used by EnergyPlus as `ZT` and `airHumRat`.
    pub zone_state: IdealLoadsZoneState,
    /// `WPrevZoneTSTemp[0..2]` in EnergyPlus source order.
    pub previous_zone_timestep_humidity_ratios: [f64; 3],
    /// Zone volume in m3.
    pub zone_volume_m3: f64,
    /// `ZoneVolCapMultpMoist`; EnergyPlus default is 1.0.
    pub zone_moisture_capacity_multiplier: f64,
    /// System timestep seconds.
    pub timestep_seconds: f64,
    /// Barometric pressure in Pa.
    pub barometric_pressure_pa: f64,
    /// Internal latent gain in W.
    pub latent_gain_w: f64,
    /// Supply inlet dry-air mass flow in kgDryAir/s after zone multiplier division.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Supply inlet humidity ratio in kgWater/kgDryAir.
    pub supply_humidity_ratio: f64,
}

/// No-OA ThirdOrder zone humidity correction result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoOaThirdOrderHumidityCorrector {
    /// Corrected EnergyPlus zone air humidity ratio.
    pub zone_air_humidity_ratio: f64,
    /// Moisture-independent coefficient in kgWater/s.
    pub b_kg_water_per_s: f64,
    /// Moisture-dependent coefficient in kgDryAir/s.
    pub a_kg_dry_air_per_s: f64,
}

/// Calculates the no-OA ThirdOrder subset of EnergyPlus
/// `ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio`.
#[must_use]
pub fn calc_no_oa_third_order_moisture_demand_compat(
    input: NoOaThirdOrderMoistureDemandInput,
) -> Option<NoOaThirdOrderMoistureDemand> {
    if !input.zone_volume_m3.is_finite()
        || input.zone_volume_m3 <= 0.0
        || !input.zone_moisture_capacity_multiplier.is_finite()
        || input.zone_moisture_capacity_multiplier <= 0.0
        || !input.timestep_seconds.is_finite()
        || input.timestep_seconds <= 0.0
        || !input.latent_gain_w.is_finite()
        || !input.zone_multiplier.is_finite()
        || input.zone_multiplier <= 0.0
        || !input
            .previous_zone_timestep_humidity_ratios
            .iter()
            .all(|value| value.is_finite())
    {
        return None;
    }

    let density_kg_per_m3 = energyplus_moist_air_density_kg_per_m3(
        input.barometric_pressure_pa,
        input.zone_state.air_temperature_c,
        input.zone_state.air_humidity_ratio,
    )?;
    let vapor_enthalpy_j_per_kg =
        energyplus_water_vapor_gas_enthalpy_j_per_kg(input.zone_state.air_temperature_c);
    if vapor_enthalpy_j_per_kg <= 0.0 {
        return None;
    }

    let c = density_kg_per_m3 * input.zone_volume_m3 * input.zone_moisture_capacity_multiplier
        / input.timestep_seconds;
    let b = input.latent_gain_w / vapor_enthalpy_j_per_kg;
    let third_order_history =
        third_order_humidity_history_term(input.previous_zone_timestep_humidity_ratios);

    let humidifying_rh_percent = if input.humidifying_relative_humidity_percent
        > input.dehumidifying_relative_humidity_percent
    {
        input.dehumidifying_relative_humidity_percent
    } else {
        input.humidifying_relative_humidity_percent
    };
    let humidifying_rh_fraction = (humidifying_rh_percent / 100.0).clamp(0.0, 1.0);
    let dehumidifying_rh_percent = input.dehumidifying_relative_humidity_percent;
    let dehumidifying_rh_fraction = (dehumidifying_rh_percent / 100.0).clamp(0.0, 1.0);

    let humidifying_setpoint_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        input.zone_state.air_temperature_c,
        humidifying_rh_fraction,
        input.barometric_pressure_pa,
    )?;
    let dehumidifying_setpoint_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        input.zone_state.air_temperature_c,
        dehumidifying_rh_fraction,
        input.barometric_pressure_pa,
    )?;

    let humidifying_unmultiplied =
        THIRD_ORDER_CURRENT_WEIGHT * c * humidifying_setpoint_humidity_ratio
            - (b + c * third_order_history);
    let dehumidifying_unmultiplied =
        THIRD_ORDER_CURRENT_WEIGHT * c * dehumidifying_setpoint_humidity_ratio
            - (b + c * third_order_history);
    let total_unmultiplied = if humidifying_rh_percent == dehumidifying_rh_percent {
        humidifying_unmultiplied
    } else if humidifying_unmultiplied > 0.0 && dehumidifying_unmultiplied > 0.0 {
        humidifying_unmultiplied
    } else if humidifying_unmultiplied < 0.0 && dehumidifying_unmultiplied < 0.0 {
        dehumidifying_unmultiplied
    } else if humidifying_unmultiplied <= 0.0 && dehumidifying_unmultiplied >= 0.0 {
        0.0
    } else {
        return None;
    };

    Some(NoOaThirdOrderMoistureDemand {
        humidifying_setpoint_load_kg_per_s: humidifying_unmultiplied * input.zone_multiplier,
        dehumidifying_setpoint_load_kg_per_s: dehumidifying_unmultiplied * input.zone_multiplier,
        total_output_required_kg_per_s: total_unmultiplied * input.zone_multiplier,
        humidifying_setpoint_humidity_ratio,
        dehumidifying_setpoint_humidity_ratio,
    })
}

/// Calculates the no-OA ThirdOrder subset of EnergyPlus
/// `ZoneSpaceHeatBalanceData::correctHumRat`.
#[must_use]
pub fn correct_no_oa_third_order_humidity_ratio_compat(
    input: NoOaThirdOrderHumidityCorrectorInput,
) -> Option<NoOaThirdOrderHumidityCorrector> {
    if !input.zone_volume_m3.is_finite()
        || input.zone_volume_m3 <= 0.0
        || !input.zone_moisture_capacity_multiplier.is_finite()
        || input.zone_moisture_capacity_multiplier <= 0.0
        || !input.timestep_seconds.is_finite()
        || input.timestep_seconds <= 0.0
        || !input.latent_gain_w.is_finite()
        || !input.supply_mass_flow_rate_kg_per_s.is_finite()
        || input.supply_mass_flow_rate_kg_per_s < 0.0
        || !input.supply_humidity_ratio.is_finite()
        || input.supply_humidity_ratio < 0.0
        || !input
            .previous_zone_timestep_humidity_ratios
            .iter()
            .all(|value| value.is_finite())
    {
        return None;
    }

    let density_kg_per_m3 = energyplus_moist_air_density_kg_per_m3(
        input.barometric_pressure_pa,
        input.zone_state.air_temperature_c,
        input.zone_state.air_humidity_ratio,
    )?;
    let vapor_enthalpy_j_per_kg =
        energyplus_water_vapor_gas_enthalpy_j_per_kg(input.zone_state.air_temperature_c);
    if vapor_enthalpy_j_per_kg <= 0.0 {
        return None;
    }

    let c = density_kg_per_m3 * input.zone_volume_m3 * input.zone_moisture_capacity_multiplier
        / input.timestep_seconds;
    let b = input.latent_gain_w / vapor_enthalpy_j_per_kg
        + input.supply_mass_flow_rate_kg_per_s * input.supply_humidity_ratio;
    let a = input.supply_mass_flow_rate_kg_per_s;
    let third_order_history =
        third_order_humidity_history_term(input.previous_zone_timestep_humidity_ratios);
    let denominator = THIRD_ORDER_CURRENT_WEIGHT * c + a;
    if denominator <= 0.0 {
        return None;
    }
    let mut corrected = (b + c * third_order_history) / denominator;
    if corrected < 0.0 {
        corrected = 0.0;
    }
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        input.zone_state.air_temperature_c,
        1.0,
        input.barometric_pressure_pa,
    )?;
    if corrected > saturation_humidity_ratio {
        corrected = saturation_humidity_ratio;
    }

    Some(NoOaThirdOrderHumidityCorrector {
        zone_air_humidity_ratio: corrected,
        b_kg_water_per_s: b,
        a_kg_dry_air_per_s: a,
    })
}
