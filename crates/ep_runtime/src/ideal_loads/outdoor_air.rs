//! IdealLoads outdoor-air design-flow helpers.

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

/// Calculates the design outdoor-air volume flow in m3/s for supported methods.
#[must_use]
pub fn calc_design_outdoor_air_volume_flow_m3_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
) -> Option<f64> {
    let per_person = nonnegative_product(
        specification.outdoor_air_flow_per_person_m3_per_s_person,
        context.design_people_count,
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

    match specification.method {
        DesignSpecificationOutdoorAirMethod::FlowPerPerson => Some(per_person),
        DesignSpecificationOutdoorAirMethod::FlowPerArea => Some(per_area),
        DesignSpecificationOutdoorAirMethod::FlowPerZone => Some(per_zone),
        DesignSpecificationOutdoorAirMethod::AirChangesPerHour => Some(air_changes),
        DesignSpecificationOutdoorAirMethod::Sum => {
            Some(per_person + per_area + per_zone + air_changes)
        }
        DesignSpecificationOutdoorAirMethod::Maximum => {
            Some(per_person.max(per_area).max(per_zone).max(air_changes))
        }
        DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure
        | DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnDesignOccupancy
        | DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnOccupancySchedule => None,
    }
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

fn schedule_multiplier(value: Option<f64>) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::{
        DesignSpecificationOutdoorAir, DesignSpecificationOutdoorAirId,
        DesignSpecificationOutdoorAirMethod, NormalizedName,
    };

    #[test]
    fn flow_zone_uses_declared_zone_volume_flow() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.05;

        let result = calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 3.0,
                zone_floor_area_m2: 20.0,
                zone_volume_m3: 60.0,
            },
        );

        assert_eq!(result, Some(0.05));
    }

    #[test]
    fn sum_combines_supported_terms() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::Sum;
        specification.outdoor_air_flow_per_person_m3_per_s_person = 0.004;
        specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.0003;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.02;
        specification.outdoor_air_flow_air_changes_per_hour = 0.6;

        let result = calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 5.0,
                zone_floor_area_m2: 40.0,
                zone_volume_m3: 90.0,
            },
        )
        .expect("sum method is supported");

        let expected = 0.004 * 5.0 + 0.0003 * 40.0 + 0.02 + 0.6 * 90.0 / 3600.0;
        assert_close(result, expected, 1.0e-12);
    }

    #[test]
    fn maximum_selects_largest_supported_term() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::Maximum;
        specification.outdoor_air_flow_per_person_m3_per_s_person = 0.004;
        specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.002;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.03;
        specification.outdoor_air_flow_air_changes_per_hour = 1.0;

        let result = calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 4.0,
                zone_floor_area_m2: 40.0,
                zone_volume_m3: 120.0,
            },
        )
        .expect("maximum method is supported");

        assert_close(result, 0.002 * 40.0, 1.0e-12);
    }

    #[test]
    fn mass_flow_applies_clamped_schedule_and_standard_density() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.1;

        let result = calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 0.0,
                zone_floor_area_m2: 0.0,
                zone_volume_m3: 0.0,
            },
            Some(2.0),
            1.2,
        );

        assert_eq!(result, Some(0.12));
    }

    #[test]
    fn unsupported_methods_remain_unresolved() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure;

        assert_eq!(
            calc_design_outdoor_air_volume_flow_m3_per_s(
                &specification,
                IdealLoadsOutdoorAirContext {
                    design_people_count: 1.0,
                    zone_floor_area_m2: 1.0,
                    zone_volume_m3: 1.0,
                },
            ),
            None
        );
    }

    fn test_specification() -> DesignSpecificationOutdoorAir {
        DesignSpecificationOutdoorAir {
            id: DesignSpecificationOutdoorAirId(0),
            name: NormalizedName::new("OUTDOOR AIR SPEC"),
            method: DesignSpecificationOutdoorAirMethod::FlowPerPerson,
            outdoor_air_flow_per_person_m3_per_s_person: 0.00944,
            outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: 0.0,
            outdoor_air_flow_per_zone_m3_per_s: 0.0,
            outdoor_air_flow_air_changes_per_hour: 0.0,
            outdoor_air_schedule: None,
            proportional_control_minimum_outdoor_air_flow_rate_schedule: None,
        }
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} was not within {tolerance} of {expected}"
        );
    }
}
