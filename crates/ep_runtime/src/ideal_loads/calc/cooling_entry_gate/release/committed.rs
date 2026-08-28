//! Opaque direct CP312 input capabilities retained for the heating guard.

use super::{minimum_oa_snapshot_is_direct_release, minimum_oa_snapshots_bitwise_equal};
use crate::ideal_loads::PurchasedAirUnitRuntimeState;
use crate::ideal_loads::calc::cooling_entry_gate::{
    PurchasedAirCalcCoolingEntryGateDirectReleaseInvocationWitness as Invocation,
    PurchasedAirTemperatureControlType, cooling_entry_gate_snapshots_bitwise_equal,
};
use crate::ideal_loads::calc::minimum_oa_prefix::calculation_entry_snapshots_bitwise_equal;

/// Sealed CP311/CP310 numeric operands needed by the line-2348 guard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands
{
    pub minimum_outdoor_air_sensible_output_w: f64,
    pub heating_setpoint_demand_w: f64,
}

/// Returns same-call CP311/CP310 numeric operands only for CP312's direct,
/// active, numeric-false Cooling-entry route.
pub(in crate::ideal_loads::calc) fn cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands(
    unit: &PurchasedAirUnitRuntimeState,
) -> Option<PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands> {
    let invocation = committed_direct_non_cooling_invocation(unit)?;
    let minimum = invocation
        .minimum_oa_prefix
        .minimum_outdoor_air_sensible_output_w?;
    let corroborated = invocation
        .cooling_entry_gate
        .minimum_outdoor_air_sensible_output_w?;
    (minimum.to_bits() == corroborated.to_bits()).then_some(
        PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands {
            minimum_outdoor_air_sensible_output_w: minimum,
            heating_setpoint_demand_w: invocation
                .calculation_entry
                .demand
                .remaining_output_req_to_heat_sp_w,
        },
    )
}

/// Returns the prevalidated thermostat type. CP431 calls this accessor only
/// after the first strict numeric comparison succeeds.
pub(in crate::ideal_loads::calc) fn cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type(
    unit: &PurchasedAirUnitRuntimeState,
) -> Option<PurchasedAirTemperatureControlType> {
    committed_direct_non_cooling_invocation(unit)
        .map(|invocation| invocation.prevalidated_temperature_control_type)
}

fn committed_direct_non_cooling_invocation(
    unit: &PurchasedAirUnitRuntimeState,
) -> Option<Invocation> {
    let state = &unit.calc_cooling_entry_gate;
    let invocation = state.direct_release_invocation_witness?;
    let entry = unit.calc_entry.latest?;
    let minimum = unit.calc_minimum_oa_prefix.latest?;
    let gate = state.latest?;
    let linked = entry.system == unit.system
        && minimum.system == unit.system
        && gate.system == unit.system
        && unit.controlled_zone == Some(entry.controlled_zone)
        && minimum.controlled_zone == entry.controlled_zone
        && gate.controlled_zone == entry.controlled_zone
        && minimum.parent_call_ordinal == entry.call_ordinal
        && gate.parent_call_ordinal == entry.call_ordinal
        && entry.call_ordinal == state.transition_count
        && state.transition_count == unit.calc_minimum_oa_prefix.transition_count
        && state.transition_count == unit.calc_entry.call_count
        && state.transition_count > 0;
    let state_counts = state
        .source_execution_count
        .checked_add(state.unit_off_skip_count)
        == Some(state.transition_count)
        && state.sensible_comparison_count == state.source_execution_count
        && state.temperature_control_type_read_count == state.sensible_comparison_satisfied_count
        && state.single_heat_block_count <= state.temperature_control_type_read_count
        && state
            .cooling_body_entry_count
            .checked_add(state.single_heat_block_count)
            == Some(state.temperature_control_type_read_count)
        && state.operating_mode_assignment_count == state.cooling_body_entry_count
        && state
            .active_fallthrough_count
            .checked_add(state.cooling_body_entry_count)
            == Some(state.source_execution_count);
    let exact_invocation =
        calculation_entry_snapshots_bitwise_equal(entry, invocation.calculation_entry)
            && minimum_oa_snapshots_bitwise_equal(minimum, invocation.minimum_oa_prefix)
            && cooling_entry_gate_snapshots_bitwise_equal(gate, invocation.cooling_entry_gate);
    let direct_non_cooling = minimum_oa_snapshot_is_direct_release(minimum)
        && invocation.prevalidated_temperature_control_type
            == PurchasedAirTemperatureControlType::DualHeatCool
        && gate.unit_body_entered
        && gate.sensible_comparison_evaluated
        && gate.sensible_comparison_satisfied == Some(false)
        && !gate.temperature_control_type_read
        && gate.temperature_control_type.is_none()
        && gate.temperature_control_type_permits_cooling.is_none()
        && !gate.single_heat_blocked
        && !gate.cooling_body_entered
        && gate.assigned_operating_mode.is_none();
    (linked && state_counts && exact_invocation && direct_non_cooling).then_some(invocation)
}

#[cfg(test)]
mod tests {
    use super::{
        cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands as numeric,
        cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type as control,
    };
    use crate::ideal_loads::PurchasedAirUnitRuntimeState;
    use crate::ideal_loads::calc::{
        PurchasedAirCalcCoolingEntryGateRuntimeState, PurchasedAirTemperatureControlType,
        advance_direct_no_oa_calc_cooling_entry_gate,
    };

    #[test]
    fn committed_accessors_are_bounded_and_separate() {
        let source = include_str!("committed.rs");
        let hot = source.split("#[cfg(test)]").next().expect("hot source");
        for forbidden in [
            "completed_",
            "private_characterization",
            "snapshot_is_exact",
        ] {
            assert!(!hot.contains(forbidden), "{forbidden}");
        }
        assert_eq!(
            hot.matches("committed_direct_non_cooling_invocation(unit)")
                .count(),
            2,
        );
    }

    #[test]
    fn committed_accessors_accept_the_opaque_direct_invocation() {
        let unit = fixture();
        let inputs = numeric(&unit).expect("numeric capability");
        assert!(inputs.minimum_outdoor_air_sensible_output_w.is_finite());
        assert!(inputs.heating_setpoint_demand_w.is_finite());
        assert_eq!(
            control(&unit),
            Some(PurchasedAirTemperatureControlType::DualHeatCool)
        );
    }

    #[test]
    fn committed_accessors_reject_current_state_witness_identity_and_overflow_forgeries() {
        enum Forgery {
            CurrentCp310Demand,
            CurrentCp311Minimum,
            CurrentCp312Minimum,
            CurrentCp310System,
            CurrentCp311Zone,
            CurrentCp312Ordinal,
            TransitionCount,
            SourcePlusSkipOverflow,
            BodyPlusBlockOverflow,
            FallthroughPlusBodyOverflow,
            WitnessCp310Demand,
            WitnessCp311Minimum,
            WitnessCp312Minimum,
            WitnessControlType,
        }
        let forgeries = [
            Forgery::CurrentCp310Demand,
            Forgery::CurrentCp311Minimum,
            Forgery::CurrentCp312Minimum,
            Forgery::CurrentCp310System,
            Forgery::CurrentCp311Zone,
            Forgery::CurrentCp312Ordinal,
            Forgery::TransitionCount,
            Forgery::SourcePlusSkipOverflow,
            Forgery::BodyPlusBlockOverflow,
            Forgery::FallthroughPlusBodyOverflow,
            Forgery::WitnessCp310Demand,
            Forgery::WitnessCp311Minimum,
            Forgery::WitnessCp312Minimum,
            Forgery::WitnessControlType,
        ];
        let unit = fixture();
        for (index, forgery) in forgeries.into_iter().enumerate() {
            let mut forged = unit.clone();
            match forgery {
                Forgery::CurrentCp310Demand => {
                    let latest = forged.calc_entry.latest.as_mut().expect("CP310");
                    latest.demand.remaining_output_req_to_heat_sp_w =
                        flip(latest.demand.remaining_output_req_to_heat_sp_w);
                }
                Forgery::CurrentCp311Minimum => {
                    let latest = forged
                        .calc_minimum_oa_prefix
                        .latest
                        .as_mut()
                        .expect("CP311");
                    latest.minimum_outdoor_air_sensible_output_w =
                        latest.minimum_outdoor_air_sensible_output_w.map(flip);
                }
                Forgery::CurrentCp312Minimum => {
                    let latest = forged
                        .calc_cooling_entry_gate
                        .latest
                        .as_mut()
                        .expect("CP312");
                    latest.minimum_outdoor_air_sensible_output_w =
                        latest.minimum_outdoor_air_sensible_output_w.map(flip);
                }
                Forgery::CurrentCp310System => {
                    forged.calc_entry.latest.as_mut().expect("CP310").system =
                        ep_model::IdealLoadsAirSystemId(forged.system.0.wrapping_add(1));
                }
                Forgery::CurrentCp311Zone => {
                    forged
                        .calc_minimum_oa_prefix
                        .latest
                        .as_mut()
                        .expect("CP311")
                        .controlled_zone =
                        ep_model::ZoneId(forged.controlled_zone.expect("zone").0.wrapping_add(1));
                }
                Forgery::CurrentCp312Ordinal => {
                    forged
                        .calc_cooling_entry_gate
                        .latest
                        .as_mut()
                        .expect("CP312")
                        .parent_call_ordinal += 1;
                }
                Forgery::TransitionCount => forged.calc_cooling_entry_gate.transition_count += 1,
                Forgery::SourcePlusSkipOverflow => {
                    forged.calc_cooling_entry_gate.source_execution_count = usize::MAX;
                    forged.calc_cooling_entry_gate.unit_off_skip_count = 1;
                }
                Forgery::BodyPlusBlockOverflow => {
                    forged.calc_cooling_entry_gate.cooling_body_entry_count = usize::MAX;
                    forged.calc_cooling_entry_gate.single_heat_block_count = 1;
                }
                Forgery::FallthroughPlusBodyOverflow => {
                    forged.calc_cooling_entry_gate.active_fallthrough_count = usize::MAX;
                    forged.calc_cooling_entry_gate.cooling_body_entry_count = 1;
                }
                Forgery::WitnessCp310Demand => {
                    let witness = forged
                        .calc_cooling_entry_gate
                        .direct_release_invocation_witness
                        .as_mut()
                        .expect("witness");
                    witness
                        .calculation_entry
                        .demand
                        .remaining_output_req_to_heat_sp_w = flip(
                        witness
                            .calculation_entry
                            .demand
                            .remaining_output_req_to_heat_sp_w,
                    );
                }
                Forgery::WitnessCp311Minimum => {
                    let witness = forged
                        .calc_cooling_entry_gate
                        .direct_release_invocation_witness
                        .as_mut()
                        .expect("witness");
                    witness
                        .minimum_oa_prefix
                        .minimum_outdoor_air_sensible_output_w = witness
                        .minimum_oa_prefix
                        .minimum_outdoor_air_sensible_output_w
                        .map(flip);
                }
                Forgery::WitnessCp312Minimum => {
                    let witness = forged
                        .calc_cooling_entry_gate
                        .direct_release_invocation_witness
                        .as_mut()
                        .expect("witness");
                    witness
                        .cooling_entry_gate
                        .minimum_outdoor_air_sensible_output_w = witness
                        .cooling_entry_gate
                        .minimum_outdoor_air_sensible_output_w
                        .map(flip);
                }
                Forgery::WitnessControlType => {
                    forged
                        .calc_cooling_entry_gate
                        .direct_release_invocation_witness
                        .as_mut()
                        .expect("witness")
                        .prevalidated_temperature_control_type =
                        PurchasedAirTemperatureControlType::SingleCool;
                }
            }
            assert!(numeric(&forged).is_none(), "numeric forgery {index}");
            assert!(control(&forged).is_none(), "control forgery {index}");
        }
    }

    fn fixture() -> PurchasedAirUnitRuntimeState {
        let (mut runtime, system, _) = crate::ideal_loads::calc::
            cooling_economizer_condition_release_tests::
            release_fixture_with_cooling_demand_and_availability(1.0, 1.0);
        let unit = runtime.units.get_mut(&system.id).expect("unit");
        let entry = unit.calc_entry.latest.expect("CP310");
        let minimum = unit.calc_minimum_oa_prefix.latest.expect("CP311");
        unit.calc_cooling_entry_gate = PurchasedAirCalcCoolingEntryGateRuntimeState::new(system.id);
        advance_direct_no_oa_calc_cooling_entry_gate(
            &mut runtime,
            &system,
            entry,
            minimum,
            PurchasedAirTemperatureControlType::DualHeatCool,
        )
        .expect("direct CP312 numeric-false route");
        runtime.units.get(&system.id).expect("unit").clone()
    }

    fn flip(value: f64) -> f64 {
        f64::from_bits(value.to_bits() ^ 1)
    }
}
