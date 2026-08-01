//! Source-ordered output of one model-bound schedule sample.

use super::super::{
    DirectZonePurchasedAirCouplingOutput, PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirCalcCoolingEntryGateSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingMixedAirCallSnapshot, PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot, PurchasedAirCalcEntrySnapshot,
    PurchasedAirCalcMinimumOaPrefixSnapshot, PurchasedAirInitSnapshot,
};
use super::DirectZonePurchasedAirScheduleSnapshot;

/// Output from one successful model-bound schedule sample and CP300 call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirScheduledCouplingOutput {
    /// Fully resolved current schedule values.
    pub schedules: DirectZonePurchasedAirScheduleSnapshot,
    /// Persistent initialization snapshot consumed by this Calc call.
    pub initialization: PurchasedAirInitSnapshot,
    /// Source-ordered `CalcPurchAirLoads` entry-prefix snapshot.
    pub calculation_entry: PurchasedAirCalcEntrySnapshot,
    /// Source-ordered minimum-outdoor-air prefix snapshot.
    pub calculation_minimum_outdoor_air: PurchasedAirCalcMinimumOaPrefixSnapshot,
    /// Source-ordered cooling-entry gate snapshot.
    pub calculation_cooling_entry_gate: PurchasedAirCalcCoolingEntryGateSnapshot,
    /// Source-ordered cooling OA maximum-flow gate snapshot.
    pub calculation_cooling_oa_max_flow_gate: PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    /// Source-ordered cooling OA maximum-flow warning-and-clamp body snapshot.
    pub calculation_cooling_oa_max_flow_body: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    /// Source-ordered cooling economizer guard snapshot.
    pub calculation_cooling_economizer_guard: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    /// Source-ordered cooling economizer differential condition snapshot.
    pub calculation_cooling_economizer_condition:
        PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    /// Source-ordered cooling economizer true-body snapshot.
    pub calculation_cooling_economizer_body: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    /// Source-ordered cooling sensible-flow snapshot.
    pub calculation_cooling_sensible_flow: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    /// Source-ordered cooling dehumidification-flow snapshot.
    pub calculation_cooling_dehumidification_flow:
        PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    /// Source-ordered cooling humidification-flow snapshot.
    pub calculation_cooling_humidification_flow: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    /// Source-ordered cooling capacity-zero candidate-reset snapshot.
    pub calculation_cooling_capacity_zero_flow_reset:
        PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    /// Source-ordered pre-EMS cooling supply mass-flow maximum snapshot.
    pub calculation_cooling_supply_mass_flow_maximum:
        PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    /// Source-ordered cooling supply mass-flow EMS-override guard snapshot.
    pub calculation_cooling_supply_mass_flow_ems_override_guard:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    /// Source-ordered cooling supply mass-flow EMS-override body snapshot.
    pub calculation_cooling_supply_mass_flow_ems_override_body:
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    /// Source-ordered cooling supply mass-flow limit-guard snapshot.
    pub calculation_cooling_supply_mass_flow_limit_guard:
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    /// Source-ordered cooling supply mass-flow limit-body snapshot.
    pub calculation_cooling_supply_mass_flow_limit_body:
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    /// Source-ordered cooling supply mass-flow very-small guard snapshot.
    pub calculation_cooling_supply_mass_flow_very_small_guard:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    /// Source-ordered cooling supply mass-flow positive-zero reset-body snapshot.
    pub calculation_cooling_supply_mass_flow_very_small_guard_body:
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    /// Source-ordered Cooling mixed-air call and bounded no-OA child snapshot.
    pub calculation_cooling_mixed_air_call: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    /// Source-ordered cooling positive supply-mass-flow guard snapshot.
    pub calculation_cooling_supply_mass_flow_positive_guard:
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    /// Source-ordered cooling positive-supply Cp-air assignment snapshot.
    pub calculation_cooling_positive_supply_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    /// Source-ordered cooling positive-supply temperature assignment snapshot.
    pub calculation_cooling_positive_supply_temperature_assignment:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    /// Source-ordered cooling positive-supply minimum-temperature limit snapshot.
    pub calculation_cooling_positive_supply_temperature_minimum_limit:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    /// Source-ordered cooling positive-supply mixed-air-temperature limit snapshot.
    pub calculation_cooling_positive_supply_temperature_mixed_air_limit:
        PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    /// Source-ordered cooling positive-supply mixed-air humidity-ratio assignment snapshot.
    pub calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    /// Source-ordered cooling positive-supply enthalpy assignment snapshot.
    pub calculation_cooling_positive_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit guard snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_guard:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit Cp-air assignment snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitCpAirAssignmentSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit sensible-output assignment snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit sensible-output guard snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_sensible_output_guard:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot,
    /// Source-ordered cooling positive-supply maximum-capacity assignment snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit supply-enthalpy assignment snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit supply-temperature assignment snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    /// Source-ordered cooling positive-supply capacity-limit supply-temperature mixed-air limit snapshot.
    pub calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot,
    /// Source-ordered post-capacity-limit mixed-air humidity-ratio assignment snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
    /// Source-ordered post-capacity-limit dehumidification-control switch snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    /// Source-ordered dehumidification-control None-case snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    /// Source-ordered constant-sensible-heat-ratio case-entry snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    /// Source-ordered constant-sensible-heat-ratio CpAir-assignment snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    /// Source-ordered constant-sensible-heat-ratio sensible-output-assignment snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    /// Source-ordered constant-sensible-heat-ratio total-output-assignment snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot,
    /// Source-ordered constant-sensible-heat-ratio supply-enthalpy-assignment snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot,
    /// Source-ordered constant-SHR supply-enthalpy overdrying-limit snapshot.
    pub calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    /// Source-ordered constant-SHR supply-humidity-ratio overdrying-limit snapshot.
    pub calculation_cooling_constant_shr_supply_humidity_ratio_overdrying_limit:
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitSnapshot,
    /// Source-ordered constant-SHR supply-humidity-ratio minimum-limit snapshot.
    pub calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit:
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
    /// Source-ordered constant-SHR supply-humidity-ratio mixed-air-limit snapshot.
    pub calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit:
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    /// Source-ordered constant-SHR case-break snapshot.
    pub calculation_cooling_constant_shr_case_break:
        PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot,
    /// Source-ordered Humidistat case-entry snapshot.
    pub calculation_cooling_humidistat_case_entry:
        PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot,
    /// Source-ordered Humidistat moisture-demand assignment snapshot.
    pub calculation_cooling_humidistat_moisture_demand_assignment:
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot,
    /// Source-ordered Humidistat supply-humidity-ratio-for-dehumidification assignment snapshot.
    pub calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    /// Source-ordered Humidistat supply-humidity-ratio-for-dehumidification minimum-limit snapshot.
    pub calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
    /// Source-ordered Humidistat purchased-air supply-humidity-ratio mixed-air-limit snapshot.
    pub calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit:
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    /// Source-ordered Humidistat case-break snapshot.
    pub calculation_cooling_humidistat_case_break:
        PurchasedAirCalcCoolingHumidistatCaseBreakSnapshot,
    /// Source-ordered constant-supply-humidity-ratio case-entry snapshot.
    pub calculation_cooling_constant_supply_humidity_ratio_case_entry:
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot,
    /// Source-ordered constant-supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_constant_supply_humidity_ratio_assignment:
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentSnapshot,
    /// Source-ordered constant-supply-humidity-ratio case-break snapshot.
    pub calculation_cooling_constant_supply_humidity_ratio_case_break:
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseBreakSnapshot,
    /// Source-ordered default supply-humidity-ratio mixed-air assignment snapshot.
    pub calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment:
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentSnapshot,
    /// Source-ordered default supply-humidity-ratio case-break snapshot.
    pub calculation_cooling_default_supply_humidity_ratio_case_break:
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot,
    /// Source-ordered Cooling humidification heating-availability guard snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot,
    /// Source-ordered Cooling humidification-control Humidistat guard snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot,
    /// Source-ordered nested dehumidification-control Humidistat-or-None guard snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot,
    /// Source-ordered humidifying-setpoint moisture-demand assignment snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshot,
    /// Source-ordered humidification supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot,
    /// Source-ordered humidification supply-humidity-ratio maximum-limit snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitSnapshot,
    /// Source-ordered humidification supply-humidity-ratio maximum-assignment snapshot.
    pub calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment:
        PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioMaximumAssignmentSnapshot,
    /// Source-ordered pre-saturation original supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment:
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    /// Source-ordered saturation supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_supply_humidity_ratio_saturation_assignment:
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot,
    /// Source-ordered saturation-limit minimum and final supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_supply_humidity_ratio_saturation_limit_assignment:
        PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot,
    /// Predictor, PurchasedAir, and feedback result from CP300.
    pub coupling: DirectZonePurchasedAirCouplingOutput,
}
