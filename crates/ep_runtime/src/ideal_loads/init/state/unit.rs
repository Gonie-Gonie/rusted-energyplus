//! Construction and public flag projection for one retained unit.

use super::*;

impl PurchasedAirUnitRuntimeState {
    pub(in crate::ideal_loads::init) const fn new(
        system: IdealLoadsAirSystemId,
        planned_first_matching_equipment_list: Option<ZoneEquipmentListId>,
    ) -> Self {
        Self {
            system,
            one_time_latched: false,
            topology_completed: false,
            sizing_needed: true,
            sized_limits: None,
            sizing_outcome: None,
            environment_initialization_needed: true,
            controlled_zone: None,
            equipment_list: None,
            supply_node: None,
            recirculation_node: None,
            recirculation_source: None,
            calc_entry: PurchasedAirCalcEntryRuntimeState::new(system),
            calc_minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixRuntimeState::new(system),
            calc_cooling_entry_gate: PurchasedAirCalcCoolingEntryGateRuntimeState::new(system),
            calc_cooling_oa_max_flow_gate: PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState::new(
                system,
            ),
            calc_cooling_oa_max_flow_body: PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState::new(
                system,
            ),
            calc_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardRuntimeState::new(
                system,
            ),
            calc_cooling_economizer_condition:
                PurchasedAirCalcCoolingEconomizerConditionRuntimeState::new(system),
            calc_cooling_economizer_body: PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(
                system,
            ),
            calc_cooling_sensible_flow: PurchasedAirCalcCoolingSensibleFlowRuntimeState::new(
                system,
            ),
            calc_cooling_dehumidification_flow:
                PurchasedAirCalcCoolingDehumidificationFlowRuntimeState::new(system),
            calc_cooling_humidification_flow:
                PurchasedAirCalcCoolingHumidificationFlowRuntimeState::new(system),
            calc_cooling_capacity_zero_flow_reset:
                PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState::new(system),
            calc_cooling_supply_mass_flow_maximum:
                PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState::new(system),
            calc_cooling_supply_mass_flow_ems_override_guard:
                PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(system),
            calc_cooling_supply_mass_flow_ems_override_body:
                PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(system),
            calc_cooling_supply_mass_flow_limit_guard:
                PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(system),
            calc_cooling_supply_mass_flow_limit_body:
                PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(system),
            calc_cooling_supply_mass_flow_very_small_guard:
                PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(system),
            calc_cooling_supply_mass_flow_very_small_guard_body:
                PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(system),
            calc_cooling_mixed_air_call: PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(
                system,
            ),
            calc_cooling_supply_mass_flow_positive_guard:
                PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(system),
            calc_cooling_positive_supply_cp_air_assignment:
                PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentRuntimeState::new(system),
            calc_cooling_positive_supply_temperature_assignment:
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentRuntimeState::new(system),
            calc_cooling_positive_supply_temperature_minimum_limit:
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_temperature_mixed_air_limit:
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment:
                PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_enthalpy_assignment:
                PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(system),
            calc_cooling_positive_supply_capacity_limit_guard:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(system),
            calc_cooling_positive_supply_capacity_limit_cp_air_assignment:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_capacity_limit_sensible_output_assignment:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_capacity_limit_sensible_output_guard:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit:
                PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit:
                PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit:
                PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit:
                PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_constant_shr_case_break:
                PurchasedAirCalcCoolingConstantShrCaseBreakRuntimeState::new(system),
            calc_cooling_humidistat_case_entry:
                PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState::new(system),
            calc_cooling_humidistat_moisture_demand_assignment:
                PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState::new(system),
            calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment:
                PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit:
                PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit:
                PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitRuntimeState::new(
                    system,
                ),
            calc_cooling_humidistat_case_break:
                PurchasedAirCalcCoolingHumidistatCaseBreakRuntimeState::new(system),
            calc_cooling_constant_supply_humidity_ratio_case_entry:
                PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState::new(system),
            calc_cooling_constant_supply_humidity_ratio_assignment:
                PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_constant_supply_humidity_ratio_case_break:
                PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakRuntimeState::new(
                    system,
                ),
            calc_cooling_default_supply_humidity_ratio_mixed_air_assignment:
                PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
                    system,
                ),
            calc_cooling_default_supply_humidity_ratio_case_break:
                PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment:
                PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment:
                PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_saturation_assignment:
                PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState::new(system),
            calc_cooling_supply_humidity_ratio_saturation_limit_assignment:
                PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState::new(system),
            calc_cooling_supply_enthalpy_post_saturation_assignment:
                PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_guard:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_guard:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntryRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState::new(system),
            calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard:
                PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState::new(system),
            rejected_exhaust_node: None,
            reported_first_return_node: None,
            topology_plan: None,
            topology_diagnostics: Vec::new(),
            topology_failure: None,
            planned_first_matching_equipment_list,
            equipment_list_scan_ordinal: None,
            first_matching_equipment_list: None,
            equipment_list_membership_found: None,
            maximum_heating_air_mass_flow_rate_kg_per_s: 0.0,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
            standard_air_density_kg_per_m3: None,
            init_call_count: 0,
            one_time_initialization_count: 0,
            topology_completion_count: 0,
            sizing_check_count: 0,
            sizing_attempt_count: 0,
            environment_initialization_count: 0,
            environment_rearm_count: 0,
            cooling_supply_temperature_error_index: 0,
            heating_supply_temperature_error_index: 0,
            cooling_supply_temperature_first_diagnostic_count: 0,
            heating_supply_temperature_first_diagnostic_count: 0,
            cooling_supply_temperature_warning_count: 0,
            heating_supply_temperature_warning_count: 0,
            economizer_flow_limit_warning_count: 0,
        }
    }

    /// Source-shaped flag snapshot after the latest call.
    #[must_use]
    pub fn flags(&self, equipment_list_checked: bool) -> IdealLoadsInitFlags {
        IdealLoadsInitFlags {
            state_machine_used: true,
            one_time_checked: self.one_time_latched,
            topology_ready: self.topology_completed && self.recirculation_node.is_some(),
            environment_initialized: self.environment_initialization_count > 0,
            environment_initialization_needed: self.environment_initialization_needed,
            sizing_checked: !self.sizing_needed,
            equipment_list_checked,
            return_plenum_inactive: true,
        }
    }
}
