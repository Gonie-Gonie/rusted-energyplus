//! Source-valid CP379/CP388 and active CP329/CP330/CP387 fixtures.

use ep_model::DehumidificationControlType as D;

use super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::tests::fixtures as cp388_fixtures;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallActiveInput,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state,
    advance_cooling_mixed_air_call_state,
    advance_cooling_supply_mass_flow_positive_guard_state,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Cp387,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as Cp388State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Cp388,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Cp379,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[derive(Clone, Copy)]
pub(super) struct Chain {
    pub cp379: Cp379,
    pub cp387: Cp387,
    pub cp388: Cp388,
    pub formula_owners: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners,
}

impl Chain {
    pub(super) const fn retained_input(
        self,
    ) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput{
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput {
            cp379_temperature_owner: self.cp379,
            active_owners: if self.cp388.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed {
                Some(self.formula_owners)
            } else {
                None
            },
        }
    }
}

pub(super) fn chain(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    selector: Option<D>,
    ordinal: usize,
    ratio: f64,
    supply_temperature_c: f64,
    flow: f64,
) -> Chain {
    let base = cp388_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        if assignment { 99.0 } else { 100.0 },
        50_000.0,
        0.008,
    );
    let mut cp388_state = Cp388State::new(base.cp387.system);
    let cp388_input = (selector == Some(D::ConstantSensibleHeatRatio))
        .then(|| cp388_fixtures::input(base, ratio));
    let cp388 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
        &mut cp388_state,
        base.cp387,
        cp388_input,
    )
    .expect("CP388");
    let cp379 = cp379_owner(cp388, supply_temperature_c);
    let formula_owners = formula_owners(cp388, base.cp387, flow);
    Chain {
        cp379,
        cp387: base.cp387,
        cp388,
        formula_owners,
    }
}

fn cp379_owner(predecessor: Cp388, temperature: f64) -> Cp379 {
    let active = !predecessor.unit_off_skipped
        && !predecessor.non_cooling_skipped
        && !predecessor.positive_guard_false_fallthrough_skipped;
    let selector = if !active {
        None
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        Some(D::Humidistat)
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        Some(D::None)
    } else if predecessor.dehumidification_control_guard_false_fallthrough {
        Some(
            predecessor
                .predecessor_dehumidification_control_type
                .unwrap_or(D::ConstantSensibleHeatRatio),
        )
    } else {
        Some(
            predecessor
                .predecessor_dehumidification_control_type
                .unwrap_or(D::None),
        )
    };
    let humidity = active.then_some(0.008);
    let enthalpy = active.then(|| energyplus_psy_h_fn_tdb_w(temperature, 0.008));
    Cp379 {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_ENTHALPY_POST_SATURATION_ASSIGNMENT_SOURCE_ORDER,
        system: predecessor.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor.dehumidification_control_guard_false_fallthrough,
        predecessor_dehumidification_control_type: selector,
        predecessor_supply_humidity_ratio_saturation_limit_assignment_performed: active,
        predecessor_resulting_supply_humidity_ratio: humidity,
        cp377_supply_temperature_owned_read: active,
        cp334_supply_temperature_mixed_air_limit_owned_read: active,
        cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read: false,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: active,
        purchased_air_supply_temperature_for_post_saturation_enthalpy_read: active,
        supply_temperature_c: active.then_some(temperature),
        purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read: active,
        supply_humidity_ratio: humidity,
        psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated: active,
        psychrometric_supply_enthalpy_j_per_kg: enthalpy,
        local_supply_enthalpy_after_saturation_limit_assignment_performed: active,
        assigned_supply_enthalpy_j_per_kg: enthalpy,
        resulting_supply_enthalpy_j_per_kg: enthalpy,
    }
}

fn formula_owners(
    predecessor: Cp388,
    cp_air_owner: Cp387,
    flow: f64,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners{
    let mut mixed_predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    mixed_predecessor.system = predecessor.system;
    mixed_predecessor.parent_call_ordinal = predecessor.parent_call_ordinal;
    mixed_predecessor.controlled_zone = predecessor.controlled_zone;
    let temperature = 23.5;
    let humidity = 0.008;
    let active_input =
        PurchasedAirCalcCoolingMixedAirCallActiveInput {
            recirculation_node: ep_model::NodeId(9),
            recirculation_temperature_c: temperature,
            recirculation_humidity_ratio: humidity,
            recirculation_enthalpy_projection_j_per_kg:
                crate::ideal_loads::moist_air_enthalpy_j_per_kg(temperature, humidity),
            outdoor_air_mass_flow_rate_kg_per_s: 0.0,
            supply_mass_flow_rate_kg_per_s: flow,
        };
    let mut mixed_state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    let mixed_air_owner = advance_cooling_mixed_air_call_state(
        &mut mixed_state,
        mixed_predecessor,
        Some(active_input),
    );
    let mut flow_state =
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
    let supply_mass_flow_owner =
        advance_cooling_supply_mass_flow_positive_guard_state(&mut flow_state, mixed_air_owner);
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners {
        mixed_air_owner,
        supply_mass_flow_owner,
        cp_air_owner,
    }
}
