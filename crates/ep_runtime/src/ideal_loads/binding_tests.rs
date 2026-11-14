use super::*;
use crate::{
    heat_balance::state::ZoneAirTemperatureCoefficients,
    ideal_loads::{
        IdealLoadsSensibleMode, IdealLoadsZoneState, PurchasedAirCalcEntryIdentityRelation,
        purchased_air_calc_entry_lifecycle_summary,
        purchased_air_calc_minimum_oa_prefix_lifecycle_summary,
        purchased_air_init_lifecycle_summary,
    },
    schedules::{ScheduleSeriesCache, precompute_schedule_cache},
};
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, LoadDistributionScheme, Node, NodeId, NodeList, NodeListId, NormalizedName,
    OutdoorAirEconomizerType, Point3, ScheduleConstant, ScheduleId, SimulationModel,
    ThermostatControlObjectType, ThermostatDualSetpoint, ThermostatSetpointId, TypedModel, Zone,
    ZoneConvectionAlgorithm, ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
    ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId, ZoneThermostat,
    ZoneThermostatControl, ZoneThermostatId,
};

#[path = "binding/cooling_capacity_zero_flow_reset_tests.rs"]
mod cooling_capacity_zero_flow_reset_tests;
#[path = "binding/cooling_dehumidification_flow_tests.rs"]
mod cooling_dehumidification_flow_tests;
#[path = "binding/cooling_economizer_body_integrity_tests.rs"]
mod cooling_economizer_body_integrity_tests;
#[path = "binding/cooling_economizer_body_tests.rs"]
mod cooling_economizer_body_tests;
#[path = "binding/cooling_economizer_condition_integrity_tests.rs"]
mod cooling_economizer_condition_integrity_tests;
#[path = "binding/cooling_economizer_condition_tests.rs"]
mod cooling_economizer_condition_tests;
#[path = "binding/cooling_economizer_guard_integrity_tests.rs"]
mod cooling_economizer_guard_integrity_tests;
#[path = "binding/cooling_economizer_guard_tests.rs"]
mod cooling_economizer_guard_tests;
#[path = "binding/cooling_entry_gate_tests.rs"]
mod cooling_entry_gate_tests;
#[path = "binding/cooling_humidification_flow_tests.rs"]
mod cooling_humidification_flow_tests;
#[path = "binding/cooling_mixed_air_call_tests.rs"]
mod cooling_mixed_air_call_tests;
#[path = "binding/cooling_oa_max_flow_body_tests.rs"]
mod cooling_oa_max_flow_body_tests;
#[path = "binding/cooling_oa_max_flow_gate_tests.rs"]
mod cooling_oa_max_flow_gate_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_case_break_tests.rs"] mod cooling_constant_shr_case_break_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_case_entry_tests.rs"] mod cooling_humidistat_case_entry_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_moisture_demand_assignment_tests.rs"] mod cooling_humidistat_moisture_demand_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_tests.rs"] mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_tests.rs"] mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_supply_humidity_ratio_mixed_air_limit_tests.rs"] mod cooling_humidistat_supply_humidity_ratio_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_case_break_tests.rs"] mod cooling_humidistat_case_break_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_supply_humidity_ratio_case_entry_tests.rs"] mod cooling_constant_supply_humidity_ratio_case_entry_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_supply_humidity_ratio_assignment_tests.rs"] mod cooling_constant_supply_humidity_ratio_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_supply_humidity_ratio_case_break_tests.rs"] mod cooling_constant_supply_humidity_ratio_case_break_tests;
#[rustfmt::skip] #[path = "binding/cooling_default_supply_humidity_ratio_mixed_air_assignment_tests.rs"] mod cooling_default_supply_humidity_ratio_mixed_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_default_supply_humidity_ratio_case_break_tests.rs"] mod cooling_default_supply_humidity_ratio_case_break_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_heating_availability_guard_tests.rs"] mod cooling_supply_humidity_ratio_humidification_heating_availability_guard_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_control_humidistat_guard_tests.rs"] mod cooling_supply_humidity_ratio_humidification_control_humidistat_guard_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_tests.rs"] mod cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_tests.rs"] mod cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_tests.rs"] mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_tests.rs"] mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_tests.rs"] mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_tests;#[rustfmt::skip] #[path = "binding/cooling_supply_humidity_ratio_pre_saturation_original_assignment_tests.rs"] mod cooling_supply_humidity_ratio_pre_saturation_original_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_control_switch_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_control_switch_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_guard_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_guard_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_tests.rs"]
mod cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_tests;
#[path = "binding/cooling_post_saturation_capacity_limit_guard_tests.rs"]
mod cooling_post_saturation_capacity_limit_guard_tests;
#[path = "binding/cooling_supply_enthalpy_post_saturation_assignment_tests.rs"]
mod cooling_supply_enthalpy_post_saturation_assignment_tests;
#[path = "binding/cooling_supply_humidity_ratio_saturation_assignment_tests.rs"]
mod cooling_supply_humidity_ratio_saturation_assignment_tests;
#[path = "binding/cooling_supply_humidity_ratio_saturation_limit_assignment_tests.rs"]
mod cooling_supply_humidity_ratio_saturation_limit_assignment_tests;
#[path = "binding/cooling_supply_mass_flow_positive_guard_else_branch_entry_tests.rs"]
mod cooling_supply_mass_flow_positive_guard_else_branch_entry_tests;
#[path = "binding/cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_tests.rs"]
mod cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_tests;
#[path = "binding/cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_tests.rs"]
mod cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_tests;
#[path = "binding/cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_tests.rs"]
mod cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_tests;
#[path = "binding/cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_tests.rs"]
mod cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_tests;
#[path = "binding/cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_tests.rs"]
mod cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_tests;
#[path = "binding/heating_mode_guard_else_branch_entry_tests.rs"]
mod heating_mode_guard_else_branch_entry_tests;
#[path = "binding/heating_mode_guard_tests.rs"]
mod heating_mode_guard_tests;
#[path = "binding/heating_operating_mode_deadband_assignment_tests.rs"]
mod heating_operating_mode_deadband_assignment_tests;
#[path = "binding/heating_operating_mode_heat_assignment_tests.rs"]
mod heating_operating_mode_heat_assignment_tests;
#[path = "binding/heating_or_no_load_case_entry_tests.rs"]
mod heating_or_no_load_case_entry_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_body_volume_flow_assignment_tests.rs"]
mod heating_outdoor_air_maximum_flow_body_volume_flow_assignment_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_continue_warning_call_tests.rs"]
mod heating_outdoor_air_maximum_flow_continue_warning_call_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_tests.rs"]
mod heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_first_warning_call_tests.rs"]
mod heating_outdoor_air_maximum_flow_first_warning_call_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_first_warning_counter_increment_tests.rs"]
mod heating_outdoor_air_maximum_flow_first_warning_counter_increment_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_tests.rs"]
mod heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_first_warning_guard_tests.rs"]
mod heating_outdoor_air_maximum_flow_first_warning_guard_tests;
#[path = "binding/heating_outdoor_air_maximum_flow_guard_tests.rs"]
mod heating_outdoor_air_maximum_flow_guard_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_supply_humidity_ratio_minimum_limit_tests.rs"] mod cooling_constant_shr_supply_humidity_ratio_minimum_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_tests.rs"] mod cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_supply_humidity_ratio_overdrying_limit_tests.rs"] mod cooling_constant_shr_supply_humidity_ratio_overdrying_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_cp_air_assignment_tests.rs"] mod cooling_positive_supply_cp_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_temperature_assignment_tests.rs"] mod cooling_positive_supply_temperature_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_temperature_minimum_limit_tests.rs"] mod cooling_positive_supply_temperature_minimum_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_temperature_mixed_air_limit_tests.rs"] mod cooling_positive_supply_temperature_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests.rs"] mod cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_enthalpy_assignment_tests.rs"] mod cooling_positive_supply_enthalpy_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_guard_tests.rs"] mod cooling_positive_supply_capacity_limit_guard_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_cp_air_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_cp_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_guard_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_guard_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_tests;
#[path = "binding/cooling_sensible_flow_tests.rs"]
mod cooling_sensible_flow_tests;
#[path = "binding/cooling_supply_mass_flow_ems_override_body_tests.rs"]
mod cooling_supply_mass_flow_ems_override_body_tests;
#[path = "binding/cooling_supply_mass_flow_ems_override_guard_tests.rs"]
mod cooling_supply_mass_flow_ems_override_guard_tests;
#[path = "binding/cooling_supply_mass_flow_limit_body_tests.rs"]
mod cooling_supply_mass_flow_limit_body_tests;
#[path = "binding/cooling_supply_mass_flow_limit_guard_tests.rs"]
mod cooling_supply_mass_flow_limit_guard_tests;
#[path = "binding/cooling_supply_mass_flow_maximum_tests.rs"]
mod cooling_supply_mass_flow_maximum_tests;
#[rustfmt::skip] #[path = "binding/cooling_supply_mass_flow_positive_guard_tests.rs"] mod cooling_supply_mass_flow_positive_guard_tests;
#[path = "binding/cooling_supply_mass_flow_very_small_guard_body_tests.rs"]
mod cooling_supply_mass_flow_very_small_guard_body_tests;
#[path = "binding/cooling_supply_mass_flow_very_small_guard_tests.rs"]
mod cooling_supply_mass_flow_very_small_guard_tests;
#[path = "binding/minimum_oa_prefix_tests.rs"]
mod minimum_oa_prefix_tests;
#[path = "binding/model_multiplier_tests.rs"]
mod model_multiplier_tests;

#[test]
fn model_binding_resolves_exact_typed_ids_and_schedule_roles() {
    let (model, _) = fixture(|_| {});

    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");

    assert_eq!(binding.zone, ZoneId(0));
    assert_eq!(binding.thermostat, ZoneThermostatId(0));
    assert_eq!(binding.dual_setpoint, ThermostatSetpointId(0));
    assert_eq!(binding.control_type_schedule, ScheduleId(0));
    assert_eq!(binding.heating_setpoint_schedule, ScheduleId(1));
    assert_eq!(binding.cooling_setpoint_schedule, ScheduleId(2));
    assert_eq!(binding.overall_availability_schedule, Some(ScheduleId(3)));
    assert_eq!(binding.equipment_list, ZoneEquipmentListId(0));
    assert_eq!(binding.ideal_loads_air_system, IdealLoadsAirSystemId(0));
    assert_eq!([binding.supply_node.0, binding.return_node.0], [0, 2]);
    assert_eq!(binding.zone_air_node, NodeId(1));
    assert_eq!(binding.nominal_system_timestep_seconds, 600.0);
    assert!(!format!("{:?}", model.graph).contains("deferred_source_node_lists"));
    assert_eq!(
        format!("{:?}", ep_model::NodeListMembers::Proven(vec![NodeId(0)])),
        format!("{:?}", vec![NodeId(0)])
    );

    // Basic positive OA is Source-only and still passes the real model
    // validator, common topology/numeric checks and initialization plans.
    for method in [
        ep_model::DesignSpecificationOutdoorAirMethod::FlowPerPerson,
        ep_model::DesignSpecificationOutdoorAirMethod::FlowPerArea,
        ep_model::DesignSpecificationOutdoorAirMethod::FlowPerZone,
        ep_model::DesignSpecificationOutdoorAirMethod::AirChangesPerHour,
        ep_model::DesignSpecificationOutdoorAirMethod::Sum,
        ep_model::DesignSpecificationOutdoorAirMethod::Maximum,
    ] {
        let (positive_oa, _) = fixture(|typed| {
            let id = ep_model::DesignSpecificationOutdoorAirId(0);
            typed
                .design_specification_outdoor_air_names
                .insert("REAL OA", id);
            typed
                .design_specification_outdoor_air
                .push(ep_model::DesignSpecificationOutdoorAir {
                    id,
                    name: NormalizedName::new("REAL OA"),
                    method,
                    outdoor_air_flow_per_person_m3_per_s_person: 0.003,
                    outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: 0.0002,
                    outdoor_air_flow_per_zone_m3_per_s: 0.01,
                    outdoor_air_flow_air_changes_per_hour: 0.25,
                    outdoor_air_schedule: Some(ScheduleId(3)),
                    proportional_control_minimum_outdoor_air_flow_rate_schedule: Some(ScheduleId(
                        3,
                    )),
                });
            typed.nodes.push(Node {
                id: NodeId(3),
                name: NormalizedName::new("REAL OUTSIDE"),
            });
            typed.node_names.insert("REAL OUTSIDE", NodeId(3));
            let unit = &mut typed.ideal_loads_air_systems[0];
            unit.design_specification_outdoor_air_object_name =
                Some(NormalizedName::new("REAL OA"));
            unit.outdoor_air_inlet_node_name = Some(NormalizedName::new("REAL OUTSIDE"));
        });
        validate_source_direct_zone_purchased_air_model(&positive_oa)
            .expect("six basic DSOA methods use actual Source owners");
        assert!(bind_direct_zone_purchased_air_model(&positive_oa).is_err());
        // Same real typed input/validator seam for all four owned selectors.
        // Both zero and positive hard sizes keep Native's actual zero boundary.
        for limit in [
            IdealLoadsLimit::NoLimit,
            IdealLoadsLimit::LimitFlowRate,
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
        ] {
            for (flow, capacity) in [(0.00005, 1.0), (0.0, 0.0)] {
                let mut typed = positive_oa.typed.clone();
                let unit = &mut typed.ideal_loads_air_systems[0];
                unit.heating_limit = limit;
                unit.cooling_limit = limit;
                // Reuse one existing positive per-person context for actual
                // current-occupancy DCV eligibility; no extra fixture or loop.
                if method == ep_model::DesignSpecificationOutdoorAirMethod::FlowPerPerson
                    && limit == IdealLoadsLimit::NoLimit
                    && flow > 0.0
                {
                    unit.demand_controlled_ventilation_type =
                        DemandControlledVentilationType::OccupancySchedule;
                } else if method == ep_model::DesignSpecificationOutdoorAirMethod::Sum
                    && limit == IdealLoadsLimit::NoLimit
                    && flow > 0.0
                {
                    unit.demand_controlled_ventilation_type =
                        DemandControlledVentilationType::Co2Setpoint;
                }
                if method == ep_model::DesignSpecificationOutdoorAirMethod::FlowPerPerson
                    && limit == IdealLoadsLimit::LimitFlowRate
                    && flow > 0.0
                {
                    unit.outdoor_air_economizer_type =
                        OutdoorAirEconomizerType::DifferentialDryBulb;
                } else if method == ep_model::DesignSpecificationOutdoorAirMethod::Sum
                    && limit == IdealLoadsLimit::LimitFlowRateAndCapacity
                    && flow > 0.0
                {
                    unit.outdoor_air_economizer_type =
                        OutdoorAirEconomizerType::DifferentialEnthalpy;
                }
                unit.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(flow));
                unit.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(flow));
                unit.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(capacity));
                unit.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(capacity));
                let candidate = SimulationModel::from_typed(typed);
                validate_source_direct_zone_purchased_air_model(&candidate)
                    .expect("Source basic OA retains owned active finite/zero hard limits");
                assert!(bind_direct_zone_purchased_air_model(&candidate).is_err());
            }
        }
        for invalid in 0..19 {
            let mut typed = positive_oa.typed.clone();
            match invalid {
                0 => {
                    typed.ideal_loads_air_systems[0].design_specification_outdoor_air_object_name =
                        Some(NormalizedName::new("MISSING"))
                }
                1 => typed.design_specification_outdoor_air.clear(),
                2 => {
                    typed.design_specification_outdoor_air[0].method =
                        ep_model::DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure
                }
                3 => {
                    typed.design_specification_outdoor_air[0].outdoor_air_flow_per_zone_m3_per_s =
                        -1.0
                }
                4 => {
                    typed.design_specification_outdoor_air[0].outdoor_air_flow_per_zone_m3_per_s =
                        f64::NAN
                }
                5 => {
                    typed.ideal_loads_air_systems[0].outdoor_air_inlet_node_name =
                        Some(NormalizedName::new("SUPPLY"))
                }
                6 => {
                    typed.ideal_loads_air_systems[0].outdoor_air_inlet_node_name =
                        Some(NormalizedName::new("ZONE AIR"))
                }
                7 => {
                    typed.ideal_loads_air_systems[0].outdoor_air_inlet_node_name =
                        Some(NormalizedName::new("RETURN"))
                }
                8 => {
                    typed.ideal_loads_air_systems[0].heating_limit = IdealLoadsLimit::LimitCapacity;
                    typed.ideal_loads_air_systems[0].maximum_sensible_heating_capacity_w =
                        Some(AutosizeOrNumber::Autosize);
                }
                9 => {
                    typed.ideal_loads_air_systems[0].cooling_limit = IdealLoadsLimit::LimitFlowRate;
                    typed.ideal_loads_air_systems[0].maximum_cooling_air_flow_rate_m3_per_s = None;
                }
                10 => {
                    typed.ideal_loads_air_systems[0].dehumidification_control_type =
                        DehumidificationControlType::ConstantSensibleHeatRatio
                }
                11 => {
                    typed.ideal_loads_air_systems[0].humidification_control_type =
                        HumidificationControlType::ConstantSupplyHumidityRatio
                }
                12 => typed.ideal_loads_air_systems[0].outdoor_air_inlet_node_name = None,
                13 => {
                    let unit = &mut typed.ideal_loads_air_systems[0];
                    unit.outdoor_air_economizer_type =
                        OutdoorAirEconomizerType::DifferentialDryBulb;
                    if method == ep_model::DesignSpecificationOutdoorAirMethod::Sum {
                        unit.cooling_limit = IdealLoadsLimit::LimitCapacity;
                        unit.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(1.0));
                    }
                }
                14 => {
                    typed.ideal_loads_air_systems[0].heat_recovery_type = HeatRecoveryType::Sensible
                }
                15 => typed.zone_humidistats.push(ep_model::ZoneHumidistat {
                    id: ep_model::ZoneHumidistatId(0),
                    name: NormalizedName::new("UNCOMPOSED CONTROL"),
                    zone: ZoneId(0),
                    humidifying_relative_humidity_setpoint_schedule: ScheduleId(3),
                    dehumidifying_relative_humidity_setpoint_schedule: ScheduleId(3),
                }),
                16 => {
                    typed
                        .node_list_names
                        .insert("MATCHED EXHAUST", NodeListId(0));
                    typed.node_lists.push(NodeList {
                        id: NodeListId(0),
                        name: NormalizedName::new("MATCHED EXHAUST"),
                        nodes: ep_model::NodeListMembers::Proven(vec![NodeId(3)]),
                    });
                    typed.zone_equipment_connections[0].zone_air_exhaust_node_or_nodelist_name =
                        Some(NormalizedName::new("MATCHED EXHAUST"));
                    typed.ideal_loads_air_systems[0].zone_exhaust_air_node_name =
                        Some(NormalizedName::new("MATCHED EXHAUST"));
                }
                17 => {
                    typed.ideal_loads_air_systems[0].cooling_limit = IdealLoadsLimit::LimitFlowRate;
                    typed.ideal_loads_air_systems[0].maximum_cooling_air_flow_rate_m3_per_s =
                        Some(AutosizeOrNumber::Value(-1.0));
                }
                18 => {
                    typed.ideal_loads_air_systems[0].heating_limit = IdealLoadsLimit::LimitCapacity;
                    typed.ideal_loads_air_systems[0].maximum_sensible_heating_capacity_w =
                        Some(AutosizeOrNumber::Value(f64::NAN));
                }
                _ => unreachable!(),
            }
            let candidate = SimulationModel::from_typed(typed);
            assert!(
                validate_source_direct_zone_purchased_air_model(&candidate).is_err(),
                "method={method:?} invalid={invalid}"
            );
        }
    }

    let (unused_deferred, _) = fixture(|typed| {
        typed.node_names = ep_model::NameMap::source_exact_namespace();
        for node in &typed.nodes {
            typed.node_names.insert(&node.name.0, node.id);
        }
        typed.node_list_names = ep_model::NameMap::source_exact_namespace();
        typed
            .node_list_names
            .insert("UNUSED ORIGINAL PREFIX", NodeListId(0));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: typed
                .node_list_names
                .normalized_name("UNUSED ORIGINAL PREFIX"),
            nodes: ep_model::NodeListMembers::DeferredSourceOriginalPrefix { used_len: 2 },
        });
    });
    let deferred = ep_model::DeferredSourceNodeList {
        id: NodeListId(0),
        used_len: 2,
    };
    assert_eq!(
        unused_deferred.graph.deferred_source_node_lists,
        vec![deferred]
    );
    validate_source_direct_zone_purchased_air_model(&unused_deferred)
        .expect("unused unresolved declaration must not block independent proven Source roles");
    bind_direct_zone_purchased_air_model(&unused_deferred)
        .expect("unchanged Legacy binding still follows independent proven roles");
    // Admission only: the real Source owner retains the humidity operation.
    // Heating limits keep their original numeric checks in every cooling shape.
    for heating in [
        IdealLoadsLimit::NoLimit,
        IdealLoadsLimit::LimitFlowRate,
        IdealLoadsLimit::LimitCapacity,
        IdealLoadsLimit::LimitFlowRateAndCapacity,
    ] {
        for cooling in [
            IdealLoadsLimit::NoLimit,
            IdealLoadsLimit::LimitFlowRate,
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
        ] {
            let (constant, _) = fixture(|typed| {
                let system = &mut typed.ideal_loads_air_systems[0];
                system.dehumidification_control_type =
                    DehumidificationControlType::ConstantSupplyHumidityRatio;
                system.heating_limit = heating;
                system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
                system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
                system.cooling_limit = cooling;
                system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.20));
                system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
            });
            let retained = format!("{:?}", constant.typed);
            assert!(has_source_constant_supply_humidity_shape(
                &constant.typed.ideal_loads_air_systems[0]
            ));
            assert!(
                !classify_no_oa_sensible_subset(&constant.typed.ideal_loads_air_systems[0])
                    .is_supported()
            );
            validate_source_direct_zone_purchased_air_model(&constant)
                .expect("Source validates constant cooling without a Legacy binding");
            let expected = match (heating, cooling) {
                (IdealLoadsLimit::NoLimit, IdealLoadsLimit::NoLimit) => {
                    DirectZonePurchasedAirBindingError::UnsupportedBranch {
                        branch: IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling,
                    }
                }
                _ => DirectZonePurchasedAirBindingError::UnsupportedFeature {
                    feature:
                        DirectZonePurchasedAirBindingFeature::SensibleOnlyHumidityControlsInactive,
                },
            };
            assert_eq!(
                bind_direct_zone_purchased_air_model(&constant)
                    .expect_err("Legacy retains its original first humidity/branch error"),
                expected
            );
            assert_eq!(format!("{:?}", constant.typed), retained);
        }
    }
    // The same Source shape handles heating-only and both constant selectors.
    // These are real typed inputs, not authored physical owner state.
    for dehumidification in [
        DehumidificationControlType::None,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        for cooling in [
            IdealLoadsLimit::NoLimit,
            IdealLoadsLimit::LimitFlowRate,
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
        ] {
            let (constant, _) = fixture(|typed| {
                let system = &mut typed.ideal_loads_air_systems[0];
                system.dehumidification_control_type = dehumidification;
                system.humidification_control_type =
                    HumidificationControlType::ConstantSupplyHumidityRatio;
                system.cooling_limit = cooling;
                system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.2));
                system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
            });
            assert!(has_source_constant_supply_humidity_shape(
                &constant.typed.ideal_loads_air_systems[0]
            ));
            validate_source_direct_zone_purchased_air_model(&constant)
                .expect("Source heating-only/both constant-humidity eligibility");
            let expected = match (cooling, dehumidification) {
                (IdealLoadsLimit::NoLimit, DehumidificationControlType::None) => {
                    DirectZonePurchasedAirBindingError::UnsupportedBranch {
                        branch: IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityHeating,
                    }
                }
                (IdealLoadsLimit::NoLimit, _) => {
                    DirectZonePurchasedAirBindingError::UnsupportedBranch {
                        branch: IdealLoadsPurchasedAirBranch::NoOaConstantSupplyHumidityCooling,
                    }
                }
                _ => DirectZonePurchasedAirBindingError::UnsupportedFeature {
                    feature:
                        DirectZonePurchasedAirBindingFeature::SensibleOnlyHumidityControlsInactive,
                },
            };
            assert_eq!(
                bind_direct_zone_purchased_air_model(&constant)
                    .expect_err("Legacy original branch/humidity priority stays unchanged"),
                expected
            );
        }
    }
    // Admission only: the real raw/getter/default and Calc owners retain SHR.
    // Reuse this one test for all limit selectors and the independent heat selector.
    for humidification in [
        HumidificationControlType::None,
        HumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        for cooling in [
            IdealLoadsLimit::NoLimit,
            IdealLoadsLimit::LimitFlowRate,
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsLimit::LimitFlowRateAndCapacity,
        ] {
            let (shr, _) = fixture(|typed| {
                let system = &mut typed.ideal_loads_air_systems[0];
                system.dehumidification_control_type =
                    DehumidificationControlType::ConstantSensibleHeatRatio;
                system.humidification_control_type = humidification;
                system.cooling_limit = cooling;
                system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.2));
                system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
                system.cooling_sensible_heat_ratio = match cooling {
                    IdealLoadsLimit::NoLimit => 0.7,
                    IdealLoadsLimit::LimitFlowRate => 1.0,
                    _ => 0.35,
                };
            });
            let retained = format!("{:?}", shr.typed);
            assert!(has_source_owned_no_oa_humidity_shape(
                &shr.typed.ideal_loads_air_systems[0]
            ));
            assert!(!has_source_constant_supply_humidity_shape(
                &shr.typed.ideal_loads_air_systems[0]
            ));
            validate_source_direct_zone_purchased_air_model(&shr)
                .expect("Source owns named/default SHR without a physical binding");
            let expected = if cooling == IdealLoadsLimit::NoLimit {
                DirectZonePurchasedAirBindingError::UnsupportedBranch {
                    branch: IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling,
                }
            } else {
                DirectZonePurchasedAirBindingError::UnsupportedFeature {
                    feature:
                        DirectZonePurchasedAirBindingFeature::SensibleOnlyHumidityControlsInactive,
                }
            };
            assert_eq!(
                bind_direct_zone_purchased_air_model(&shr).expect_err("Legacy SHR priority"),
                expected
            );
            assert_eq!(format!("{:?}", shr.typed), retained);
        }
    }
    // Typed eligibility only: no physical moisture-demand owner is authored.
    // Missing Humidistat is Native's real no-control producer path; one row
    // must refer to the same actual Zone. Legacy still rejects these selectors.
    for (dehumidification, humidification) in [
        (
            DehumidificationControlType::Humidistat,
            HumidificationControlType::None,
        ),
        (
            DehumidificationControlType::None,
            HumidificationControlType::Humidistat,
        ),
        (
            DehumidificationControlType::Humidistat,
            HumidificationControlType::Humidistat,
        ),
    ] {
        for with_control in [false, true] {
            let (humidity, _) = fixture(|typed| {
                let system = &mut typed.ideal_loads_air_systems[0];
                system.dehumidification_control_type = dehumidification;
                system.humidification_control_type = humidification;
                if with_control {
                    system.cooling_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
                    system.maximum_cooling_air_flow_rate_m3_per_s =
                        Some(AutosizeOrNumber::Value(0.2));
                    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
                    typed.zone_humidistats.push(ep_model::ZoneHumidistat {
                        id: ep_model::ZoneHumidistatId(0),
                        name: NormalizedName::new("HUMIDITY CONTROL"),
                        zone: ZoneId(0),
                        humidifying_relative_humidity_setpoint_schedule: ScheduleId(1),
                        dehumidifying_relative_humidity_setpoint_schedule: ScheduleId(2),
                    });
                    typed
                        .zone_humidistat_names
                        .insert("HUMIDITY CONTROL", ep_model::ZoneHumidistatId(0));
                }
            });
            let retained = format!("{:?}", humidity.typed);
            assert!(has_source_owned_no_oa_humidity_shape(
                &humidity.typed.ideal_loads_air_systems[0]
            ));
            assert!(!has_source_constant_supply_humidity_shape(
                &humidity.typed.ideal_loads_air_systems[0]
            ));
            validate_source_direct_zone_purchased_air_model(&humidity)
                .expect("Source Humidistat uses genuine absent or connected producer");
            assert!(bind_direct_zone_purchased_air_model(&humidity).is_err());
            assert_eq!(format!("{:?}", humidity.typed), retained);
            if with_control {
                let mut wrong_zone = humidity.typed.clone();
                wrong_zone.zone_humidistats[0].zone = ZoneId(1);
                assert_eq!(
                    validate_source_direct_zone_purchased_air_model(&SimulationModel::from_typed(
                        wrong_zone
                    )),
                    Err(DirectZonePurchasedAirBindingError::UnsupportedFeature {
                        feature:
                            DirectZonePurchasedAirBindingFeature::SourceHumidityControlAssociation,
                    }),
                );
                let mut duplicate = humidity.typed.clone();
                let mut second = duplicate.zone_humidistats[0].clone();
                second.id = ep_model::ZoneHumidistatId(1);
                second.name = NormalizedName::new("SECOND HUMIDITY CONTROL");
                duplicate
                    .zone_humidistat_names
                    .insert("SECOND HUMIDITY CONTROL", second.id);
                duplicate.zone_humidistats.push(second);
                assert!(
                    validate_source_direct_zone_purchased_air_model(&SimulationModel::from_typed(
                        duplicate
                    ))
                    .is_err()
                );
            }
        }
    }
    for invalid in [-1.0_f64, 0.0, 1.1, f64::NAN, f64::INFINITY] {
        let (shr, _) = fixture(|typed| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.dehumidification_control_type =
                DehumidificationControlType::ConstantSensibleHeatRatio;
            system.cooling_sensible_heat_ratio = invalid;
        });
        assert_eq!(
            validate_source_direct_zone_purchased_air_model(&shr),
            Err(DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
            })
        );
    }
    let blocked: [fn(&mut ep_model::IdealLoadsAirSystem); 5] = [
        |system| system.outdoor_air_inlet_node_name = Some(NormalizedName::new("SUPPLY")),
        |system| {
            system.design_specification_outdoor_air_object_name =
                Some(NormalizedName::new("OUTSIDE AIR"));
        },
        |system| {
            // Named crossed selectors are ignored by the real NoOA getter.
            system.demand_controlled_ventilation_type =
                DemandControlledVentilationType::Co2Setpoint;
            system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
            system.heat_recovery_type = HeatRecoveryType::Sensible;
            system.sensible_heat_recovery_effectiveness = 0.0;
            system.latent_heat_recovery_effectiveness = 1.0;
        },
        |system| {
            system.demand_controlled_ventilation_type =
                DemandControlledVentilationType::OccupancySchedule;
            system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialEnthalpy;
            system.heat_recovery_type = HeatRecoveryType::Enthalpy;
            system.sensible_heat_recovery_effectiveness = 1.0;
            system.latent_heat_recovery_effectiveness = 0.0;
        },
        |system| system.heat_recovery_type = HeatRecoveryType::Sensible,
    ];
    for mutate in blocked {
        let (blocked, _) = fixture(|typed| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.dehumidification_control_type =
                DehumidificationControlType::ConstantSupplyHumidityRatio;
            system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.2));
            system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
            mutate(system);
        });
        let inactive_ventilation =
            has_source_inactive_no_oa_ventilation_shape(&blocked.typed.ideal_loads_air_systems[0]);
        assert_eq!(
            has_source_constant_supply_humidity_shape(&blocked.typed.ideal_loads_air_systems[0]),
            inactive_ventilation
        );
        assert_eq!(
            validate_source_direct_zone_purchased_air_model(&blocked).is_ok(),
            inactive_ventilation
        );
        assert!(bind_direct_zone_purchased_air_model(&blocked).is_err());
    }
    for unresolved in [
        None,
        Some(AutosizeOrNumber::Autosize),
        Some(AutosizeOrNumber::Value(-1.0)),
        Some(AutosizeOrNumber::Value(f64::NAN)),
        Some(AutosizeOrNumber::Value(f64::INFINITY)),
    ] {
        for invalid_field in ["cooling_flow", "heating_capacity", "cooling_capacity"] {
            let (unresolved, _) = fixture(|typed| {
                let system = &mut typed.ideal_loads_air_systems[0];
                system.dehumidification_control_type =
                    DehumidificationControlType::ConstantSupplyHumidityRatio;
                system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
                system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.2));
                if invalid_field == "heating_capacity" {
                    system.heating_limit = IdealLoadsLimit::LimitCapacity;
                    system.maximum_sensible_heating_capacity_w = unresolved;
                } else if invalid_field == "cooling_capacity" {
                    system.cooling_limit = IdealLoadsLimit::LimitFlowRateAndCapacity;
                    system.maximum_total_cooling_capacity_w = unresolved;
                } else {
                    system.maximum_cooling_air_flow_rate_m3_per_s = unresolved;
                }
            });
            assert_eq!(
                validate_source_direct_zone_purchased_air_model(&unresolved),
                Err(DirectZonePurchasedAirBindingError::UnsupportedFeature {
                    feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
                })
            );
        }
    }

    // Unit127 validates admission and real typed schedule identity only.
    // Live cooling values and suppression remain the existing Source owner.
    for value in [0.0_f64, 1.0] {
        let mut typed = unused_deferred.typed.clone();
        let cooling = ScheduleId(u32::try_from(typed.schedules.len()).expect("dense schedule ID"));
        typed.schedules.push(ScheduleConstant {
            id: cooling,
            name: NormalizedName::new("127 COOLING AVAILABILITY"),
            schedule_type_limits: None,
            hourly_value: value,
        });
        typed
            .schedule_names
            .insert("127 COOLING AVAILABILITY", cooling);
        typed.ideal_loads_air_systems[0].cooling_availability_schedule = Some(cooling);
        let model = SimulationModel::from_typed(typed.clone());
        validate_source_direct_zone_purchased_air_model(&model)
            .expect("Source retains actual independent cooling schedule");
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model).expect_err("Legacy cooling premise held"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoCoolingAvailabilitySchedule,
            }
        );
        typed.ideal_loads_air_systems[0].heating_availability_schedule = Some(cooling);
        let with_heating = SimulationModel::from_typed(typed);
        let expected = DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::NoHeatingAvailabilitySchedule,
        };
        validate_source_direct_zone_purchased_air_model(&with_heating)
            .expect("Source now admits actual heating schedule role");
        assert_eq!(
            with_heating.typed.ideal_loads_air_systems[0].heating_availability_schedule,
            Some(cooling)
        );
        assert_eq!(
            bind_direct_zone_purchased_air_model(&with_heating)
                .expect_err("heating stays fenced first"),
            expected
        );
    }

    // Unit129 admits real typed cooling-fuel schedule roles only.
    // No runtime fuel values, owner fields or division are authored by this test.
    for value in [0.0_f64, 0.5, 2.0] {
        let mut typed = unused_deferred.typed.clone();
        let efficiency =
            ScheduleId(u32::try_from(typed.schedules.len()).expect("dense schedule ID"));
        typed.schedules.push(ScheduleConstant {
            id: efficiency,
            name: NormalizedName::new("129 COOLING FUEL EFFICIENCY"),
            schedule_type_limits: None,
            hourly_value: value,
        });
        typed
            .schedule_names
            .insert("129 COOLING FUEL EFFICIENCY", efficiency);
        typed.ideal_loads_air_systems[0].cooling_fuel_efficiency_schedule = Some(efficiency);
        if value == 0.0 {
            // Reuse this existing no-OA schedule context; no new fixture.
            typed.ideal_loads_air_systems[0].demand_controlled_ventilation_type =
                DemandControlledVentilationType::OccupancySchedule;
        }
        let cooling = SimulationModel::from_typed(typed.clone());
        assert_eq!(
            cooling.typed.ideal_loads_air_systems[0].cooling_fuel_efficiency_schedule,
            Some(efficiency)
        );
        assert_eq!(
            cooling
                .typed
                .schedules
                .iter()
                .find(|row| row.id == efficiency)
                .expect("actual fuel schedule arena member")
                .hourly_value
                .to_bits(),
            value.to_bits()
        );
        validate_source_direct_zone_purchased_air_model(&cooling)
            .expect("Source retains actual cooling fuel schedule identity");
        let expected = DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::NoFuelEfficiencySchedules,
        };
        assert_eq!(
            bind_direct_zone_purchased_air_model(&cooling).expect_err("Legacy cooling fuel fence"),
            expected
        );
        for cooling_selected in [false, true] {
            let mut heating = typed.clone();
            heating.ideal_loads_air_systems[0].cooling_fuel_efficiency_schedule =
                cooling_selected.then_some(efficiency);
            heating.ideal_loads_air_systems[0].heating_fuel_efficiency_schedule = Some(efficiency);
            let heating = SimulationModel::from_typed(heating);
            validate_source_direct_zone_purchased_air_model(&heating)
                .expect("Source now admits actual heating schedule role");
            assert_eq!(
                heating.typed.ideal_loads_air_systems[0].heating_fuel_efficiency_schedule,
                Some(efficiency)
            );
            assert_eq!(
                bind_direct_zone_purchased_air_model(&heating)
                    .expect_err("heating fuel stays fenced in either mode"),
                expected
            );
        }
    }

    // Admission only: Source retains the actual raw schedule/basis owners at
    // runtime; typed eligibility must not invent Config arrays or return flow.
    for (fraction, basis) in [(true, false), (false, true), (true, true)] {
        let mut typed = unused_deferred.typed.clone();
        typed.node_list_names.insert("RETURN BASIS", NodeListId(1));
        typed.node_lists.push(NodeList {
            id: NodeListId(1),
            name: typed.node_list_names.normalized_name("RETURN BASIS"),
            nodes: vec![NodeId(0), NodeId(1)].into(),
        });
        if fraction {
            typed.zone_equipment_connections[0]
                .zone_return_air_node_1_flow_rate_fraction_schedule = Some(ScheduleId(3));
        }
        if basis {
            typed.zone_equipment_connections[0]
                .zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name =
                Some(typed.node_list_names.normalized_name("RETURN BASIS"));
        }
        let controlled = SimulationModel::from_typed(typed.clone());
        validate_source_direct_zone_purchased_air_model(&controlled)
            .expect("actual Source return-control roles retain proven node identities");
        assert_eq!(
            bind_direct_zone_purchased_air_model(&controlled)
                .expect_err("Legacy return-control boundary remains exact"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoZoneReturnFlowControl,
            }
        );
        if basis {
            typed.node_lists[1].nodes =
                ep_model::NodeListMembers::DeferredSourceOriginalPrefix { used_len: 2 };
            assert_eq!(
                validate_source_direct_zone_purchased_air_model(&SimulationModel::from_typed(
                    typed
                )),
                Err(DirectZonePurchasedAirBindingError::DeferredSourceNodeList {
                    deferred: ep_model::DeferredSourceNodeList {
                        id: NodeListId(1),
                        used_len: 2
                    },
                })
            );
        }
    }

    // Admission over real typed roles only, not physical Config/Node arrays.
    // Counts above two ensure the Source relation is a vector, not a pair cap.
    for count in [0_usize, 1, 3, 5] {
        let mut typed = unused_deferred.typed.clone();
        let exhaust = NodeId(u32::try_from(typed.nodes.len()).expect("dense exhaust ID"));
        typed.nodes.push(Node {
            id: exhaust,
            name: typed.node_names.normalized_name("EXHAUST"),
        });
        typed.node_names.insert("EXHAUST", exhaust);
        let mut returns = vec![NodeId(2)];
        for number in 2..=5 {
            let name = format!("RETURN {number}");
            let id = NodeId(u32::try_from(typed.nodes.len()).expect("dense real typed role ID"));
            typed.nodes.push(Node {
                id,
                name: typed.node_names.normalized_name(&name),
            });
            typed.node_names.insert(&name, id);
            returns.push(id);
        }
        typed.zone_equipment_connections[0].zone_air_exhaust_node_or_nodelist_name =
            Some(typed.node_names.normalized_name("EXHAUST"));
        typed.ideal_loads_air_systems[0].zone_exhaust_air_node_name =
            Some(typed.node_names.normalized_name("EXHAUST"));
        let list_id =
            NodeListId(u32::try_from(typed.node_lists.len()).expect("dense return list ID"));
        typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name = if count == 0 {
            None
        } else {
            typed.node_list_names.insert("RETURN VECTOR", list_id);
            typed.node_lists.push(NodeList {
                id: list_id,
                name: typed.node_list_names.normalized_name("RETURN VECTOR"),
                nodes: returns[..count].to_vec().into(),
            });
            Some(typed.node_list_names.normalized_name("RETURN VECTOR"))
        };
        let model = SimulationModel::from_typed(typed.clone());
        validate_source_direct_zone_purchased_air_model(&model)
            .expect("matched genuine exhaust selects recirculation before return cardinality");
        if let Some(name) = typed.zone_equipment_connections[0]
            .zone_return_air_node_or_nodelist_name
            .as_ref()
        {
            assert_eq!(
                typed
                    .resolve_advisory_node_reference(&name.0)
                    .require_proven_nodes()
                    .expect("actual ordered Proven returns"),
                returns[..count]
            );
        }
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("Legacy exhaust check still precedes return cardinality"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoZoneExhaustTopology,
            }
        );
        let mut without_exhaust = typed.clone();
        without_exhaust.zone_equipment_connections[0].zone_air_exhaust_node_or_nodelist_name = None;
        without_exhaust.ideal_loads_air_systems[0].zone_exhaust_air_node_name = None;
        let without_exhaust = SimulationModel::from_typed(without_exhaust);
        if count == 1 {
            validate_source_direct_zone_purchased_air_model(&without_exhaust)
                .expect("original no-exhaust singleton");
            bind_direct_zone_purchased_air_model(&without_exhaust)
                .expect("Legacy singleton unchanged");
        } else {
            let expected = DirectZonePurchasedAirBindingError::Cardinality {
                relation: DirectZonePurchasedAirBindingRelation::ZoneReturnNode,
                expected: 1,
                actual: count,
            };
            assert_eq!(
                validate_source_direct_zone_purchased_air_model(&without_exhaust),
                Err(expected.clone())
            );
            assert_eq!(
                bind_direct_zone_purchased_air_model(&without_exhaust)
                    .expect_err("Legacy return cardinality unchanged"),
                expected
            );
        }
        if count == 5 {
            // Every member, including late members, must be disjoint from every
            // genuine supply/ZoneAir/exhaust role; never inspect only the first.
            for position in 0..count {
                for (collision, feature) in [
                    (
                        NodeId(0),
                        DirectZonePurchasedAirBindingFeature::DistinctZoneReturnNode,
                    ),
                    (
                        NodeId(1),
                        DirectZonePurchasedAirBindingFeature::DistinctZoneReturnNode,
                    ),
                    (
                        exhaust,
                        DirectZonePurchasedAirBindingFeature::DistinctSourceExhaustNode,
                    ),
                ] {
                    let mut invalid = typed.clone();
                    let mut members = returns[..count].to_vec();
                    members[position] = collision;
                    invalid
                        .node_lists
                        .last_mut()
                        .expect("real return vector")
                        .nodes = members.into();
                    assert_eq!(
                        validate_source_direct_zone_purchased_air_model(
                            &SimulationModel::from_typed(invalid)
                        ),
                        Err(DirectZonePurchasedAirBindingError::UnsupportedFeature { feature })
                    );
                }
            }
        }
        if count == 3 {
            let mut deferred = typed.clone();
            deferred
                .node_lists
                .last_mut()
                .expect("selected actual return vector")
                .nodes = ep_model::NodeListMembers::DeferredSourceOriginalPrefix { used_len: 2 };
            assert_eq!(
                validate_source_direct_zone_purchased_air_model(&SimulationModel::from_typed(
                    deferred
                )),
                Err(DirectZonePurchasedAirBindingError::DeferredSourceNodeList {
                    deferred: ep_model::DeferredSourceNodeList {
                        id: list_id,
                        used_len: 2
                    },
                })
            );
        }
        // A present alias with no real declaration is not the optional None case.
        let mut missing = typed;
        missing.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name = Some(
            missing
                .node_list_names
                .normalized_name("MISSING RETURN ALIAS"),
        );
        assert_eq!(
            validate_source_direct_zone_purchased_air_model(&SimulationModel::from_typed(missing)),
            Err(DirectZonePurchasedAirBindingError::Cardinality {
                relation: DirectZonePurchasedAirBindingRelation::ZoneReturnNode,
                expected: 1,
                actual: 0,
            })
        );
    }
    let mut required = unused_deferred.typed.clone();
    required.zone_equipment_connections[0].zone_air_inlet_node_or_nodelist_name = Some(
        required
            .node_list_names
            .normalized_name("UNUSED ORIGINAL PREFIX"),
    );
    let required = SimulationModel::from_typed(required);
    assert_eq!(
        validate_source_direct_zone_purchased_air_model(&required),
        Err(DirectZonePurchasedAirBindingError::DeferredSourceNodeList { deferred })
    );
}

#[test]
fn binding_rejects_stale_public_model_graph() {
    let (mut model, _) = fixture(|_| {});
    model.typed.ideal_loads_air_systems[0].zone_supply_air_node_name =
        NormalizedName::new("ZONE AIR");

    assert_eq!(
        bind_direct_zone_purchased_air_model(&model)
            .expect_err("typed mutation must not reuse a stale graph"),
        DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::CoherentTypedModelGraph,
        }
    );
}

#[test]
fn scheduled_binding_samples_thermostat_and_drives_heating_feedback() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled heating coupling");

    assert_eq!(
        output.schedules,
        DirectZonePurchasedAirScheduleSnapshot {
            sample_index: 0,
            control_type: 4.0,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            overall_availability: 1.0,
            unit_available: true,
        }
    );
    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Heating
    );
    assert_eq!(
        output
            .coupling
            .purchased_air
            .trace
            .zone_state
            .air_temperature_c,
        original.mean_air_temperature_c
    );
    assert_eq!(
        output.coupling.purchased_air.trace.recirculation_state,
        IdealLoadsZoneState {
            air_temperature_c: original.mean_air_temperature_c,
            air_humidity_ratio: original.air_humidity_ratio,
        },
        "the source-valid single return receives the bounded direct-Zone T/W projection"
    );
    assert_eq!(
        output.coupling.prediction.predicted_loads.predicted_rate_w,
        output.coupling.prediction.predicted_loads.raw_total_load_w
    );
    assert!(state.sum_sys_mcp_w_per_k > 0.0);
    assert_only_system_air_sums_changed(&original, &state);
}

#[test]
fn scheduled_binding_preserves_negative_cooling_threshold() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(3_000.0);

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled cooling coupling");

    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Cooling
    );
    assert_eq!(
        output
            .coupling
            .prediction
            .zone_demand
            .remaining_output_req_to_cool_sp_w,
        -600.0
    );
    assert_eq!(
        output
            .calculation_entry
            .demand
            .remaining_output_req_to_cool_sp_w,
        -600.0
    );
    assert_eq!(output.calculation_entry.call_ordinal, 1);
    assert!(output.calculation_entry.unit_body_entered);
    let minimum_oa = output.calculation_minimum_outdoor_air;
    assert_eq!(minimum_oa.parent_call_ordinal, 1);
    assert!(minimum_oa.zone_heat_balance_reference_bound);
    assert!(minimum_oa.minimum_oa_child_called);
    assert!(minimum_oa.minimum_oa_child_no_outdoor_air_route);
    assert_eq!(
        minimum_oa.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert_eq!(
        minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert_eq!(minimum_oa.minimum_outdoor_air_sensible_output_w, Some(0.0));
    assert_eq!(
        minimum_oa.minimum_outdoor_air_moisture_output_kg_per_s,
        Some(0.0)
    );
    assert!(state.sum_sys_mcp_w_per_k > 0.0);
}

#[test]
fn overall_availability_off_clears_stale_feedback_without_changing_other_state() {
    let (model, cache) = fixture(|typed| schedule_mut(typed, ScheduleId(3)).hourly_value = 0.0);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled off coupling");

    assert!(!output.schedules.unit_available);
    assert!(!output.calculation_entry.unit_on);
    assert!(output.calculation_entry.heating_on);
    assert!(output.calculation_entry.cooling_on);
    assert!(output.calculation_entry.reset.all_zero());
    let minimum_oa = output.calculation_minimum_outdoor_air;
    assert!(!minimum_oa.unit_body_entered);
    assert!(!minimum_oa.zone_heat_balance_reference_bound);
    assert!(!minimum_oa.minimum_oa_child_called);
    assert!(!minimum_oa.ems_override_flag_read);
    assert!(!minimum_oa.outdoor_air_flag_read);
    assert_eq!(
        minimum_oa.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        None
    );
    assert_eq!(minimum_oa.minimum_outdoor_air_sensible_output_w, None);
    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Off
    );
    assert_eq!(state.sum_sys_mcp_w_per_k, 0.0);
    assert_eq!(state.sum_sys_mcp_t_w, 0.0);
    assert_only_system_air_sums_changed(&original, &state);
}

#[test]
fn deadband_sample_clears_stale_feedback_exactly() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(2_200.0);

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled deadband coupling");

    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Deadband
    );
    assert_eq!(state.sum_sys_mcp_w_per_k, 0.0);
    assert_eq!(state.sum_sys_mcp_t_w, 0.0);
}

#[test]
fn binding_rejects_ambiguous_thermostat_topology() {
    let (model, _) = fixture(|typed| {
        let mut thermostat = typed.zone_thermostats[0].clone();
        thermostat.id = ZoneThermostatId(1);
        typed.zone_thermostats.push(thermostat);
    });

    let error =
        bind_direct_zone_purchased_air_model(&model).expect_err("two thermostat edges must fail");

    assert_eq!(
        error,
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneThermostatEdge,
            expected: 1,
            actual: 2,
        }
    );
}

#[test]
fn binding_rejects_distribution_sequence_and_fraction_variants() {
    let cases = [
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_lists[0].load_distribution_scheme =
                    LoadDistributionScheme::UniformLoad;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::SequentialLoadDistribution,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_lists[0].equipment[0].cooling_sequence = 2;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::FirstEquipmentSequence,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_lists[0].equipment[0].sequential_heating_fraction_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoSequentialFractionSchedules,
        ),
    ];

    for (mutate, expected_feature) in cases {
        let (model, _) = fixture(mutate);
        let error = bind_direct_zone_purchased_air_model(&model)
            .expect_err("unsupported equipment topology must fail");
        assert_eq!(
            error,
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: expected_feature,
            }
        );
    }
}

#[test]
fn binding_rejects_multi_inlet_return_and_mode_availability_topology() {
    let (multi_inlet, _) = fixture(|typed| {
        typed.nodes.push(Node {
            id: NodeId(3),
            name: NormalizedName::new("SECOND SUPPLY"),
        });
        typed.node_names.insert("SECOND SUPPLY", NodeId(3));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("INLETS"),
            nodes: vec![NodeId(0), NodeId(3)].into(),
        });
        typed.node_list_names.insert("INLETS", NodeListId(0));
        typed.ideal_loads_air_systems[0].zone_supply_air_node_name = NormalizedName::new("INLETS");
        typed.zone_equipment_connections[0].zone_air_inlet_node_or_nodelist_name =
            Some(NormalizedName::new("INLETS"));
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&multi_inlet).expect_err("two supply edges must fail"),
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::IdealLoadsSupplyNode,
            expected: 1,
            actual: 2,
        }
    );

    let (missing_return, _) = fixture(|typed| {
        typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name = None;
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&missing_return)
            .expect_err("blank exhaust requires one Zone return node"),
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneReturnNode,
            expected: 1,
            actual: 0,
        }
    );
    let (multiple_returns, _) = fixture(|typed| {
        typed.nodes.push(Node {
            id: NodeId(3),
            name: NormalizedName::new("SECOND RETURN"),
        });
        typed.node_names.insert("SECOND RETURN", NodeId(3));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("RETURNS"),
            nodes: vec![NodeId(2), NodeId(3)].into(),
        });
        typed.node_list_names.insert("RETURNS", NodeListId(0));
        typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name =
            Some(NormalizedName::new("RETURNS"));
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&multiple_returns)
            .expect_err("the bounded blank-exhaust fallback requires exactly one return"),
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneReturnNode,
            expected: 1,
            actual: 2,
        }
    );

    let cases = [
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name =
                    Some(NormalizedName::new("ZONE AIR"));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::DistinctZoneReturnNode,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_connections[0]
                    .zone_return_air_node_1_flow_rate_fraction_schedule = Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoZoneReturnFlowControl,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].heating_availability_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoHeatingAvailabilitySchedule,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].cooling_availability_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoCoolingAvailabilitySchedule,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].design_specification_zonehvac_sizing_object_name =
                    Some(NormalizedName::new("ZONE HVAC SIZING"));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoZoneHvacSizingObject,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].heating_fuel_efficiency_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoFuelEfficiencySchedules,
        ),
    ];
    for (mutate, expected_feature) in cases {
        let (model, _) = fixture(mutate);
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("unsupported direct topology must fail"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: expected_feature,
            }
        );
    }
}

#[test]
fn binding_accepts_all_resolved_numeric_finite_limit_branches() {
    let cases = [
        (
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity,
        ),
        (
            IdealLoadsLimit::LimitFlowRate,
            IdealLoadsPurchasedAirBranch::NoOaFiniteFlow,
        ),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity,
        ),
    ];

    for (limit, expected_branch) in cases {
        let (model, _) = fixture(|typed| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.heating_limit = limit;
            system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
            system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
            system.cooling_limit = limit;
            system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.20));
            system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
        });

        let binding =
            bind_direct_zone_purchased_air_model(&model).expect("resolved finite-limit binding");
        assert_eq!(binding.branch, expected_branch);
    }
}

#[test]
fn binding_rejects_autosized_or_missing_finite_limit_values() {
    for mutate in [
        (|typed: &mut TypedModel| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.heating_limit = IdealLoadsLimit::LimitFlowRate;
            system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Autosize);
        }) as fn(&mut TypedModel),
        (|typed: &mut TypedModel| {
            typed.ideal_loads_air_systems[0].cooling_limit = IdealLoadsLimit::LimitCapacity;
        }) as fn(&mut TypedModel),
    ] {
        let (model, _) = fixture(mutate);
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("unresolved finite limits must fail closed"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
            }
        );
    }

    let (negative, _) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(-1.0));
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&negative)
            .expect_err("negative public TypedModel hard size must fail closed"),
        DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
        }
    );

    for value in [f64::NAN, f64::INFINITY] {
        let (nonfinite, _) = fixture(|typed| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
            system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(value));
        });
        assert_eq!(
            bind_direct_zone_purchased_air_model(&nonfinite)
                .expect_err("nonfinite public TypedModel hard size must fail closed"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
            }
        );
    }
}

#[test]
fn binding_still_rejects_non_sensible_purchased_air_branches() {
    let (model, _) = fixture(|typed| {
        typed.ideal_loads_air_systems[0].dehumidification_control_type =
            DehumidificationControlType::ConstantSensibleHeatRatio;
    });

    assert_eq!(
        bind_direct_zone_purchased_air_model(&model)
            .expect_err("humidity-selected branch remains outside direct coupling"),
        DirectZonePurchasedAirBindingError::UnsupportedBranch {
            branch: IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling,
        }
    );

    let (finite_constant_shr, _) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
        system.dehumidification_control_type =
            DehumidificationControlType::ConstantSensibleHeatRatio;
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&finite_constant_shr)
            .expect_err("finite-limit branch selection must not hide humidity controls"),
        DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::SensibleOnlyHumidityControlsInactive,
        }
    );
}

#[test]
fn binding_rejects_hysteresis_and_hidden_no_oa_feature_flags() {
    let cases = [
        (
            (|typed: &mut TypedModel| {
                typed.zone_thermostats[0]
                    .temperature_difference_between_cutout_and_setpoint_delta_c = 0.5;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::ZeroCutoutDelta,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].outdoor_air_economizer_type =
                    OutdoorAirEconomizerType::DifferentialDryBulb;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
        ),
    ];

    for (mutate, expected_feature) in cases {
        let (model, _) = fixture(mutate);
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("unsupported thermostat/system feature must fail"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: expected_feature,
            }
        );
    }
}

#[test]
fn schedule_errors_are_distinct_and_transactional() {
    let (model, cache) = fixture(|typed| schedule_mut(typed, ScheduleId(0)).hourly_value = 3.0);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    assert_eq!(
        couple(&binding, &cache, &mut state, 0).expect_err("control type three must fail"),
        DirectZonePurchasedAirScheduledCouplingError::UnsupportedControlType { value: 3.0 }
    );
    assert_eq!(state, original);

    assert_eq!(
        couple(&binding, &cache, &mut state, 2).expect_err("sample two is out of range"),
        DirectZonePurchasedAirScheduledCouplingError::SampleIndexOutOfRange {
            sample_index: 2,
            sample_count: 2,
        }
    );
    assert_eq!(state, original);

    let (missing_model, missing_cache) = fixture(|typed| {
        typed.thermostat_dual_setpoints[0].heating_setpoint_schedule = ScheduleId(99);
    });
    let missing_binding =
        bind_direct_zone_purchased_air_model(&missing_model).expect("topology still binds");
    assert_eq!(
        couple(&missing_binding, &missing_cache, &mut state, 0)
            .expect_err("missing heating schedule must fail"),
        DirectZonePurchasedAirScheduledCouplingError::MissingSchedule {
            role: DirectZonePurchasedAirScheduleRole::HeatingSetpoint,
            schedule: ScheduleId(99),
        }
    );
    assert_eq!(state, original);
}

#[test]
fn nonfinite_and_inverted_setpoints_are_transactional() {
    let cases = [
        (
            f64::NAN,
            24.0,
            DirectZonePurchasedAirScheduledCouplingError::NonFiniteScheduleValue {
                role: DirectZonePurchasedAirScheduleRole::HeatingSetpoint,
                schedule: ScheduleId(1),
            },
        ),
        (
            25.0,
            24.0,
            DirectZonePurchasedAirScheduledCouplingError::HeatingSetpointAboveCoolingSetpoint {
                heating_setpoint_c: 25.0,
                cooling_setpoint_c: 24.0,
            },
        ),
    ];

    for (heating, cooling, expected) in cases {
        let (model, cache) = fixture(|typed| {
            schedule_mut(typed, ScheduleId(1)).hourly_value = heating;
            schedule_mut(typed, ScheduleId(2)).hourly_value = cooling;
        });
        let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
        let mut state = zone_state_for_temp_independent_load(0.0);
        let original = state.clone();
        let error = couple(&binding, &cache, &mut state, 0)
            .expect_err("invalid current setpoints must fail");
        assert_eq!(error, expected);
        assert_eq!(state, original);
    }
}

#[test]
fn dual_setpoint_schedule_error_precedence_is_cooling_then_heating() {
    let (model, cache) = fixture(|typed| {
        schedule_mut(typed, ScheduleId(1)).hourly_value = f64::NAN;
        schedule_mut(typed, ScheduleId(2)).hourly_value = f64::NAN;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    assert_eq!(
        couple(&binding, &cache, &mut state, 0)
            .expect_err("cooling schedule is sampled before heating"),
        DirectZonePurchasedAirScheduledCouplingError::NonFiniteScheduleValue {
            role: DirectZonePurchasedAirScheduleRole::CoolingSetpoint,
            schedule: ScheduleId(2),
        }
    );
    assert_eq!(state, original);
}

#[test]
fn wrapped_cp300_failure_is_transactional() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.mean_air_temperature_c = f64::INFINITY;
    let original = state.clone();

    assert_eq!(
        couple(&binding, &cache, &mut state, 0).expect_err("CP300 must reject infinite Zone MAT"),
        DirectZonePurchasedAirScheduledCouplingError::Coupling(
            DirectZonePurchasedAirCouplingError::InputNotFinite {
                field: "zone_node_temperature_c",
            }
        )
    );
    assert_eq!(state, original);
}

#[test]
fn predictor_failure_precedes_and_preserves_purchased_air_initialization() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.convective_internal_gain_w = f64::INFINITY;
    let original = state.clone();
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    purchased_air_runtime_state.module_initialized = true;
    let original_init_state = purchased_air_runtime_state.clone();

    let error = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut state,
            purchased_air_runtime_state: &mut purchased_air_runtime_state,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect_err("predictor must reject the nonfinite source term before Init");

    assert!(matches!(
        error,
        DirectZonePurchasedAirScheduledCouplingError::Coupling(
            DirectZonePurchasedAirCouplingError::Prediction(_)
        )
    ));
    assert_eq!(purchased_air_runtime_state, original_init_state);
    assert_eq!(state, original);
}

#[test]
fn initialization_failure_precedes_calc_only_input_validation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.air_humidity_ratio = -0.001;
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    purchased_air_runtime_state.module_initialized = true;

    let error = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut state,
            purchased_air_runtime_state: &mut purchased_air_runtime_state,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect_err("Init failure must precede the Calc-only humidity rejection");

    assert_eq!(
        error,
        DirectZonePurchasedAirScheduledCouplingError::Initialization(
            PurchasedAirInitError::DeclaredSystemOrderChanged {
                expected: Vec::new(),
                actual: vec![binding.ideal_loads_air_system],
            }
        )
    );
}

#[test]
fn post_init_calc_failure_retains_init_but_preserves_zone_state() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.air_humidity_ratio = -0.001;
    let original = state.clone();
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();

    let error = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut state,
            purchased_air_runtime_state: &mut purchased_air_runtime_state,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect_err("Calc-only humidity validation must run after successful Init");

    assert_eq!(
        error,
        DirectZonePurchasedAirScheduledCouplingError::Coupling(
            DirectZonePurchasedAirCouplingError::InputNegative {
                field: "recirculation_state.air_humidity_ratio",
                value: -0.001,
            }
        )
    );
    assert_eq!(state, original);
    let lifecycle = purchased_air_init_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .expect("successful Init prefix must remain reportable");
    assert_eq!(lifecycle.init_call_count, 1);
    assert_eq!(lifecycle.topology_completion_count, 1);
    assert!(lifecycle.flags.topology_ready);
    assert_eq!(lifecycle.environment_initialization_count, 1);
    let calc_lifecycle = purchased_air_calc_entry_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .expect("successful Calc-entry prefix must remain reportable");
    assert_eq!(calc_lifecycle.state.call_count, 1);
    assert_eq!(calc_lifecycle.state.reset_count, 1);
    assert_eq!(calc_lifecycle.state.demand_read_count, 1);
    assert_eq!(
        calc_lifecycle
            .state
            .latest
            .expect("retained Calc-entry snapshot")
            .demand
            .zone,
        binding.zone
    );
}

#[test]
fn public_calc_entry_replay_and_identity_errors_do_not_mutate_lifecycle() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(0.0);
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("first source-ordered coupling");
    let before_prefix_replay = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_minimum_oa_prefix(
            &mut runtime,
            binding.system,
            output.calculation_entry,
        ),
        Err(
            PurchasedAirCalcMinimumOaPrefixError::CalculationEntryCallOrder {
                system: binding.ideal_loads_air_system,
                calculation_entry_call_count: 1,
                minimum_oa_prefix_transition_count: 1,
            }
        )
    );
    assert_eq!(runtime, before_prefix_replay);
    let base_context = PurchasedAirCalcEntryContext {
        controlled_zone: binding.zone,
        supply_node: binding.supply_node,
        zone_node: binding.zone_air_node,
        outdoor_air_node: None,
        recirculation_node: binding.return_node,
        demand: output.coupling.prediction.zone_demand,
        zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
        overall_availability: output.schedules.overall_availability,
        heating_availability: 1.0,
        cooling_availability: 1.0,
    };

    let before_replay = runtime.clone();
    assert_eq!(
        advance_purchased_air_calc_entry(
            &mut runtime,
            binding.ideal_loads_air_system,
            base_context
        ),
        Err(PurchasedAirCalcEntryError::InitializationCallOrder {
            system: binding.ideal_loads_air_system,
            init_call_count: 1,
            calc_call_count: 1,
        })
    );
    assert_eq!(runtime, before_replay);

    init_purchased_air_runtime(
        &mut runtime,
        &binding.init_manager_plan,
        &binding.init_topology_plan,
        binding.system,
        PurchasedAirInitCallContext {
            zone_equipment_inputs_filled: true,
            system_sizing_calculation: false,
            sizing: PurchasedAirHardSizeLegacyContext {
                current_zone_equipment_index: 1,
                zone_sizing_run_done: false,
            },
            begin_environment: false,
            standard_air_density_kg_per_m3: binding.limit_context.standard_air_density_kg_per_m3,
            heating_setpoint_c: output.schedules.heating_setpoint_c,
            cooling_setpoint_c: output.schedules.cooling_setpoint_c,
            overall_availability: output.schedules.overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("second initialization prefix");

    for (context, relation) in [
        (
            PurchasedAirCalcEntryContext {
                controlled_zone: ZoneId(99),
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::ControlledZone,
        ),
        (
            PurchasedAirCalcEntryContext {
                supply_node: NodeId(99),
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::SupplyNode,
        ),
        (
            PurchasedAirCalcEntryContext {
                recirculation_node: NodeId(99),
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::RecirculationNode,
        ),
        (
            PurchasedAirCalcEntryContext {
                demand: crate::zone_equipment::ZoneSysEnergyDemand {
                    zone: ZoneId(99),
                    ..base_context.demand
                },
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::DemandZone,
        ),
    ] {
        let before_error = runtime.clone();
        assert_eq!(
            advance_purchased_air_calc_entry(&mut runtime, binding.ideal_loads_air_system, context),
            Err(PurchasedAirCalcEntryError::IdentityMismatch {
                system: binding.ideal_loads_air_system,
                relation,
            })
        );
        assert_eq!(runtime, before_error);
    }

    let second = advance_purchased_air_calc_entry(
        &mut runtime,
        binding.ideal_loads_air_system,
        base_context,
    )
    .expect("valid second Calc-entry prefix");
    assert_eq!(second.call_ordinal, 2);
}

#[test]
fn fixed_timestep_state_guards_are_transactional() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let base = zone_state_for_temp_independent_load(0.0);

    let mut zone_history = base.clone();
    zone_history.use_zone_timestep_history = true;
    assert_runtime_invariant(
        &binding,
        &cache,
        zone_history,
        DirectZonePurchasedAirRuntimeInvariant::SystemTimestepHistory,
    );

    let mut wrong_zone = base.clone();
    wrong_zone.zone_id = ZoneId(7);
    assert_runtime_invariant(
        &binding,
        &cache,
        wrong_zone,
        DirectZonePurchasedAirRuntimeInvariant::BoundZoneIdentity,
    );

    let mut shortened = base.clone();
    shortened.shorten_timestep_sys = true;
    assert_runtime_invariant(
        &binding,
        &cache,
        shortened,
        DirectZonePurchasedAirRuntimeInvariant::UnshortenedSystemTimestep,
    );

    let mut multiple_steps = base.clone();
    multiple_steps.previous_system_timestep_count = 2;
    assert_runtime_invariant(
        &binding,
        &cache,
        multiple_steps,
        DirectZonePurchasedAirRuntimeInvariant::SinglePreviousSystemTimestep,
    );

    let mut wrong_prior = base;
    wrong_prior.prior_timestep_seconds = 300.0;
    assert_runtime_invariant(
        &binding,
        &cache,
        wrong_prior,
        DirectZonePurchasedAirRuntimeInvariant::NominalPriorTimestep,
    );

    let mut wrong_step = zone_state_for_temp_independent_load(0.0);
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    let original = wrong_step.clone();
    assert_eq!(
        couple_model_bound_direct_zone_purchased_air(
            DirectZonePurchasedAirScheduledCouplingInput {
                binding: &binding,
                schedule_cache: &cache,
                schedule_sample_index: 0,
                zone_state: &mut wrong_step,
                purchased_air_runtime_state: &mut purchased_air_runtime_state,
                begin_environment: true,
                barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
                system_timestep_seconds: 300.0,
            }
        )
        .expect_err("non-nominal requested timestep must fail"),
        DirectZonePurchasedAirScheduledCouplingError::RuntimeInvariant {
            invariant: DirectZonePurchasedAirRuntimeInvariant::NominalSystemTimestep,
        }
    );
    assert_eq!(wrong_step, original);
}

fn fixture(mutate: impl FnOnce(&mut TypedModel)) -> (SimulationModel, ScheduleSeriesCache) {
    let mut typed = base_typed_model();
    mutate(&mut typed);
    let cache = precompute_schedule_cache(&typed, 2).expect("test schedule cache");
    (SimulationModel::from_typed(typed), cache)
}

fn base_typed_model() -> TypedModel {
    let mut typed = TypedModel::default();
    for (id, name, value) in [
        (ScheduleId(0), "CONTROL TYPE", 4.0),
        (ScheduleId(1), "HEATING SETPOINT", 20.0),
        (ScheduleId(2), "COOLING SETPOINT", 24.0),
        (ScheduleId(3), "IDEAL LOADS AVAILABILITY", 1.0),
    ] {
        typed.schedules.push(ScheduleConstant {
            id,
            name: NormalizedName::new(name),
            schedule_type_limits: None,
            hourly_value: value,
        });
        typed.schedule_names.insert(name, id);
    }
    typed.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("ZONE ONE"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::Value(100.0),
        floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            ep_model::InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            ep_model::OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: true,
        linked_outdoor_air_node: None,
        spaces: Vec::new(),
    });
    typed
        .thermostat_dual_setpoints
        .push(ThermostatDualSetpoint {
            id: ThermostatSetpointId(0),
            name: NormalizedName::new("DUAL SETPOINT"),
            heating_setpoint_schedule: ScheduleId(1),
            cooling_setpoint_schedule: ScheduleId(2),
        });
    typed.zone_thermostats.push(ZoneThermostat {
        id: ZoneThermostatId(0),
        name: NormalizedName::new("ZONE THERMOSTAT"),
        zone: ZoneId(0),
        control_type_schedule: ScheduleId(0),
        controls: vec![ZoneThermostatControl {
            object_type: ThermostatControlObjectType::DualSetpoint,
            dual_setpoint: ThermostatSetpointId(0),
        }],
        temperature_difference_between_cutout_and_setpoint_delta_c: 0.0,
    });
    for (id, name) in [
        (NodeId(0), "SUPPLY"),
        (NodeId(1), "ZONE AIR"),
        (NodeId(2), "RETURN"),
    ] {
        typed.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        typed.node_names.insert(name, id);
    }
    typed.ideal_loads_air_systems.push(ideal_loads_system());
    typed.zone_equipment_lists.push(ZoneEquipmentList {
        id: ZoneEquipmentListId(0),
        name: NormalizedName::new("ZONE EQUIPMENT"),
        load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
        equipment: vec![ZoneEquipmentListEntry {
            object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
            ideal_loads_air_system: IdealLoadsAirSystemId(0),
            cooling_sequence: 1,
            heating_or_no_load_sequence: 1,
            sequential_cooling_fraction_schedule: None,
            sequential_heating_fraction_schedule: None,
        }],
    });
    typed
        .zone_equipment_connections
        .push(ZoneEquipmentConnection {
            id: ZoneEquipmentConnectionId(0),
            zone: ZoneId(0),
            equipment_list: ZoneEquipmentListId(0),
            zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("SUPPLY")),
            zone_air_exhaust_node_or_nodelist_name: None,
            zone_air_node_name: NormalizedName::new("ZONE AIR"),
            zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("RETURN")),
            zone_return_air_node_1_flow_rate_fraction_schedule: None,
            zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
        });
    typed
}

fn ideal_loads_system() -> ep_model::IdealLoadsAirSystem {
    ep_model::IdealLoadsAirSystem {
        id: IdealLoadsAirSystemId(0),
        name: NormalizedName::new("ZONE IDEAL LOADS"),
        availability_schedule: Some(ScheduleId(3)),
        zone_supply_air_node_name: NormalizedName::new("SUPPLY"),
        zone_exhaust_air_node_name: None,
        system_inlet_air_node_name: None,
        maximum_heating_supply_air_temperature_c: 50.0,
        minimum_cooling_supply_air_temperature_c: 13.0,
        maximum_heating_supply_air_humidity_ratio: 0.0156,
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
        heating_limit: IdealLoadsLimit::NoLimit,
        maximum_heating_air_flow_rate_m3_per_s: None,
        maximum_sensible_heating_capacity_w: None,
        cooling_limit: IdealLoadsLimit::NoLimit,
        maximum_cooling_air_flow_rate_m3_per_s: None,
        maximum_total_cooling_capacity_w: None,
        heating_availability_schedule: None,
        cooling_availability_schedule: None,
        dehumidification_control_type: DehumidificationControlType::None,
        cooling_sensible_heat_ratio: 0.7,
        humidification_control_type: HumidificationControlType::None,
        design_specification_outdoor_air_object_name: None,
        outdoor_air_inlet_node_name: None,
        demand_controlled_ventilation_type: DemandControlledVentilationType::None,
        outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
        heat_recovery_type: HeatRecoveryType::None,
        sensible_heat_recovery_effectiveness: 0.7,
        latent_heat_recovery_effectiveness: 0.65,
        design_specification_zonehvac_sizing_object_name: None,
        heating_fuel_efficiency_schedule: None,
        heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
        cooling_fuel_efficiency_schedule: None,
        cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
    }
}

fn zone_state_for_temp_independent_load(temp_independent_load_w: f64) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id: ZoneId(0),
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio: 0.008,
        zone_timestep_average_air_humidity_ratio: 0.008,
        previous_air_humidity_ratios: [0.008; 3],
        previous_system_air_humidity_ratios: [0.008; 3],
        use_zone_timestep_history: false,
        shorten_timestep_sys: false,
        prior_timestep_seconds: 600.0,
        volume_m3: 100.0,
        air_heat_capacity_j_per_k: 0.0,
        convective_internal_gain_w: 0.0,
        opaque_surface_conductance_w_per_k: 100.0,
        opaque_surface_heat_gain_w: 0.0,
        opaque_surface_outside_conduction_w: 0.0,
        sum_ha_w_per_k: 100.0,
        sum_hat_surf_w: temp_independent_load_w,
        sum_hat_ref_w: 0.0,
        sum_mcp_w_per_k: 0.0,
        sum_mcp_t_w: 0.0,
        sum_sys_mcp_w_per_k: 7.0,
        sum_sys_mcp_t_w: 11.0,
        system_dependent_zone_loads_lagged_w: 0.0,
        zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        system_timestep_average_surface_convection_report_w: None,
        system_timestep_average_air_storage_report_w: None,
    }
}

fn schedule_mut(model: &mut TypedModel, schedule: ScheduleId) -> &mut ScheduleConstant {
    model
        .schedules
        .iter_mut()
        .find(|candidate| candidate.id == schedule)
        .expect("fixture schedule")
}

fn couple(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    cache: &ScheduleSeriesCache,
    state: &mut ZoneHeatBalanceState,
    sample_index: usize,
) -> Result<
    DirectZonePurchasedAirScheduledCouplingOutput,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    couple_model_bound_direct_zone_purchased_air(DirectZonePurchasedAirScheduledCouplingInput {
        binding,
        schedule_cache: cache,
        schedule_sample_index: sample_index,
        zone_state: state,
        purchased_air_runtime_state: &mut purchased_air_runtime_state,
        begin_environment: true,
        barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
        system_timestep_seconds: 600.0,
    })
}

fn assert_runtime_invariant(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    cache: &ScheduleSeriesCache,
    mut state: ZoneHeatBalanceState,
    invariant: DirectZonePurchasedAirRuntimeInvariant,
) {
    let original = state.clone();
    assert_eq!(
        couple(binding, cache, &mut state, 0).expect_err("runtime invariant must fail"),
        DirectZonePurchasedAirScheduledCouplingError::RuntimeInvariant { invariant }
    );
    assert_eq!(state, original);
}

fn assert_only_system_air_sums_changed(
    original: &ZoneHeatBalanceState,
    actual: &ZoneHeatBalanceState,
) {
    let mut expected = original.clone();
    expected.sum_sys_mcp_w_per_k = actual.sum_sys_mcp_w_per_k;
    expected.sum_sys_mcp_t_w = actual.sum_sys_mcp_t_w;
    assert_eq!(*actual, expected);
}
