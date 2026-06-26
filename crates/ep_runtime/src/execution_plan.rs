//! Runtime execution plan and EnergyPlus source-order stage metadata.

use crate::{RuntimeOutputRegistry, manage_heat_balance_source_order_stages};
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
    /// EnergyPlus `ZoneTempPredictorCorrector::ManageZoneAirUpdates`.
    ManageZoneAirUpdates,
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
    /// EnergyPlus `PurchasedAirManager::SimPurchasedAir`.
    PurchasedAirManagerSim,
    /// EnergyPlus `PurchasedAirManager::GetPurchasedAir`.
    PurchasedAirManagerGet,
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
            Self::ManageZoneAirUpdates => "manage_zone_air_updates",
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
            Self::PurchasedAirManagerSim => "purchased_air_manager_sim",
            Self::PurchasedAirManagerGet => "purchased_air_manager_get",
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
    /// Enter `PurchasedAirManager::SimPurchasedAir` for one IdealLoads system.
    SimPurchasedAir(IdealLoadsAirSystemId),
    /// Resolve one `ZoneHVAC:IdealLoadsAirSystem` through `GetPurchasedAir`.
    GetIdealLoadsAirSystem(IdealLoadsAirSystemId),
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

    /// Returns the expected EnergyPlus source-order stage identifiers.
    #[must_use]
    pub fn expected_source_order_stage_ids(&self) -> Vec<&'static str> {
        self.compatibility_stages
            .iter()
            .map(|stage| stage.stage_name)
            .collect()
    }

    /// Returns the source-order stage identifiers represented by the execution plan.
    #[must_use]
    pub fn actual_source_order_stage_ids(&self) -> Vec<&str> {
        self.stages
            .iter()
            .filter(|stage| stage.kind.is_source_order_barrier())
            .map(|stage| stage.name.as_str())
            .collect()
    }

    /// Returns whether expected source-order stages match the executable plan.
    #[must_use]
    pub fn source_order_stages_match(&self) -> bool {
        self.expected_source_order_stage_ids()
            .iter()
            .copied()
            .eq(self.actual_source_order_stage_ids())
    }
}

/// EnergyPlus heat-balance source order used as the compatibility-mode contract.
#[must_use]
pub fn energyplus_heat_balance_compatibility_stages() -> Vec<EnergyPlusCompatibilityStage> {
    manage_heat_balance_source_order_stages()
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
            kind: ExecutionStageKind::PurchasedAirManagerSim,
            stage_name: "sim-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "SimPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerGet,
            stage_name: "get-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "GetPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerInit,
            stage_name: "init-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "InitPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerCalc,
            stage_name: "calc-purch-air-loads",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "CalcPurchAirLoads",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerUpdate,
            stage_name: "update-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "UpdatePurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::PurchasedAirManagerReport,
            stage_name: "report-purchased-air",
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
    let mut purchased_air_sim_steps = Vec::new();
    let mut purchased_air_get_steps = Vec::new();
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
            purchased_air_sim_steps
                .push(ExecutionStep::SimPurchasedAir(edge.ideal_loads_air_system));
            purchased_air_get_steps.push(ExecutionStep::GetIdealLoadsAirSystem(
                edge.ideal_loads_air_system,
            ));
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

    let mut compatibility_stages = energyplus_heat_balance_compatibility_stages();
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
        ExecutionStageKind::ManageZoneAirUpdates,
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
        let ideal_loads_stages = energyplus_ideal_loads_compatibility_stages();
        stages.extend(
            ideal_loads_stages
                .iter()
                .copied()
                .map(ExecutionStage::from_compatibility_stage),
        );
        compatibility_stages.extend(ideal_loads_stages);
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::ZoneEquipmentManager,
            zone_equipment_manager_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::PurchasedAirManagerSim,
            purchased_air_sim_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::PurchasedAirManagerGet,
            purchased_air_get_steps,
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

    let Some(stage) = stages.iter_mut().find(|stage| stage.kind == kind) else {
        return;
    };
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
