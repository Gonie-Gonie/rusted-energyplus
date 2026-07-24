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
pub const ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION: &str = "negative W requests cooling; positive means no active cooling request; exact zero priority depends on the demand input kind";
/// Current conformance fixture source for zone demand input values.
pub const ZONE_SYS_ENERGY_DEMAND_INPUT_SOURCE: &str = "EnergyPlus Zone System Predicted Sensible Load to Setpoint output split into active heat/cool ZoneSysEnergyDemand inputs";
/// Mismatch classification used when the upstream zone demand input diverges.
pub const ZONE_SYS_ENERGY_DEMAND_MISMATCH_CLASSIFICATION: &str = "upstream_zone_heat_balance_input";
/// Current IdealLoads fixture lane for zone demand.
pub const ZONE_SYS_ENERGY_DEMAND_FIXTURE_MODE: &str = "source-order-oracle-demand-input";

/// Interpretation of the two sensible setpoint fields at the compatibility boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZoneSensibleDemandInputKind {
    /// Oracle/default fixtures use zero as the inactive side of an active split.
    ActiveLoadSplitCompatibility,
    /// Source `OutputRequiredTo*Setpoint` values retain both thresholds and zero priority.
    SourceSetpointThresholds,
}

/// EnergyPlus `ZoneSysEnergyDemand` subset needed by the first IdealLoads path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneSysEnergyDemand {
    /// Controlled zone.
    pub zone: ZoneId,
    /// Whether zero is an inactive split sentinel or a source threshold.
    pub sensible_input_kind: ZoneSensibleDemandInputKind,
    /// EnergyPlus `RemainingOutputReqToHeatSP` equivalent in W.
    ///
    /// Positive values request heating. Non-positive values are inactive for
    /// the current sensible-only IdealLoads subset.
    pub remaining_output_req_to_heat_sp_w: f64,
    /// EnergyPlus `RemainingOutputReqToCoolSP` equivalent in W.
    ///
    /// Negative values request cooling and positive values are inactive.
    /// Exact zero follows `sensible_input_kind` at the no-OA consumer.
    pub remaining_output_req_to_cool_sp_w: f64,
    /// Moisture demand to humidifying setpoint. Inactive in the first subset.
    pub remaining_output_req_to_humid_sp_kg_per_s: f64,
    /// Moisture demand to dehumidifying setpoint. Inactive in the first subset.
    pub remaining_output_req_to_dehumid_sp_kg_per_s: f64,
}

impl ZoneSysEnergyDemand {
    /// Creates a sensible-only active-split compatibility snapshot.
    ///
    /// This retains the oracle/default fixture convention where exact zero is
    /// the inactive side. Use `from_output_required_setpoint_loads` when both
    /// source setpoint thresholds must be preserved.
    #[must_use]
    pub const fn sensible_only(
        zone: ZoneId,
        remaining_output_req_to_heat_sp_w: f64,
        remaining_output_req_to_cool_sp_w: f64,
    ) -> Self {
        Self {
            zone,
            sensible_input_kind: ZoneSensibleDemandInputKind::ActiveLoadSplitCompatibility,
            remaining_output_req_to_heat_sp_w,
            remaining_output_req_to_cool_sp_w,
            remaining_output_req_to_humid_sp_kg_per_s: 0.0,
            remaining_output_req_to_dehumid_sp_kg_per_s: 0.0,
        }
    }

    /// Creates Remaining H/C from finalized sensible setpoint loads.
    ///
    /// This scalar constructor does not validate predictor provenance, scaling,
    /// controlled-Zone identity, or equipment distribution. The caller owns
    /// those source-order constraints; Total, unadjusted, sequenced, residual,
    /// and persistent demand state remain outside this DTO subset.
    #[must_use]
    pub const fn from_output_required_setpoint_loads(
        zone: ZoneId,
        output_required_to_heating_setpoint_w: f64,
        output_required_to_cooling_setpoint_w: f64,
    ) -> Self {
        Self {
            zone,
            sensible_input_kind: ZoneSensibleDemandInputKind::SourceSetpointThresholds,
            remaining_output_req_to_heat_sp_w: output_required_to_heating_setpoint_w,
            remaining_output_req_to_cool_sp_w: output_required_to_cooling_setpoint_w,
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
