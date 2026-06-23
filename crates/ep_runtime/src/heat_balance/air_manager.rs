//! HeatBalanceAirManager source-order stage contract.

use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};

/// EnergyPlus `HeatBalanceAirManager::ManageAirHeatBalance`.
#[must_use]
pub const fn manage_air_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind: ExecutionStageKind::ManageAirHeatBalance,
        stage_name: "manage-air-heat-balance",
        source_file: "src/EnergyPlus/HeatBalanceAirManager.cc",
        source_routine: "ManageAirHeatBalance",
    }
}
