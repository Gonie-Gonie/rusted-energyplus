//! DesignSpecification:OutdoorAir design-flow helpers.

use ep_model::{DesignSpecificationOutdoorAir, DesignSpecificationOutdoorAirMethod};

/// Zone context needed by `DesignSpecification:OutdoorAir`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsOutdoorAirContext {
    /// Zone design occupant count used by Flow/Person.
    pub design_people_count: f64,
    /// Zone floor area in m2 used by Flow/Area.
    pub zone_floor_area_m2: f64,
    /// Zone volume in m3 used by AirChanges/Hour.
    pub zone_volume_m3: f64,
}

/// Component terms for `DesignSpecification:OutdoorAir` design flow.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsOutdoorAirDesignFlowComponents {
    /// Flow/Person contribution in m3/s.
    pub flow_per_person_m3_per_s: f64,
    /// Flow/Area contribution in m3/s.
    pub flow_per_area_m3_per_s: f64,
    /// Flow/Zone contribution in m3/s.
    pub flow_per_zone_m3_per_s: f64,
    /// AirChanges/Hour contribution in m3/s.
    pub air_changes_m3_per_s: f64,
    /// Final selected design flow for the requested method in m3/s.
    pub final_design_volume_flow_rate_m3_per_s: f64,
}

/// Calculates the design outdoor-air volume flow in m3/s for supported methods.
#[must_use]
pub fn design_outdoor_air_volume_flow_components_m3_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
) -> Option<IdealLoadsOutdoorAirDesignFlowComponents> {
    design_outdoor_air_volume_flow_components_for_people_count_m3_per_s(
        specification,
        context,
        context.design_people_count,
    )
}

pub(in crate::ideal_loads::outdoor_air) fn design_outdoor_air_volume_flow_components_for_people_count_m3_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
    people_count: f64,
) -> Option<IdealLoadsOutdoorAirDesignFlowComponents> {
    let per_person = nonnegative_product(
        specification.outdoor_air_flow_per_person_m3_per_s_person,
        people_count,
    );
    let per_area = nonnegative_product(
        specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2,
        context.zone_floor_area_m2,
    );
    let per_zone = nonnegative(specification.outdoor_air_flow_per_zone_m3_per_s);
    let air_changes = nonnegative_product(
        specification.outdoor_air_flow_air_changes_per_hour,
        context.zone_volume_m3,
    ) / 3600.0;

    let final_design_volume_flow_rate_m3_per_s = match specification.method {
        DesignSpecificationOutdoorAirMethod::FlowPerPerson => per_person,
        DesignSpecificationOutdoorAirMethod::FlowPerArea => per_area,
        DesignSpecificationOutdoorAirMethod::FlowPerZone => per_zone,
        DesignSpecificationOutdoorAirMethod::AirChangesPerHour => air_changes,
        DesignSpecificationOutdoorAirMethod::Sum => per_person + per_area + per_zone + air_changes,
        DesignSpecificationOutdoorAirMethod::Maximum => {
            per_person.max(per_area).max(per_zone).max(air_changes)
        }
        DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure
        | DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnDesignOccupancy
        | DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnOccupancySchedule => {
            return None;
        }
    };
    Some(IdealLoadsOutdoorAirDesignFlowComponents {
        flow_per_person_m3_per_s: per_person,
        flow_per_area_m3_per_s: per_area,
        flow_per_zone_m3_per_s: per_zone,
        air_changes_m3_per_s: air_changes,
        final_design_volume_flow_rate_m3_per_s,
    })
}

/// Calculates the design outdoor-air volume flow in m3/s for supported methods.
#[must_use]
pub fn calc_design_outdoor_air_volume_flow_m3_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
) -> Option<f64> {
    Some(
        design_outdoor_air_volume_flow_components_m3_per_s(specification, context)?
            .final_design_volume_flow_rate_m3_per_s,
    )
}

/// Applies the current OA schedule and standard density to the design volume flow.
#[must_use]
pub fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
    schedule_value: Option<f64>,
    standard_air_density_kg_per_m3: f64,
) -> Option<f64> {
    if !standard_air_density_kg_per_m3.is_finite() || standard_air_density_kg_per_m3 < 0.0 {
        return None;
    }
    let design_volume_flow_m3_per_s =
        calc_design_outdoor_air_volume_flow_m3_per_s(specification, context)?;
    Some(
        design_volume_flow_m3_per_s
            * schedule_multiplier(schedule_value)
            * standard_air_density_kg_per_m3,
    )
}

pub(in crate::ideal_loads::outdoor_air) fn schedule_multiplier(value: Option<f64>) -> f64 {
    let Some(value) = value else {
        return 1.0;
    };
    if !value.is_finite() {
        return 0.0;
    }
    value.clamp(0.0, 1.0)
}

fn nonnegative(value: f64) -> f64 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn nonnegative_product(left: f64, right: f64) -> f64 {
    nonnegative(left) * nonnegative(right)
}
