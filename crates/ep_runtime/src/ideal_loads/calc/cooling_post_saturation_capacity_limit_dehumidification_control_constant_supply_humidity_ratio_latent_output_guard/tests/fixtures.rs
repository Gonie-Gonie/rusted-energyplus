//! CP401 predecessor fixtures spanning all thirty retained routes.

use ep_model::DehumidificationControlType as D;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_state,
};

use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::tests::fixtures as cp388_fixtures;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry::tests::fixtures as cp398_fixtures;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentActiveOwners as Cp401Owners,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners as Cp400Owners,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state,
    advance_cooling_supply_mass_flow_positive_guard_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState as Cp398State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Cp398,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentActiveInput as Cp399Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState as Cp399State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState as Cp401State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as Cp400State,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
};

pub(super) fn all_predecessors() -> Vec<Predecessor> {
    let mut predecessors = Vec::new();
    let mut ordinal = 1;
    for inherited in 0..3 {
        predecessors.push(chain(
            inherited, 0, false, None, ordinal, 0.7, 18.0, 1.0, 0.001,
        ));
        ordinal += 1;
    }
    for inherited in 3..8 {
        for outcome in [0, 2, 1] {
            predecessors.push(chain(
                inherited, outcome, false, None, ordinal, 0.7, 18.0, 1.0, 0.001,
            ));
            ordinal += 1;
        }
    }
    for inherited in [3, 4] {
        for selector in [
            D::ConstantSensibleHeatRatio,
            D::Humidistat,
            D::None,
            D::ConstantSupplyHumidityRatio,
        ] {
            predecessors.push(chain(
                inherited,
                1,
                true,
                Some(selector),
                ordinal,
                0.7,
                18.0,
                1.0,
                0.001,
            ));
            ordinal += 1;
        }
    }
    for (inherited, selectors) in [
        (5, &[D::Humidistat][..]),
        (6, &[D::None][..]),
        (
            7,
            &[D::ConstantSensibleHeatRatio, D::ConstantSupplyHumidityRatio][..],
        ),
    ] {
        for selector in selectors {
            predecessors.push(chain(
                inherited,
                1,
                true,
                Some(*selector),
                ordinal,
                0.65,
                19.0,
                1.0,
                0.001,
            ));
            ordinal += 1;
        }
    }
    predecessors
}

pub(super) fn active_input(predecessor: Predecessor, capacity: f64) -> Option<ActiveInput> {
    predecessor.cooling_latent_output_w.map(|cooling_latent_output_w| ActiveInput {
        cooling_latent_output_w,
        maximum_total_cooling_capacity_w: capacity,
        cp401_cooling_latent_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    })
}

pub(super) fn advance(
    state: &mut State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_state(
        state,
        predecessor,
        input,
    )
}

#[allow(clippy::too_many_arguments)]
fn chain(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    selector: Option<D>,
    ordinal: usize,
    ratio: f64,
    supply_temperature_c: f64,
    flow: f64,
    formula_flow: f64,
) -> Predecessor {
    let base = cp398_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut cp398_state = Cp398State::new(base.cp397.system);
    let cp398: Cp398 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(
        &mut cp398_state,
        base.cp397,
    )
    .expect("CP398");
    let active = assignment
        && matches!(selector, Some(D::None | D::ConstantSupplyHumidityRatio));
    let mut cp399_state = Cp399State::new(cp398.system);
    let cp399 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state(
        &mut cp399_state,
        cp398,
        active.then_some(Cp399Input {
            mixed_air_humidity_ratio: 0.007_25,
        }),
    )
    .expect("CP399");
    let mut cp400_state = Cp400State::new(cp399.system);
    let cp400 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state(
        &mut cp400_state,
        cp399,
        active.then(|| cp400_owners(cp399, formula_flow, 24.0)),
    )
    .expect("CP400");
    let owner_chain = cp388_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        if assignment { 99.0 } else { 100.0 },
        50_000.0,
        0.008,
    );
    let owners = active.then_some(Cp401Owners {
        cooling_total_output_owner: owner_chain.cp384,
        cooling_total_output_corroborator: owner_chain.cp385,
    });
    let mut cp401_state = Cp401State::new(cp400.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state(
        &mut cp401_state,
        cp400,
        owners,
    )
    .expect("CP401")
}

fn cp400_owners(
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot,
    flow: f64,
    mixed_temperature: f64,
) -> Cp400Owners {
    let mut mixed_predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    mixed_predecessor.system = predecessor.system;
    mixed_predecessor.parent_call_ordinal = predecessor.parent_call_ordinal;
    mixed_predecessor.controlled_zone = predecessor.controlled_zone;
    let humidity = 0.008;
    let mut mixed_state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    let mixed_air_owner = advance_cooling_mixed_air_call_state(
        &mut mixed_state,
        mixed_predecessor,
        Some(PurchasedAirCalcCoolingMixedAirCallActiveInput {
            recirculation_node: ep_model::NodeId(9),
            recirculation_temperature_c: mixed_temperature,
            recirculation_humidity_ratio: humidity,
            recirculation_enthalpy_projection_j_per_kg:
                crate::ideal_loads::moist_air_enthalpy_j_per_kg(mixed_temperature, humidity),
            outdoor_air_mass_flow_rate_kg_per_s: 0.0,
            supply_mass_flow_rate_kg_per_s: flow,
        }),
    );
    let mut flow_state =
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
    let supply_mass_flow_owner =
        advance_cooling_supply_mass_flow_positive_guard_state(&mut flow_state, mixed_air_owner);
    Cp400Owners {
        mixed_air_owner,
        supply_mass_flow_owner,
    }
}
