//! HeatBalanceSurfaceManager source-order stage contract.

use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};

const SOURCE_FILE: &str = "src/EnergyPlus/HeatBalanceSurfaceManager.cc";

const fn stage(
    kind: ExecutionStageKind,
    stage_name: &'static str,
    source_routine: &'static str,
) -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind,
        stage_name,
        source_file: SOURCE_FILE,
        source_routine,
    }
}

/// EnergyPlus `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance`.
#[must_use]
pub const fn manage_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::ManageSurfaceHeatBalance,
        "manage-surface-heat-balance",
        "ManageSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::InitSurfaceHeatBalance`.
#[must_use]
pub const fn init_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::InitSurfaceHeatBalance,
        "init-surface-heat-balance",
        "InitSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceOutsideSurf`.
#[must_use]
pub const fn calc_heat_balance_outside_surf_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::CalcHeatBalanceOutsideSurf,
        "calc-heat-balance-outside-surf",
        "CalcHeatBalanceOutsideSurf",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceInsideSurf`.
#[must_use]
pub const fn calc_heat_balance_inside_surf_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::CalcHeatBalanceInsideSurf,
        "calc-heat-balance-inside-surf",
        "CalcHeatBalanceInsideSurf",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::UpdateFinalSurfaceHeatBalance`.
#[must_use]
pub const fn update_final_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::UpdateFinalSurfaceHeatBalance,
        "update-final-surface-heat-balance",
        "UpdateFinalSurfaceHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::UpdateThermalHistories`.
#[must_use]
pub const fn update_thermal_histories_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::UpdateThermalHistories,
        "update-thermal-histories",
        "UpdateThermalHistories",
    )
}

/// EnergyPlus `HeatBalanceSurfaceManager::ReportSurfaceHeatBalance`.
#[must_use]
pub const fn report_surface_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::ReportSurfaceHeatBalance,
        "report-surface-heat-balance",
        "ReportSurfaceHeatBalance",
    )
}
