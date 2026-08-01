//! Fixed-timestep direct-Zone heat-balance/PurchasedAir release runtime.

use std::fmt::{Display, Formatter};

use ep_model::{AutosizeOrNumber, IdealLoadsLimit, SimulationModel};

use crate::error::RuntimeError;
use crate::heat_balance::air_manager::seed_zone_air_humidity_ratios_from_weather_series;
use crate::heat_balance::algorithm::{
    HeatBalanceRuntimeConfig, direct_zone_purchased_air_fixed_step_runtime_config,
};
use crate::heat_balance::initialization::initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled;
use crate::heat_balance::manager::init_heat_balance_source_order_path;
use crate::heat_balance::reports::{
    HeatBalanceResultSeriesTraces, heat_balance_result_store_from_traces,
};
use crate::heat_balance::run_period::sample_heat_balance_run_period_with_step_driver;
use crate::heat_balance::state::{
    HeatBalanceCtfInitialHistoryPolicy, HeatBalanceSimulationOptions, HeatBalanceState,
};
use crate::heat_balance::surface_boundary::{
    seed_energyplus_initial_surface_ctf_histories, seed_initial_surface_ctf_boundary_histories,
};
use crate::heat_balance::timestep::advance_heat_balance_state_one_timestep_with_direct_zone_purchased_air;
use crate::heat_balance::trace::HeatBalanceRunPeriodSamples;
use crate::schedules::{
    HeatBalanceInternalGainScheduleOperationProfile, ScheduleSeriesCache,
    precompute_hour_only_internal_gain_schedule_cache_profiled,
};
use crate::time_axis::run_period_first_hour_interpolation_starting_values;
use crate::weather::WeatherTimestepSeries;
use crate::{ResultStore, ZoneSensibleDemandInputKind};

use super::{
    DirectZonePurchasedAirBindingError, DirectZonePurchasedAirHourlyOutputError,
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirRuntimeStepError,
    IdealLoadsPurchasedAirBranch, IdealLoadsSensibleMode, PURCHASED_AIR_CALC_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER, PurchasedAirAvailabilityStatus,
    PurchasedAirCalcCoolingCapacityZeroFlowResetError,
    PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrCaseBreakError,
    PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitError,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitError,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakError,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakError,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingDehumidificationFlowError,
    PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerBodyError,
    PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionError,
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardError,
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary, PurchasedAirCalcCoolingEntryGateError,
    PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    PurchasedAirCalcCoolingHumidificationFlowError,
    PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseBreakError,
    PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseEntryError,
    PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitError,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingMixedAirCallError, PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodyError,
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateError,
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    PurchasedAirCalcCoolingSensibleFlowError, PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardError,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentError,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumError,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    PurchasedAirCalcEntryError, PurchasedAirCalcEntryLifecycleSummary,
    PurchasedAirCalcEntrySnapshot, PurchasedAirCalcMinimumOaPrefixError,
    PurchasedAirCalcMinimumOaPrefixLifecycleSummary, PurchasedAirHardSizeLegacyRoute,
    PurchasedAirInitError, PurchasedAirInitLifecycleSummary, PurchasedAirRecirculationSource,
    PurchasedAirRuntimeState, PurchasedAirSizedLimits,
    append_direct_zone_purchased_air_hourly_output_series, bind_direct_zone_purchased_air_model,
    purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary,
    purchased_air_calc_cooling_constant_shr_case_break_lifecycle_summary,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle_summary,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle_summary,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle_summary,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary,
    purchased_air_calc_cooling_default_supply_humidity_ratio_case_break_lifecycle_summary,
    purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_dehumidification_flow_lifecycle_summary,
    purchased_air_calc_cooling_economizer_body_lifecycle_summary,
    purchased_air_calc_cooling_economizer_condition_lifecycle_summary,
    purchased_air_calc_cooling_economizer_guard_lifecycle_summary,
    purchased_air_calc_cooling_entry_gate_lifecycle_summary,
    purchased_air_calc_cooling_humidification_flow_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_case_entry_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle_summary,
    purchased_air_calc_cooling_mixed_air_call_lifecycle_summary,
    purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary,
    purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary,
    purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle_summary,
    purchased_air_calc_cooling_sensible_flow_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary,
    purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary,
    purchased_air_calc_entry_lifecycle_summary,
    purchased_air_calc_minimum_oa_prefix_lifecycle_summary, purchased_air_init_lifecycle_summary,
};

mod cooling_capacity_zero_flow_reset_validation;
mod cooling_constant_shr_case_break_validation;
mod cooling_constant_shr_supply_humidity_ratio_minimum_limit_validation;
mod cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation;
mod cooling_constant_shr_supply_humidity_ratio_overdrying_limit_validation;
mod cooling_constant_supply_humidity_ratio_assignment_validation;
mod cooling_constant_supply_humidity_ratio_case_break_validation;
mod cooling_constant_supply_humidity_ratio_case_entry_validation;
mod cooling_default_supply_humidity_ratio_case_break_validation;
mod cooling_default_supply_humidity_ratio_mixed_air_assignment_validation;
mod cooling_dehumidification_flow_validation;
mod cooling_economizer_body_validation;
mod cooling_economizer_condition_validation;
mod cooling_economizer_guard_validation;
mod cooling_entry_validation;
mod cooling_humidification_flow_validation;
mod cooling_humidistat_case_break_validation;
mod cooling_humidistat_case_entry_validation;
mod cooling_humidistat_moisture_demand_assignment_validation;
mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation;
mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_validation;
mod cooling_humidistat_supply_humidity_ratio_mixed_air_limit_validation;
mod cooling_mixed_air_call_validation;
mod cooling_oa_max_flow_body_validation;
mod cooling_oa_max_flow_validation;
mod cooling_positive_supply_capacity_limit_cp_air_assignment_validation;
mod cooling_positive_supply_capacity_limit_guard_validation;
mod cooling_positive_supply_capacity_limit_sensible_output_assignment_validation;
mod cooling_positive_supply_capacity_limit_sensible_output_guard_validation;
mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_validation;
mod cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_validation;
mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_validation;
mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_validation;
mod cooling_positive_supply_cp_air_assignment_validation;
mod cooling_positive_supply_enthalpy_assignment_validation;
mod cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_validation;
mod cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_validation;
mod cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_validation;
mod cooling_positive_supply_temperature_assignment_validation;
mod cooling_positive_supply_temperature_minimum_limit_validation;
mod cooling_positive_supply_temperature_mixed_air_limit_validation;
mod cooling_sensible_flow_validation;
mod cooling_supply_humidity_ratio_humidification_control_humidistat_guard_validation;
mod cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_validation;
mod cooling_supply_humidity_ratio_humidification_heating_availability_guard_validation;
mod cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_validation;
pub(in crate::ideal_loads) mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_validation;
mod cooling_supply_mass_flow_ems_override_body_validation;
mod cooling_supply_mass_flow_ems_override_guard_validation;
mod cooling_supply_mass_flow_limit_body_validation;
mod cooling_supply_mass_flow_limit_guard_validation;
mod cooling_supply_mass_flow_maximum_validation;
mod cooling_supply_mass_flow_positive_guard_validation;
mod cooling_supply_mass_flow_very_small_guard_body_validation;
mod cooling_supply_mass_flow_very_small_guard_validation;
mod minimum_oa_validation;

const SECONDS_PER_HOUR: f64 = 3_600.0;

/// Stable release-loop demand provenance for the bounded direct-Zone runtime.
pub const DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE: &str =
    "rust-predictor-source-setpoint-thresholds";

/// Stable recirculation-state provenance for the source-valid single-return subset.
pub const DIRECT_ZONE_PURCHASED_AIR_RECIRCULATION_SOURCE: &str =
    "rust-direct-zone-return-projection";

/// Actual coupled source-order stages executed once per nominal system step.
pub const DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER: &[&str] = &[
    "predict-system-loads",
    "init-purchased-air",
    "calc-purch-air-loads",
    "update-purchased-air",
    "report-purchased-air",
    "correct-zone-air-temps",
];

/// Options for the bounded fixed-timestep direct-Zone coupled runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCoupledOptions {
    /// Number of hourly run-period samples to report.
    pub sample_count: usize,
    /// Initial Zone mean air temperature.
    pub initial_zone_air_temperature_c: f64,
}

impl DirectZonePurchasedAirCoupledOptions {
    /// Creates fixed-timestep options for an hourly result prefix.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: 23.0,
        }
    }
}

/// Summary of one bounded coupled heat-balance/PurchasedAir run.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCoupledSummary {
    /// Number of hourly samples reported.
    pub samples: usize,
    /// Number of nominal system/Zone timesteps executed.
    pub timestep_count: usize,
    /// Number of Zone timesteps per reporting hour.
    pub zone_timesteps_per_hour: u32,
    /// Fixed nominal timestep duration.
    pub timestep_seconds: f64,
    /// Number of successful CP301 calls.
    pub coupling_call_count: usize,
    /// Bound IdealLoads system name.
    pub system_name: String,
    /// Bound supply-node name.
    pub supply_node_name: String,
    /// Bound Zone return-node name used by blank-exhaust PurchasedAir.
    pub return_node_name: String,
    /// PurchasedAir branch enforced by the binding.
    pub branch: IdealLoadsPurchasedAirBranch,
    /// Zone-demand provenance used by every call.
    pub zone_demand_source: &'static str,
    /// Whether the oracle/default active-split constructor was used.
    pub fixture_demand_injection_used: bool,
    /// Provenance of the state projected onto the bound direct return node.
    pub recirculation_state_source: &'static str,
    /// Actual nested predictor/HVAC/corrector order.
    pub actual_coupled_source_order: &'static [&'static str],
    /// Persistent bounded `InitPurchasedAir` lifecycle report.
    pub init_lifecycle: PurchasedAirInitLifecycleSummary,
    /// Persistent bounded `CalcPurchAirLoads` entry-prefix lifecycle report.
    pub calc_entry_lifecycle: PurchasedAirCalcEntryLifecycleSummary,
    /// Persistent bounded minimum-outdoor-air prefix lifecycle report.
    pub calc_minimum_oa_prefix_lifecycle: PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
    /// Persistent bounded cooling-entry gate lifecycle report.
    pub calc_cooling_entry_gate_lifecycle: PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    /// Persistent bounded cooling OA maximum-flow gate lifecycle report.
    pub calc_cooling_oa_max_flow_gate_lifecycle:
        PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    /// Persistent bounded cooling OA maximum-flow warning-and-clamp body lifecycle report.
    pub calc_cooling_oa_max_flow_body_lifecycle:
        PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    /// Persistent bounded cooling economizer guard lifecycle report.
    pub calc_cooling_economizer_guard_lifecycle:
        PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    /// Persistent bounded cooling economizer differential-condition lifecycle report.
    pub calc_cooling_economizer_condition_lifecycle:
        PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    /// Persistent bounded cooling economizer true-body lifecycle report.
    pub calc_cooling_economizer_body_lifecycle:
        PurchasedAirCalcCoolingEconomizerBodyLifecycleSummary,
    /// Persistent bounded cooling sensible-flow lifecycle report.
    pub calc_cooling_sensible_flow_lifecycle: PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
    /// Persistent bounded cooling dehumidification-flow lifecycle report.
    pub calc_cooling_dehumidification_flow_lifecycle:
        PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
    /// Persistent bounded cooling humidification-flow lifecycle report.
    pub calc_cooling_humidification_flow_lifecycle:
        PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
    /// Persistent bounded cooling capacity-zero candidate-reset lifecycle report.
    pub calc_cooling_capacity_zero_flow_reset_lifecycle:
        PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
    /// Persistent bounded pre-EMS cooling supply mass-flow maximum lifecycle report.
    pub calc_cooling_supply_mass_flow_maximum_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
    /// Persistent bounded cooling supply mass-flow EMS-override guard lifecycle report.
    pub calc_cooling_supply_mass_flow_ems_override_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    /// Persistent bounded cooling supply mass-flow EMS-override body lifecycle report.
    pub calc_cooling_supply_mass_flow_ems_override_body_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    /// Persistent bounded cooling supply mass-flow limit-guard lifecycle report.
    pub calc_cooling_supply_mass_flow_limit_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
    /// Persistent bounded cooling supply mass-flow limit-body lifecycle report.
    pub calc_cooling_supply_mass_flow_limit_body_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
    /// Persistent bounded cooling supply mass-flow very-small guard lifecycle report.
    pub calc_cooling_supply_mass_flow_very_small_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
    /// Persistent bounded cooling supply mass-flow positive-zero reset-body lifecycle report.
    pub calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
    /// Persistent bounded cooling mixed-air call lifecycle report.
    pub calc_cooling_mixed_air_call_lifecycle: PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
    /// Persistent bounded cooling positive supply mass-flow guard lifecycle report.
    pub calc_cooling_supply_mass_flow_positive_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
    /// Persistent bounded cooling positive-supply Cp-air assignment lifecycle report.
    pub calc_cooling_positive_supply_cp_air_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply temperature assignment lifecycle report.
    pub calc_cooling_positive_supply_temperature_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply temperature minimum-limit lifecycle report.
    pub calc_cooling_positive_supply_temperature_minimum_limit_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleSummary,
    /// Persistent bounded cooling positive-supply mixed-air-temperature limit lifecycle report.
    pub calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleSummary,
    /// Persistent bounded cooling positive-supply mixed-air humidity-ratio assignment lifecycle report.
    pub calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply enthalpy assignment lifecycle report.
    pub calc_cooling_positive_supply_enthalpy_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit guard lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_guard_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit Cp-air assignment lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit sensible-output assignment lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit sensible-output guard lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
    /// Persistent bounded cooling positive-supply sensible-output maximum-capacity assignment lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit supply-enthalpy assignment lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit supply-temperature assignment lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleSummary,
    /// Persistent bounded cooling positive-supply capacity-limit supply-temperature mixed-air-limit lifecycle report.
    pub calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleSummary,
    /// Persistent bounded post-capacity-limit mixed-air humidity-ratio assignment lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleSummary,
    /// Persistent bounded post-capacity-limit dehumidification-control switch lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary,
    /// Persistent bounded dehumidification-control None-case lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary,
    /// Persistent bounded constant-sensible-heat-ratio case-entry lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleSummary,
    /// Persistent bounded constant-sensible-heat-ratio CpAir-assignment lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary,
    /// Persistent bounded constant-sensible-heat-ratio sensible-output-assignment lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleSummary,
    /// Persistent bounded constant-sensible-heat-ratio total-output-assignment lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleSummary,
    /// Persistent bounded constant-sensible-heat-ratio supply-enthalpy-assignment lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleSummary,
    /// Persistent bounded constant-SHR overdrying-limit lifecycle report.
    pub calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleSummary,
    /// Persistent bounded constant-SHR supply-humidity-ratio overdrying-limit lifecycle report.
    pub calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle:
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleSummary,
    /// Persistent bounded constant-SHR supply-humidity-ratio minimum-limit lifecycle report.
    pub calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle:
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleSummary,
    /// Persistent bounded constant-SHR supply-humidity-ratio mixed-air-limit lifecycle report.
    pub calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle:
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    /// Persistent bounded constant-SHR case-break lifecycle report.
    pub calc_cooling_constant_shr_case_break_lifecycle:
        PurchasedAirCalcCoolingConstantShrCaseBreakLifecycleSummary,
    /// Persistent bounded Humidistat case-entry lifecycle report.
    pub calc_cooling_humidistat_case_entry_lifecycle:
        PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    /// Persistent bounded Humidistat moisture-demand assignment lifecycle report.
    pub calc_cooling_humidistat_moisture_demand_assignment_lifecycle:
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    /// Persistent bounded Humidistat supply-humidity-ratio-for-dehumidification assignment lifecycle report.
    pub calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleSummary,
    /// Persistent bounded Humidistat supply-humidity-ratio-for-dehumidification minimum-limit lifecycle report.
    pub calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleSummary,
    /// Persistent bounded Humidistat purchased-air supply-humidity-ratio mixed-air-limit lifecycle report.
    pub calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleSummary,
    /// Persistent bounded Humidistat case-break lifecycle report.
    pub calc_cooling_humidistat_case_break_lifecycle:
        PurchasedAirCalcCoolingHumidistatCaseBreakLifecycleSummary,
    /// Persistent bounded constant-supply-humidity-ratio case-entry lifecycle report.
    pub calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle:
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
    /// Persistent bounded constant-supply-humidity-ratio assignment lifecycle report.
    pub calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle:
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    /// Persistent bounded constant-supply-humidity-ratio case-break lifecycle report.
    pub calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle:
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleSummary,
    /// Persistent bounded default supply-humidity-ratio mixed-air assignment lifecycle report.
    pub calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle:
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleSummary,
    /// Persistent bounded default supply-humidity-ratio case-break lifecycle report.
    pub calc_cooling_default_supply_humidity_ratio_case_break_lifecycle:
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleSummary,
    /// Persistent bounded Cooling humidification heating-availability guard lifecycle report.
    pub calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleSummary,
    /// Persistent bounded Cooling humidification-control Humidistat guard lifecycle report.
    pub calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleSummary,
    /// Persistent bounded nested dehumidification-control Humidistat-or-None guard lifecycle report.
    pub calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleSummary,
    /// Persistent bounded humidifying-setpoint moisture-demand assignment lifecycle report.
    pub calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary,
    /// Persistent bounded humidification supply-humidity-ratio assignment lifecycle report.
    pub calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleSummary,
}

/// Result of the bounded coupled release runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectZonePurchasedAirCoupledSimulation {
    /// Final heat-balance state after reported run-period samples.
    pub state: HeatBalanceState,
    /// Combined heat-balance and PurchasedAir result series.
    pub results: ResultStore,
    /// Bounded runtime summary and provenance.
    pub summary: DirectZonePurchasedAirCoupledSummary,
    /// Deterministic internal-gain schedule operation counts.
    pub internal_gain_schedule_cache_profile: HeatBalanceInternalGainScheduleOperationProfile,
}

/// Fail-closed error from the bounded coupled release runtime.
#[derive(Debug, PartialEq)]
pub enum DirectZonePurchasedAirCoupledRuntimeError {
    /// Static CP301 topology/model binding failed.
    Binding(DirectZonePurchasedAirBindingError),
    /// Heat-balance initialization or weather input failed.
    HeatBalance(RuntimeError),
    /// A release run with no system timestep cannot execute initialization.
    NoTimestepsRequested,
    /// The active zone-timestep cache cannot cover the requested prefix.
    ScheduleCacheCoverage {
        /// Required zone-timestep samples.
        required: usize,
        /// Available cache samples.
        available: usize,
    },
    /// Requested hourly and Zone-timestep counts overflowed `usize`.
    TimestepCountOverflow,
    /// One live predictor-bound CP301 call failed.
    RuntimeStep(DirectZonePurchasedAirRuntimeStepError),
    /// Final lifecycle summary could not resolve the bound unit.
    InitLifecycle(PurchasedAirInitError),
    /// Final Calc-entry lifecycle summary could not resolve the bound unit.
    CalcEntryLifecycle(PurchasedAirCalcEntryError),
    /// Final minimum-outdoor-air prefix summary could not resolve the bound unit.
    CalcMinimumOaPrefixLifecycle(PurchasedAirCalcMinimumOaPrefixError),
    /// Final cooling-entry gate summary could not resolve the bound unit.
    CalcCoolingEntryGateLifecycle(PurchasedAirCalcCoolingEntryGateError),
    /// Final cooling OA maximum-flow gate summary could not resolve the bound unit.
    CalcCoolingOaMaxFlowGateLifecycle(PurchasedAirCalcCoolingOaMaxFlowGateError),
    /// Final cooling OA maximum-flow body summary could not resolve the bound unit.
    CalcCoolingOaMaxFlowBodyLifecycle(PurchasedAirCalcCoolingOaMaxFlowBodyError),
    /// Final cooling economizer guard summary could not resolve the bound unit.
    CalcCoolingEconomizerGuardLifecycle(PurchasedAirCalcCoolingEconomizerGuardError),
    /// Final cooling economizer condition summary could not resolve the bound unit.
    CalcCoolingEconomizerConditionLifecycle(PurchasedAirCalcCoolingEconomizerConditionError),
    /// Final cooling economizer true-body summary could not resolve the bound unit.
    CalcCoolingEconomizerBodyLifecycle(PurchasedAirCalcCoolingEconomizerBodyError),
    /// Final cooling sensible-flow summary could not resolve the bound unit.
    CalcCoolingSensibleFlowLifecycle(PurchasedAirCalcCoolingSensibleFlowError),
    /// Final cooling dehumidification-flow summary could not resolve the bound unit.
    CalcCoolingDehumidificationFlowLifecycle(PurchasedAirCalcCoolingDehumidificationFlowError),
    /// Final cooling humidification-flow summary could not resolve the bound unit.
    CalcCoolingHumidificationFlowLifecycle(PurchasedAirCalcCoolingHumidificationFlowError),
    /// Final cooling capacity-zero reset summary could not resolve the bound unit.
    CalcCoolingCapacityZeroFlowResetLifecycle(PurchasedAirCalcCoolingCapacityZeroFlowResetError),
    /// Final cooling supply mass-flow maximum summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowMaximumLifecycle(PurchasedAirCalcCoolingSupplyMassFlowMaximumError),
    /// Final cooling supply mass-flow EMS-override guard summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowEmsOverrideGuardLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardError,
    ),
    /// Final cooling supply mass-flow EMS-override body summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowEmsOverrideBodyLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyError,
    ),
    /// Final cooling supply mass-flow limit-guard summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowLimitGuardLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardError,
    ),
    /// Final cooling supply mass-flow limit-body summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowLimitBodyLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyError,
    ),
    /// Final cooling supply mass-flow very-small guard summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowVerySmallGuardLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardError,
    ),
    /// Final cooling supply mass-flow positive-zero reset-body summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyError,
    ),
    /// Final cooling mixed-air call summary could not resolve the bound unit.
    CalcCoolingMixedAirCallLifecycle(PurchasedAirCalcCoolingMixedAirCallError),
    /// Final cooling positive supply mass-flow guard summary could not resolve the bound unit.
    CalcCoolingSupplyMassFlowPositiveGuardLifecycle(
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardError,
    ),
    /// Final cooling positive-supply Cp-air assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCpAirAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentError,
    ),
    /// Final cooling positive-supply temperature assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyTemperatureAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError,
    ),
    /// Final cooling positive-supply temperature minimum-limit summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError,
    ),
    /// Final cooling positive-supply mixed-air-temperature limit summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError,
    ),
    /// Final cooling positive-supply mixed-air humidity-ratio assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentError,
    ),
    /// Final cooling positive-supply enthalpy assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentError,
    ),
    /// Final cooling positive-supply capacity-limit guard summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitGuardLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardError,
    ),
    /// Final cooling positive-supply capacity-limit Cp-air assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentError,
    ),
    /// Final cooling positive-supply capacity-limit sensible-output assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentError,
    ),
    /// Final cooling positive-supply capacity-limit sensible-output guard summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardError,
    ),
    /// Final cooling positive-supply sensible-output maximum-capacity assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentError,
    ),
    /// Final cooling positive-supply capacity-limit supply-enthalpy assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentError,
    ),
    /// Final cooling positive-supply capacity-limit supply-temperature assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentError,
    ),
    /// Final cooling positive-supply capacity-limit supply-temperature mixed-air-limit summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitError,
    ),
    /// Final post-capacity-limit mixed-air humidity-ratio assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentError,
    ),
    /// Final post-capacity-limit dehumidification-control switch summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchError,
    ),
    /// Final dehumidification-control None-case summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseError,
    ),
    /// Final constant-sensible-heat-ratio case-entry summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryError,
    ),
    /// Final constant-sensible-heat-ratio CpAir-assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentError,
    ),
    /// Final constant-sensible-heat-ratio sensible-output-assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentError,
    ),
    /// Final constant-sensible-heat-ratio total-output-assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentError,
    ),
    /// Final constant-sensible-heat-ratio supply-enthalpy-assignment summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentError,
    ),
    /// Final constant-SHR overdrying-limit summary could not resolve the bound unit.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycle(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitError,
    ),
    /// Final constant-SHR supply-humidity-ratio overdrying-limit summary could not resolve the bound unit.
    CalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycle(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitError,
    ),
    /// Final constant-SHR supply-humidity-ratio minimum-limit summary could not resolve the bound unit.
    CalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycle(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitError,
    ),
    /// Final constant-SHR supply-humidity-ratio mixed-air-limit summary could not resolve the bound unit.
    CalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycle(
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitError,
    ),
    /// Final constant-SHR case-break summary could not resolve the bound unit.
    CalcCoolingConstantShrCaseBreakLifecycle(PurchasedAirCalcCoolingConstantShrCaseBreakError),
    /// Final Humidistat case-entry summary could not resolve the bound unit.
    CalcCoolingHumidistatCaseEntryLifecycle(PurchasedAirCalcCoolingHumidistatCaseEntryError),
    /// Final Humidistat moisture-demand assignment summary could not resolve the bound unit.
    CalcCoolingHumidistatMoistureDemandAssignmentLifecycle(
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentError,
    ),
    /// Final Humidistat supply-humidity-ratio-for-dehumidification assignment summary could not resolve the bound unit.
    CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycle(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentError,
    ),
    /// Final Humidistat supply-humidity-ratio-for-dehumidification minimum-limit summary could not resolve the bound unit.
    CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycle(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitError,
    ),
    /// Final Humidistat purchased-air supply-humidity-ratio mixed-air-limit summary could not resolve the bound unit.
    CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycle(
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError,
    ),
    /// Final Humidistat case-break summary could not resolve the bound unit.
    CalcCoolingHumidistatCaseBreakLifecycle(PurchasedAirCalcCoolingHumidistatCaseBreakError),
    /// Final constant-supply-humidity-ratio case-entry summary could not resolve the bound unit.
    CalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycle(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryError,
    ),
    /// Final constant-supply-humidity-ratio assignment summary could not resolve the bound unit.
    CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycle(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentError,
    ),
    /// Final constant-supply-humidity-ratio case-break summary could not resolve the bound unit.
    CalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycle(
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakError,
    ),
    /// Final default supply-humidity-ratio mixed-air assignment summary could not resolve the bound unit.
    CalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycle(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentError,
    ),
    /// Final default supply-humidity-ratio case-break summary could not resolve the bound unit.
    CalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycle(
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakError,
    ),
    /// Final Cooling humidification heating-availability guard summary could not resolve the bound unit.
    CalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycle(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError,
    ),
    /// Final Cooling humidification-control Humidistat guard summary could not resolve the bound unit.
    CalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycle(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardError,
    ),
    /// Final nested dehumidification-control Humidistat-or-None guard summary could not resolve the bound unit.
    CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycle(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardError,
    ),
    /// Final humidifying-setpoint moisture-demand assignment summary could not resolve the bound unit.
    CalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycle(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentError,
    ),
    /// Final humidification supply-humidity-ratio assignment summary could not resolve the bound unit.
    CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycle(
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError,
    ),
    /// A lifecycle transition count did not match the single-environment run.
    InitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Calc-entry lifecycle transition did not match the executed run.
    CalcEntryLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A minimum-outdoor-air prefix lifecycle invariant did not match the run.
    CalcMinimumOaPrefixLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling-entry gate lifecycle invariant did not match the run.
    CalcCoolingEntryGateLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling OA maximum-flow gate lifecycle invariant did not match the run.
    CalcCoolingOaMaxFlowGateLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling OA maximum-flow body lifecycle invariant did not match the run.
    CalcCoolingOaMaxFlowBodyLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling economizer guard lifecycle invariant did not match the run.
    CalcCoolingEconomizerGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling economizer condition lifecycle invariant did not match the run.
    CalcCoolingEconomizerConditionLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling economizer true-body lifecycle invariant did not match the run.
    CalcCoolingEconomizerBodyLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling sensible-flow lifecycle invariant did not match the run.
    CalcCoolingSensibleFlowLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling dehumidification-flow lifecycle invariant did not match the run.
    CalcCoolingDehumidificationFlowLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling humidification-flow lifecycle invariant did not match the run.
    CalcCoolingHumidificationFlowLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling capacity-zero reset lifecycle invariant did not match the run.
    CalcCoolingCapacityZeroFlowResetLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow maximum lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowMaximumLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow EMS-override guard lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow EMS-override body lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow limit-guard lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowLimitGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow limit-body lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowLimitBodyLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow very-small guard lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowVerySmallGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling supply mass-flow positive-zero reset-body lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling mixed-air call lifecycle invariant did not match the run.
    CalcCoolingMixedAirCallLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive supply mass-flow guard lifecycle invariant did not match the run.
    CalcCoolingSupplyMassFlowPositiveGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply Cp-air assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCpAirAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply temperature assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyTemperatureAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply temperature minimum-limit lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply mixed-air-temperature limit lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply mixed-air humidity-ratio assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply enthalpy assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit guard lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit Cp-air assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit sensible-output assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit sensible-output guard lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply sensible-output maximum-capacity assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit supply-enthalpy assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit supply-temperature assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A cooling positive-supply capacity-limit supply-temperature mixed-air-limit lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A post-capacity-limit mixed-air humidity-ratio assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A post-capacity-limit dehumidification-control switch lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A dehumidification-control None-case lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-sensible-heat-ratio case-entry lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-sensible-heat-ratio CpAir-assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-sensible-heat-ratio sensible-output-assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-sensible-heat-ratio total-output-assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-sensible-heat-ratio supply-enthalpy-assignment lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-SHR overdrying-limit lifecycle invariant did not match the run.
    CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-SHR supply-humidity-ratio overdrying-limit lifecycle invariant did not match the run.
    CalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-SHR supply-humidity-ratio minimum-limit lifecycle invariant did not match the run.
    CalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-SHR supply-humidity-ratio mixed-air-limit lifecycle invariant did not match the run.
    CalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-SHR case-break lifecycle invariant did not match the run.
    CalcCoolingConstantShrCaseBreakLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Humidistat case-entry lifecycle invariant did not match the run.
    CalcCoolingHumidistatCaseEntryLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Humidistat moisture-demand assignment lifecycle invariant did not match the run.
    CalcCoolingHumidistatMoistureDemandAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Humidistat supply-humidity-ratio-for-dehumidification assignment lifecycle invariant did not match the run.
    CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Humidistat supply-humidity-ratio-for-dehumidification minimum-limit lifecycle invariant did not match the run.
    CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Humidistat purchased-air supply-humidity-ratio mixed-air-limit lifecycle invariant did not match the run.
    CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Humidistat case-break lifecycle invariant did not match the run.
    CalcCoolingHumidistatCaseBreakLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-supply-humidity-ratio case-entry lifecycle invariant did not match the run.
    CalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-supply-humidity-ratio assignment lifecycle invariant did not match the run.
    CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A constant-supply-humidity-ratio case-break lifecycle invariant did not match the run.
    CalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A default supply-humidity-ratio mixed-air assignment lifecycle invariant did not match the run.
    CalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A default supply-humidity-ratio case-break lifecycle invariant did not match the run.
    CalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Cooling humidification heating-availability guard lifecycle invariant did not match the run.
    CalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Cooling humidification-control Humidistat guard lifecycle invariant did not match the run.
    CalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A nested dehumidification-control Humidistat-or-None guard lifecycle invariant did not match the run.
    CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A humidifying-setpoint moisture-demand assignment lifecycle invariant did not match the run.
    CalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A humidification supply-humidity-ratio assignment lifecycle invariant did not match the run.
    CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleInvariant {
        /// Stable invariant field.
        field: &'static str,
        /// Required count or boolean-as-count.
        expected: usize,
        /// Observed count or boolean-as-count.
        actual: usize,
    },
    /// A Calc call did not retain the exact persistent initialization flags.
    UnexpectedInitializationFlags {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Calc-entry prefix snapshot did not match its bound release call.
    UnexpectedCalculationEntry {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A minimum-outdoor-air prefix snapshot did not match its bound release call.
    UnexpectedCalculationMinimumOutdoorAir {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling-entry gate snapshot did not match its bound release call.
    UnexpectedCalculationCoolingEntryGate {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling OA maximum-flow gate snapshot did not match its bound release call.
    UnexpectedCalculationCoolingOaMaxFlowGate {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling OA maximum-flow body snapshot did not match its bound release call.
    UnexpectedCalculationCoolingOaMaxFlowBody {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling economizer guard snapshot did not match its bound release call.
    UnexpectedCalculationCoolingEconomizerGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling economizer condition snapshot did not match its bound release call.
    UnexpectedCalculationCoolingEconomizerCondition {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling economizer true-body snapshot did not match its bound release call.
    UnexpectedCalculationCoolingEconomizerBody {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling sensible-flow snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSensibleFlow {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling dehumidification-flow snapshot did not match its bound release call.
    UnexpectedCalculationCoolingDehumidificationFlow {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling humidification-flow snapshot did not match its bound release call.
    UnexpectedCalculationCoolingHumidificationFlow {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling capacity-zero reset snapshot did not match its bound release call.
    UnexpectedCalculationCoolingCapacityZeroFlowReset {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow maximum snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowMaximum {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow EMS-override guard snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowEmsOverrideGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow EMS-override body snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowEmsOverrideBody {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow limit-guard snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowLimitGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow limit-body snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowLimitBody {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow very-small guard snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowVerySmallGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling supply mass-flow positive-zero reset-body snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowVerySmallGuardBody {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling mixed-air call snapshot did not match its bound release call.
    UnexpectedCalculationCoolingMixedAirCall {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive supply mass-flow guard snapshot did not match its bound release call.
    UnexpectedCalculationCoolingSupplyMassFlowPositiveGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply Cp-air assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCpAirAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply temperature assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyTemperatureAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply temperature minimum-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyTemperatureMinimumLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply mixed-air-temperature limit snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyTemperatureMixedAirLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply mixed-air humidity-ratio assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyHumidityRatioMixedAirAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply enthalpy assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyEnthalpyAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit guard snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit Cp-air assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitCpAirAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit sensible-output assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit sensible-output guard snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply sensible-output maximum-capacity assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit supply-enthalpy assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit supply-temperature assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A cooling positive-supply capacity-limit supply-temperature mixed-air-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A post-capacity-limit mixed-air humidity-ratio assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A post-capacity-limit dehumidification-control switch snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitch {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A dehumidification-control None-case snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCase {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-sensible-heat-ratio case-entry snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntry {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-sensible-heat-ratio CpAir-assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-sensible-heat-ratio sensible-output-assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-sensible-heat-ratio total-output-assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-sensible-heat-ratio supply-enthalpy-assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-SHR overdrying-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-SHR supply-humidity-ratio overdrying-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioOverdryingLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-SHR supply-humidity-ratio minimum-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioMinimumLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-SHR supply-humidity-ratio mixed-air-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioMixedAirLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-SHR case-break snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantShrCaseBreak {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Humidistat case-entry snapshot did not match its release call.
    UnexpectedCalculationCoolingHumidistatCaseEntry {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Humidistat moisture-demand assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingHumidistatMoistureDemandAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Humidistat supply-humidity-ratio-for-dehumidification assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Humidistat supply-humidity-ratio-for-dehumidification minimum-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Humidistat purchased-air supply-humidity-ratio mixed-air-limit snapshot did not match its release call.
    UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioMixedAirLimit {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Humidistat case-break snapshot did not match its release call.
    UnexpectedCalculationCoolingHumidistatCaseBreak {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-supply-humidity-ratio case-entry snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantSupplyHumidityRatioCaseEntry {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-supply-humidity-ratio assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantSupplyHumidityRatioAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A constant-supply-humidity-ratio case-break snapshot did not match its release call.
    UnexpectedCalculationCoolingConstantSupplyHumidityRatioCaseBreak {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A default supply-humidity-ratio mixed-air assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingDefaultSupplyHumidityRatioMixedAirAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A default supply-humidity-ratio case-break snapshot did not match its release call.
    UnexpectedCalculationCoolingDefaultSupplyHumidityRatioCaseBreak {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Cooling humidification heating-availability guard snapshot did not match its release call.
    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A Cooling humidification-control Humidistat guard snapshot did not match its release call.
    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationControlHumidistatGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A nested dehumidification-control Humidistat-or-None guard snapshot did not match its release call.
    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuard {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A humidifying-setpoint moisture-demand assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A humidification supply-humidity-ratio assignment snapshot did not match its release call.
    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignment {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
    },
    /// A successful CP301 call did not retain source-setpoint demand provenance.
    UnexpectedDemandInputKind {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
        /// Unexpected demand kind.
        actual: ZoneSensibleDemandInputKind,
    },
    /// A timestep dispatched a PurchasedAir branch different from the immutable binding.
    UnexpectedPurchasedAirBranch {
        /// Zero-based nominal system-step index.
        timestep_index: usize,
        /// Branch retained by the immutable binding.
        expected: IdealLoadsPurchasedAirBranch,
        /// Branch returned by the generic PurchasedAir wrapper.
        actual: IdealLoadsPurchasedAirBranch,
    },
    /// Hourly PurchasedAir aggregation rejected the collected outputs.
    HourlyOutput(DirectZonePurchasedAirHourlyOutputError),
}

impl Display for DirectZonePurchasedAirCoupledRuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binding(error) => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir binding failed: {error:?}"
                )
            }
            Self::HeatBalance(error) => Display::fmt(error, formatter),
            Self::NoTimestepsRequested => write!(
                formatter,
                "direct-Zone PurchasedAir requires at least one system timestep"
            ),
            Self::ScheduleCacheCoverage {
                required,
                available,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir schedule cache requires {required} zone-timestep samples but contains {available}"
            ),
            Self::TimestepCountOverflow => write!(
                formatter,
                "direct-Zone PurchasedAir requested timestep count overflowed usize"
            ),
            Self::RuntimeStep(error) => write!(
                formatter,
                "direct-Zone PurchasedAir predictor step failed: {error:?}"
            ),
            Self::InitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir lifecycle summary failed: {error:?}"
            ),
            Self::CalcEntryLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Calc-entry lifecycle summary failed: {error:?}"
            ),
            Self::CalcMinimumOaPrefixLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir minimum-OA prefix lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingEntryGateLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling-entry gate lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingOaMaxFlowGateLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling OA maximum-flow gate lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingOaMaxFlowBodyLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling OA maximum-flow body lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingEconomizerGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling economizer guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingEconomizerConditionLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling economizer condition lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingEconomizerBodyLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling economizer body lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSensibleFlowLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling sensible-flow lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingDehumidificationFlowLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling dehumidification-flow lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidificationFlowLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling humidification-flow lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingCapacityZeroFlowResetLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling capacity-zero reset lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowMaximumLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow maximum lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowEmsOverrideGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow EMS-override guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowEmsOverrideBodyLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow EMS-override body lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowLimitGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow limit-guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowLimitBodyLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow limit-body lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowVerySmallGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow very-small guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow positive-zero reset-body lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingMixedAirCallLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling mixed-air call lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyMassFlowPositiveGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive supply mass-flow guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCpAirAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply Cp-air assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyTemperatureAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply temperature assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply temperature minimum-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply mixed-air-temperature limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycle(error) => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir cooling positive-supply mixed-air humidity-ratio assignment lifecycle summary failed: {error:?}"
                )
            }
            Self::CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply enthalpy assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit Cp-air assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit sensible-output assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit sensible-output guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply sensible-output maximum-capacity assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit supply-enthalpy assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit supply-temperature assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit supply-temperature mixed-air-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir post-capacity-limit mixed-air humidity-ratio assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir post-capacity-limit dehumidification-control switch lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir dehumidification-control None-case lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio case-entry lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio CpAir-assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio sensible-output-assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio total-output-assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio supply-enthalpy-assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio overdrying-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR supply-humidity-ratio overdrying-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR supply-humidity-ratio minimum-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR supply-humidity-ratio mixed-air-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantShrCaseBreakLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR case-break lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidistatCaseEntryLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat case-entry lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidistatMoistureDemandAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat moisture-demand assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat supply-humidity-ratio-for-dehumidification assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat supply-humidity-ratio-for-dehumidification minimum-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat purchased-air supply-humidity-ratio mixed-air-limit lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingHumidistatCaseBreakLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat case-break lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-supply-humidity-ratio case-entry lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-supply-humidity-ratio assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir constant-supply-humidity-ratio case-break lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir default supply-humidity-ratio mixed-air assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir default supply-humidity-ratio case-break lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Cooling humidification heating-availability guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir Cooling humidification-control Humidistat guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir nested dehumidification-control Humidistat-or-None guard lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir humidifying-setpoint moisture-demand assignment lifecycle summary failed: {error:?}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycle(error) => write!(
                formatter,
                "direct-Zone PurchasedAir humidification supply-humidity-ratio assignment lifecycle summary failed: {error:?}"
            ),
            Self::InitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcEntryLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Calc-entry lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcMinimumOaPrefixLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir minimum-OA prefix lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingEntryGateLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling-entry gate lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingOaMaxFlowGateLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling OA maximum-flow gate lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingOaMaxFlowBodyLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling OA maximum-flow body lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingEconomizerGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling economizer guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingEconomizerConditionLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling economizer condition lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingEconomizerBodyLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling economizer body lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSensibleFlowLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling sensible-flow lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingDehumidificationFlowLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling dehumidification-flow lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidificationFlowLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling humidification-flow lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingCapacityZeroFlowResetLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling capacity-zero reset lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowMaximumLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow maximum lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow EMS-override guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow EMS-override body lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowLimitGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow limit-guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowLimitBodyLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow limit-body lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowVerySmallGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow very-small guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling supply mass-flow positive-zero reset-body lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingMixedAirCallLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling mixed-air call lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyMassFlowPositiveGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive supply mass-flow guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCpAirAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply Cp-air assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyTemperatureAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply temperature assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply temperature minimum-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply mixed-air-temperature limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply mixed-air humidity-ratio assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply enthalpy assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit Cp-air assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit sensible-output assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit sensible-output guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply sensible-output maximum-capacity assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit supply-enthalpy assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit supply-temperature assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir cooling positive-supply capacity-limit supply-temperature mixed-air-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir post-capacity-limit mixed-air humidity-ratio assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir post-capacity-limit dehumidification-control switch lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir dehumidification-control None-case lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio case-entry lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio CpAir-assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio sensible-output-assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio total-output-assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio supply-enthalpy-assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-sensible-heat-ratio overdrying-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR supply-humidity-ratio overdrying-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR supply-humidity-ratio minimum-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR supply-humidity-ratio mixed-air-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantShrCaseBreakLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-SHR case-break lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidistatCaseEntryLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat case-entry lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidistatMoistureDemandAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat moisture-demand assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat supply-humidity-ratio-for-dehumidification assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat supply-humidity-ratio-for-dehumidification minimum-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat purchased-air supply-humidity-ratio mixed-air-limit lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingHumidistatCaseBreakLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Humidistat case-break lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-supply-humidity-ratio case-entry lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-supply-humidity-ratio assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir constant-supply-humidity-ratio case-break lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir default supply-humidity-ratio mixed-air assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir default supply-humidity-ratio case-break lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Cooling humidification heating-availability guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir Cooling humidification-control Humidistat guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir nested dehumidification-control Humidistat-or-None guard lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir humidifying-setpoint moisture-demand assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycleInvariant {
                field,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir humidification supply-humidity-ratio assignment lifecycle invariant {field} expected {expected}, got {actual}"
            ),
            Self::UnexpectedInitializationFlags { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not consume its persistent initialization flags"
            ),
            Self::UnexpectedCalculationEntry { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its bound Calc-entry prefix"
            ),
            Self::UnexpectedCalculationMinimumOutdoorAir { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its minimum-OA prefix"
            ),
            Self::UnexpectedCalculationCoolingEntryGate { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling-entry gate"
            ),
            Self::UnexpectedCalculationCoolingOaMaxFlowGate { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling OA maximum-flow gate"
            ),
            Self::UnexpectedCalculationCoolingOaMaxFlowBody { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling OA maximum-flow body"
            ),
            Self::UnexpectedCalculationCoolingEconomizerGuard { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling economizer guard"
            ),
            Self::UnexpectedCalculationCoolingEconomizerCondition { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling economizer condition"
            ),
            Self::UnexpectedCalculationCoolingEconomizerBody { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling economizer body"
            ),
            Self::UnexpectedCalculationCoolingSensibleFlow { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling sensible-flow calculation"
            ),
            Self::UnexpectedCalculationCoolingDehumidificationFlow { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling dehumidification-flow calculation"
            ),
            Self::UnexpectedCalculationCoolingHumidificationFlow { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling humidification-flow calculation"
            ),
            Self::UnexpectedCalculationCoolingCapacityZeroFlowReset { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling capacity-zero reset"
            ),
            Self::UnexpectedCalculationCoolingSupplyMassFlowMaximum { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow maximum"
            ),
            Self::UnexpectedCalculationCoolingSupplyMassFlowEmsOverrideGuard { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow EMS-override guard"
                )
            }
            Self::UnexpectedCalculationCoolingSupplyMassFlowEmsOverrideBody { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow EMS-override body"
                )
            }
            Self::UnexpectedCalculationCoolingSupplyMassFlowLimitGuard { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow limit guard"
                )
            }
            Self::UnexpectedCalculationCoolingSupplyMassFlowLimitBody { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow limit body"
                )
            }
            Self::UnexpectedCalculationCoolingSupplyMassFlowVerySmallGuard { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow very-small guard"
                )
            }
            Self::UnexpectedCalculationCoolingSupplyMassFlowVerySmallGuardBody {
                timestep_index,
            } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling supply mass-flow positive-zero reset body"
                )
            }
            Self::UnexpectedCalculationCoolingMixedAirCall { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling mixed-air call"
                )
            }
            Self::UnexpectedCalculationCoolingSupplyMassFlowPositiveGuard { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive supply mass-flow guard"
                )
            }
            Self::UnexpectedCalculationCoolingPositiveSupplyCpAirAssignment { timestep_index } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply Cp-air assignment"
                )
            }
            Self::UnexpectedCalculationCoolingPositiveSupplyTemperatureAssignment {
                timestep_index,
            } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply temperature assignment"
                )
            }
            Self::UnexpectedCalculationCoolingPositiveSupplyTemperatureMinimumLimit {
                timestep_index,
            } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply temperature minimum limit"
                )
            }
            Self::UnexpectedCalculationCoolingPositiveSupplyTemperatureMixedAirLimit {
                timestep_index,
            } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply mixed-air-temperature limit"
                )
            }
            Self::UnexpectedCalculationCoolingPositiveSupplyHumidityRatioMixedAirAssignment {
                timestep_index,
            } => {
                write!(
                    formatter,
                    "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply mixed-air humidity-ratio assignment"
                )
            }
            Self::UnexpectedCalculationCoolingPositiveSupplyEnthalpyAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply enthalpy assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitGuard {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit guard"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitCpAirAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit Cp-air assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit sensible-output assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputGuard {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit sensible-output guard"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply sensible-output maximum-capacity assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit supply-enthalpy assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit supply-temperature assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its cooling positive-supply capacity-limit supply-temperature mixed-air limit"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its post-capacity-limit mixed-air humidity-ratio assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitch {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its post-capacity-limit dehumidification-control switch dispatch"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCase {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its dehumidification-control None case"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntry {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-sensible-heat-ratio case entry"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-sensible-heat-ratio CpAir assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-sensible-heat-ratio sensible-output assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-sensible-heat-ratio total-output assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-sensible-heat-ratio supply-enthalpy assignment"
            ),
            Self::UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-sensible-heat-ratio overdrying limit"
            ),
            Self::UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioOverdryingLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-SHR supply-humidity-ratio overdrying limit"
            ),
            Self::UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioMinimumLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-SHR supply-humidity-ratio minimum limit"
            ),
            Self::UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioMixedAirLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-SHR supply-humidity-ratio mixed-air limit"
            ),
            Self::UnexpectedCalculationCoolingConstantShrCaseBreak { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-SHR case break"
            ),
            Self::UnexpectedCalculationCoolingHumidistatCaseEntry { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Humidistat case entry"
            ),
            Self::UnexpectedCalculationCoolingHumidistatMoistureDemandAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Humidistat moisture-demand assignment"
            ),
            Self::UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Humidistat supply-humidity-ratio-for-dehumidification assignment"
            ),
            Self::UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Humidistat supply-humidity-ratio-for-dehumidification minimum limit"
            ),
            Self::UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioMixedAirLimit {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Humidistat purchased-air supply-humidity-ratio mixed-air limit"
            ),
            Self::UnexpectedCalculationCoolingHumidistatCaseBreak { timestep_index } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Humidistat case break"
            ),
            Self::UnexpectedCalculationCoolingConstantSupplyHumidityRatioCaseEntry {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-supply-humidity-ratio case entry"
            ),
            Self::UnexpectedCalculationCoolingConstantSupplyHumidityRatioAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-supply-humidity-ratio assignment"
            ),
            Self::UnexpectedCalculationCoolingConstantSupplyHumidityRatioCaseBreak {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its constant-supply-humidity-ratio case break"
            ),
            Self::UnexpectedCalculationCoolingDefaultSupplyHumidityRatioMixedAirAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its default supply-humidity-ratio mixed-air assignment"
            ),
            Self::UnexpectedCalculationCoolingDefaultSupplyHumidityRatioCaseBreak {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its default supply-humidity-ratio case break"
            ),
            Self::UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuard {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Cooling humidification heating-availability guard"
            ),
            Self::UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationControlHumidistatGuard {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its Cooling humidification-control Humidistat guard"
            ),
            Self::UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuard {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its nested dehumidification-control Humidistat-or-None guard"
            ),
            Self::UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its humidifying-setpoint moisture-demand assignment"
            ),
            Self::UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignment {
                timestep_index,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} did not retain its humidification supply-humidity-ratio assignment"
            ),
            Self::UnexpectedDemandInputKind {
                timestep_index,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} produced unexpected demand input kind {actual:?}"
            ),
            Self::UnexpectedPurchasedAirBranch {
                timestep_index,
                expected,
                actual,
            } => write!(
                formatter,
                "direct-Zone PurchasedAir timestep {timestep_index} dispatched branch {actual:?}, expected bound branch {expected:?}"
            ),
            Self::HourlyOutput(error) => write!(
                formatter,
                "direct-Zone PurchasedAir hourly output aggregation failed: {error:?}"
            ),
        }
    }
}

impl std::error::Error for DirectZonePurchasedAirCoupledRuntimeError {}

/// Executes the exact one-Zone/no-OA sensible subset through a shared fixed
/// ThirdOrder heat-balance and PurchasedAir loop.
///
/// The caller must supply the zone-timestep schedule cache built from the same
/// `SimulationModel` and active environment axis. Binding is performed once;
/// CP301 is then called exactly once inside each `PredictSystemLoads` step, and
/// the existing corrector consumes the committed `SumSysMCp`/`SumSysMCpT` in
/// that same timestep.
pub fn simulate_direct_zone_purchased_air_coupled_heat_balance(
    model: &SimulationModel,
    weather_series: &WeatherTimestepSeries,
    coupling_schedule_cache: &ScheduleSeriesCache,
    options: DirectZonePurchasedAirCoupledOptions,
) -> Result<DirectZonePurchasedAirCoupledSimulation, DirectZonePurchasedAirCoupledRuntimeError> {
    let weather_dry_bulb_c = weather_series.hourly_dry_bulb_c();
    if weather_dry_bulb_c.is_empty() {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::NoWeatherData,
        ));
    }
    if options.sample_count == 0 {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::NoTimestepsRequested);
    }
    if options.sample_count > weather_dry_bulb_c.len() {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::SampleCountExceedsWeather {
                requested: options.sample_count,
                available: weather_dry_bulb_c.len(),
            },
        ));
    }
    if model.typed.zones.is_empty() {
        return Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::NoZones,
        ));
    }

    let binding = bind_direct_zone_purchased_air_model(model)
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::Binding)?;
    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour;
    let required_timestep_count = options
        .sample_count
        .checked_mul(zone_steps_per_hour as usize)
        .ok_or(DirectZonePurchasedAirCoupledRuntimeError::TimestepCountOverflow)?;
    if coupling_schedule_cache.sample_count() < required_timestep_count {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::ScheduleCacheCoverage {
                required: required_timestep_count,
                available: coupling_schedule_cache.sample_count(),
            },
        );
    }
    let seconds_per_timestep = SECONDS_PER_HOUR / f64::from(zone_steps_per_hour);
    let first_hour_interpolation_starting_values =
        run_period_first_hour_interpolation_starting_values(&model.typed);
    let runtime_config = direct_zone_purchased_air_fixed_step_runtime_config();
    validate_fixed_runtime_config(runtime_config);

    let heat_balance_options = HeatBalanceSimulationOptions {
        sample_count: options.sample_count,
        initial_zone_air_temperature_c: options.initial_zone_air_temperature_c,
        ..HeatBalanceSimulationOptions::hourly_samples(options.sample_count)
    };
    let (mut state, internal_gain_schedule_cache, mut internal_gain_schedule_cache_profile) =
        init_heat_balance_source_order_path(|| {
            let (schedule_cache, mut schedule_cache_profile) =
                precompute_hour_only_internal_gain_schedule_cache_profiled(&model.typed)
                    .map_err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance)?;
            let mut state =
                initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled(
                    model,
                    options.initial_zone_air_temperature_c,
                    &[],
                    &schedule_cache,
                    &mut schedule_cache_profile,
                )
                .map_err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance)?;
            seed_zone_air_humidity_ratios_from_weather_series(
                &mut state,
                Some(weather_series),
                weather_dry_bulb_c[0],
                zone_steps_per_hour,
                first_hour_interpolation_starting_values,
            );
            match heat_balance_options.ctf_initial_history_policy {
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue => {
                    seed_initial_surface_ctf_boundary_histories(&mut state, weather_dry_bulb_c[0]);
                }
                HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial => {
                    seed_energyplus_initial_surface_ctf_histories(
                        &mut state,
                        options.initial_zone_air_temperature_c,
                        weather_dry_bulb_c[0],
                    );
                }
            }
            Ok::<_, DirectZonePurchasedAirCoupledRuntimeError>((
                state,
                schedule_cache,
                schedule_cache_profile,
            ))
        })?;
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();

    let (samples, timestep_outputs) = sample_heat_balance_run_period_with_step_driver(
        model,
        &mut state,
        weather_dry_bulb_c,
        Some(weather_series.hourly_records()),
        Some(weather_series),
        heat_balance_options,
        runtime_config,
        zone_steps_per_hour,
        seconds_per_timestep,
        first_hour_interpolation_starting_values,
        |state, input, weather_context, hour_index, substep| {
            let sample_index =
                hour_index * zone_steps_per_hour as usize + (substep.saturating_sub(1) as usize);
            advance_heat_balance_state_one_timestep_with_direct_zone_purchased_air(
                &model.typed,
                &internal_gain_schedule_cache,
                &mut internal_gain_schedule_cache_profile.run_period,
                state,
                input,
                weather_context,
                runtime_config,
                heat_balance_options.surface_iteration_count,
                heat_balance_options.inside_hconv_reevaluation_interval,
                heat_balance_options.surface_loop_zone_air_correction,
                &binding,
                &mut purchased_air_runtime_state,
                sample_index == 0,
                coupling_schedule_cache,
                sample_index,
            )
            .map_err(DirectZonePurchasedAirCoupledRuntimeError::RuntimeStep)
        },
    )?;

    for (timestep_index, output) in timestep_outputs.iter().enumerate() {
        if !calc_entry_snapshot_matches_release(output, timestep_index + 1, &binding) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationEntry {
                    timestep_index,
                },
            );
        }
        if !minimum_oa_validation::snapshot_matches_release(output, timestep_index + 1, &binding) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationMinimumOutdoorAir {
                    timestep_index,
                },
            );
        }
        if !cooling_entry_validation::snapshot_matches_release(output, timestep_index + 1, &binding)
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationCoolingEntryGate {
                    timestep_index,
                },
            );
        }
        if !cooling_oa_max_flow_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationCoolingOaMaxFlowGate {
                    timestep_index,
                },
            );
        }
        if !cooling_oa_max_flow_body_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationCoolingOaMaxFlowBody {
                    timestep_index,
                },
            );
        }
        if !cooling_economizer_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationCoolingEconomizerGuard {
                    timestep_index,
                },
            );
        }
        if !cooling_economizer_condition_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedCalculationCoolingEconomizerCondition {
                    timestep_index,
                },
            );
        }
        if !cooling_economizer_body_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingEconomizerBody { timestep_index },
            );
        }
        if !cooling_sensible_flow_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSensibleFlow { timestep_index },
            );
        }
        if !cooling_dehumidification_flow_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingDehumidificationFlow { timestep_index },
            );
        }
        if !cooling_humidification_flow_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidificationFlow { timestep_index },
            );
        }
        if !cooling_capacity_zero_flow_reset_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingCapacityZeroFlowReset { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_maximum_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowMaximum { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_ems_override_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowEmsOverrideGuard { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_ems_override_body_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowEmsOverrideBody { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_limit_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowLimitGuard { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_limit_body_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowLimitBody { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_very_small_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowVerySmallGuard { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_very_small_guard_body_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowVerySmallGuardBody {
                        timestep_index,
                    },
            );
        }
        if !output.initialization.flags.state_machine_used
            || output.coupling.purchased_air.init_flags != output.initialization.flags
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedInitializationFlags {
                    timestep_index,
                },
            );
        }
        if !cooling_mixed_air_call_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingMixedAirCall { timestep_index },
            );
        }
        if !cooling_supply_mass_flow_positive_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyMassFlowPositiveGuard { timestep_index },
            );
        }
        if !cooling_positive_supply_cp_air_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCpAirAssignment { timestep_index },
            );
        }
        if !cooling_positive_supply_temperature_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyTemperatureAssignment {
                        timestep_index,
                },
            );
        }
        if !cooling_positive_supply_temperature_minimum_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyTemperatureMinimumLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_temperature_mixed_air_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyTemperatureMixedAirLimit {
                        timestep_index,
                },
            );
        }
        if !cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyHumidityRatioMixedAirAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_enthalpy_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyEnthalpyAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_capacity_limit_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitGuard {
                        timestep_index,
                    },
            );
        }
        let actual_branch = output.coupling.purchased_air.branch;
        if actual_branch != binding.branch {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedPurchasedAirBranch {
                    timestep_index,
                    expected: binding.branch,
                    actual: actual_branch,
                },
            );
        }
        if !cooling_positive_supply_capacity_limit_cp_air_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitCpAirAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_capacity_limit_sensible_output_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputAssignment {
                        timestep_index,
                    },
            );
        }
        let actual = output
            .coupling
            .purchased_air
            .trace
            .demand
            .sensible_input_kind;
        if actual != ZoneSensibleDemandInputKind::SourceSetpointThresholds {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::UnexpectedDemandInputKind {
                    timestep_index,
                    actual,
                },
            );
        }
        if !cooling_positive_supply_capacity_limit_sensible_output_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputGuard {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitch {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCase {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntry {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_shr_supply_humidity_ratio_overdrying_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioOverdryingLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_shr_supply_humidity_ratio_minimum_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioMinimumLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantShrSupplyHumidityRatioMixedAirLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_shr_case_break_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantShrCaseBreak {
                        timestep_index,
                    },
            );
        }
        if !cooling_humidistat_case_entry_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidistatCaseEntry {
                        timestep_index,
                    },
            );
        }
        if !cooling_humidistat_moisture_demand_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidistatMoistureDemandAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::
            snapshot_matches_release(output, timestep_index + 1, &binding)
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_validation::
            snapshot_matches_release(output, timestep_index + 1, &binding)
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_humidistat_supply_humidity_ratio_mixed_air_limit_validation::
            snapshot_matches_release(output, timestep_index + 1, &binding)
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidistatSupplyHumidityRatioMixedAirLimit {
                        timestep_index,
                    },
            );
        }
        if !cooling_humidistat_case_break_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingHumidistatCaseBreak {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_supply_humidity_ratio_case_entry_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantSupplyHumidityRatioCaseEntry {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_supply_humidity_ratio_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantSupplyHumidityRatioAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_constant_supply_humidity_ratio_case_break_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingConstantSupplyHumidityRatioCaseBreak {
                        timestep_index,
                    },
            );
        }
        if !cooling_default_supply_humidity_ratio_mixed_air_assignment_validation::
            snapshot_matches_release(output, timestep_index + 1, &binding)
        {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingDefaultSupplyHumidityRatioMixedAirAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_default_supply_humidity_ratio_case_break_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingDefaultSupplyHumidityRatioCaseBreak {
                        timestep_index,
                    },
            );
        }
        if !cooling_supply_humidity_ratio_humidification_heating_availability_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuard {
                        timestep_index,
                    },
            );
        }
        if !cooling_supply_humidity_ratio_humidification_control_humidistat_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationControlHumidistatGuard {
                        timestep_index,
                    },
            );
        }
        if !cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuard {
                        timestep_index,
                    },
            );
        }
        if !cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignment {
                        timestep_index,
                    },
            );
        }
        if !cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_validation::snapshot_matches_release(
            output,
            timestep_index + 1,
            &binding,
        ) {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::
                    UnexpectedCalculationCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignment {
                        timestep_index,
                    },
            );
        }
    }
    let init_lifecycle = purchased_air_init_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::InitLifecycle)?;
    validate_init_lifecycle(&init_lifecycle, timestep_outputs.len(), &binding)?;
    let calc_entry_lifecycle = purchased_air_calc_entry_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycle)?;
    let latest_output = timestep_outputs.last().ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
            field: "latest_output_present",
            expected: 1,
            actual: 0,
        },
    )?;
    validate_calc_entry_lifecycle(
        &calc_entry_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_minimum_oa_prefix_lifecycle = purchased_air_calc_minimum_oa_prefix_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcMinimumOaPrefixLifecycle)?;
    minimum_oa_validation::validate_lifecycle(
        &calc_minimum_oa_prefix_lifecycle,
        &calc_entry_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_entry_gate_lifecycle =
        purchased_air_calc_cooling_entry_gate_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEntryGateLifecycle)?;
    let numerical_cooling_count = timestep_outputs
        .iter()
        .filter(|output| {
            output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling
        })
        .count();
    cooling_entry_validation::validate_lifecycle(
        &calc_cooling_entry_gate_lifecycle,
        &calc_minimum_oa_prefix_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_oa_max_flow_gate_lifecycle =
        purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingOaMaxFlowGateLifecycle)?;
    cooling_oa_max_flow_validation::validate_lifecycle(
        &calc_cooling_oa_max_flow_gate_lifecycle,
        &calc_cooling_entry_gate_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_oa_max_flow_body_lifecycle =
        purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingOaMaxFlowBodyLifecycle)?;
    cooling_oa_max_flow_body_validation::validate_lifecycle(
        &calc_cooling_oa_max_flow_body_lifecycle,
        &calc_cooling_oa_max_flow_gate_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_economizer_guard_lifecycle =
        purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEconomizerGuardLifecycle)?;
    cooling_economizer_guard_validation::validate_lifecycle(
        &calc_cooling_economizer_guard_lifecycle,
        &calc_cooling_oa_max_flow_body_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_economizer_condition_lifecycle =
        purchased_air_calc_cooling_economizer_condition_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEconomizerConditionLifecycle,
        )?;
    cooling_economizer_condition_validation::validate_lifecycle(
        &calc_cooling_economizer_condition_lifecycle,
        &calc_cooling_economizer_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_economizer_body_lifecycle =
        purchased_air_calc_cooling_economizer_body_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEconomizerBodyLifecycle)?;
    cooling_economizer_body_validation::validate_lifecycle(
        &calc_cooling_economizer_body_lifecycle,
        &calc_cooling_economizer_condition_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_sensible_flow_lifecycle =
        purchased_air_calc_cooling_sensible_flow_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingSensibleFlowLifecycle)?;
    cooling_sensible_flow_validation::validate_lifecycle(
        &calc_cooling_sensible_flow_lifecycle,
        &calc_cooling_economizer_body_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_dehumidification_flow_lifecycle =
        purchased_air_calc_cooling_dehumidification_flow_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingDehumidificationFlowLifecycle,
        )?;
    cooling_dehumidification_flow_validation::validate_lifecycle(
        &calc_cooling_dehumidification_flow_lifecycle,
        &calc_cooling_sensible_flow_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_humidification_flow_lifecycle =
        purchased_air_calc_cooling_humidification_flow_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingHumidificationFlowLifecycle,
        )?;
    cooling_humidification_flow_validation::validate_lifecycle(
        &calc_cooling_humidification_flow_lifecycle,
        &calc_cooling_dehumidification_flow_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_capacity_zero_flow_reset_lifecycle =
        purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingCapacityZeroFlowResetLifecycle,
        )?;
    cooling_capacity_zero_flow_reset_validation::validate_lifecycle(
        &calc_cooling_capacity_zero_flow_reset_lifecycle,
        &calc_cooling_humidification_flow_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_maximum_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_maximum_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingSupplyMassFlowMaximumLifecycle,
        )?;
    cooling_supply_mass_flow_maximum_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_maximum_lifecycle,
        &calc_cooling_capacity_zero_flow_reset_lifecycle,
        &calc_minimum_oa_prefix_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_ems_override_guard_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_ems_override_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyMassFlowEmsOverrideGuardLifecycle,
        )?;
    cooling_supply_mass_flow_ems_override_guard_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_ems_override_guard_lifecycle,
        &calc_cooling_supply_mass_flow_maximum_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_ems_override_body_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyMassFlowEmsOverrideBodyLifecycle,
        )?;
    cooling_supply_mass_flow_ems_override_body_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_ems_override_body_lifecycle,
        &calc_cooling_supply_mass_flow_ems_override_guard_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_limit_guard_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingSupplyMassFlowLimitGuardLifecycle,
        )?;
    cooling_supply_mass_flow_limit_guard_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_limit_guard_lifecycle,
        &calc_cooling_supply_mass_flow_ems_override_body_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_limit_body_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingSupplyMassFlowLimitBodyLifecycle,
        )?;
    cooling_supply_mass_flow_limit_body_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_limit_body_lifecycle,
        &calc_cooling_supply_mass_flow_limit_guard_lifecycle,
        timestep_outputs.len(),
        numerical_cooling_count,
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_very_small_guard_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyMassFlowVerySmallGuardLifecycle,
        )?;
    cooling_supply_mass_flow_very_small_guard_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_very_small_guard_lifecycle,
        &calc_cooling_supply_mass_flow_limit_body_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycle,
        )?;
    cooling_supply_mass_flow_very_small_guard_body_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle,
        &calc_cooling_supply_mass_flow_very_small_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_mixed_air_call_lifecycle =
        purchased_air_calc_cooling_mixed_air_call_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingMixedAirCallLifecycle)?;
    cooling_mixed_air_call_validation::validate_lifecycle(
        &calc_cooling_mixed_air_call_lifecycle,
        &calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_mass_flow_positive_guard_lifecycle =
        purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyMassFlowPositiveGuardLifecycle,
        )?;
    cooling_supply_mass_flow_positive_guard_validation::validate_lifecycle(
        &calc_cooling_supply_mass_flow_positive_guard_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_cp_air_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_cp_air_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCpAirAssignmentLifecycle,
        )?;
    cooling_positive_supply_cp_air_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_cp_air_assignment_lifecycle,
        &calc_cooling_supply_mass_flow_positive_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_temperature_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_temperature_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyTemperatureAssignmentLifecycle,
        )?;
    cooling_positive_supply_temperature_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_temperature_assignment_lifecycle,
        &calc_cooling_positive_supply_cp_air_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_temperature_minimum_limit_lifecycle =
        purchased_air_calc_cooling_positive_supply_temperature_minimum_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyTemperatureMinimumLimitLifecycle,
        )?;
    cooling_positive_supply_temperature_minimum_limit_validation::validate_lifecycle(
        &calc_cooling_positive_supply_temperature_minimum_limit_lifecycle,
        &calc_cooling_positive_supply_temperature_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle =
        purchased_air_calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyTemperatureMixedAirLimitLifecycle,
        )?;
    cooling_positive_supply_temperature_mixed_air_limit_validation::validate_lifecycle(
        &calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
        &calc_cooling_positive_supply_temperature_minimum_limit_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentLifecycle,
        )?;
    cooling_positive_supply_humidity_ratio_mixed_air_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_enthalpy_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyEnthalpyAssignmentLifecycle,
        )?;
    cooling_positive_supply_enthalpy_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
        &calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_guard_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitGuardLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_guard_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
        &calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_cp_air_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_sensible_output_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
        &calc_cooling_supply_mass_flow_positive_guard_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        &calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_sensible_output_guard_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle,
        &calc_cooling_capacity_zero_flow_reset_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle,
        &calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
        &calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitLifecycle,
        )?;
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_validation::validate_lifecycle(
        &calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        &calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        &calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_supply_mass_flow_positive_guard_lifecycle,
        &calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
        &calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_dehumidification_flow_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle,
        &calc_cooling_mixed_air_call_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle =
        purchased_air_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitLifecycle,
        )?;
    cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_validation::validate_lifecycle(
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle =
        purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitLifecycle,
        )?;
    cooling_constant_shr_supply_humidity_ratio_overdrying_limit_validation::validate_lifecycle(
        &calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle,
        &calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle =
        purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingConstantShrSupplyHumidityRatioMinimumLimitLifecycle,
        )?;
    cooling_constant_shr_supply_humidity_ratio_minimum_limit_validation::validate_lifecycle(
        &calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle,
        &calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle =
        purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitLifecycle,
        )?;
    cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_validation::validate_lifecycle(
        &calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle,
        &calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_shr_case_break_lifecycle =
        purchased_air_calc_cooling_constant_shr_case_break_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingConstantShrCaseBreakLifecycle,
        )?;
    cooling_constant_shr_case_break_validation::validate_lifecycle(
        &calc_cooling_constant_shr_case_break_lifecycle,
        &calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_humidistat_case_entry_lifecycle =
        purchased_air_calc_cooling_humidistat_case_entry_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingHumidistatCaseEntryLifecycle,
        )?;
    cooling_humidistat_case_entry_validation::validate_lifecycle(
        &calc_cooling_humidistat_case_entry_lifecycle,
        &calc_cooling_constant_shr_case_break_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_humidistat_moisture_demand_assignment_lifecycle =
        purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingHumidistatMoistureDemandAssignmentLifecycle,
        )?;
    cooling_humidistat_moisture_demand_assignment_validation::validate_lifecycle(
        &calc_cooling_humidistat_moisture_demand_assignment_lifecycle,
        &calc_cooling_humidistat_case_entry_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle =
        purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentLifecycle,
        )?;
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_validation::
        validate_lifecycle(
            &calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle,
            &calc_cooling_humidistat_moisture_demand_assignment_lifecycle,
            timestep_outputs.len(),
            latest_output,
            &binding,
        )?;
    let calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle =
        purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitLifecycle,
        )?;
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_validation::
        validate_lifecycle(
            &calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle,
            &calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle,
            timestep_outputs.len(),
            latest_output,
            &binding,
        )?;

    let calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle =
        purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitLifecycle,
        )?;
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_validation::validate_lifecycle(
        &calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle,
        &calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_humidistat_case_break_lifecycle =
        purchased_air_calc_cooling_humidistat_case_break_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingHumidistatCaseBreakLifecycle,
        )?;
    cooling_humidistat_case_break_validation::validate_lifecycle(
        &calc_cooling_humidistat_case_break_lifecycle,
        &calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle =
        purchased_air_calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycle,
        )?;
    cooling_constant_supply_humidity_ratio_case_entry_validation::validate_lifecycle(
        &calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle,
        &calc_cooling_humidistat_case_break_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle =
        purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingConstantSupplyHumidityRatioAssignmentLifecycle,
        )?;
    cooling_constant_supply_humidity_ratio_assignment_validation::validate_lifecycle(
        &calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle,
        &calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle =
        purchased_air_calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingConstantSupplyHumidityRatioCaseBreakLifecycle,
        )?;
    cooling_constant_supply_humidity_ratio_case_break_validation::validate_lifecycle(
        &calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle,
        &calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle =
        purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentLifecycle,
        )?;
    cooling_default_supply_humidity_ratio_mixed_air_assignment_validation::validate_lifecycle(
        &calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle,
        &calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_default_supply_humidity_ratio_case_break_lifecycle =
        purchased_air_calc_cooling_default_supply_humidity_ratio_case_break_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingDefaultSupplyHumidityRatioCaseBreakLifecycle,
        )?;
    cooling_default_supply_humidity_ratio_case_break_validation::validate_lifecycle(
        &calc_cooling_default_supply_humidity_ratio_case_break_lifecycle,
        &calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle =
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardLifecycle,
        )?;
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_validation::validate_lifecycle(
        &calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle,
        &calc_cooling_default_supply_humidity_ratio_case_break_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle =
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardLifecycle,
        )?;
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_validation::validate_lifecycle(
        &calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle,
        &calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle =
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardLifecycle,
        )?;
    cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_validation::validate_lifecycle(
        &calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle,
        &calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle =
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycle,
        )?;
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_validation::validate_lifecycle(
        &calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle,
        &calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;
    let calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle =
        purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary(
            &purchased_air_runtime_state,
            binding.ideal_loads_air_system,
        )
        .map_err(
            DirectZonePurchasedAirCoupledRuntimeError::
                CalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentLifecycle,
        )?;
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_validation::validate_lifecycle(
        &calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle,
        &calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle,
        timestep_outputs.len(),
        latest_output,
        &binding,
    )?;

    let HeatBalanceRunPeriodSamples {
        zone_temperatures,
        zone_humidity_ratios,
        zone_conduction_rates,
        inside_surface_iteration_counts,
        zone_air_heat_balance_rates,
        zone_air_debug_traces,
        surface_temperatures,
        outdoor_temperatures,
        outdoor_wet_bulb_temperatures,
        sky_temperatures,
        horizontal_infrared_radiation_rates,
        rain_statuses,
        ..
    } = samples;
    let mut results = heat_balance_result_store_from_traces(HeatBalanceResultSeriesTraces {
        zone_temperatures,
        zone_humidity_ratios,
        zone_conduction_rates,
        inside_surface_iteration_counts,
        zone_air_heat_balance_rates,
        zone_air_debug_traces,
        surface_temperatures,
        outdoor_temperatures,
        outdoor_wet_bulb_temperatures,
        sky_temperatures,
        horizontal_infrared_radiation_rates,
        rain_statuses,
    });
    let supply_node_name = node_name(model, binding.supply_node);
    let return_node_name = node_name(model, binding.return_node);
    let zone_name = zone_name(model, binding.zone);
    append_direct_zone_purchased_air_hourly_output_series(
        &mut results,
        binding.system,
        &zone_name,
        binding.supply_node,
        &supply_node_name,
        binding.limit_context,
        &timestep_outputs,
        zone_steps_per_hour,
        seconds_per_timestep,
    )
    .map_err(DirectZonePurchasedAirCoupledRuntimeError::HourlyOutput)?;

    Ok(DirectZonePurchasedAirCoupledSimulation {
        summary: DirectZonePurchasedAirCoupledSummary {
            samples: options.sample_count,
            timestep_count: state.timestep_index,
            zone_timesteps_per_hour: zone_steps_per_hour,
            timestep_seconds: seconds_per_timestep,
            coupling_call_count: timestep_outputs.len(),
            system_name: binding.system.name.0.clone(),
            supply_node_name,
            return_node_name,
            branch: binding.branch,
            zone_demand_source: DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE,
            fixture_demand_injection_used: false,
            recirculation_state_source: DIRECT_ZONE_PURCHASED_AIR_RECIRCULATION_SOURCE,
            actual_coupled_source_order: DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER,
            init_lifecycle,
            calc_entry_lifecycle,
            calc_minimum_oa_prefix_lifecycle,
            calc_cooling_entry_gate_lifecycle,
            calc_cooling_oa_max_flow_gate_lifecycle,
            calc_cooling_oa_max_flow_body_lifecycle,
            calc_cooling_economizer_guard_lifecycle,
            calc_cooling_economizer_condition_lifecycle,
            calc_cooling_economizer_body_lifecycle,
            calc_cooling_sensible_flow_lifecycle,
            calc_cooling_dehumidification_flow_lifecycle,
            calc_cooling_humidification_flow_lifecycle,
            calc_cooling_capacity_zero_flow_reset_lifecycle,
            calc_cooling_supply_mass_flow_maximum_lifecycle,
            calc_cooling_supply_mass_flow_ems_override_guard_lifecycle,
            calc_cooling_supply_mass_flow_ems_override_body_lifecycle,
            calc_cooling_supply_mass_flow_limit_guard_lifecycle,
            calc_cooling_supply_mass_flow_limit_body_lifecycle,
            calc_cooling_supply_mass_flow_very_small_guard_lifecycle,
            calc_cooling_supply_mass_flow_very_small_guard_body_lifecycle,
            calc_cooling_mixed_air_call_lifecycle,
            calc_cooling_supply_mass_flow_positive_guard_lifecycle,
            calc_cooling_positive_supply_cp_air_assignment_lifecycle,
            calc_cooling_positive_supply_temperature_assignment_lifecycle,
            calc_cooling_positive_supply_temperature_minimum_limit_lifecycle,
            calc_cooling_positive_supply_temperature_mixed_air_limit_lifecycle,
            calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment_lifecycle,
            calc_cooling_positive_supply_enthalpy_assignment_lifecycle,
            calc_cooling_positive_supply_capacity_limit_guard_lifecycle,
            calc_cooling_positive_supply_capacity_limit_cp_air_assignment_lifecycle,
            calc_cooling_positive_supply_capacity_limit_sensible_output_assignment_lifecycle,
            calc_cooling_positive_supply_capacity_limit_sensible_output_guard_lifecycle,
            calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_lifecycle,
            calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_lifecycle,
            calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle,
            calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_lifecycle,
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle,
            calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_lifecycle,
            calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit_lifecycle,
            calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle,
            calc_cooling_constant_shr_case_break_lifecycle,
            calc_cooling_humidistat_case_entry_lifecycle,
            calc_cooling_humidistat_moisture_demand_assignment_lifecycle,
            calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle,
            calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle,
            calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle,
            calc_cooling_humidistat_case_break_lifecycle,
            calc_cooling_constant_supply_humidity_ratio_case_entry_lifecycle,
            calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle,
            calc_cooling_constant_supply_humidity_ratio_case_break_lifecycle,
            calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle,
            calc_cooling_default_supply_humidity_ratio_case_break_lifecycle,
            calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard_lifecycle,
            calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle,
            calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_lifecycle,
            calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle,
            calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle,
        },
        state,
        results,
        internal_gain_schedule_cache_profile,
    })
}

fn validate_calc_entry_lifecycle(
    lifecycle: &PurchasedAirCalcEntryLifecycleSummary,
    timestep_count: usize,
    latest_output: &super::DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), DirectZonePurchasedAirCoupledRuntimeError> {
    let state = &lifecycle.state;
    for (field, expected, actual) in [
        ("call_count", timestep_count, state.call_count),
        ("reset_count", timestep_count, state.reset_count),
        ("demand_read_count", timestep_count, state.demand_read_count),
        (
            "overall_availability_read_count",
            timestep_count,
            state.overall_availability_read_count,
        ),
        (
            "heating_availability_read_count",
            timestep_count,
            state.heating_availability_read_count,
        ),
        (
            "cooling_availability_read_count",
            timestep_count,
            state.cooling_availability_read_count,
        ),
        (
            "availability_manager_read_count",
            timestep_count,
            state.availability_manager_read_count,
        ),
        (
            "availability_manager_zone_write_count",
            timestep_count,
            state.availability_manager_zone_write_count,
        ),
        (
            "availability_status_copy_count",
            timestep_count,
            state.availability_status_copy_count,
        ),
        ("force_off_count", 0, state.force_off_count),
        ("heating_on_count", timestep_count, state.heating_on_count),
        ("cooling_on_count", timestep_count, state.cooling_on_count),
        (
            "unit_on_off_partition",
            timestep_count,
            state.unit_body_entry_count + state.unit_off_count,
        ),
        (
            "overall_gate_partition",
            timestep_count,
            state.unit_body_entry_count + state.overall_schedule_off_count,
        ),
    ] {
        if actual != expected {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let latest = state.latest.as_ref().ok_or(
        DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
            field: "latest_snapshot_present",
            expected: 1,
            actual: 0,
        },
    )?;
    let ready = lifecycle.source == PURCHASED_AIR_CALC_ENTRY_SOURCE
        && state.system == binding.ideal_loads_air_system
        && state.availability_manager_zone == Some(binding.zone)
        && state.availability_status == PurchasedAirAvailabilityStatus::NoAction
        && state.minimum_outdoor_air_mass_flow_rate_kg_per_s == 0.0
        && state.economizer_active_time_hours == 0.0
        && state.heat_recovery_active_time_hours == 0.0
        && latest == &latest_output.calculation_entry;
    if !ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcEntryLifecycleInvariant {
                field: "latest_release_snapshot_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    Ok(())
}

fn calc_entry_snapshot_matches_release(
    output: &super::DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let entry: PurchasedAirCalcEntrySnapshot = output.calculation_entry;
    let demand = output.coupling.prediction.zone_demand;
    entry.source == PURCHASED_AIR_CALC_ENTRY_SOURCE
        && entry.source_order == PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER
        && entry.system == binding.ideal_loads_air_system
        && entry.call_ordinal == call_ordinal
        && entry.controlled_zone == binding.zone
        && entry.supply_node == binding.supply_node
        && entry.zone_node == binding.zone_air_node
        && entry.outdoor_air_node.is_none()
        && entry.recirculation_node == binding.return_node
        && entry.reset.all_zero()
        && entry.demand.zone == demand.zone
        && entry.demand.sensible_input_kind == demand.sensible_input_kind
        && entry.demand.remaining_output_req_to_heat_sp_w
            == demand.remaining_output_req_to_heat_sp_w
        && entry.demand.remaining_output_req_to_cool_sp_w
            == demand.remaining_output_req_to_cool_sp_w
        && entry.unit_defaulted_on
        && !entry.economizer_defaulted_on
        && entry.availability_manager_read_site_visited
        && entry.availability_manager_zone_written
        && entry.copied_availability_status == Some(PurchasedAirAvailabilityStatus::NoAction)
        && !entry.force_off_applied
        && entry.overall_availability_read_site_visited
        && entry.heating_availability_read_site_visited
        && entry.cooling_availability_read_site_visited
        && entry.overall_availability == output.schedules.overall_availability
        && entry.heating_availability == 1.0
        && entry.cooling_availability == 1.0
        && entry.unit_on == output.schedules.unit_available
        && entry.heating_on
        && entry.cooling_on
        && entry.unit_body_entered == entry.unit_on
}

fn validate_init_lifecycle(
    lifecycle: &PurchasedAirInitLifecycleSummary,
    timestep_count: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), DirectZonePurchasedAirCoupledRuntimeError> {
    for (field, expected, actual) in [
        ("init_call_count", timestep_count, lifecycle.init_call_count),
        (
            "module_initialization_count",
            1,
            lifecycle.module_initialization_count,
        ),
        (
            "equipment_list_check_count",
            1,
            lifecycle.equipment_list_check_count,
        ),
        (
            "declared_system_count",
            1,
            lifecycle.declared_system_order.len(),
        ),
        (
            "equipment_list_scanned_unit_count",
            1,
            lifecycle.equipment_list_scanned_unit_count,
        ),
        (
            "equipment_list_missing_unit_count",
            0,
            lifecycle.equipment_list_missing_unit_count,
        ),
        (
            "equipment_list_diagnostic_count",
            0,
            lifecycle.equipment_list_diagnostics.len(),
        ),
        (
            "one_time_initialization_count",
            1,
            lifecycle.one_time_initialization_count,
        ),
        (
            "topology_completion_count",
            1,
            lifecycle.topology_completion_count,
        ),
        ("sizing_attempt_count", 1, lifecycle.sizing_attempt_count),
        ("sizing_check_count", 1, lifecycle.sizing_check_count),
        (
            "environment_initialization_count",
            1,
            lifecycle.environment_initialization_count,
        ),
        (
            "environment_rearm_count",
            usize::from(timestep_count > 1),
            lifecycle.environment_rearm_count,
        ),
    ] {
        if actual != expected {
            return Err(
                DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                    field,
                    expected,
                    actual,
                },
            );
        }
    }
    let flags = lifecycle.flags;
    let ready = flags.state_machine_used
        && flags.one_time_checked
        && flags.topology_ready
        && flags.environment_initialized
        && flags.sizing_checked
        && flags.equipment_list_checked
        && flags.return_plenum_inactive
        && lifecycle.equipment_list_scan_order == lifecycle.declared_system_order
        && lifecycle.declared_system_order == vec![binding.system.id]
        && lifecycle.equipment_list_scan_ordinal == Some(1)
        && lifecycle.first_matching_equipment_list == Some(binding.equipment_list)
        && lifecycle.equipment_list_membership_found == Some(true)
        && lifecycle.controlled_zone == Some(binding.zone)
        && lifecycle.equipment_list == Some(binding.equipment_list)
        && lifecycle.supply_node == Some(binding.supply_node)
        && lifecycle.recirculation_node == Some(binding.return_node)
        && lifecycle.recirculation_source
            == Some(PurchasedAirRecirculationSource::SingleZoneReturn)
        && lifecycle.rejected_exhaust_node.is_none()
        && lifecycle.reported_first_return_node.is_none()
        && lifecycle.topology_diagnostics.is_empty()
        && lifecycle.topology_failure.is_none()
        && lifecycle.economizer_flow_limit_warning_count == 0
        && lifecycle.supply_temperature_registered_recurring_diagnostic_count == 0
        && lifecycle.supply_temperature_diagnostic_event_count == 0
        && lifecycle.supply_temperature_characterized_severe_error_count_increment == 0
        && lifecycle.cooling_supply_temperature_error_index == 0
        && lifecycle.heating_supply_temperature_error_index == 0
        && lifecycle.cooling_supply_temperature_first_diagnostic_count == 0
        && lifecycle.heating_supply_temperature_first_diagnostic_count == 0
        && lifecycle.supply_temperature_diagnostics.is_empty()
        && lifecycle.cooling_supply_temperature_warning_count == 0
        && lifecycle.heating_supply_temperature_warning_count == 0
        && flags.environment_initialization_needed == (timestep_count > 1);
    if !ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                field: "final_flags_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    let expected_sized_limits = PurchasedAirSizedLimits::from_system(binding.system);
    let sizing_ready = lifecycle.sized_limits == Some(expected_sized_limits)
        && lifecycle.sizing_outcome.is_some_and(|outcome| {
            outcome.route == PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun
                && outcome.sized_limits == expected_sized_limits
        });
    if !sizing_ready {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                field: "sizing_overlay_ready",
                expected: 1,
                actual: 0,
            },
        );
    }
    let density = lifecycle.standard_air_density_kg_per_m3;
    let density_valid = density.is_some_and(|value| value.is_finite() && value > 0.0);
    let caches_valid = lifecycle
        .maximum_heating_air_mass_flow_rate_kg_per_s
        .is_finite()
        && lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s >= 0.0
        && lifecycle
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_finite()
        && lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s >= 0.0;
    let expected_mass_flow = |limit: IdealLoadsLimit, volume_flow: Option<AutosizeOrNumber>| {
        if matches!(
            limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        ) {
            match (volume_flow, density) {
                (Some(AutosizeOrNumber::Value(volume_flow)), Some(density)) => {
                    Some(volume_flow * density)
                }
                _ => None,
            }
        } else {
            Some(0.0)
        }
    };
    let flow_caches_match_sizing = expected_mass_flow(
        binding.system.heating_limit,
        expected_sized_limits.maximum_heating_air_flow_rate_m3_per_s,
    )
    .is_some_and(|expected| {
        (lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s - expected).abs() <= 1.0e-12
    }) && expected_mass_flow(
        binding.system.cooling_limit,
        expected_sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
    )
    .is_some_and(|expected| {
        (lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s - expected).abs() <= 1.0e-12
    });
    if !density_valid || !caches_valid || !flow_caches_match_sizing {
        return Err(
            DirectZonePurchasedAirCoupledRuntimeError::InitLifecycleInvariant {
                field: "environment_cache_valid",
                expected: 1,
                actual: 0,
            },
        );
    }
    Ok(())
}

fn validate_fixed_runtime_config(runtime_config: HeatBalanceRuntimeConfig) {
    debug_assert!(runtime_config.use_third_order_zone_air_correction);
    debug_assert!(!runtime_config.use_energyplus_adaptive_system_timestep_zone_air_correction);
}

fn node_name(model: &SimulationModel, node: ep_model::NodeId) -> String {
    model
        .typed
        .nodes
        .iter()
        .find(|candidate| candidate.id == node)
        .map(|candidate| candidate.name.0.clone())
        .unwrap_or_else(|| format!("NODE {}", node.0))
}

#[cfg(test)]
#[path = "coupled_runtime_tests.rs"]
mod tests;

fn zone_name(model: &SimulationModel, zone: ep_model::ZoneId) -> String {
    model
        .typed
        .zones
        .iter()
        .find(|candidate| candidate.id == zone)
        .map(|candidate| candidate.name.0.clone())
        .unwrap_or_else(|| format!("ZONE {}", zone.0))
}
