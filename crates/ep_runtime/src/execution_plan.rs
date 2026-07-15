//! Runtime execution plan and EnergyPlus source-order stage metadata.

use crate::{RuntimeOutputRegistry, manage_heat_balance_source_order_stages};
use ep_model::{
    ConstructionId, IdealLoadsAirSystemId, OutputHandle, ScheduleId, SimulationModel, SurfaceId,
    ZoneEquipmentListId, ZoneId, ZoneThermostatId,
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
    SimPurchasedAir,
    /// EnergyPlus `PurchasedAirManager::GetPurchasedAir`.
    GetPurchasedAir,
    /// EnergyPlus `PurchasedAirManager::InitPurchasedAir`.
    InitPurchasedAir,
    /// EnergyPlus `PurchasedAirManager::CalcPurchAirLoads`.
    CalcPurchAirLoads,
    /// EnergyPlus `PurchasedAirManager::UpdatePurchasedAir`.
    UpdatePurchasedAir,
    /// EnergyPlus `PurchasedAirManager::ReportPurchasedAir`.
    ReportPurchasedAir,
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
            Self::SimPurchasedAir => "sim_purchased_air",
            Self::GetPurchasedAir => "get_purchased_air",
            Self::InitPurchasedAir => "init_purchased_air",
            Self::CalcPurchAirLoads => "calc_purch_air_loads",
            Self::UpdatePurchasedAir => "update_purchased_air",
            Self::ReportPurchasedAir => "report_purchased_air",
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

/// Per-stage state dependency contract compiled from the typed model.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExecutionStageDependency {
    /// State domains read by this stage.
    pub reads: Vec<&'static str>,
    /// State domains written by this stage.
    pub writes: Vec<&'static str>,
}

impl ExecutionStageDependency {
    fn new(reads: &[&'static str], writes: &[&'static str]) -> Self {
        Self {
            reads: reads.to_vec(),
            writes: writes.to_vec(),
        }
    }
}

/// Typed IDs prebound to a stage before runtime execution begins.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExecutionStagePreboundSet {
    /// ResultStore output handles written by this stage.
    pub output_handles: Vec<OutputHandle>,
    /// Surface loop targets resolved from the typed model.
    pub surface_ids: Vec<SurfaceId>,
    /// Zone loop targets resolved from the typed model.
    pub zone_ids: Vec<ZoneId>,
    /// Construction coefficient references resolved from the typed model.
    pub construction_ids: Vec<ConstructionId>,
    /// Schedule IDs resolved from the typed model.
    pub schedule_ids: Vec<ScheduleId>,
    /// Weather series indices consumed by this stage.
    pub weather_series_indices: Vec<usize>,
}

/// Runtime lookup policy attached to a compiled execution plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionPlanRuntimePolicy {
    /// Object lookup policy after RawModel/TypedModel compilation.
    pub post_typed_model_object_lookup: &'static str,
    /// String comparison policy for runtime stage execution.
    pub stage_execution_string_comparison: &'static str,
    /// HashMap lookup policy for runtime stage execution.
    pub stage_execution_hash_map_lookup: &'static str,
    /// Ordering policy for compatibility mode.
    pub compatibility_plan_order: &'static str,
    /// Stage grouping policy across compatibility and fast modes.
    pub fast_mode_grouping_policy: &'static str,
}

impl ExecutionPlanRuntimePolicy {
    /// Compatibility-mode runtime policy for precompiled plans.
    #[must_use]
    pub const fn compatibility_precompiled() -> Self {
        Self {
            post_typed_model_object_lookup: "forbidden-after-rawmodel-typedmodel-runtime-uses-prebound-typed-ids",
            stage_execution_string_comparison: "forbidden-in-source-order-stage-execution",
            stage_execution_hash_map_lookup: "compile-and-report-only-hot-stages-use-vecs-and-typed-ids",
            compatibility_plan_order: "deterministic-energyplus-source-order-then-typed-model-order",
            fast_mode_grouping_policy: "compatibility-mode-forbids-stage-reordering-fast-mode-only-safe-grouping",
        }
    }
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
    /// State dependency contract for this stage.
    pub dependencies: ExecutionStageDependency,
    /// Typed IDs and handles resolved before runtime execution.
    pub prebound: ExecutionStagePreboundSet,
}

impl ExecutionStage {
    /// Creates a stage with dependency metadata and no prebound IDs.
    #[must_use]
    pub fn new(
        kind: ExecutionStageKind,
        name: impl Into<String>,
        steps: Vec<ExecutionStep>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            steps,
            dependencies: stage_dependencies(kind),
            prebound: ExecutionStagePreboundSet::default(),
        }
    }

    fn from_compatibility_stage(stage: EnergyPlusCompatibilityStage) -> Self {
        Self::new(stage.kind, stage.stage_name, Vec::new())
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
    /// Runtime lookup and ordering policy for this plan.
    pub runtime_policy: ExecutionPlanRuntimePolicy,
}

impl ExecutionPlan {
    /// Creates a deterministic execution plan with compatibility runtime policy.
    #[must_use]
    pub fn new(
        stages: Vec<ExecutionStage>,
        compatibility_stages: Vec<EnergyPlusCompatibilityStage>,
    ) -> Self {
        Self {
            stages,
            compatibility_stages,
            runtime_policy: ExecutionPlanRuntimePolicy::compatibility_precompiled(),
        }
    }

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
            kind: ExecutionStageKind::SimPurchasedAir,
            stage_name: "sim-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "SimPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::GetPurchasedAir,
            stage_name: "get-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "GetPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::InitPurchasedAir,
            stage_name: "init-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "InitPurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::CalcPurchAirLoads,
            stage_name: "calc-purch-air-loads",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "CalcPurchAirLoads",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::UpdatePurchasedAir,
            stage_name: "update-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "UpdatePurchasedAir",
        },
        EnergyPlusCompatibilityStage {
            kind: ExecutionStageKind::ReportPurchasedAir,
            stage_name: "report-purchased-air",
            source_file: "src/EnergyPlus/PurchasedAirManager.cc",
            source_routine: "ReportPurchasedAir",
        },
    ]
}

/// Builds the first deterministic execution plan for the typed subset.
#[must_use]
pub fn build_execution_plan(model: &SimulationModel) -> ExecutionPlan {
    let output_registry = RuntimeOutputRegistry::from_model(model);
    build_execution_plan_with_output_registry(model, &output_registry)
}

/// Builds the execution plan from a precomputed output registry.
#[must_use]
pub fn build_execution_plan_with_output_registry(
    model: &SimulationModel,
    output_registry: &RuntimeOutputRegistry,
) -> ExecutionPlan {
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
        output_registry
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
            ExecutionStageKind::SimPurchasedAir,
            purchased_air_sim_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::GetPurchasedAir,
            purchased_air_get_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::InitPurchasedAir,
            purchased_air_init_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::CalcPurchAirLoads,
            purchased_air_calc_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::UpdatePurchasedAir,
            purchased_air_update_steps,
        );
        push_steps_to_stage(
            &mut stages,
            ExecutionStageKind::ReportPurchasedAir,
            purchased_air_report_steps,
        );
    }

    compile_stage_contracts(&mut stages, model, output_registry);

    ExecutionPlan::new(stages, compatibility_stages)
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
        .file_shading_schedule
        .iter()
        .flat_map(|schedule| schedule.columns.iter())
        .map(|column| column.id)
        .chain(model.typed.schedules.iter().map(|schedule| schedule.id))
        .chain(
            model
                .typed
                .compact_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
        .chain(
            model
                .typed
                .file_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
        .chain(
            model
                .typed
                .year_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
        .chain(
            model
                .typed
                .external_interface_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
        .chain(
            model
                .typed
                .external_interface_fmu_import_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
        .chain(
            model
                .typed
                .external_interface_fmu_export_schedules
                .iter()
                .map(|schedule| schedule.id),
        )
}

fn compile_stage_contracts(
    stages: &mut [ExecutionStage],
    model: &SimulationModel,
    output_registry: &RuntimeOutputRegistry,
) {
    let output_handles = output_registry
        .outputs()
        .iter()
        .map(|output| output.handle)
        .collect::<Vec<_>>();
    let surface_ids = model
        .typed
        .surfaces
        .iter()
        .map(|surface| surface.id)
        .collect::<Vec<_>>();
    let zone_ids = model
        .typed
        .zones
        .iter()
        .map(|zone| zone.id)
        .collect::<Vec<_>>();
    let construction_ids = model
        .typed
        .constructions
        .iter()
        .map(|construction| construction.id)
        .collect::<Vec<_>>();
    let schedule_ids = schedule_ids(model).collect::<Vec<_>>();
    let weather_series_indices = vec![0_usize];

    for stage in stages {
        stage.dependencies = stage_dependencies(stage.kind);
        stage.prebound = stage_prebound_set(
            stage.kind,
            &output_handles,
            &surface_ids,
            &zone_ids,
            &construction_ids,
            &schedule_ids,
            &weather_series_indices,
        );
    }
}

fn stage_prebound_set(
    kind: ExecutionStageKind,
    output_handles: &[OutputHandle],
    surface_ids: &[SurfaceId],
    zone_ids: &[ZoneId],
    construction_ids: &[ConstructionId],
    schedule_ids: &[ScheduleId],
    weather_series_indices: &[usize],
) -> ExecutionStagePreboundSet {
    let mut prebound = ExecutionStagePreboundSet::default();
    match kind {
        ExecutionStageKind::InitHeatBalance => {
            prebound.schedule_ids.extend_from_slice(schedule_ids);
            prebound
                .weather_series_indices
                .extend_from_slice(weather_series_indices);
        }
        ExecutionStageKind::ManageSurfaceHeatBalance
        | ExecutionStageKind::InitSurfaceHeatBalance
        | ExecutionStageKind::CalcHeatBalanceOutsideSurf
        | ExecutionStageKind::CalcHeatBalanceInsideSurf
        | ExecutionStageKind::UpdateFinalSurfaceHeatBalance
        | ExecutionStageKind::UpdateThermalHistories
        | ExecutionStageKind::ReportSurfaceHeatBalance => {
            prebound.surface_ids.extend_from_slice(surface_ids);
            prebound
                .construction_ids
                .extend_from_slice(construction_ids);
            if matches!(kind, ExecutionStageKind::CalcHeatBalanceOutsideSurf) {
                prebound
                    .weather_series_indices
                    .extend_from_slice(weather_series_indices);
            }
        }
        ExecutionStageKind::ManageAirHeatBalance
        | ExecutionStageKind::ManageZoneAirUpdates
        | ExecutionStageKind::RecKeepHeatBalance
        | ExecutionStageKind::ReportHeatBalance
        | ExecutionStageKind::CheckWarmupConvergence => {
            prebound.zone_ids.extend_from_slice(zone_ids);
            if matches!(kind, ExecutionStageKind::ReportHeatBalance) {
                prebound.output_handles.extend_from_slice(output_handles);
            }
        }
        ExecutionStageKind::ZoneEquipmentManager => {
            prebound.zone_ids.extend_from_slice(zone_ids);
        }
        ExecutionStageKind::ReportPurchasedAir => {
            prebound.output_handles.extend_from_slice(output_handles);
        }
        _ => {}
    }
    prebound
}

fn stage_dependencies(kind: ExecutionStageKind) -> ExecutionStageDependency {
    match kind {
        ExecutionStageKind::GetHeatBalanceInput => {
            ExecutionStageDependency::new(&["typed_model"], &["heat_balance_input"])
        }
        ExecutionStageKind::InitHeatBalance => ExecutionStageDependency::new(
            &["weather_series", "schedule_series", "heat_balance_input"],
            &["environment_state", "schedule_state", "heat_balance_state"],
        ),
        ExecutionStageKind::ManageSurfaceHeatBalance => ExecutionStageDependency::new(
            &["surface_state", "zone_state", "construction_thermal_data"],
            &["surface_heat_balance_state"],
        ),
        ExecutionStageKind::InitSurfaceHeatBalance => ExecutionStageDependency::new(
            &["surface_state", "construction_thermal_data"],
            &["surface_state"],
        ),
        ExecutionStageKind::CalcHeatBalanceOutsideSurf => ExecutionStageDependency::new(
            &[
                "surface_state",
                "weather_series",
                "construction_thermal_data",
            ],
            &["outside_surface_heat_balance"],
        ),
        ExecutionStageKind::CalcHeatBalanceInsideSurf => ExecutionStageDependency::new(
            &["surface_state", "zone_state", "construction_thermal_data"],
            &["inside_surface_heat_balance"],
        ),
        ExecutionStageKind::ManageAirHeatBalance => ExecutionStageDependency::new(
            &["zone_state", "surface_heat_balance_state"],
            &["zone_air_heat_balance_state"],
        ),
        ExecutionStageKind::ManageZoneAirUpdates => ExecutionStageDependency::new(
            &["zone_air_heat_balance_state", "thermostat_state"],
            &["zone_state"],
        ),
        ExecutionStageKind::UpdateFinalSurfaceHeatBalance => ExecutionStageDependency::new(
            &[
                "inside_surface_heat_balance",
                "outside_surface_heat_balance",
            ],
            &["surface_state"],
        ),
        ExecutionStageKind::UpdateThermalHistories => ExecutionStageDependency::new(
            &["surface_state", "construction_thermal_data"],
            &["surface_history_state"],
        ),
        ExecutionStageKind::ReportSurfaceHeatBalance => ExecutionStageDependency::new(
            &["surface_state", "surface_history_state"],
            &["result_store"],
        ),
        ExecutionStageKind::RecKeepHeatBalance => {
            ExecutionStageDependency::new(&["zone_state", "surface_state"], &["result_store"])
        }
        ExecutionStageKind::ReportHeatBalance => ExecutionStageDependency::new(
            &["zone_state", "surface_state", "result_store"],
            &["result_store"],
        ),
        ExecutionStageKind::CheckWarmupConvergence => ExecutionStageDependency::new(
            &["zone_state", "surface_state"],
            &["warmup_convergence_state"],
        ),
        ExecutionStageKind::ZoneEquipmentManager => {
            ExecutionStageDependency::new(&["zone_state"], &["zone_equipment_state"])
        }
        ExecutionStageKind::SimPurchasedAir => ExecutionStageDependency::new(
            &["zone_equipment_state"],
            &["purchased_air_dispatch_state"],
        ),
        ExecutionStageKind::GetPurchasedAir => ExecutionStageDependency::new(
            &["purchased_air_dispatch_state"],
            &["purchased_air_input_state"],
        ),
        ExecutionStageKind::InitPurchasedAir => ExecutionStageDependency::new(
            &["purchased_air_input_state", "schedule_series"],
            &["purchased_air_state"],
        ),
        ExecutionStageKind::CalcPurchAirLoads => ExecutionStageDependency::new(
            &["zone_state", "purchased_air_state"],
            &["purchased_air_loads_state"],
        ),
        ExecutionStageKind::UpdatePurchasedAir => ExecutionStageDependency::new(
            &["purchased_air_loads_state"],
            &["node_state", "purchased_air_state"],
        ),
        ExecutionStageKind::ReportPurchasedAir => {
            ExecutionStageDependency::new(&["purchased_air_state", "node_state"], &["result_store"])
        }
        ExecutionStageKind::EmsBeginZoneTimestepBeforeInitHeatBalance
        | ExecutionStageKind::EmsBeginZoneTimestepAfterInitHeatBalance
        | ExecutionStageKind::EmsEndZoneTimestepBeforeZoneReporting
        | ExecutionStageKind::EmsEndZoneTimestepAfterZoneReporting => {
            ExecutionStageDependency::new(&["ems_state"], &["ems_state"])
        }
        ExecutionStageKind::Environment
        | ExecutionStageKind::Zone
        | ExecutionStageKind::ZoneEquipment
        | ExecutionStageKind::Output => ExecutionStageDependency::default(),
    }
}
