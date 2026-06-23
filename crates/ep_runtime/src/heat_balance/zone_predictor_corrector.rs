//! ZoneTempPredictorCorrector source-order stage contract.

use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};

/// EnergyPlus `ZoneTempPredictorCorrector::ManageZoneAirUpdates`.
#[must_use]
pub const fn manage_zone_air_updates_stage() -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind: ExecutionStageKind::ManageZoneAirUpdates,
        stage_name: "manage-zone-air-updates",
        source_file: "src/EnergyPlus/ZoneTempPredictorCorrector.cc",
        source_routine: "ManageZoneAirUpdates",
    }
}

/// Source-order ownership note for current zone-air history state.
pub const ZONE_AIR_HISTORY_OWNER: &str =
    "MAT history and zone-air output timing are owned by ManageZoneAirUpdates.";
