//! Runtime execution plan and EnergyPlus source-order stage metadata.

use crate::RuntimeOutputRegistry;
use ep_model::{
    IdealLoadsAirSystemId, OutputHandle, ScheduleId, SimulationModel, ZoneEquipmentListId, ZoneId,
    ZoneThermostatId,
};

/// Runtime execution-plan stage kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionStageKind {
    /// Runtime environment setup and weather/schedule evaluation.
    Environment,
    /// Zone-level heat-balance and thermostat stage.
    Zone,
    /// Zone equipment dispatch stage.
    ZoneEquipment,
    /// Runtime output export stage.
    Output,
    /// EnergyPlus `HeatBalanceManager::GetHeatBalanceInput`.
    GetHeatBalanceInput,
    /// EMS begin-zone-timestep callback before heat-balance initialization.
    EmsBeginZoneTimestepBeforeInitHeatBalance,
    /// EnergyPlus `HeatBalanceManager::InitHeatBalance`.
    InitHeatBalance,
    /// EMS begin-zone-timestep callback after heat-balance initialization.
    EmsBeginZoneTimestepAfterInitHeatBalance,
    /// EnergyPlus `HeatBalanceSurfaceManager::ManageSurfaceHeatBalance`.
    ManageSurfaceHeatBalance,
    /// EnergyPlus `HeatBalanceSurfaceManager::InitSurfaceHeatBalance`.
    InitSurfaceHeatBalance,
    /// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceOutsideSurf`.
    CalcHeatBalanceOutsideSurf,
    /// EnergyPlus `HeatBalanceSurfaceManager::CalcHeatBalanceInsideSurf`.
    CalcHeatBalanceInsideSurf,
    /// EnergyPlus `HeatBalanceAirManager::ManageAirHeatBalance`.
    ManageAirHeatBalance,
    /// EnergyPlus `HeatBalanceSurfaceManager::UpdateFinalSurfaceHeatBalance`.
    UpdateFinalSurfaceHeatBalance,
    /// EnergyPlus `HeatBalanceSurfaceManager::UpdateThermalHistories`.
    UpdateThermalHistories,
    /// EnergyPlus `HeatBalanceSurfaceManager::ReportSurfaceHeatBalance`.
    ReportSurfaceHeatBalance,
    /// EMS end-zone-timestep callback before zone reporting.
    EmsEndZoneTimestepBeforeZoneReporting,
    /// EnergyPlus `HeatBalanceManager::RecKeepHeatBalance`.
    RecKeepHeatBalance,
    /// EnergyPlus `HeatBalanceManager::ReportHeatBalance`.
    ReportHeatBalance,
    /// EMS end-zone-timestep callback after zone reporting.
    EmsEndZoneTimestepAfterZoneReporting,
    /// EnergyPlus `HeatBalanceManager::CheckWarmupConvergence`.
    CheckWarmupConvergence,
    /// EnergyPlus `ZoneEquipmentManager::ManageZoneEquipment`.
    ZoneEquipmentManager,
    /// EnergyPlus `PurchasedAirManager::InitPurchasedAir`.
    PurchasedAirManagerInit,
    /// EnergyPlus `PurchasedAirManager::CalcPurchAirLoads`.
    PurchasedAirManagerCalc,
    /// EnergyPlus `PurchasedAirManager::UpdatePurchasedAir`.
    PurchasedAirManagerUpdate,
    /// EnergyPlus `PurchasedAirManager::ReportPurchasedAir`.
    PurchasedAirManagerReport,
}

impl ExecutionStageKind {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Environment => "environment",
            Self::Zone => "zone",
            Self::ZoneEquipment => "zone_equipment",
            Self::Output => "output",
            Self::GetHeatBalanceInput => "get_heat_balance_input",
            Self::EmsBeginZoneTimestepBeforeInitHeatBalance => {
                "ems_begin_zone_timestep_before_init_heat_balance"
            }
            Self::InitHeatBalance => "init_heat_balance",
            Self::EmsBeginZoneTimestepAfterInitHeatBalance => {
                "ems_begin_zone_timestep_after_init_heat_balance"
            }
            Self::ManageSurfaceHeatBalance => "manage_surface_heat_balance",
            Self::InitSurfaceHeatBalance => "init_surface_heat_balance",
            Self::CalcHeatBalanceOutsideSurf => "calc_heat_balance_outside_surf",
            Self::CalcHeatBalanceInsideSurf => "calc_heat_balance_inside_surf",
            Self::ManageAirHeatBalance => "manage_air_heat_balance",
            Self::UpdateFinalSurfaceHeatBalance => "update_final_surface_heat_balance",
            Self::UpdateThermalHistories => "update_thermal_histories",
            Self::ReportSurfaceHeatBalance => "report_surface_heat_balance",
            Self::EmsEndZoneTimestepBeforeZoneReporting => {
                "ems_end_zone_timestep_before_zone_reporting"
            }
            Self::RecKeepHeatBalance => "rec_keep_heat_balance",
            Self::ReportHeatBalance => "report_heat_balance",
            Self::EmsEndZoneTimestepAfterZoneReporting => {
                "ems_end_zone_timestep_after_zone_reporting"
            }
            Self::CheckWarmupConvergence => "check_warmup_convergence",
            Self::ZoneEquipmentManager => "zone_equipment_manager",
            Self::PurchasedAirManagerInit => "purchased_air_manager_init",
            Self::PurchasedAirManagerCalc => "purchased_air_manager_calc",
            Self::PurchasedAirManagerUpdate => "purchased_air_manager_update",
            Self::PurchasedAirManagerReport => "purchased_air_manager_report",
        }
    }

    /// Returns true for EnergyPlus source routine ordering barriers.
    #[must_use]
    pub const fn is_source_order_barrier(self) -> bool {
        !matches!(
            self,
            Self::Environment | Self::Zone | Self::ZoneEquipment | Self::Output
        )
    }
}

/// Minimal execution step set for v0.1 architecture boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionStep {
    /// Update weather-derived state.
    UpdateWeather,
    /// Evaluate one schedule.
    EvaluateSchedule(ScheduleId),
    /// Evaluate one zone thermostat control.
    EvaluateZoneThermostat(ZoneThermostatId),
    /// Solve one zone.
    SolveZone(ZoneId),
    /// Enter EnergyPlus `ZoneEquipmentManager::ManageZoneEquipment` for one zone.
    ManageZoneEquipment(ZoneId),
    /// Dispatch one `ZoneHVAC:EquipmentList` through `SimZoneEquipment`.
    SimZoneEquipment(ZoneEquipmentListId),
    /// Initialize one `ZoneHVAC:IdealLoadsAirSystem` through `InitPurchasedAir`.
    InitIdealLoadsAirSystem(IdealLoadsAirSystemId),
    /// Evaluate one IdealLoads air system assigned to a zone.
    EvaluateIdealLoadsAirSystem(IdealLoadsAirSystemId),
    /// Update one `ZoneHVAC:IdealLoadsAirSystem` through `UpdatePurchasedAir`.
    UpdateIdealLoadsAirSystem(IdealLoadsAirSystemId),
    /// Report one `ZoneHVAC:IdealLoadsAirSystem` through `ReportPurchasedAir`.
    ReportIdealLoadsAirSystem(IdealLoadsAirSystemId),
    /// Write one output handle.
    WriteOutput(OutputHandle),
}

/// Named runtime execution stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionStage {
    /// Stable stage kind.
    pub kind: ExecutionStageKind,
    /// Stage name.
    pub name: String,
    /// Ordered execution steps in this stage.
    pub steps: Vec<ExecutionStep>,
}

impl ExecutionStage {
    fn from_compatibility_stage(stage: EnergyPlusCompatibilityStage) -> Self {
        Self {
            kind: stage.kind,
            name: stage.stage_name.to_string(),
            steps: Vec::new(),
        }
    }
}

/// EnergyPlus source routine that owns one compatibility-mode ordering barrier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnergyPlusCompatibilityStage {
    /// Stable source-order stage kind.
    pub kind: ExecutionStageKind,
    /// Stable stage name used in Rust reports and traces.
    pub stage_name: &'static str,
    /// EnergyPlus source file that owns the stage.
    pub source_file: &'static str,
    /// EnergyPlus routine or callback barrier name.
    pub source_routine: &'static str,
}

/// Minimal deterministic execution plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlan {
    /// Ordered stages.
    pub stages: Vec<ExecutionStage>,
    /// EnergyPlus heat-balance routine order that compatibility mode must preserve.
    pub compatibility_stages: Vec<EnergyPlusCompatibilityStage>,
}

impl ExecutionPlan {
    /// Returns the total step count across all stages.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.stages.iter().map(|stage| stage.steps.len()).sum()
    }
}

/// EnergyPlus heat-balance source order used as the compatibility-mode contract.
#[must_use]
pub fn energyplus_heat_balance_compatibility_stages() -> Vec<EnergyPlusCompatibilityStage> {
    vec![
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::GetHeatBalanceInput,
            stage_name: "get-heat-balance-input",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "GetHeatBalanceInput",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::EmsBeginZoneTimestepBeforeInitHeatBalance,
            stage_name: "ems-begin-zone-timestep-before-init-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "EMS BeginZoneTimestepBeforeInitHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::InitHeatBalance,
            stage_name: "init-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "InitHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::EmsBeginZoneTimestepAfterInitHeatBalance,
            stage_name: "ems-begin-zone-timestep-after-init-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "EMS BeginZoneTimestepAfterInitHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::ManageSurfaceHeatBalance,
            stage_name: "manage-surface-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "ManageSurfaceHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::InitSurfaceHeatBalance,
            stage_name: "init-surface-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "InitSurfaceHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::CalcHeatBalanceOutsideSurf,
            stage_name: "calc-heat-balance-outside-surf",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "CalcHeatBalanceOutsideSurf",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::CalcHeatBalanceInsideSurf,
            stage_name: "calc-heat-balance-inside-surf",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "CalcHeatBalanceInsideSurf",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::ManageAirHeatBalance,
            stage_name: "manage-air-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceAirManager.cc",
            source_routine: "ManageAirHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::UpdateFinalSurfaceHeatBalance,
            stage_name: "update-final-surface-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "UpdateFinalSurfaceHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::UpdateThermalHistories,
            stage_name: "update-thermal-histories",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "UpdateThermalHistories",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::ReportSurfaceHeatBalance,
            stage_name: "report-surface-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceSurfaceManager.cc",
            source_routine: "ReportSurfaceHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::EmsEndZoneTimestepBeforeZoneReporting,
            stage_name: "ems-end-zone-timestep-before-zone-reporting",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "EMS EndZoneTimestepBeforeZoneReporting",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::RecKeepHeatBalance,
            stage_name: "rec-keep-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "RecKeepHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::ReportHeatBalance,
            stage_name: "report-heat-balance",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "ReportHeatBalance",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::EmsEndZoneTimestepAfterZoneReporting,
            stage_name: "ems-end-zone-timestep-after-zone-reporting",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "EMS EndZoneTimestepAfterZoneReporting",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::CheckWarmupConvergence,
            stage_name: "check-warmup-convergence",
            source_file: "src/EnergyPlus/HeatBalanceManager.cc",
            source_routine: "CheckWarmupConvergence",
        },
    ]
}

/// EnergyPlus IdealLoads source order used when an IdealLoads system is active.
#[must_use]
pub fn energyplus_ideal_loads_compatibility_stages() -> Vec<EnergyPlusCompatibilityStage> {
    vec![
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::ZoneEquipmentManager,
            stage_name: "zone-equipment-manager",
            source_file: "src/EnergyPlus/ZoneEquipmentManager.cc",
            source_routine: "ManageZoneEquipment",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerInit,
            stage_name: "purchased-air-manager-init",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "InitPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerCalc,
            stage_name: "purchased-air-manager-calc",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "CalcPurchAirLoads",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerUpdate,
            stage_name: "purchased-air-manager-update",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "UpdatePurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerReport,
            stage_name: "purchased-air-manager-report",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "ReportPurchasedAir",
        },
    ]
}

/// Builds the first deterministic execution plan for the typed subset.
#[must_use]
pub fn build_execution_plan(model: &SimulationModel) -> ExecutionPlan {
    let mut setup_steps = vec![ExecutionStep::UpdateWeather];
    setup_steps.extend(schedule_ids(model).map(ExecutionStep::EvaluateSchedule));

    let mut zone_steps = Vec::new();
    let mut zone_equipment_manager_steps = Vec::new();
    let mut purchased_air_init_steps = Vec::new();
    let mut purchased_air_calc_steps = Vec::new();
    let mut purchased_air_update_steps = Vec::new();
    let mut purchased_air_report_steps = Vec::new();
    for zone in &model.typed.zones {
        zone_steps.extend(
            model
                .graph
                .zone_thermostats
                .iter()
                .filter(|edge| edge.zone == zone.id)
                .map(|edge| ExecutionStep::EvaluateZoneThermostat(edge.thermostat)),
        );
        zone_steps.push(ExecutionStep::SolveZone(zone.id));
        let zone_ideal_loads = model
            .graph
            .zone_ideal_loads
            .iter()
            .filter(|edge| edge.zone == zone.id)
            .collect::<Vec<_>>();
        if !zone_ideal_loads.is_empty() {
            zone_equipment_manager_steps.push(ExecutionStep::ManageZoneEquipment(zone.id));
        }
        for edge in zone_ideal_loads {
            zone_equipment_manager_steps.push(ExecutionStep::SimZoneEquipment(edge.equipment_list));
            purchased_air_init_steps.push(ExecutionStep::InitIdealLoadsAirSystem(
                edge.ideal_loads_air_system,
            ));
            purchased_air_calc_steps.push(ExecutionStep::EvaluateIdealLoadsAirSystem(
                edge.ideal_loads_air_system,
            ));
            purchased_air_update_steps.push(ExecutionStep::UpdateIdealLoadsAirSystem(
                edge.ideal_loads_air_system,
            ));
            purchased_air_report_steps.push(ExecutionStep::ReportIdealLoadsAirSystem(
                edge.ideal_loads_air_system,
            ));
        }
    }

    let compatibility_stages = energyplus_heat_balance_compatibility_stages();
    let mut stages = compatibility_stages
        .iter()
        .copied()
        .map(ExecutionStage::from_compatibility_stage)
        .collect::<Vec<_>>();
    push_steps_to_stage(
        &mut stages,
        ExecutionStageKind::InitHeatBalance,
        setup_steps,
    );
    push_steps_to_stage(
        &mut stages,
        ExecutionStageKind::ManageAirHeatBalance,
        zone_steps,
    );
    push_steps_to_stage(
        &mut stages,
        ExecutionStageKind::ReportHeatBalance,
        RuntimeOutputRegistry::from_model(model)
            .outputs()
            .iter()
            .map(|output| ExecutionStep::WriteOutput(output.handle))
            .collect(),
    );

    if !zone_equipment_manager_steps.is_empty() {
        stages.extend(
            energyplus_ideal_loads_compatibility_stages()
                .iter()
                .copied()
                .map(ExecutionStage::from_compatibility_stage),
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::ZoneEquipmentManager,
            zone_equipment_manager_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::PurchasedAirManagerInit,
            purchased_air_init_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::PurchasedAirManagerCalc,
            purchased_air_calc_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::PurchasedAirManagerUpdate,
            purchased_air_update_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::PurchasedAirManagerReport,
            purchased_air_report_steps,
        );
    }

    ExecutionPlan {
        stages,
        compatibility_stages,
    }
}

fn push_steps_to_stage(
    stages: &mut [ExecutionStage],
    kind: ExecutionStageKind,
    steps: Vec<ExecutionStep>,
) {
    if steps.is_empty() {
        return;
    }

    let stage = stages
        .iter_mut()
        .find(|stage| stage.kind == kind)
        .expect("source-order stage must exist before adding execution steps");
    stage.steps.extend(steps);
}

fn schedule_ids(model: &SimulationModel) -> impl Iterator<Item = ScheduleId> + '_ {
    model
        .typed
        .schedules
        .iter()
        .map(|schedule| schedule.id)
        .chain(
            model
                .typed
                .compact_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
}
