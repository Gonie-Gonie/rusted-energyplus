//! Source-ordered output of one model-bound schedule sample.

use super::super::{
    DirectZonePurchasedAirCouplingOutput, PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirCalcCoolingEntryGateSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot, PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
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
    /// Predictor, PurchasedAir, and feedback result from CP300.
    pub coupling: DirectZonePurchasedAirCouplingOutput,
}
