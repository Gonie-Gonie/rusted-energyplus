//! Restricted pure CP389 route characterization.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners as ActiveOwners,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput as RetainedInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as CpAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as TemperatureOwner,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as FlowOwner,
};

/// Characterizes one non-public CP389 route without mutating retained runtime state.
#[allow(dead_code)]
pub(in crate::ideal_loads) fn private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_characterization(
    predecessor: Predecessor,
    temperature_owner: TemperatureOwner,
    mixed_air_owner: Option<MixedAirOwner>,
    supply_mass_flow_owner: Option<FlowOwner>,
    cp_air_owner: Option<CpAirOwner>,
) -> Option<Snapshot> {
    let route = super::super::transition::routes::predecessor_route(predecessor)?;
    let active_owners = match route.active {
        true => Some(ActiveOwners {
            mixed_air_owner: mixed_air_owner?,
            supply_mass_flow_owner: supply_mass_flow_owner?,
            cp_air_owner: cp_air_owner?,
        }),
        false
            if mixed_air_owner.is_none()
                && supply_mass_flow_owner.is_none()
                && cp_air_owner.is_none() =>
        {
            None
        }
        false => return None,
    };
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        &mut state,
        predecessor,
        RetainedInput {
            cp379_temperature_owner: temperature_owner,
            active_owners,
        },
    )
}
