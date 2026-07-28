//! Private latest-witness access for source-ordered calculation stages.

mod cooling_mixed_air_call;
mod cooling_positive_supply_capacity_limit_guard;
mod cooling_positive_supply_cp_air_assignment;
mod cooling_positive_supply_enthalpy_assignment;
mod cooling_positive_supply_humidity_ratio_mixed_air_assignment;
mod cooling_positive_supply_temperature_assignment;
mod cooling_positive_supply_temperature_minimum_limit;
mod cooling_positive_supply_temperature_mixed_air_limit;
mod cooling_supply_mass_flow_positive_guard;
mod cooling_supply_mass_flow_very_small_guard_body;

use ep_model::IdealLoadsAirSystemId;

use super::PurchasedAirRuntimeState;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot, PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

impl PurchasedAirRuntimeState {
    pub(in crate::ideal_loads) fn cooling_economizer_condition_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot> {
        self.cooling_economizer_condition_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_economizer_condition_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    ) {
        self.cooling_economizer_condition_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_economizer_body_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingEconomizerBodySnapshot> {
        self.cooling_economizer_body_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_economizer_body_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    ) {
        self.cooling_economizer_body_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_sensible_flow_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSensibleFlowSnapshot> {
        self.cooling_sensible_flow_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_sensible_flow_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    ) {
        self.cooling_sensible_flow_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_dehumidification_flow_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingDehumidificationFlowSnapshot> {
        self.cooling_dehumidification_flow_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_dehumidification_flow_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    ) {
        self.cooling_dehumidification_flow_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_humidification_flow_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingHumidificationFlowSnapshot> {
        self.cooling_humidification_flow_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_humidification_flow_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    ) {
        self.cooling_humidification_flow_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_capacity_zero_flow_reset_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot> {
        self.cooling_capacity_zero_flow_reset_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_capacity_zero_flow_reset_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    ) {
        self.cooling_capacity_zero_flow_reset_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_maximum_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot> {
        self.cooling_supply_mass_flow_maximum_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_maximum_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    ) {
        self.cooling_supply_mass_flow_maximum_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_ems_override_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot> {
        self.cooling_supply_mass_flow_ems_override_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_ems_override_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    ) {
        self.cooling_supply_mass_flow_ems_override_guard_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_ems_override_body_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot> {
        self.cooling_supply_mass_flow_ems_override_body_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_ems_override_body_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    ) {
        self.cooling_supply_mass_flow_ems_override_body_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_limit_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot> {
        self.cooling_supply_mass_flow_limit_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_limit_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    ) {
        self.cooling_supply_mass_flow_limit_guard_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_limit_body_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot> {
        self.cooling_supply_mass_flow_limit_body_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_limit_body_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    ) {
        self.cooling_supply_mass_flow_limit_body_latest_witnesses
            .insert(system, snapshot);
    }

    pub(in crate::ideal_loads) fn cooling_supply_mass_flow_very_small_guard_latest_witness(
        &self,
        system: IdealLoadsAirSystemId,
    ) -> Option<PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot> {
        self.cooling_supply_mass_flow_very_small_guard_latest_witnesses
            .get(&system)
            .copied()
    }

    pub(in crate::ideal_loads) fn set_cooling_supply_mass_flow_very_small_guard_latest_witness(
        &mut self,
        system: IdealLoadsAirSystemId,
        snapshot: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    ) {
        self.cooling_supply_mass_flow_very_small_guard_latest_witnesses
            .insert(system, snapshot);
    }
}
