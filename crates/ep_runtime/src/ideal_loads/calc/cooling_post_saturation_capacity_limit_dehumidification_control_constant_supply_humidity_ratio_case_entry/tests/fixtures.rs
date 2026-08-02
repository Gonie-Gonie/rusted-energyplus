//! Source-valid CP397 fixtures for CP398.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry::tests::fixtures as cp397_fixtures;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntryRuntimeState as Cp397State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot as Cp397,
};

#[derive(Clone, Copy)]
pub(in crate::ideal_loads::calc) struct Chain {
    pub cp397: Cp397,
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
    let base = cp397_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp397State::new(base.cp396.system);
    let cp397 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state(
        &mut state,
        base.cp396,
    )
    .expect("CP397");
    Chain { cp397 }
}

pub(in crate::ideal_loads::calc) fn all_chains() -> Vec<Chain> {
    cp397_fixtures::all_chains()
        .into_iter()
        .map(|base| {
            let mut state = Cp397State::new(base.cp396.system);
            let cp397 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state(
                &mut state,
                base.cp396,
            )
            .expect("CP397");
            Chain { cp397 }
        })
        .collect()
}
