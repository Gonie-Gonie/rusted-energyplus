//! No-OA/no-limit IdealLoads sensible load calculation.

use crate::{energyplus_moist_air_specific_heat_j_per_kg_k, zone_equipment::ZoneSysEnergyDemand};
use ep_model::IdealLoadsAirSystem;

const SMALL_TEMPERATURE_DIFFERENCE_C: f64 = 0.001;

/// Operating mode selected by the first IdealLoads sensible subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsSensibleMode {
    /// Unit is unavailable.
    Off,
    /// No sensible load is active.
    Deadband,
    /// Cooling branch is active.
    Cooling,
    /// Heating branch is active.
    Heating,
}

/// Zone-side inputs to `CalcPurchAirLoads`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsZoneState {
    /// Zone air temperature in C.
    pub air_temperature_c: f64,
    /// Zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
}

/// Result of the no-OA/no-limit sensible calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsSensibleResult {
    /// Selected operating mode.
    pub mode: IdealLoadsSensibleMode,
    /// EnergyPlus `PsyCpAirFnW` value in J/kg-K.
    pub cp_air_j_per_kg_k: f64,
    /// Final supply temperature in C.
    pub supply_temperature_c: f64,
    /// Final supply humidity ratio in kgWater/kgDryAir.
    pub supply_humidity_ratio: f64,
    /// Final supply enthalpy in J/kg.
    pub supply_enthalpy_j_per_kg: f64,
    /// Final supply mass flow in kg/s.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Sensible heating flow contribution in kg/s.
    pub heating_mass_flow_rate_kg_per_s: f64,
    /// Sensible cooling flow contribution in kg/s.
    pub cooling_mass_flow_rate_kg_per_s: f64,
    /// Zone total heating rate in W.
    pub zone_total_heating_rate_w: f64,
    /// Zone total cooling rate in W.
    pub zone_total_cooling_rate_w: f64,
    /// Zone sensible heating rate in W.
    pub zone_sensible_heating_rate_w: f64,
    /// Zone sensible cooling rate in W.
    pub zone_sensible_cooling_rate_w: f64,
    /// Supply air total heating rate in W.
    pub supply_air_total_heating_rate_w: f64,
    /// Supply air total cooling rate in W.
    pub supply_air_total_cooling_rate_w: f64,
}

/// Calculates the no-outdoor-air, no-limit, sensible-only IdealLoads branch.
///
/// The zone demand values must already come from a source-order
/// `ZoneSysEnergyDemand` equivalent. This function intentionally does not
/// synthesize zone demand from a simplified zone model.
#[must_use]
pub fn calc_no_oa_no_limit_sensible_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    unit_available: bool,
) -> IdealLoadsSensibleResult {
    let cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let supply_humidity_ratio = zone_state.air_humidity_ratio;

    if !unit_available {
        return zero_result(
            IdealLoadsSensibleMode::Off,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        );
    }

    let heating_load_w = demand.remaining_output_req_to_heat_sp_w.max(0.0);
    let cooling_load_w = demand.remaining_output_req_to_cool_sp_w.abs().max(0.0);

    let heating_delta_t =
        system.maximum_heating_supply_air_temperature_c - zone_state.air_temperature_c;
    let heating_mass_flow_rate_kg_per_s =
        if heating_load_w > 0.0 && heating_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            heating_load_w / (cp_air_j_per_kg_k * heating_delta_t)
        } else {
            0.0
        };

    let cooling_delta_t =
        zone_state.air_temperature_c - system.minimum_cooling_supply_air_temperature_c;
    let cooling_mass_flow_rate_kg_per_s =
        if cooling_load_w > 0.0 && cooling_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            cooling_load_w / (cp_air_j_per_kg_k * cooling_delta_t)
        } else {
            0.0
        };

    if heating_mass_flow_rate_kg_per_s > 0.0
        && heating_mass_flow_rate_kg_per_s >= cooling_mass_flow_rate_kg_per_s
    {
        let supply_temperature_c = system.maximum_heating_supply_air_temperature_c;
        let supply_enthalpy_j_per_kg =
            moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);
        IdealLoadsSensibleResult {
            mode: IdealLoadsSensibleMode::Heating,
            cp_air_j_per_kg_k,
            supply_temperature_c,
            supply_humidity_ratio,
            supply_enthalpy_j_per_kg,
            supply_mass_flow_rate_kg_per_s: heating_mass_flow_rate_kg_per_s,
            heating_mass_flow_rate_kg_per_s,
            cooling_mass_flow_rate_kg_per_s: 0.0,
            zone_total_heating_rate_w: heating_load_w,
            zone_total_cooling_rate_w: 0.0,
            zone_sensible_heating_rate_w: heating_load_w,
            zone_sensible_cooling_rate_w: 0.0,
            supply_air_total_heating_rate_w: heating_load_w,
            supply_air_total_cooling_rate_w: 0.0,
        }
    } else if cooling_mass_flow_rate_kg_per_s > 0.0 {
        let supply_temperature_c = system.minimum_cooling_supply_air_temperature_c;
        let supply_enthalpy_j_per_kg =
            moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);
        IdealLoadsSensibleResult {
            mode: IdealLoadsSensibleMode::Cooling,
            cp_air_j_per_kg_k,
            supply_temperature_c,
            supply_humidity_ratio,
            supply_enthalpy_j_per_kg,
            supply_mass_flow_rate_kg_per_s: cooling_mass_flow_rate_kg_per_s,
            heating_mass_flow_rate_kg_per_s: 0.0,
            cooling_mass_flow_rate_kg_per_s,
            zone_total_heating_rate_w: 0.0,
            zone_total_cooling_rate_w: cooling_load_w,
            zone_sensible_heating_rate_w: 0.0,
            zone_sensible_cooling_rate_w: cooling_load_w,
            supply_air_total_heating_rate_w: 0.0,
            supply_air_total_cooling_rate_w: cooling_load_w,
        }
    } else {
        zero_result(
            IdealLoadsSensibleMode::Deadband,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        )
    }
}

/// EnergyPlus `PsyHFnTdbW`-style moist-air enthalpy in J/kg.
#[must_use]
pub fn moist_air_enthalpy_j_per_kg(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    1000.0 * (1.006 * dry_bulb_c + humidity_ratio * (2501.0 + 1.86 * dry_bulb_c))
}

fn zero_result(
    mode: IdealLoadsSensibleMode,
    cp_air_j_per_kg_k: f64,
    supply_temperature_c: f64,
    supply_humidity_ratio: f64,
) -> IdealLoadsSensibleResult {
    IdealLoadsSensibleResult {
        mode,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        supply_enthalpy_j_per_kg: moist_air_enthalpy_j_per_kg(
            supply_temperature_c,
            supply_humidity_ratio,
        ),
        supply_mass_flow_rate_kg_per_s: 0.0,
        heating_mass_flow_rate_kg_per_s: 0.0,
        cooling_mass_flow_rate_kg_per_s: 0.0,
        zone_total_heating_rate_w: 0.0,
        zone_total_cooling_rate_w: 0.0,
        zone_sensible_heating_rate_w: 0.0,
        zone_sensible_cooling_rate_w: 0.0,
        supply_air_total_heating_rate_w: 0.0,
        supply_air_total_cooling_rate_w: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zone_equipment::ZoneSysEnergyDemand;
    use ep_model::{
        DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
        HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
        IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType, ZoneId,
    };

    #[test]
    fn no_oa_sensible_heating_uses_supply_delta_t_and_moist_air_cp() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };
        let result = calc_no_oa_no_limit_sensible_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
            true,
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert!((result.cp_air_j_per_kg_k - cp).abs() < 1.0e-12);
        assert!((result.supply_temperature_c - 50.0).abs() < 1.0e-12);
        assert!((result.supply_mass_flow_rate_kg_per_s - 3000.0 / (cp * 30.0)).abs() < 1.0e-12);
        assert!((result.zone_total_heating_rate_w - 3000.0).abs() < 1.0e-12);
        assert_eq!(result.zone_total_cooling_rate_w, 0.0);
    }

    #[test]
    fn no_oa_sensible_cooling_uses_absolute_cooling_demand() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 25.0,
            air_humidity_ratio: 0.008,
        };
        let result = calc_no_oa_no_limit_sensible_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
            true,
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert!((result.supply_temperature_c - 13.0).abs() < 1.0e-12);
        assert!((result.supply_mass_flow_rate_kg_per_s - 2400.0 / (cp * 12.0)).abs() < 1.0e-12);
        assert!((result.zone_total_cooling_rate_w - 2400.0).abs() < 1.0e-12);
        assert_eq!(result.zone_total_heating_rate_w, 0.0);
    }

    #[test]
    fn unavailable_unit_writes_dead_flow_and_zone_condition() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 22.5,
            air_humidity_ratio: 0.007,
        };
        let result = calc_no_oa_no_limit_sensible_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, -3000.0),
            false,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Off);
        assert_eq!(result.supply_mass_flow_rate_kg_per_s, 0.0);
        assert!((result.supply_temperature_c - 22.5).abs() < 1.0e-12);
    }

    fn test_system() -> IdealLoadsAirSystem {
        IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLETS"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        }
    }
}
