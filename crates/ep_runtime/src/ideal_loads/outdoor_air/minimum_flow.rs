//! Source-order minimum outdoor-air flow resolution.

use ep_model::{
    DemandControlledVentilationType, DesignSpecificationOutdoorAir,
    DesignSpecificationOutdoorAirId, DesignSpecificationOutdoorAirMethod, IdealLoadsAirSystem,
    IdealLoadsAirSystemId, ScheduleId,
};

use crate::ideal_loads::IdealLoadsSensibleLimitContext;

use super::{
    IdealLoadsOutdoorAirContext, IdealLoadsOutdoorAirDesignFlowComponents,
    calc_co2_setpoint_dcv_outdoor_air_mass_flow_rate_kg_per_s,
    calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s,
    calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s,
    design_outdoor_air_volume_flow_components_m3_per_s,
    occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s, schedule_multiplier,
};

const ENERGYPLUS_VERY_SMALL_MASS_FLOW_KG_PER_S: f64 = 1.0e-30;

/// Raw timestep inputs consumed by `CalcPurchAirMinOAMassFlow` compatibility logic.
#[derive(Clone, Copy, Debug)]
pub struct IdealLoadsMinimumOutdoorAirCompatInput<'a> {
    /// Resolved `DesignSpecification:OutdoorAir` object.
    pub specification: &'a DesignSpecificationOutdoorAir,
    /// Zone geometry and design-occupancy context.
    pub context: IdealLoadsOutdoorAirContext,
    /// Evaluated OA schedule value; ignored when the specification has no schedule.
    pub outdoor_air_schedule_value: Option<f64>,
    /// Current scheduled occupants required by `OccupancySchedule` DCV.
    pub current_people_count: Option<f64>,
    /// Zone contaminant demand required by `CO2Setpoint` DCV in kg/s.
    pub co2_setpoint_required_mass_flow_rate_kg_per_s: Option<f64>,
}

/// Source-order minimum outdoor-air flow resolution result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsMinimumOutdoorAirCompatResult {
    /// Design-occupancy component snapshot in m3/s.
    pub design_flow_components: IdealLoadsOutdoorAirDesignFlowComponents,
    /// Components selected after any OccupancySchedule occupant substitution.
    pub selected_flow_components: IdealLoadsOutdoorAirDesignFlowComponents,
    /// OA schedule multiplier after EnergyPlus-compatible normalization.
    pub applied_schedule_multiplier: f64,
    /// Scheduled design-occupancy minimum mass flow before DCV in kg/s.
    pub scheduled_design_mass_flow_rate_kg_per_s: f64,
    /// Raw DCV-adjusted minimum before the calc-stage finite-value guard in kg/s.
    pub dcv_adjusted_mass_flow_rate_kg_per_s: f64,
    /// Finite, nonnegative minimum mass flow consumed by the calc stage in kg/s.
    pub final_minimum_mass_flow_rate_kg_per_s: f64,
    /// DCV branch applied to the selected minimum flow.
    pub dcv_type: DemandControlledVentilationType,
}

/// Failure to resolve source-order minimum outdoor-air flow inputs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimPurchasedAirOutdoorAirCompatError {
    /// The design-flow method is outside the supported compatibility subset.
    UnsupportedDesignFlowMethod {
        /// Typed IdealLoads system ID.
        system_id: IdealLoadsAirSystemId,
        /// Typed outdoor-air specification ID.
        specification_id: DesignSpecificationOutdoorAirId,
        /// Unsupported design-flow method.
        method: DesignSpecificationOutdoorAirMethod,
    },
    /// Standard air density was negative or nonfinite.
    InvalidStandardAirDensity {
        /// Typed IdealLoads system ID.
        system_id: IdealLoadsAirSystemId,
        /// Rejected density in kg/m3.
        value: f64,
    },
    /// A referenced OA schedule had no evaluated timestep value.
    MissingOutdoorAirScheduleValue {
        /// Typed IdealLoads system ID.
        system_id: IdealLoadsAirSystemId,
        /// Schedule that requires evaluation.
        schedule_id: ScheduleId,
    },
    /// OccupancySchedule DCV had no current occupant count.
    MissingOccupancySchedulePeopleCount {
        /// Typed IdealLoads system ID.
        system_id: IdealLoadsAirSystemId,
    },
    /// CO2Setpoint DCV had no contaminant-demand mass flow.
    MissingCo2SetpointDemand {
        /// Typed IdealLoads system ID.
        system_id: IdealLoadsAirSystemId,
    },
}

pub(super) fn resolve_minimum_outdoor_air_compat(
    system: &IdealLoadsAirSystem,
    input: IdealLoadsMinimumOutdoorAirCompatInput<'_>,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Result<IdealLoadsMinimumOutdoorAirCompatResult, SimPurchasedAirOutdoorAirCompatError> {
    let density = limit_context.standard_air_density_kg_per_m3;
    if !density.is_finite() || density < 0.0 {
        return Err(
            SimPurchasedAirOutdoorAirCompatError::InvalidStandardAirDensity {
                system_id: system.id,
                value: density,
            },
        );
    }
    let outdoor_air_schedule_value = match input.specification.outdoor_air_schedule {
        Some(schedule_id) => Some(input.outdoor_air_schedule_value.ok_or(
            SimPurchasedAirOutdoorAirCompatError::MissingOutdoorAirScheduleValue {
                system_id: system.id,
                schedule_id,
            },
        )?),
        None => None,
    };

    let unsupported_method_error =
        SimPurchasedAirOutdoorAirCompatError::UnsupportedDesignFlowMethod {
            system_id: system.id,
            specification_id: input.specification.id,
            method: input.specification.method,
        };
    let design_flow_components =
        design_outdoor_air_volume_flow_components_m3_per_s(input.specification, input.context)
            .ok_or(unsupported_method_error)?;
    let scheduled_design_mass_flow_rate_kg_per_s =
        calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
            input.specification,
            input.context,
            outdoor_air_schedule_value,
            density,
        )
        .ok_or(unsupported_method_error)?;

    let (selected_flow_components, dcv_adjusted_mass_flow_rate_kg_per_s) = match system
        .demand_controlled_ventilation_type
    {
        DemandControlledVentilationType::None => (
            design_flow_components,
            scheduled_design_mass_flow_rate_kg_per_s,
        ),
        DemandControlledVentilationType::OccupancySchedule => {
            let current_people_count = input.current_people_count.ok_or(
                SimPurchasedAirOutdoorAirCompatError::MissingOccupancySchedulePeopleCount {
                    system_id: system.id,
                },
            )?;
            let selected_flow_components =
                occupancy_schedule_dcv_outdoor_air_volume_flow_components_m3_per_s(
                    input.specification,
                    input.context,
                    current_people_count,
                )
                .ok_or(unsupported_method_error)?;
            let mass_flow = calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s(
                input.specification,
                input.context,
                current_people_count,
                outdoor_air_schedule_value,
                density,
            )
            .ok_or(unsupported_method_error)?;
            (selected_flow_components, mass_flow)
        }
        DemandControlledVentilationType::Co2Setpoint => {
            let required_mass_flow = input.co2_setpoint_required_mass_flow_rate_kg_per_s.ok_or(
                SimPurchasedAirOutdoorAirCompatError::MissingCo2SetpointDemand {
                    system_id: system.id,
                },
            )?;
            (
                design_flow_components,
                calc_co2_setpoint_dcv_outdoor_air_mass_flow_rate_kg_per_s(
                    scheduled_design_mass_flow_rate_kg_per_s,
                    required_mass_flow,
                ),
            )
        }
    };
    let final_minimum_mass_flow_rate_kg_per_s = if dcv_adjusted_mass_flow_rate_kg_per_s.is_finite()
        && dcv_adjusted_mass_flow_rate_kg_per_s > ENERGYPLUS_VERY_SMALL_MASS_FLOW_KG_PER_S
    {
        dcv_adjusted_mass_flow_rate_kg_per_s
    } else {
        0.0
    };

    Ok(IdealLoadsMinimumOutdoorAirCompatResult {
        design_flow_components,
        selected_flow_components,
        applied_schedule_multiplier: schedule_multiplier(outdoor_air_schedule_value),
        scheduled_design_mass_flow_rate_kg_per_s,
        dcv_adjusted_mass_flow_rate_kg_per_s,
        final_minimum_mass_flow_rate_kg_per_s,
        dcv_type: system.demand_controlled_ventilation_type,
    })
}
