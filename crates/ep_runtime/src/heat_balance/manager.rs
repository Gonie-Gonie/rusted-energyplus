//! HeatBalanceManager source-order stage contract.

use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};

use super::{air_manager, surface_manager, zone_predictor_corrector};

const SOURCE_FILE: &str = "src/EnergyPlus/HeatBalanceManager.cc";

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

/// EnergyPlus `HeatBalanceManager::GetHeatBalanceInput`.
#[must_use]
pub const fn get_heat_balance_input_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::GetHeatBalanceInput,
        "get-heat-balance-input",
        "GetHeatBalanceInput",
    )
}

/// EMS begin-zone-timestep callback before `InitHeatBalance`.
#[must_use]
pub const fn ems_begin_before_init_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::EmsBeginZoneTimestepBeforeInitHeatBalance,
        "ems-begin-zone-timestep-before-init-heat-balance",
        "EMS BeginZoneTimestepBeforeInitHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceManager::ManageHeatBalance` source-order wrapper.
pub(crate) fn manage_heat_balance_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EnergyPlus `HeatBalanceManager::InitHeatBalance`.
#[must_use]
pub const fn init_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::InitHeatBalance,
        "init-heat-balance",
        "InitHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceManager::InitHeatBalance` source-order wrapper.
pub(crate) fn init_heat_balance_source_order_path<T>(execute: impl FnOnce() -> T) -> T {
    execute()
}

/// EMS begin-zone-timestep callback after `InitHeatBalance`.
#[must_use]
pub const fn ems_begin_after_init_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::EmsBeginZoneTimestepAfterInitHeatBalance,
        "ems-begin-zone-timestep-after-init-heat-balance",
        "EMS BeginZoneTimestepAfterInitHeatBalance",
    )
}

/// EMS end-zone-timestep callback before zone reporting.
#[must_use]
pub const fn ems_end_before_zone_reporting_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::EmsEndZoneTimestepBeforeZoneReporting,
        "ems-end-zone-timestep-before-zone-reporting",
        "EMS EndZoneTimestepBeforeZoneReporting",
    )
}

/// EnergyPlus `HeatBalanceManager::RecKeepHeatBalance`.
#[must_use]
pub const fn rec_keep_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::RecKeepHeatBalance,
        "rec-keep-heat-balance",
        "RecKeepHeatBalance",
    )
}

/// EnergyPlus `HeatBalanceManager::ReportHeatBalance`.
#[must_use]
pub const fn report_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::ReportHeatBalance,
        "report-heat-balance",
        "ReportHeatBalance",
    )
}

/// EMS end-zone-timestep callback after zone reporting.
#[must_use]
pub const fn ems_end_after_zone_reporting_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::EmsEndZoneTimestepAfterZoneReporting,
        "ems-end-zone-timestep-after-zone-reporting",
        "EMS EndZoneTimestepAfterZoneReporting",
    )
}

/// EnergyPlus `HeatBalanceManager::CheckWarmupConvergence`.
#[must_use]
pub const fn check_warmup_convergence_stage() -> EnergyPlusCompatibilityStage {
    stage(
        ExecutionStageKind::CheckWarmupConvergence,
        "check-warmup-convergence",
        "CheckWarmupConvergence",
    )
}

/// Full heat-balance source-order barrier list for the current compatibility lane.
#[must_use]
pub fn manage_heat_balance_source_order_stages() -> Vec<EnergyPlusCompatibilityStage> {
    vec![
        get_heat_balance_input_stage(),
        ems_begin_before_init_heat_balance_stage(),
        init_heat_balance_stage(),
        ems_begin_after_init_heat_balance_stage(),
        surface_manager::manage_surface_heat_balance_stage(),
        surface_manager::init_surface_heat_balance_stage(),
        surface_manager::calc_heat_balance_outside_surf_stage(),
        surface_manager::calc_heat_balance_inside_surf_stage(),
        air_manager::manage_air_heat_balance_stage(),
        zone_predictor_corrector::manage_zone_air_updates_stage(),
        surface_manager::update_final_surface_heat_balance_stage(),
        surface_manager::update_thermal_histories_stage(),
        surface_manager::report_surface_heat_balance_stage(),
        ems_end_before_zone_reporting_stage(),
        rec_keep_heat_balance_stage(),
        report_heat_balance_stage(),
        ems_end_after_zone_reporting_stage(),
        check_warmup_convergence_stage(),
    ]
}
