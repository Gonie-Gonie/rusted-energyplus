//! Psychrometric helpers shared by IdealLoads calculation branches.

use crate::energyplus_moist_air_density_kg_per_m3;

pub(super) const DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3: f64 = 1.2;
pub(super) const STANDARD_PRESSURE_SEA_LEVEL_PA: f64 = 101_325.0;
pub(super) const MINIMUM_HUMIDITY_RATIO: f64 = 1.0e-5;

const ENERGYPLUS_STANDARD_DRY_BULB_C: f64 = 20.0;
const ENERGYPLUS_STANDARD_HUMIDITY_RATIO: f64 = 0.0;
const ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K: f64 = 1.004_84;
const ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG: f64 = 2500.94;
const ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K: f64 = 1.858_95;

/// EnergyPlus `PsyHFnTdbW`-style moist-air enthalpy in J/kg.
#[must_use]
pub fn moist_air_enthalpy_j_per_kg(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    1000.0
        * (ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * dry_bulb_c
            + humidity_ratio
                * (ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG
                    + ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * dry_bulb_c))
}

/// Returns EnergyPlus `StdRhoAir` from site elevation.
#[must_use]
pub fn energyplus_standard_air_density_kg_per_m3(elevation_m: f64) -> Option<f64> {
    let base = standard_pressure_elevation_base(elevation_m)?;
    let standard_barometric_pressure_pa = STANDARD_PRESSURE_SEA_LEVEL_PA * base.powf(5.2559);
    energyplus_moist_air_density_kg_per_m3(
        standard_barometric_pressure_pa,
        ENERGYPLUS_STANDARD_DRY_BULB_C,
        ENERGYPLUS_STANDARD_HUMIDITY_RATIO,
    )
}

pub(super) fn standard_pressure_elevation_base(elevation_m: f64) -> Option<f64> {
    if !elevation_m.is_finite() {
        return None;
    }
    let base = 1.0 - 2.255_77e-05 * elevation_m;
    (base > 0.0).then_some(base)
}

pub(super) fn humidity_ratio_from_enthalpy_and_dry_bulb(
    enthalpy_j_per_kg: f64,
    dry_bulb_c: f64,
) -> f64 {
    (enthalpy_j_per_kg / 1000.0 - ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * dry_bulb_c)
        / (ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG
            + ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * dry_bulb_c)
}

pub(super) fn nearly_equal_humidity(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1.0e-12
}
