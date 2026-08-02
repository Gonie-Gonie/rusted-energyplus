//! Source-valid CP392 fixtures for CP393.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::tests::fixtures as cp392_fixtures;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState as Cp392State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot as Cp392,
};

#[derive(Clone, Copy)]
pub(in crate::ideal_loads::calc) struct Chain {
    pub cp392: Cp392,
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
    let base = cp392_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut state = Cp392State::new(base.cp391.system);
    let cp392 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state(
        &mut state,
        base.cp391,
    )
    .expect("CP392");
    Chain { cp392 }
}

pub(in crate::ideal_loads::calc) fn all_chains() -> Vec<Chain> {
    cp392_fixtures::all_chains()
        .into_iter()
        .map(|base| {
            let mut state = Cp392State::new(base.cp391.system);
            let cp392 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state(
                &mut state,
                base.cp391,
            )
            .expect("CP392");
            Chain { cp392 }
        })
        .collect()
}
