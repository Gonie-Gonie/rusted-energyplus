//! Retained module and per-unit state for `InitPurchasedAir`.

#[rustfmt::skip] mod diagnostic; #[rustfmt::skip] mod unit; #[rustfmt::skip] mod witnesses;

pub use self::diagnostic::*;

use std::collections::BTreeMap;

use ep_model::{IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId, ZoneId};

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot as Cp348Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Cp349Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Cp353Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Cp350Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Cp352Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot as Cp351Snapshot,
    *,
};
use super::{
    IdealLoadsInitFlags, PurchasedAirInitTopologyDiagnostic, PurchasedAirInitTopologyError,
    PurchasedAirInitTopologyPlan, PurchasedAirRecirculationSource,
    PurchasedAirSupplyTemperatureDiagnosticRegistry,
};

/// Mutable state retained across PurchasedAir initialization calls.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PurchasedAirRuntimeState {
    /// Whether the source-shaped per-unit flag arena has been allocated.
    pub module_initialized: bool,
    /// Whether Zone equipment-list membership has been checked.
    pub equipment_list_checked: bool,
    /// IdealLoads systems in immutable typed declaration order.
    pub declared_system_order: Vec<IdealLoadsAirSystemId>,
    /// Systems visited by the one manager-wide equipment-list sweep.
    pub equipment_list_scan_order: Vec<IdealLoadsAirSystemId>,
    /// Ordered source-shaped severe diagnostics emitted by the sweep.
    pub equipment_list_diagnostics: Vec<PurchasedAirInitDiagnostic>,
    /// Rust-owned bounded registry for supply-temperature recurring identities.
    pub supply_temperature_diagnostic_registry: PurchasedAirSupplyTemperatureDiagnosticRegistry,
    /// Per-system lifecycle state in typed-ID order.
    pub units: BTreeMap<IdealLoadsAirSystemId, PurchasedAirUnitRuntimeState>,
    /// Number of module arena allocations.
    pub module_initialization_count: usize,
    /// Number of completed global equipment-list checks.
    pub equipment_list_check_count: usize,
    /// Total units visited by the manager-wide equipment-list sweep.
    pub equipment_list_scanned_unit_count: usize,
    /// Units missing from every Zone equipment list during the sweep.
    pub equipment_list_missing_unit_count: usize,
    cooling_economizer_condition_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerConditionSnapshot>,
    cooling_economizer_body_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingEconomizerBodySnapshot>,
    cooling_sensible_flow_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSensibleFlowSnapshot>,
    cooling_dehumidification_flow_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingDehumidificationFlowSnapshot>,
    cooling_humidification_flow_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidificationFlowSnapshot>,
    cooling_capacity_zero_flow_reset_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
    cooling_supply_mass_flow_maximum_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
    cooling_supply_mass_flow_ems_override_guard_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    >,
    cooling_supply_mass_flow_ems_override_body_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    >,
    cooling_supply_mass_flow_limit_guard_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot>,
    cooling_supply_mass_flow_limit_body_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot>,
    cooling_supply_mass_flow_very_small_guard_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    >,
    cooling_supply_mass_flow_very_small_guard_body_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    >,
    cooling_mixed_air_call_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingMixedAirCallSnapshot>,
    cooling_supply_mass_flow_positive_guard_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot>,
    cooling_positive_supply_cp_air_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    >,
    cooling_positive_supply_temperature_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    >,
    cooling_positive_supply_temperature_minimum_limit_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    >,
    cooling_positive_supply_temperature_mixed_air_limit_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    >,
    cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    >,
    cooling_positive_supply_enthalpy_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    >,
    cooling_positive_supply_capacity_limit_guard_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    >,
    cooling_positive_supply_capacity_limit_cp_air_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    >,
    cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    >,
    cooling_positive_supply_capacity_limit_sensible_output_guard_latest_witnesses: BTreeMap<
        IdealLoadsAirSystemId,
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    >,
    cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
        >,
    cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
        >,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
        >,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
        >,
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
        >,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
        >,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witnesses:
        BTreeMap<
            IdealLoadsAirSystemId,
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
        >,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, Cp348Snapshot>,
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_latest_witnesses:
        BTreeMap<IdealLoadsAirSystemId, Cp349Snapshot>,
    #[rustfmt::skip] cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, Cp350Snapshot>,
    #[rustfmt::skip] cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, Cp351Snapshot>,
    #[rustfmt::skip] cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, Cp352Snapshot>,
    #[rustfmt::skip] cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, Cp353Snapshot>, #[rustfmt::skip] cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot>, #[rustfmt::skip] cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot>, #[rustfmt::skip] cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot>, #[rustfmt::skip] cooling_constant_shr_case_break_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot>, #[rustfmt::skip] cooling_humidistat_case_entry_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot>, #[rustfmt::skip] cooling_humidistat_moisture_demand_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot>, #[rustfmt::skip] cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot>, #[rustfmt::skip] cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot>,
    #[rustfmt::skip] cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot>, #[rustfmt::skip] cooling_humidistat_case_break_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot>, #[rustfmt::skip] cooling_constant_supply_humidity_ratio_case_entry_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot>, #[rustfmt::skip] cooling_constant_supply_humidity_ratio_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot>, #[rustfmt::skip] cooling_constant_supply_humidity_ratio_case_break_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot>, #[rustfmt::skip] cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot>, #[rustfmt::skip] cooling_default_supply_humidity_ratio_case_break_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_control_humidistat_guard_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot>,
    #[rustfmt::skip] cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot>,
    #[rustfmt::skip] cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_saturation_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot>, #[rustfmt::skip] cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot>, #[rustfmt::skip] cooling_supply_enthalpy_post_saturation_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot>, #[rustfmt::skip] cooling_post_saturation_capacity_limit_guard_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot>, #[rustfmt::skip] cooling_post_saturation_capacity_limit_dehumidification_guard_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot>, #[rustfmt::skip] cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot>, #[rustfmt::skip] cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot>, #[rustfmt::skip] cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot>, #[rustfmt::skip] cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witnesses: BTreeMap<IdealLoadsAirSystemId, PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot>,
}

/// Persistent `InitPurchasedAir` state for one IdealLoads system.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirUnitRuntimeState {
    /// Typed system identity.
    pub system: IdealLoadsAirSystemId,
    /// Source one-time latch, committed before semantic topology validation.
    pub one_time_latched: bool,
    /// Whether the selected-unit topology pass reached its normal tail.
    pub topology_completed: bool,
    /// Source `MySizeFlag`; true means the hard-size/sizing gate is pending.
    pub sizing_needed: bool,
    /// Four-field PurchasedAir object overlay seeded after topology succeeds.
    pub sized_limits: Option<PurchasedAirSizedLimits>,
    /// Successful direct hard-size child outcome retained for downstream stages.
    pub sizing_outcome: Option<PurchasedAirHardSizeLegacyOutcome>,
    /// Source `MyEnvrnFlag`; true means a begin-environment write is pending.
    pub environment_initialization_needed: bool,
    /// Controlled Zone captured by the one-time topology pass.
    pub controlled_zone: Option<ZoneId>,
    /// Equipment list captured by the one-time topology pass.
    pub equipment_list: Option<ZoneEquipmentListId>,
    /// Supply node captured by the one-time topology pass.
    pub supply_node: Option<NodeId>,
    /// Exhaust-or-return recirculation node captured by the one-time pass.
    pub recirculation_node: Option<NodeId>,
    /// Source branch that selected or left recirculation unassigned.
    pub recirculation_source: Option<PurchasedAirRecirculationSource>,
    /// Persistent bounded `CalcPurchAirLoads` entry-prefix state.
    pub calc_entry: PurchasedAirCalcEntryRuntimeState,
    /// Persistent bounded minimum-outdoor-air prefix state.
    pub calc_minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixRuntimeState,
    /// Persistent bounded cooling-entry gate state.
    pub calc_cooling_entry_gate: PurchasedAirCalcCoolingEntryGateRuntimeState,
    /// Persistent bounded cooling OA/max-flow gate state.
    pub calc_cooling_oa_max_flow_gate: PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    /// Persistent bounded cooling OA/max-flow warning-and-clamp body state.
    pub calc_cooling_oa_max_flow_body: PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
    /// Persistent bounded cooling economizer outer-guard state.
    pub calc_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
    /// Persistent bounded cooling economizer inner-condition state.
    pub calc_cooling_economizer_condition: PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
    /// Persistent bounded cooling economizer true-body state.
    pub calc_cooling_economizer_body: PurchasedAirCalcCoolingEconomizerBodyRuntimeState,
    /// Persistent bounded cooling sensible-flow state.
    pub calc_cooling_sensible_flow: PurchasedAirCalcCoolingSensibleFlowRuntimeState,
    /// Persistent bounded cooling dehumidification-flow state.
    pub calc_cooling_dehumidification_flow: PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    /// Persistent bounded cooling humidification-flow state.
    pub calc_cooling_humidification_flow: PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    /// Persistent bounded cooling capacity-zero candidate-reset state.
    pub calc_cooling_capacity_zero_flow_reset:
        PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    /// Persistent bounded cooling supply-mass-flow maximum state.
    pub calc_cooling_supply_mass_flow_maximum:
        PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    /// Persistent bounded cooling supply-mass-flow EMS-override guard state.
    pub calc_cooling_supply_mass_flow_ems_override_guard:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    /// Persistent bounded cooling supply-mass-flow EMS-override body state.
    pub calc_cooling_supply_mass_flow_ems_override_body:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    /// Persistent bounded cooling supply-mass-flow limit-guard state.
    pub calc_cooling_supply_mass_flow_limit_guard:
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    /// Persistent bounded cooling supply-mass-flow limit-body state.
    pub calc_cooling_supply_mass_flow_limit_body:
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    /// Persistent bounded cooling supply-mass-flow very-small-guard state.
    pub calc_cooling_supply_mass_flow_very_small_guard:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    /// Persistent bounded cooling supply-mass-flow positive-zero reset-body state.
    pub calc_cooling_supply_mass_flow_very_small_guard_body:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    /// Persistent bounded Cooling mixed-air call and no-OA fallback state.
    pub calc_cooling_mixed_air_call: PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    /// Persistent bounded cooling positive supply-mass-flow guard state.
    pub calc_cooling_supply_mass_flow_positive_guard:
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    /// Persistent bounded Cooling positive-supply `CpAir` assignment state.
    pub calc_cooling_positive_supply_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState,
    /// Persistent bounded Cooling positive-supply temperature-assignment state.
    pub calc_cooling_positive_supply_temperature_assignment:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState,
    /// Persistent bounded Cooling positive-supply minimum-temperature limit state.
    pub calc_cooling_positive_supply_temperature_minimum_limit:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState,
    /// Persistent bounded Cooling positive-supply mixed-air temperature limit state.
    pub calc_cooling_positive_supply_temperature_mixed_air_limit:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState,
    /// Persistent bounded Cooling positive-supply mixed-air humidity-ratio assignment state.
    pub calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    /// Persistent bounded Cooling positive-supply enthalpy-assignment state.
    pub calc_cooling_positive_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState,
    /// Persistent bounded Cooling positive-supply capacity-limit guard state.
    pub calc_cooling_positive_supply_capacity_limit_guard:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState,
    /// Persistent bounded Cooling capacity-limit `CpAir` assignment state.
    pub calc_cooling_positive_supply_capacity_limit_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState,
    /// Persistent bounded Cooling capacity-limit sensible-output assignment state.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState,
    /// Persistent bounded Cooling capacity-limit sensible-output guard state.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_guard:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState,
    /// Persistent bounded Cooling sensible-output maximum-capacity assignment state.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState,
    /// Persistent bounded Cooling capacity-limit supply-enthalpy assignment state.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState,
    /// Persistent bounded Cooling capacity-limit supply-temperature assignment state.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState,
    /// Persistent bounded Cooling capacity-limit supply-temperature mixed-air-limit state.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState,
    /// Persistent bounded post-capacity-limit mixed-air humidity-ratio assignment state.
    pub calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState,
    /// Persistent bounded post-capacity-limit dehumidification-control switch state.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
    /// Persistent bounded post-capacity-limit dehumidification-control `None` case state.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    /// Persistent bounded constant-SHR dehumidification-control case-entry state.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    /// Persistent bounded constant-SHR dehumidification-control `CpAir` assignment state.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState,
    /// Persistent bounded constant-SHR dehumidification-control sensible-output assignment state.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState,
    /// Persistent bounded constant-SHR dehumidification-control total-output assignment state.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState,
    #[doc = "Persistent bounded constant-SHR dehumidification-control supply-enthalpy assignment state."] #[rustfmt::skip] pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment: PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState,
    #[doc = "Persistent bounded constant-SHR supply-enthalpy overdrying-limit state."] #[rustfmt::skip] pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit: PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState,
    #[doc = "Persistent bounded constant-SHR supply-humidity-ratio overdrying-limit state."] #[rustfmt::skip] pub calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState,
    #[doc = "Persistent bounded constant-SHR supply-humidity-ratio minimum-limit state."] #[rustfmt::skip] pub calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState,
    #[doc = "Persistent bounded constant-SHR supply-humidity-ratio mixed-air-limit state."] #[rustfmt::skip] pub calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState,
    #[doc = "Persistent bounded constant-SHR case-break state."] #[rustfmt::skip] pub calc_cooling_constant_shr_case_break: PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState,
    #[doc = "Persistent bounded Humidistat case-entry state."] #[rustfmt::skip] pub calc_cooling_humidistat_case_entry: PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
    #[doc = "Persistent bounded Humidistat moisture-demand assignment state."] #[rustfmt::skip] pub calc_cooling_humidistat_moisture_demand_assignment: PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState,
    #[doc = "Persistent bounded Humidistat supply-humidity-ratio-for-dehumidification assignment state."] #[rustfmt::skip] pub calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState,
    #[doc = "Persistent bounded Humidistat supply-humidity-ratio-for-dehumidification minimum-limit state."] #[rustfmt::skip] pub calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState,
    #[doc = "Persistent bounded Humidistat purchased-air supply-humidity-ratio mixed-air-limit state."] #[rustfmt::skip] pub calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState, #[doc = "Persistent bounded Humidistat case-break state."] #[rustfmt::skip] pub calc_cooling_humidistat_case_break: PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState, #[doc = "Persistent bounded constant-supply-humidity-ratio case-entry state."] #[rustfmt::skip] pub calc_cooling_constant_supply_humidity_ratio_case_entry: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState, #[doc = "Persistent bounded constant-supply-humidity-ratio assignment state."] #[rustfmt::skip] pub calc_cooling_constant_supply_humidity_ratio_assignment: PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState, #[doc = "Persistent bounded constant-supply-humidity-ratio case-break state."] #[rustfmt::skip] pub calc_cooling_constant_supply_humidity_ratio_case_break: PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState, #[doc = "Persistent bounded default supply-humidity-ratio mixed-air assignment state."] #[rustfmt::skip] pub calc_cooling_default_supply_humidity_ratio_mixed_air_assignment: PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState, #[doc = "Persistent bounded default supply-humidity-ratio case-break state."] #[rustfmt::skip] pub calc_cooling_default_supply_humidity_ratio_case_break: PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState, #[doc = "Persistent bounded Cooling supply-humidity-ratio humidification heating-availability guard state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState, #[doc = "Persistent bounded Cooling supply-humidity-ratio humidification-control Humidistat guard state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState, #[doc = "Persistent bounded Cooling supply-humidity-ratio nested dehumidification-control Humidistat-or-None guard state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState, #[doc = "Persistent bounded Cooling humidifying-setpoint moisture-demand assignment state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState, #[doc = "Persistent bounded Cooling humidification supply-humidity-ratio assignment state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState, #[doc = "Persistent bounded Cooling humidification supply-humidity-ratio maximum-limit state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState,
    /// Persistent bounded Cooling humidification purchased-air humidity-ratio maximum assignment.
    pub calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment: PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState,
    /// Persistent bounded pre-saturation original supply-humidity-ratio assignment state.
    pub calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment: PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState,
    /// Persistent bounded local saturation supply-humidity-ratio assignment state.
    pub calc_cooling_supply_humidity_ratio_saturation_assignment: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState,
    #[doc = "Persistent bounded final purchased-air saturation-limit assignment state."] #[rustfmt::skip] pub calc_cooling_supply_humidity_ratio_saturation_limit_assignment: PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState,
    #[doc = "Persistent bounded post-saturation purchased-air supply-enthalpy assignment state."] #[rustfmt::skip] pub calc_cooling_supply_enthalpy_post_saturation_assignment: PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState, #[doc = "Persistent bounded post-saturation purchased-air capacity-limit guard state."] #[rustfmt::skip] pub calc_cooling_post_saturation_capacity_limit_guard: PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState, #[doc = "Persistent bounded post-saturation capacity-limit dehumidification guard state."] #[rustfmt::skip] pub calc_cooling_post_saturation_capacity_limit_dehumidification_guard: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState, #[doc = "Persistent bounded post-saturation dehumidifying total-output assignment state."] #[rustfmt::skip] pub calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState, #[doc = "Persistent bounded post-saturation dehumidifying total-output capacity-guard state."] #[rustfmt::skip] pub calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState, #[doc = "Persistent bounded post-saturation total-output maximum-capacity assignment state."] #[rustfmt::skip] pub calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState, #[doc = "Persistent bounded post-saturation capacity-limited dehumidification supply-enthalpy assignment state."] #[rustfmt::skip] pub calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState,
    /// Configured exhaust rejected before return fallback.
    pub rejected_exhaust_node: Option<NodeId>,
    /// First return node named by the source multiple-return warning.
    pub reported_first_return_node: Option<NodeId>,
    /// Immutable selected-unit topology retained by the first call.
    pub topology_plan: Option<PurchasedAirInitTopologyPlan>,
    /// Ordered diagnostics retained from the one-time topology block.
    pub topology_diagnostics: Vec<PurchasedAirInitTopologyDiagnostic>,
    /// Fatal topology result retained after the source latch is committed.
    pub topology_failure: Option<PurchasedAirInitTopologyError>,
    /// Immutable first-match result captured when the manager arena is allocated.
    pub planned_first_matching_equipment_list: Option<ZoneEquipmentListId>,
    /// One-based manager sweep ordinal, once Zone equipment input is ready.
    pub equipment_list_scan_ordinal: Option<usize>,
    /// First controlled-Zone-referenced equipment list containing this unit.
    pub first_matching_equipment_list: Option<ZoneEquipmentListId>,
    /// Whether the manager sweep found this unit in any equipment list.
    pub equipment_list_membership_found: Option<bool>,
    #[doc = "Cached maximum heating air mass flow from begin-environment initialization."] #[rustfmt::skip] pub maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    #[doc = "Cached maximum cooling air mass flow from begin-environment initialization."] #[rustfmt::skip] pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    /// Standard air density used for the cached environment values.
    pub standard_air_density_kg_per_m3: Option<f64>,
    /// Total calls for this unit.
    pub init_call_count: usize,
    /// Source one-time latch transitions.
    pub one_time_initialization_count: usize,
    /// One-time topology blocks that reached their normal tail.
    pub topology_completion_count: usize,
    /// Completed hard-size/sizing gates.
    pub sizing_check_count: usize,
    /// Hard-size/sizing child attempts, including fail-closed returns.
    pub sizing_attempt_count: usize,
    /// Completed begin-environment writes.
    pub environment_initialization_count: usize,
    /// False-begin-environment calls that rearmed the environment latch.
    pub environment_rearm_count: usize,
    /// Source `CoolErrIndex`; zero means no recurring identity is registered.
    pub cooling_supply_temperature_error_index: usize,
    /// Source `HeatErrIndex`; zero means no recurring identity is registered.
    pub heating_supply_temperature_error_index: usize,
    /// First detailed cooling diagnostic groups emitted.
    pub cooling_supply_temperature_first_diagnostic_count: usize,
    /// First detailed heating diagnostic groups emitted.
    pub heating_supply_temperature_first_diagnostic_count: usize,
    /// Active cooling supply-temperature recurring diagnostic count.
    pub cooling_supply_temperature_warning_count: usize,
    /// Active heating supply-temperature recurring diagnostic count.
    pub heating_supply_temperature_warning_count: usize,
    /// Nonfatal OA/economizer flow-limit advisories emitted once.
    pub economizer_flow_limit_warning_count: usize,
}
