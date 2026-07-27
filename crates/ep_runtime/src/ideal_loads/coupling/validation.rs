//! Fail-closed release-coupling and initialization-cache validation.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::{
    DirectZonePurchasedAirCouplingError, DirectZonePurchasedAirCouplingInput,
    DirectZonePurchasedAirInitializationRelation,
};
use crate::ideal_loads::{IdealLoadsSensibleLimitContext, select_purchased_air_branch};

pub(super) fn validate_supported_branch(
    system: &IdealLoadsAirSystem,
) -> Result<(), DirectZonePurchasedAirCouplingError> {
    let branch = select_purchased_air_branch(system);
    if !branch.is_no_oa_sensible_with_optional_limits()
        || system.dehumidification_control_type != DehumidificationControlType::None
        || system.humidification_control_type != HumidificationControlType::None
    {
        Err(DirectZonePurchasedAirCouplingError::UnsupportedBranch { branch })
    } else {
        Ok(())
    }
}

pub(super) fn initialized_limit_context(
    input: &DirectZonePurchasedAirCouplingInput<'_>,
) -> Result<IdealLoadsSensibleLimitContext, DirectZonePurchasedAirCouplingError> {
    let initialization = input.initialization;
    if initialization.system != input.system.id {
        return Err(
            DirectZonePurchasedAirCouplingError::InitializationIdentityMismatch {
                relation: DirectZonePurchasedAirInitializationRelation::System,
            },
        );
    }
    if initialization.controlled_zone != input.zone_state.zone_id {
        return Err(
            DirectZonePurchasedAirCouplingError::InitializationIdentityMismatch {
                relation: DirectZonePurchasedAirInitializationRelation::ControlledZone,
            },
        );
    }
    if initialization.supply_node != input.supply_node {
        return Err(
            DirectZonePurchasedAirCouplingError::InitializationIdentityMismatch {
                relation: DirectZonePurchasedAirInitializationRelation::SupplyNode,
            },
        );
    }
    let initialized_recirculation_node = initialization
        .recirculation_node
        .ok_or(DirectZonePurchasedAirCouplingError::InitializationNotReady)?;
    if initialized_recirculation_node != input.recirculation_node {
        return Err(
            DirectZonePurchasedAirCouplingError::InitializationIdentityMismatch {
                relation: DirectZonePurchasedAirInitializationRelation::RecirculationNode,
            },
        );
    }
    let flags = initialization.flags;
    if !flags.state_machine_used
        || !flags.one_time_checked
        || !flags.topology_ready
        || !flags.environment_initialized
        || !flags.sizing_checked
        || !flags.equipment_list_checked
        || !flags.return_plenum_inactive
    {
        return Err(DirectZonePurchasedAirCouplingError::InitializationNotReady);
    }
    let density = initialization.standard_air_density_kg_per_m3.ok_or(
        DirectZonePurchasedAirCouplingError::InitializationCacheInvalid {
            field: "standard_air_density_kg_per_m3",
            value: f64::NAN,
        },
    )?;
    require_initialization_cache_positive(density, "standard_air_density_kg_per_m3")?;
    require_initialization_cache_nonnegative(
        initialization.maximum_heating_air_mass_flow_rate_kg_per_s,
        "maximum_heating_air_mass_flow_rate_kg_per_s",
    )?;
    require_initialization_cache_nonnegative(
        initialization.maximum_cooling_air_mass_flow_rate_kg_per_s,
        "maximum_cooling_air_mass_flow_rate_kg_per_s",
    )?;
    Ok(input.limit_context.with_initialized_flow_limits(
        density,
        initialization.maximum_heating_air_mass_flow_rate_kg_per_s,
        initialization.maximum_cooling_air_mass_flow_rate_kg_per_s,
    ))
}

pub(super) fn validate_coupling_inputs(
    input: &DirectZonePurchasedAirCouplingInput<'_>,
) -> Result<(), DirectZonePurchasedAirCouplingError> {
    require_finite_input(input.zone_node_temperature_c, "zone_node_temperature_c")?;
    require_finite_input(
        input.recirculation_state.air_temperature_c,
        "recirculation_state.air_temperature_c",
    )?;
    require_nonnegative_input(
        input.recirculation_state.air_humidity_ratio,
        "recirculation_state.air_humidity_ratio",
    )?;
    require_nonnegative_input(
        input.zone_state.air_humidity_ratio,
        "zone_state.air_humidity_ratio",
    )?;
    require_nonnegative_input(
        input.zone_state.air_heat_capacity_j_per_k,
        "zone_state.air_heat_capacity_j_per_k",
    )?;
    require_nonnegative_input(input.zone_state.sum_ha_w_per_k, "zone_state.sum_ha_w_per_k")?;
    require_nonnegative_input(
        input.zone_state.sum_mcp_w_per_k,
        "zone_state.sum_mcp_w_per_k",
    )?;
    require_finite_input(
        input.system.maximum_heating_supply_air_temperature_c,
        "system.maximum_heating_supply_air_temperature_c",
    )?;
    require_finite_input(
        input.system.minimum_cooling_supply_air_temperature_c,
        "system.minimum_cooling_supply_air_temperature_c",
    )?;
    Ok(())
}

fn require_initialization_cache_positive(
    value: f64,
    field: &'static str,
) -> Result<(), DirectZonePurchasedAirCouplingError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(DirectZonePurchasedAirCouplingError::InitializationCacheInvalid { field, value })
    }
}

fn require_initialization_cache_nonnegative(
    value: f64,
    field: &'static str,
) -> Result<(), DirectZonePurchasedAirCouplingError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(DirectZonePurchasedAirCouplingError::InitializationCacheInvalid { field, value })
    }
}

fn require_finite_input(
    value: f64,
    field: &'static str,
) -> Result<(), DirectZonePurchasedAirCouplingError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(DirectZonePurchasedAirCouplingError::InputNotFinite { field })
    }
}

fn require_nonnegative_input(
    value: f64,
    field: &'static str,
) -> Result<(), DirectZonePurchasedAirCouplingError> {
    require_finite_input(value, field)?;
    if value < 0.0 {
        Err(DirectZonePurchasedAirCouplingError::InputNegative { field, value })
    } else {
        Ok(())
    }
}
