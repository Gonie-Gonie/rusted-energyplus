//! Source-ordered output of one model-bound schedule sample.
#[rustfmt::skip] use super::super::{
    DirectZonePurchasedAirCouplingOutput, PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingConstantShrCaseBreakSnapshot, PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitSnapshot,
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
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
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
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
    PurchasedAirCalcEntrySnapshot, PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    PurchasedAirCalcHeatingModeGuardSnapshot,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot, PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot, PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot, PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    PurchasedAirCalcMinimumOaPrefixSnapshot, PurchasedAirInitSnapshot,
};
#[doc = "Output from one successful model-bound schedule sample and CP300 call."] #[derive(Clone, Copy, Debug, PartialEq)]
#[rustfmt::skip]
pub struct DirectZonePurchasedAirScheduledCouplingOutput {
    #[doc = "Fully resolved current schedule values."] pub schedules: super::DirectZonePurchasedAirScheduleSnapshot,
    #[doc = "Persistent initialization snapshot consumed by this Calc call."] pub initialization: PurchasedAirInitSnapshot,
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
    /// Source-ordered post-saturation purchased-air supply-enthalpy assignment snapshot.
    pub calculation_cooling_supply_enthalpy_post_saturation_assignment:
        PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    /// Source-ordered post-saturation cooling capacity-limit guard snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_guard:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
    /// Source-ordered post-saturation capacity-limit dehumidification guard snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
    /// Source-ordered post-saturation dehumidification total-output assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
    /// Source-ordered post-saturation dehumidification total-output capacity guard snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot,
    /// Source-ordered post-saturation dehumidification total-output maximum-capacity assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot,
    /// Source-ordered post-saturation capacity-limited supply-enthalpy assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
    /// Source-ordered post-saturation dehumidification-control switch snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot,
    /// Source-ordered constant-SHR case-entry and local `CpAir` assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    /// Source-ordered constant-SHR sensible-output assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
    /// Source-ordered constant-SHR supply-temperature assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot,
    /// Source-ordered constant-SHR supply-temperature mixed-air limit snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
    /// Source-ordered post-saturation constant-SHR supply-enthalpy overdrying-limit snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    /// Source-ordered post-saturation constant-SHR supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
    /// Source-ordered post-saturation constant-SHR case-break snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot,
    /// Source-ordered post-saturation Humidistat case-entry snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
    /// Source-ordered post-saturation Humidistat supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    /// Source-ordered post-saturation Humidistat case-break snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakSnapshot,
    /// Source-ordered post-saturation dehumidification-control `None` case-entry snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot,
    /// Source-ordered post-saturation shared `None`/constant-supply-humidity-ratio case-entry snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot,
    /// Source-ordered post-saturation shared `None`/constant-supply-humidity-ratio `CpAir` assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot,
    /// Source-ordered post-saturation shared `None`/constant-supply-humidity-ratio sensible-output assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot,
    /// Source-ordered post-saturation shared `None`/constant-supply-humidity-ratio latent-output assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot,
    /// Source-ordered post-saturation shared `None`/constant-supply-humidity-ratio latent-output capacity-guard snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot,
    /// Source-ordered post-saturation shared capacity-body supply-temperature mixed-air assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentSnapshot,
    /// Source-ordered post-saturation shared capacity-body supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentSnapshot,
    /// Source-ordered post-saturation shared capacity-body latent-output maximum-capacity assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot,
    /// Source-ordered post-saturation latent-output capacity-guard else-entry snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot,
    /// Source-ordered post-saturation latent-output supply-temperature assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot,
    /// Source-ordered post-saturation latent-output supply-temperature mixed-air-limit snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot,
    /// Source-ordered post-saturation shared `None`/constant-supply-humidity-ratio case-break snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot,
    /// Source-ordered post-saturation untyped-default case-break snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot,
    /// Source-ordered post-saturation pre-saturation original humidity-ratio assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot,
    /// Source-ordered post-saturation saturation humidity-ratio assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot,
    /// Source-ordered post-saturation saturation humidity-ratio guard snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot,
    /// Source-ordered post-saturation saturation supply-temperature assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot,
    /// Source-ordered post-saturation saturation-temperature mixed-air-limit snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot,
    /// Source-ordered post-saturation supply-humidity-ratio assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot,
    /// Source-ordered post-saturation supply-enthalpy assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot,
    /// Source-ordered post-saturation dehumidification-guard else-branch-entry snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot,
    /// Source-ordered post-saturation not-dehumidifying `CpAir` assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot,
    /// Source-ordered post-saturation not-dehumidifying sensible-output assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot,
    /// Source-ordered post-saturation sensible-output maximum-capacity guard snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardSnapshot,
    /// Source-ordered post-saturation sensible-output maximum-capacity assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentSnapshot,
    /// Source-ordered post-saturation sensible-output supply-temperature assignment snapshot.
    pub calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment:
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot,
    /// Source-ordered cooling positive-supply guard else-branch-entry snapshot.
    pub calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry:
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot,
    /// Source-ordered zero-flow supply-enthalpy mixed-air assignment snapshot.
    pub calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment:
        PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
    /// Source-ordered zero-flow supply-humidity-ratio mixed-air assignment snapshot.
    pub calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment:
        PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    /// Source-ordered zero-flow supply-temperature mixed-air assignment snapshot.
    pub calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
    /// Source-ordered zero-flow sensible-output positive-zero assignment snapshot.
    pub calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment: PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
    /// Source-ordered zero-flow total-output positive-zero assignment snapshot.
    pub calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment: PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
    /// Source-ordered Heating-or-no-load case-entry snapshot.
    pub calculation_heating_or_no_load_case_entry: PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    /// Source-ordered heating-mode guard snapshot.
    pub calculation_heating_mode_guard: PurchasedAirCalcHeatingModeGuardSnapshot,
    /// Source-ordered heating operating-mode Heat assignment snapshot.
    pub calculation_heating_operating_mode_heat_assignment: PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
    /// Source-ordered heating-mode guard else-branch-entry snapshot.
    pub calculation_heating_mode_guard_else_branch_entry: PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    /// Source-ordered heating operating-mode Deadband assignment snapshot.
    pub calculation_heating_operating_mode_deadband_assignment: PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    /// Source-ordered heating outdoor-air maximum-flow guard snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_guard: PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    /// Source-ordered heating maximum-flow-body volume-flow assignment snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment: PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
    /// Source-ordered heating maximum-flow first-warning guard snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_first_warning_guard: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    /// Source-ordered heating maximum-flow first-warning counter-increment snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    /// Source-ordered heating maximum-flow first-warning call-site snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_first_warning_call: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
    /// Source-ordered heating maximum-flow continue-warning call-site snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_continue_warning_call: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    /// Source-ordered heating maximum-flow continue-warning timestamp call-site snapshot.
    pub calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot,
    #[doc = "Predictor, PurchasedAir, and feedback result from CP300."] pub coupling: DirectZonePurchasedAirCouplingOutput,
}
