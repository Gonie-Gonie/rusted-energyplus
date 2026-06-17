//! Psychrometric helpers used by IdealLoads outdoor-air compatibility code.

use crate::{
    energyplus_psychrometric_humidity_ratio_from_rh, ideal_loads::moist_air_enthalpy_j_per_kg,
};

const ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K: f64 = 1.004_84;
const ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG: f64 = 2500.94;
const ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K: f64 = 1.858_95;
const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1.0e-5;

pub(super) fn heat_recovery_saturation_adjusted_state(
    temperature_c: f64,
    humidity_ratio: f64,
    enthalpy_j_per_kg: f64,
    barometric_pressure_pa: f64,
) -> (f64, f64) {
    let Some(saturation_temperature_c) = saturation_temperature_from_enthalpy_and_pressure_c(
        enthalpy_j_per_kg,
        barometric_pressure_pa,
    ) else {
        return (temperature_c, humidity_ratio);
    };
    if saturation_temperature_c <= temperature_c {
        return (temperature_c, humidity_ratio);
    }
    (
        saturation_temperature_c,
        humidity_ratio_from_enthalpy_and_dry_bulb(enthalpy_j_per_kg, saturation_temperature_c)
            .max(ENERGYPLUS_MIN_HUMIDITY_RATIO),
    )
}

pub(super) fn dry_bulb_from_enthalpy_and_humidity_ratio(
    enthalpy_j_per_kg: f64,
    humidity_ratio: f64,
) -> f64 {
    (enthalpy_j_per_kg / 1000.0 - ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG * humidity_ratio)
        / (ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K
            + ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * humidity_ratio)
}

fn humidity_ratio_from_enthalpy_and_dry_bulb(enthalpy_j_per_kg: f64, dry_bulb_c: f64) -> f64 {
    (enthalpy_j_per_kg / 1000.0 - ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * dry_bulb_c)
        / (ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG
            + ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * dry_bulb_c)
}

fn saturation_temperature_from_enthalpy_and_pressure_c(
    enthalpy_j_per_kg: f64,
    barometric_pressure_pa: f64,
) -> Option<f64> {
    if !enthalpy_j_per_kg.is_finite()
        || !barometric_pressure_pa.is_finite()
        || barometric_pressure_pa <= 0.0
    {
        return None;
    }

    let mut low_c = -100.0;
    let mut high_c = 200.0;
    let low_enthalpy = saturated_air_enthalpy_j_per_kg(low_c, barometric_pressure_pa)?;
    let high_enthalpy = saturated_air_enthalpy_j_per_kg(high_c, barometric_pressure_pa)?;
    if enthalpy_j_per_kg <= low_enthalpy {
        return Some(low_c);
    }
    if enthalpy_j_per_kg >= high_enthalpy {
        return Some(high_c);
    }

    for _ in 0..80 {
        let mid_c = 0.5 * (low_c + high_c);
        let mid_enthalpy = saturated_air_enthalpy_j_per_kg(mid_c, barometric_pressure_pa)?;
        if mid_enthalpy < enthalpy_j_per_kg {
            low_c = mid_c;
        } else {
            high_c = mid_c;
        }
    }
    Some(0.5 * (low_c + high_c))
}

fn saturated_air_enthalpy_j_per_kg(temperature_c: f64, barometric_pressure_pa: f64) -> Option<f64> {
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        temperature_c,
        1.0,
        barometric_pressure_pa,
    )?;
    Some(moist_air_enthalpy_j_per_kg(
        temperature_c,
        saturation_humidity_ratio,
    ))
}
