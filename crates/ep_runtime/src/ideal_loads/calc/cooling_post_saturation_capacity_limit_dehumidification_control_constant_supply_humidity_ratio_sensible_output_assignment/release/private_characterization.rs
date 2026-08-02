//! Restricted pure CP400 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners as ActiveOwners,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as FlowOwner,
};

/// Characterizes one non-public CP400 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization(
    predecessor: Predecessor,
    mixed_air_owner: Option<MixedAirOwner>,
    supply_mass_flow_owner: Option<FlowOwner>,
) -> Option<Snapshot> {
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    let active_owners = match route.active {
        true => Some(ActiveOwners {
            mixed_air_owner: mixed_air_owner?,
            supply_mass_flow_owner: supply_mass_flow_owner?,
        }),
        false if mixed_air_owner.is_none() && supply_mass_flow_owner.is_none() => None,
        false => return None,
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state(
        &mut state,
        predecessor,
        active_owners,
    )
}
