//! Exact CP386 route refinement for CP387.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Predecessor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub active: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    if !crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact(predecessor) {
        return None;
    }
    if predecessor.unit_off_skipped {
        return Some(RetainedRoute {
            predecessor_index: 0,
            active: false,
        });
    }
    if predecessor.non_cooling_skipped {
        return Some(RetainedRoute {
            predecessor_index: 1,
            active: false,
        });
    }
    if predecessor.positive_guard_false_fallthrough_skipped {
        return Some(RetainedRoute {
            predecessor_index: 2,
            active: false,
        });
    }

    let lineage = if predecessor.heating_availability_guard_false_fallthrough {
        0
    } else if predecessor.humidification_control_guard_false_fallthrough {
        1
    } else if predecessor.dehumidification_control_humidistat_maximum_assignment_executed {
        2
    } else if predecessor.dehumidification_control_none_maximum_assignment_executed {
        3
    } else if predecessor.dehumidification_control_guard_false_fallthrough {
        4
    } else {
        return None;
    };
    let stage = if predecessor.predecessor_active_capacity_limit_guard_false_fallthrough {
        0
    } else if predecessor.predecessor_dehumidification_guard_false_fallthrough {
        1
    } else if predecessor.dehumidification_total_output_capacity_guard_false_fallthrough {
        2
    } else if predecessor.dehumidification_total_output_maximum_capacity_assignment_executed {
        3
    } else {
        return None;
    };
    if stage < 3 {
        return Some(RetainedRoute {
            predecessor_index: 3 + lineage * 3 + stage,
            active: false,
        });
    }

    let selector = predecessor.dehumidification_control_type?;
    let predecessor_index = match (lineage, selector) {
        (0, D::ConstantSensibleHeatRatio) => 18,
        (0, D::Humidistat) => 19,
        (0, D::None) => 20,
        (0, D::ConstantSupplyHumidityRatio) => 21,
        (1, D::ConstantSensibleHeatRatio) => 22,
        (1, D::Humidistat) => 23,
        (1, D::None) => 24,
        (1, D::ConstantSupplyHumidityRatio) => 25,
        (2, D::Humidistat) => 26,
        (3, D::None) => 27,
        (4, D::ConstantSensibleHeatRatio) => 28,
        (4, D::ConstantSupplyHumidityRatio) => 29,
        _ => return None,
    };
    Some(RetainedRoute {
        predecessor_index,
        active: selector == D::ConstantSensibleHeatRatio,
    })
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 18 | 22 | 28)
}
