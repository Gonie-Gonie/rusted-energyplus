use super::*;

pub(in crate::ideal_loads::calc) fn cp419_all_snapshots_for_successor_tests() -> Vec<Snapshot> {
    let system = ep_model::IdealLoadsAirSystemId(412);
    let mut cp413_state = Cp413State::new(system);
    let mut cp414_state = Cp414State::new(system);
    let mut cp415_state = Cp415State::new(system);
    let mut cp416_state = Cp416State::new(system);
    let mut cp417_state = Cp417State::new(system);
    let mut cp418_state = Cp418State::new(system);
    let mut cp419_state = State::new(system);
    let mut snapshots = Vec::new();
    let mut ordinal = 0usize;
    for route in all_routes() {
        let outcomes: &[bool] = if route.active {
            &[false, true]
        } else {
            &[false]
        };
        for &body_entered in outcomes {
            ordinal += 1;
            let cp412 = predecessor_for_outcome(route, ordinal, body_entered);
            let cp413 = advance_cp413(&mut cp413_state, cp412).expect("CP413");
            let cp414 = advance_cp414(&mut cp414_state, cp413, 91_325.0).expect("CP414");
            let owner = body_entered.then(|| matching_mixed_air_owner(cp414, 17.0));
            let cp415 = advance_cp415(&mut cp415_state, cp414, owner).expect("CP415");
            let cp416 = advance_cp416(&mut cp416_state, cp415).expect("CP416");
            let cp417 = advance_cp417(&mut cp417_state, cp416).expect("CP417");
            let cp418 = advance_cp418(&mut cp418_state, cp417).expect("CP418");
            snapshots.push(advance(&mut cp419_state, cp418, active_input(cp418)).expect("CP419"));
        }
    }
    assert_eq!(snapshots.len(), 54);
    snapshots
}
