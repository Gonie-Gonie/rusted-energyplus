//! Source-valid CP390 fixtures for CP391.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit::tests::fixtures as cp390_fixtures;
use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState as Cp390State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot as Cp390,
};

#[derive(Clone, Copy)]
pub(super) struct Chain {
    pub cp390: Cp390,
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
    let base = cp390_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp390State::new(base.cp389.system);
    let cp390 =
        advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
            &mut state,
            base.cp389,
            base.owner(),
        )
        .expect("CP390");
    Chain { cp390 }
}

pub(super) fn all_chains() -> Vec<Chain> {
    cp390_fixtures::all_chains()
        .into_iter()
        .map(|base| {
            let mut state = Cp390State::new(base.cp389.system);
            let cp390 =
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
                    &mut state,
                    base.cp389,
                    base.owner(),
                )
                .expect("CP390");
            Chain { cp390 }
        })
        .collect()
}
