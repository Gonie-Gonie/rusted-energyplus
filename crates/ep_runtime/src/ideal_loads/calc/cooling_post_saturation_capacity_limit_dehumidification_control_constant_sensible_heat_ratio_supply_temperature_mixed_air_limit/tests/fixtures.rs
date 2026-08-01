//! Source-valid CP389 and active CP329 fixtures.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment::tests::fixtures as cp389_fixtures;
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallActiveInput,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329,
    PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState as Cp389State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Cp389,
};

#[derive(Clone, Copy)]
pub(super) struct Chain {
    pub cp389: Cp389,
    pub mixed_air_owner: Cp329,
}

pub(super) fn alternate_exact_mixed_air_owner(chain: Chain, temperature: f64) -> Cp329 {
    let mut predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    predecessor.system = chain.cp389.system;
    predecessor.parent_call_ordinal = chain.cp389.parent_call_ordinal;
    predecessor.controlled_zone = chain.cp389.controlled_zone;
    let humidity = 0.008;
    let flow = chain
        .mixed_air_owner
        .supply_mass_flow_rate_kg_per_s
        .expect("flow");
    let input =
        PurchasedAirCalcCoolingMixedAirCallActiveInput {
            recirculation_node: ep_model::NodeId(9),
            recirculation_temperature_c: temperature,
            recirculation_humidity_ratio: humidity,
            recirculation_enthalpy_projection_j_per_kg:
                crate::ideal_loads::moist_air_enthalpy_j_per_kg(temperature, humidity),
            outdoor_air_mass_flow_rate_kg_per_s: 0.0,
            supply_mass_flow_rate_kg_per_s: flow,
        };
    let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(chain.cp389.system);
    advance_cooling_mixed_air_call_state(&mut state, predecessor, Some(input))
}

impl Chain {
    pub(super) fn owner(self) -> Option<Cp329> {
        self.cp389
            .dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed
            .then_some(self.mixed_air_owner)
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
    let base = cp389_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp389State::new(base.cp388.system);
    let cp389 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        &mut state,
        base.cp388,
        base.retained_input(),
    )
    .expect("CP389");
    Chain {
        cp389,
        mixed_air_owner: base.formula_owners.mixed_air_owner,
    }
}

pub(super) fn all_chains() -> Vec<Chain> {
    let mut chains = Vec::new();
    let mut ordinal = 1;
    for inherited in 0..3 {
        chains.push(chain(inherited, 0, false, None, ordinal, 0.7, 18.0, 1.0));
        ordinal += 1;
    }
    for inherited in 3..8 {
        for outcome in [0, 2, 1] {
            chains.push(chain(
                inherited, outcome, false, None, ordinal, 0.7, 18.0, 1.0,
            ));
            ordinal += 1;
        }
    }
    let selectors = [
        D::ConstantSensibleHeatRatio,
        D::Humidistat,
        D::None,
        D::ConstantSupplyHumidityRatio,
    ];
    for inherited in [3, 4] {
        for selector in selectors {
            chains.push(chain(
                inherited,
                1,
                true,
                Some(selector),
                ordinal,
                0.7,
                18.0,
                1.0,
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
            chains.push(chain(
                inherited,
                1,
                true,
                Some(*selector),
                ordinal,
                0.65,
                19.0,
                1.0,
            ));
            ordinal += 1;
        }
    }
    chains
}
