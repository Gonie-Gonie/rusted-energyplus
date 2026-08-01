//! Persistent CP376 pre-saturation original-assignment state.

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) enum PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRetainedRoute
{
    UnitOff,
    NonCooling,
    PositiveGuardFalseFallthrough,
    HeatingAvailabilityGuardFalseFallthrough,
    HumidificationControlGuardFalseFallthrough,
    DehumidificationControlHumidistatSupplyHumidityRatioMaximumAssignmentExecuted,
    DehumidificationControlNoneSupplyHumidityRatioMaximumAssignmentExecuted,
    DehumidificationControlGuardFalseFallthrough,
}

/// Persistent bounded state and exact source/owner counters for one system.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState {
    pub system: IdealLoadsAirSystemId,
    pub transition_count: usize,
    pub unit_off_skip_count: usize,
    pub non_cooling_skip_count: usize,
    pub positive_guard_false_fallthrough_skip_count: usize,
    pub heating_availability_guard_false_fallthrough_count: usize,
    pub humidification_control_guard_false_fallthrough_count: usize,
    pub dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count: usize,
    pub dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count: usize,
    pub dehumidification_control_guard_false_fallthrough_count: usize,
    pub source_site_execution_count: usize,
    pub purchased_air_supply_humidity_ratio_before_saturation_limit_read_count: usize,
    pub local_original_supply_humidity_ratio_before_saturation_limit_assignment_count: usize,
    pub cp375_maximum_assignment_owner_count: usize,
    pub cp347_none_case_owner_count: usize,
    pub cp356_constant_shr_owner_count: usize,
    pub cp362_humidistat_owner_count: usize,
    pub cp365_constant_supply_humidity_ratio_owner_count: usize,
    pub latest:
        Option<PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot>,
    pub(super) latest_route: Option<
        PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRetainedRoute,
    >,
    pub(super) latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentRuntimeState {
    /// Creates zeroed CP376 state for one system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            positive_guard_false_fallthrough_skip_count: 0,
            heating_availability_guard_false_fallthrough_count: 0,
            humidification_control_guard_false_fallthrough_count: 0,
            dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count: 0,
            dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count: 0,
            dehumidification_control_guard_false_fallthrough_count: 0,
            source_site_execution_count: 0,
            purchased_air_supply_humidity_ratio_before_saturation_limit_read_count: 0,
            local_original_supply_humidity_ratio_before_saturation_limit_assignment_count: 0,
            cp375_maximum_assignment_owner_count: 0,
            cp347_none_case_owner_count: 0,
            cp356_constant_shr_owner_count: 0,
            cp362_humidistat_owner_count: 0,
            cp365_constant_supply_humidity_ratio_owner_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}
