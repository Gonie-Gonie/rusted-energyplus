//! OccupancySchedule DCV outdoor-air helpers.

use ep_model::DesignSpecificationOutdoorAir;

use super::{
    IdealLoadsOutdoorAirContext, IdealLoadsOutdoorAirDesignFlowComponents,
    design_outdoor_air_volume_flow_components_for_people_count_m3_per_s, schedule_multiplier,
};

/// Calculates outdoor-air volume flow with OccupancySchedule DCV current people.
///
/// EnergyPlus `CalcPurchAirMinOAMassFlow` calls
/// `DataSizing::calcDesignSpecificationOutdoorAir` with `UseOccSchFlag=true`
/// for `OccupancySchedule` DCV, which replaces design occupants with the
/// current scheduled occupants for the per-person term.
#[must_use]
pub fn occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
    current_people_count: f64,
) -> Option<IdealLoadsOutdoorAirDesignFlowComponents> {
    design_outdoor_air_volume_flow_components_for_people_count_m3_per_s(
        specification,
        context,
        current_people_count,
    )
}

/// Applies OccupancySchedule DCV current people, OA schedule, and StdRhoAir.
#[must_use]
pub fn calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
    current_people_count: f64,
    schedule_value: Option<f64>,
    standard_air_density_kg_per_m3: f64,
) -> Option<f64> {
    if !standard_air_density_kg_per_m3.is_finite() || standard_air_density_kg_per_m3 < 0.0 {
        return None;
    }
    let dcv_volume_flow_m3_per_s =
        occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s(
            specification,
            context,
            current_people_count,
        )?
        .final_design_volume_flow_rate_m3_per_s;
    Some(
        dcv_volume_flow_m3_per_s
            * schedule_multiplier(schedule_value)
            * standard_air_density_kg_per_m3,
    )
}
