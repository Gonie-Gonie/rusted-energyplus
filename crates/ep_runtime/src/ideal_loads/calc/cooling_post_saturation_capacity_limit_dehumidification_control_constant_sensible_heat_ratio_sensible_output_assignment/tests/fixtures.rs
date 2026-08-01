//! Source-valid CP384-to-CP387 fixture chain for CP388 tests.

use ep_model::{DehumidificationControlType as D, IdealLoadsAirSystem};

use super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_switch::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput as Cp386Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as Cp386State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state as advance_cp386,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput as Cp387Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState as Cp387State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state as advance_cp387,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as Cp383State,
    active_input_for_cp384_test as cp383_active_input,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance_cp383,
    predecessor_for_cp384_test as cp382_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as Cp384State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state as advance_cp384,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands as Cp385Operands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput as Cp385Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as Cp385State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance_cp385,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Cp387,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Cp384,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Cp385,
};

#[derive(Clone, Copy)]
pub(super) struct Chain {
    pub cp384: Cp384,
    pub cp385: Cp385,
    pub cp387: Cp387,
}

pub(super) fn chain(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    selector: Option<D>,
    ordinal: usize,
    maximum: f64,
    mixed_enthalpy: f64,
    humidity: f64,
) -> Chain {
    let cp382 = cp382_predecessor(inherited, outcome, ordinal);
    let mut cp383_state = Cp383State::new(cp382.system);
    let cp383_input = (outcome == 1).then(|| {
        cp383_active_input(cp382, maximum).expect("active CP383 input")
    });
    let cp383 = advance_cp383(&mut cp383_state, cp382, cp383_input).expect("CP383");
    let mut cp384_state = Cp384State::new(cp383.system);
    let cp384 = advance_cp384(&mut cp384_state, cp383).expect("CP384");
    let cp385_input = cp384
        .predecessor_dehumidification_total_output_capacity_guard_evaluated
        .then(|| Cp385Input {
            preexisting_supply_enthalpy_j_per_kg: f64::from_bits(0x40e4_86a0_0000_0001),
            active_operands: cp384
                .dehumidification_total_output_maximum_capacity_assignment_executed
                .then(|| Cp385Operands {
                    mixed_air_enthalpy_j_per_kg: mixed_enthalpy,
                    cooling_total_output_w: cp384
                        .resulting_cooling_total_output_w
                        .expect("CP384 output"),
                    supply_mass_flow_rate_kg_per_s: 2.0,
                }),
        });
    let mut cp385_state = Cp385State::new(cp384.system);
    let cp385 = advance_cp385(&mut cp385_state, cp384, cp385_input).expect("CP385");
    let mut cp386_state = Cp386State::new(cp385.system);
    let cp386_input = assignment.then(|| Cp386Input {
        dehumidification_control_type: selector.expect("active selector"),
    });
    let cp386 = advance_cp386(&mut cp386_state, cp385, cp386_input).expect("CP386");
    let mut cp387_state = Cp387State::new(cp386.system);
    let cp387_input = (selector == Some(D::ConstantSensibleHeatRatio)).then_some(Cp387Input {
        mixed_air_humidity_ratio: humidity,
    });
    let cp387 = advance_cp387(&mut cp387_state, cp386, cp387_input).expect("CP387");
    Chain { cp384, cp385, cp387 }
}

pub(super) const fn input(
    chain: Chain,
    ratio: f64,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput {
        cooling_total_output_owner: chain.cp384,
        cooling_total_output_corroborator: chain.cp385,
        cooling_sensible_heat_ratio: ratio,
    }
}

pub(super) fn selected_system(chain: Chain, ratio: f64) -> IdealLoadsAirSystem {
    let (_, mut system, _, _) = release_case();
    system.id = chain.cp387.system;
    system.dehumidification_control_type = D::ConstantSensibleHeatRatio;
    system.cooling_sensible_heat_ratio = ratio;
    system
}
