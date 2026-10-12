//! Literal field order/defaults/names from the pinned four official schema definitions.
use super::{DefaultValue, Field};
const F0: &[Field] = &[Field {
    json: "version_identifier",
    legacy: "Version Identifier",
    default: DefaultValue::Alpha("26.1"),
    retain_case: false,
}];
const F1: &[Field] = &[
    Field {
        json: "algorithm",
        legacy: "Algorithm",
        default: DefaultValue::Alpha("CONDUCTIONTRANSFERFUNCTION"),
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
const F2: &[Field] = &[Field {
    json: "number_of_timesteps_per_hour",
    legacy: "Number of Timesteps per Hour",
    default: DefaultValue::Numeric(6.0),
    retain_case: false,
}];
const F3: &[Field] = &[
    Field {
        json: "do_zone_sizing_calculation",
        legacy: "Do Zone Sizing Calculation",
        default: DefaultValue::Alpha("NO"),
        retain_case: false,
    },
    Field {
        json: "do_system_sizing_calculation",
        legacy: "Do System Sizing Calculation",
        default: DefaultValue::Alpha("NO"),
        retain_case: false,
    },
    Field {
        json: "do_plant_sizing_calculation",
        legacy: "Do Plant Sizing Calculation",
        default: DefaultValue::Alpha("NO"),
        retain_case: false,
    },
    Field {
        json: "run_simulation_for_sizing_periods",
        legacy: "Run Simulation for Sizing Periods",
        default: DefaultValue::Alpha("YES"),
        retain_case: false,
    },
    Field {
        json: "run_simulation_for_weather_file_run_periods",
        legacy: "Run Simulation for Weather File Run Periods",
        default: DefaultValue::Alpha("YES"),
        retain_case: false,
    },
    Field {
        json: "do_hvac_sizing_simulation_for_sizing_periods",
        legacy: "Do HVAC Sizing Simulation for Sizing Periods",
        default: DefaultValue::Alpha("NO"),
        retain_case: false,
    },
    Field {
        json: "maximum_number_of_hvac_sizing_simulation_passes",
        legacy: "Maximum Number of HVAC Sizing Simulation Passes",
        default: DefaultValue::Numeric(1.0),
        retain_case: false,
    },
];
pub(super) fn fields(family: &str) -> Option<(&'static [Field], usize)> {
    match family {
        "Version" => Some((F0, 0)),
        "HeatBalanceAlgorithm" => Some((F1, 0)),
        "Timestep" => Some((F2, 0)),
        "SimulationControl" => Some((F3, 7)),
        _ => None,
    }
}
