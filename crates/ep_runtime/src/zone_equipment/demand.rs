//! Zone system demand state for compatibility-mode HVAC components.

use ep_model::ZoneId;

/// EnergyPlus source file for zone system demand state.
pub const ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE: &str = "src/EnergyPlus/DataZoneEnergyDemands.hh";
/// EnergyPlus source struct for zone system demand state.
pub const ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT: &str = "ZoneSysEnergyDemand";
/// EnergyPlus heating sensible demand field.
pub const ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD: &str = "RemainingOutputReqToHeatSP";
/// EnergyPlus cooling sensible demand field.
pub const ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD: &str = "RemainingOutputReqToCoolSP";
/// Rust/EnergyPlus sign convention for the heating sensible demand field.
pub const ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION: &str =
    "positive W requests heating; non-positive means no active heating request";
/// Rust/EnergyPlus sign convention for the cooling sensible demand field.
pub const ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION: &str =
    "negative W requests cooling; non-negative means no active cooling request";
/// Current conformance fixture source for zone demand input values.
pub const ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE: &str = "EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs";
/// Mismatch classification used when the upstream zone demand input diverges.
pub const ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION: &str = "upstream_zone_heat_balance_input";
/// Current IdealLoads fixture lane for zone demand.
pub const ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE: &str = "source-order-oracle-demand-input";

/// EnergyPlus `ZoneSysEnergyDemand` subset needed by the first IdealLoads path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneSysEnergyDemand {
    /// Controlled zone.
    pub zone: ZoneId,
    /// EnergyPlus `RemainingOutputReqToHeatSP` equivalent in W.
    ///
    /// Positive values request heating. Non-positive values are inactive for
    /// the current sensible-only IdealLoads subset.
    pub remaining_output_req_to_heat_sp_w: f64,
    /// EnergyPlus `RemainingOutputReqToCoolSP` equivalent in W.
    ///
    /// Negative values request cooling. Non-negative values are inactive for
    /// the current sensible-only IdealLoads subset.
    pub remaining_output_req_to_cool_sp_w: f64,
    /// Moisture demand to humidifying setpoint. Inactive in the first subset.
    pub remaining_output_req_to_humid_sp_kg_per_s: f64,
    /// Moisture demand to dehumidifying setpoint. Inactive in the first subset.
    pub remaining_output_req_to_dehumid_sp_kg_per_s: f64,
}

impl ZoneSysEnergyDemand {
    /// Creates a sensible-only zone demand snapshot.
    #[must_use]
    pub const fn sensible_only(
        zone: ZoneId,
        remaining_output_req_to_heat_sp_w: f64,
        remaining_output_req_to_cool_sp_w: f64,
    ) -> Self {
        Self {
            zone,
            remaining_output_req_to_heat_sp_w,
            remaining_output_req_to_cool_sp_w,
            remaining_output_req_to_humid_sp_kg_per_s: 0.0,
            remaining_output_req_to_dehumid_sp_kg_per_s: 0.0,
        }
    }

    /// Returns true when moisture demand branches are inactive.
    #[must_use]
    pub fn has_inactive_moisture_demand(self) -> bool {
        self.remaining_output_req_to_humid_sp_kg_per_s.abs() <= f64::EPSILON
            && self.remaining_output_req_to_dehumid_sp_kg_per_s.abs() <= f64::EPSILON
    }
}
