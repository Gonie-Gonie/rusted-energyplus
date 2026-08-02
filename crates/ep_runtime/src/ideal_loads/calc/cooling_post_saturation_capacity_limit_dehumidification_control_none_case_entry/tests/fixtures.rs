//! Source-valid CP396 fixtures for CP397.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break::tests::fixtures as cp396_fixtures;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState as Cp396State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakSnapshot as Cp396,
};

#[derive(Clone, Copy)]
pub(in crate::ideal_loads::calc) struct Chain {
    pub cp396: Cp396,
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
    let base = cp396_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp396State::new(base.cp395.system);
    let cp396 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
        &mut state,
        base.cp395,
    )
    .expect("CP396");
    Chain { cp396 }
}

pub(in crate::ideal_loads::calc) fn all_chains() -> Vec<Chain> {
    cp396_fixtures::all_chains()
        .into_iter()
        .map(|base| {
            let mut state = Cp396State::new(base.cp395.system);
            let cp396 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
                &mut state,
                base.cp395,
            )
            .expect("CP396");
            Chain { cp396 }
        })
        .collect()
}
