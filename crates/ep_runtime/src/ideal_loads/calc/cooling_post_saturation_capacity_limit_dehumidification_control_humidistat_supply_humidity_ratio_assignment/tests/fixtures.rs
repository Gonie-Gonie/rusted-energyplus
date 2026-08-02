//! Source-valid CP394 fixtures for CP395.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry::tests::fixtures as cp394_fixtures;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState as Cp394State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot as Cp394,
};

#[derive(Clone, Copy)]
pub(in crate::ideal_loads::calc) struct Chain {
    pub cp394: Cp394,
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
    let base = cp394_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp394State::new(base.cp393.system);
    let cp394 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
        &mut state,
        base.cp393,
    )
    .expect("CP394");
    Chain { cp394 }
}

pub(in crate::ideal_loads::calc) fn all_chains() -> Vec<Chain> {
    cp394_fixtures::all_chains()
        .into_iter()
        .map(|base| {
            let mut state = Cp394State::new(base.cp393.system);
            let cp394 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
                &mut state,
                base.cp393,
            )
            .expect("CP394");
            Chain { cp394 }
        })
        .collect()
}
