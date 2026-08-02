//! Source-valid CP393 fixtures for CP394.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_state;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break::tests::fixtures as cp393_fixtures;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakRuntimeState as Cp393State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot as Cp393,
};

#[derive(Clone, Copy)]
pub(in crate::ideal_loads::calc) struct Chain {
    pub cp393: Cp393,
}

pub(in crate::ideal_loads::calc) fn chain(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    selector: Option<D>,
    ordinal: usize,
    ratio: f64,
    supply_temperature_c: f64,
    flow: f64,
) -> Chain {
    let base = cp393_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp393State::new(base.cp392.system);
    let cp393 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_state(
        &mut state,
        base.cp392,
    )
    .expect("CP393");
    Chain { cp393 }
}

pub(in crate::ideal_loads::calc) fn all_chains() -> Vec<Chain> {
    cp393_fixtures::all_chains()
        .into_iter()
        .map(|base| {
            let mut state = Cp393State::new(base.cp392.system);
            let cp393 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_state(
                &mut state,
                base.cp392,
            )
            .expect("CP393");
            Chain { cp393 }
        })
        .collect()
}
