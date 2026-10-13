//! Literal six-family official field order/defaults/names, no count ownership.
use super::{DefaultValue, Field};
const F0: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha("NONE"),
        retain_case: true,
    },
    Field {
        json: "north_axis",
        legacy: "North Axis",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "terrain",
        legacy: "Terrain",
        default: DefaultValue::Alpha("Suburbs"),
        retain_case: false,
    },
    Field {
        json: "loads_convergence_tolerance_value",
        legacy: "Loads Convergence Tolerance Value",
        default: DefaultValue::Numeric(0.04),
        retain_case: false,
    },
    Field {
        json: "temperature_convergence_tolerance_value",
        legacy: "Temperature Convergence Tolerance Value",
        default: DefaultValue::Numeric(0.4),
        retain_case: false,
    },
    Field {
        json: "solar_distribution",
        legacy: "Solar Distribution",
        default: DefaultValue::Alpha("FullExterior"),
        retain_case: false,
    },
    Field {
        json: "maximum_number_of_warmup_days",
        legacy: "Maximum Number of Warmup Days",
        default: DefaultValue::Numeric(25.0),
        retain_case: false,
    },
    Field {
        json: "minimum_number_of_warmup_days",
        legacy: "Minimum Number of Warmup Days",
        default: DefaultValue::Numeric(1.0),
        retain_case: false,
    },
];
const F1: &[Field] = &[Field {
    json: "algorithm",
    legacy: "Algorithm",
    default: DefaultValue::Alpha("TARP"),
    retain_case: false,
}];
const F2: &[Field] = &[Field {
    json: "algorithm",
    legacy: "Algorithm",
    default: DefaultValue::Alpha("DOE-2"),
    retain_case: false,
}];
const F3: &[Field] = &[
    Field {
        json: "algorithm",
        legacy: "Algorithm",
        default: DefaultValue::Alpha("ConductionTransferFunction"),
        retain_case: false,
    },
    Field {
        json: "surface_temperature_upper_limit",
        legacy: "Surface Temperature Upper Limit",
        default: DefaultValue::Numeric(200.0),
        retain_case: false,
    },
    Field {
        json: "minimum_surface_convection_heat_transfer_coefficient_value",
        legacy: "Minimum Surface Convection Heat Transfer Coefficient Value",
        default: DefaultValue::Numeric(0.1),
        retain_case: false,
    },
    Field {
        json: "maximum_surface_convection_heat_transfer_coefficient_value",
        legacy: "Maximum Surface Convection Heat Transfer Coefficient Value",
        default: DefaultValue::Numeric(1000.0),
        retain_case: false,
    },
];
const F4: &[Field] = &[
    Field {
        json: "algorithm",
        legacy: "Algorithm",
        default: DefaultValue::Alpha("ThirdOrderBackwardDifference"),
        retain_case: false,
    },
    Field {
        json: "do_space_heat_balance_for_sizing",
        legacy: "Do Space Heat Balance for Sizing",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
    Field {
        json: "do_space_heat_balance_for_simulation",
        legacy: "Do Space Heat Balance for Simulation",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
];
const F5: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "outside_layer",
        legacy: "Outside Layer",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_2",
        legacy: "Layer 2",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_3",
        legacy: "Layer 3",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_4",
        legacy: "Layer 4",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_5",
        legacy: "Layer 5",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_6",
        legacy: "Layer 6",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_7",
        legacy: "Layer 7",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_8",
        legacy: "Layer 8",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_9",
        legacy: "Layer 9",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_10",
        legacy: "Layer 10",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
];
pub(super) fn fields(family: &str) -> Option<(&'static [Field], usize, bool)> {
    match family {
        "Building" => Some((F0, 8, true)),
        "SurfaceConvectionAlgorithm:Inside" => Some((F1, 0, false)),
        "SurfaceConvectionAlgorithm:Outside" => Some((F2, 0, false)),
        "HeatBalanceAlgorithm" => Some((F3, 0, false)),
        "ZoneAirHeatBalanceAlgorithm" => Some((F4, 2, false)),
        "Construction" => Some((F5, 0, true)),
        _ => None,
    }
}
