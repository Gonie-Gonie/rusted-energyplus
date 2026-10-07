//! Psychrometric helpers shared by IdealLoads calculation branches.

use crate::{
    energyplus_moist_air_density_kg_per_m3,
    psychrometrics::{energyplus_psy_h_fn_tdb_w, energyplus_psy_w_fn_tdb_h},
};

pub(super) const DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3: f64 = 1.2;
pub(super) const STANDARD_PRESSURE_SEA_LEVEL_PA: f64 = 101_325.0;
pub(super) const MINIMUM_HUMIDITY_RATIO: f64 = 1.0e-5;

const ENERGYPLUS_STANDARD_DRY_BULB_C: f64 = 20.0;
const ENERGYPLUS_STANDARD_HUMIDITY_RATIO: f64 = 0.0;

/// EnergyPlus `PsyHFnTdbW` moist-air enthalpy in J/kg, including its W floor.
#[must_use]
#[track_caller]
pub fn moist_air_enthalpy_j_per_kg(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    energyplus_psy_h_fn_tdb_w(dry_bulb_c, humidity_ratio)
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

#[track_caller]
pub(super) fn humidity_ratio_from_enthalpy_and_dry_bulb(
    enthalpy_j_per_kg: f64,
    dry_bulb_c: f64,
) -> f64 {
    // PurchasedAirManager.cc:2222 directly consumes its canonical enthalpy.
    // Preserve source J/kg grouping and its strictly-negative-only correction.
    energyplus_psy_w_fn_tdb_h(dry_bulb_c, enthalpy_j_per_kg)
}

pub(super) fn nearly_equal_humidity(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1.0e-12
}

#[cfg(test)]
mod tests {
    use super::moist_air_enthalpy_j_per_kg;
    use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

    #[test]
    fn production_enthalpy_applies_the_normal_source_floor_and_operation_order() {
        let at_floor = energyplus_psy_h_fn_tdb_w(24.0, 1.0e-5);
        for humidity_ratio in [-0.001, -0.0, 0.0, 5.0e-6, 1.0e-5] {
            assert_eq!(
                moist_air_enthalpy_j_per_kg(24.0, humidity_ratio).to_bits(),
                at_floor.to_bits()
            );
        }
        for (temperature, humidity) in [(14.0, 0.008), (-20.0, 0.02), (0.0, 0.01)] {
            assert_eq!(
                moist_air_enthalpy_j_per_kg(temperature, humidity).to_bits(),
                energyplus_psy_h_fn_tdb_w(temperature, humidity).to_bits()
            );
        }
        assert!(moist_air_enthalpy_j_per_kg(24.0, f64::NAN).is_nan());
    }
}
