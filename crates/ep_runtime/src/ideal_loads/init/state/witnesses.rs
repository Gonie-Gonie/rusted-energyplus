#[rustfmt::skip] mod cooling_constant_shr_case_break; #[rustfmt::skip] mod cooling_constant_shr_supply_humidity_ratio_minimum_limit;
mod cooling_constant_shr_supply_humidity_ratio_mixed_air_limit;
mod cooling_constant_shr_supply_humidity_ratio_overdrying_limit;
#[rustfmt::skip] mod cooling_humidistat_case_break; #[rustfmt::skip] mod cooling_humidistat_case_entry; #[rustfmt::skip] mod cooling_humidistat_moisture_demand_assignment; #[rustfmt::skip] mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment; #[rustfmt::skip] mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit; #[rustfmt::skip] mod cooling_humidistat_supply_humidity_ratio_mixed_air_limit; #[rustfmt::skip] mod cooling_constant_supply_humidity_ratio_case_entry; #[rustfmt::skip] mod cooling_constant_supply_humidity_ratio_assignment; #[rustfmt::skip] mod cooling_constant_supply_humidity_ratio_case_break; #[rustfmt::skip] mod cooling_default_supply_humidity_ratio_mixed_air_assignment; #[rustfmt::skip] mod cooling_default_supply_humidity_ratio_case_break; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_heating_availability_guard; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_control_humidistat_guard; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_moisture_demand_assignment; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit; #[rustfmt::skip] mod cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
#[rustfmt::skip] mod cooling_supply_humidity_ratio_pre_saturation_original_assignment; #[rustfmt::skip] mod cooling_supply_humidity_ratio_saturation_assignment; #[rustfmt::skip] mod cooling_supply_humidity_ratio_saturation_limit_assignment; #[rustfmt::skip] mod cooling_supply_enthalpy_post_saturation_assignment; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_guard; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_guard; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_total_output_guard; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment; #[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_switch;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
#[rustfmt::skip] mod cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;
#[rustfmt::skip] mod cooling_mixed_air_call; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_cp_air_assignment; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_guard; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_sensible_output_assignment; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_sensible_output_guard; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment; #[rustfmt::skip] mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit; #[rustfmt::skip] mod cooling_positive_supply_cp_air_assignment; #[rustfmt::skip] mod cooling_positive_supply_enthalpy_assignment; #[rustfmt::skip] mod cooling_positive_supply_humidity_ratio_mixed_air_assignment; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_switch; #[rustfmt::skip] mod cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment; #[rustfmt::skip] mod cooling_positive_supply_temperature_assignment; #[rustfmt::skip] mod cooling_positive_supply_temperature_minimum_limit; #[rustfmt::skip] mod cooling_positive_supply_temperature_mixed_air_limit; #[rustfmt::skip] mod cooling_supply_mass_flow_positive_guard; #[rustfmt::skip] mod cooling_supply_mass_flow_very_small_guard_body;

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
