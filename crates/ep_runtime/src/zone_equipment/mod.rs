//! Zone equipment demand state used by compatibility-mode HVAC components.

use ep_model::ZoneId;

/// EnergyPlus `ZoneSysEnergyDemand` subset needed by the first IdealLoads path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneSysEnergyDemand {
    /// Controlled zone.
    pub zone: ZoneId,
    /// EnergyPlus `RemainingOutputReqToHeatSP` equivalent in W.
    pub remaining_output_req_to_heat_sp_w: f64,
    /// EnergyPlus `RemainingOutputReqToCoolSP` equivalent in W.
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

/// Source-order entry point reserved for zone equipment orchestration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZoneEquipmentCompatibilityStage {
    /// Stable Rust stage name.
    pub stage_name: &'static str,
    /// EnergyPlus source file.
    pub source_file: &'static str,
    /// EnergyPlus source routine.
    pub source_routine: &'static str,
}

/// EnergyPlus zone equipment order relevant to PurchasedAir/IdealLoads.
#[must_use]
pub const fn ideal_loads_zone_equipment_stages() -> [ZoneEquipmentCompatibilityStage; 3] {
    [
        ZoneEquipmentCompatibilityStage {
            stage_name: "manage-zone-equipment",
            source_file: "src/EnergyPlus/ZoneEquipmentManager.cc",
            source_routine: "ManageZoneEquipment",
        },
        ZoneEquipmentCompatibilityStage {
            stage_name: "simulate-zone-equipment",
            source_file: "src/EnergyPlus/ZoneEquipmentManager.cc",
            source_routine: "SimZoneEquipment",
        },
        ZoneEquipmentCompatibilityStage {
            stage_name: "simulate-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "SimPurchasedAir",
        },
    ]
}
