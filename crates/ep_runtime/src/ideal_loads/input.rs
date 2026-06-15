//! IdealLoads input boundary checks for sensible IdealLoads candidates.

use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsLimit,
    OutdoorAirEconomizerType,
};

/// Unsupported feature flags that keep the first IdealLoads case diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsUnsupportedFeature {
    /// Outdoor air object or outdoor air node is present.
    OutdoorAir,
    /// Demand controlled ventilation is active.
    DemandControlledVentilation,
    /// Economizer is active.
    Economizer,
    /// Heat recovery is active.
    HeatRecovery,
    /// Heating flow or capacity limit is active.
    HeatingLimit,
    /// Cooling flow or capacity limit is active.
    CoolingLimit,
    /// Humidification branch is active.
    Humidification,
    /// Humidistat dehumidification branch is active.
    Dehumidification,
    /// Heating limit is active but needs autosizing or a missing numeric field.
    UnresolvedHeatingLimit,
    /// Cooling limit is active but needs autosizing or a missing numeric field.
    UnresolvedCoolingLimit,
}

/// Classification for a no-OA sensible compatibility branch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdealLoadsSubsetBoundary {
    /// Unsupported features found on the system.
    pub unsupported_features: Vec<IdealLoadsUnsupportedFeature>,
}

impl IdealLoadsSubsetBoundary {
    /// Returns true when the system can use the no-OA/no-limit sensible branch.
    #[must_use]
    pub fn is_supported(&self) -> bool {
        self.unsupported_features.is_empty()
    }
}

/// Classifies an IdealLoads system for the first no-OA/no-limit sensible claim.
#[must_use]
pub fn classify_no_oa_no_limit_sensible_subset(
    system: &IdealLoadsAirSystem,
) -> IdealLoadsSubsetBoundary {
    let mut unsupported_features = no_oa_sensible_unsupported_features(system);

    if system.heating_limit != IdealLoadsLimit::NoLimit {
        unsupported_features.push(IdealLoadsUnsupportedFeature::HeatingLimit);
    }
    if system.cooling_limit != IdealLoadsLimit::NoLimit {
        unsupported_features.push(IdealLoadsUnsupportedFeature::CoolingLimit);
    }

    IdealLoadsSubsetBoundary {
        unsupported_features,
    }
}

/// Classifies an IdealLoads system for no-OA sensible diagnostics with numeric
/// flow/capacity limits allowed.
#[must_use]
pub fn classify_no_oa_sensible_subset(system: &IdealLoadsAirSystem) -> IdealLoadsSubsetBoundary {
    let mut unsupported_features = no_oa_sensible_unsupported_features(system);

    if !limit_fields_are_numeric(
        system.heating_limit,
        system.maximum_heating_air_flow_rate_m3_per_s,
        system.maximum_sensible_heating_capacity_w,
    ) {
        unsupported_features.push(IdealLoadsUnsupportedFeature::UnresolvedHeatingLimit);
    }
    if !limit_fields_are_numeric(
        system.cooling_limit,
        system.maximum_cooling_air_flow_rate_m3_per_s,
        system.maximum_total_cooling_capacity_w,
    ) {
        unsupported_features.push(IdealLoadsUnsupportedFeature::UnresolvedCoolingLimit);
    }

    IdealLoadsSubsetBoundary {
        unsupported_features,
    }
}

fn no_oa_sensible_unsupported_features(
    system: &IdealLoadsAirSystem,
) -> Vec<IdealLoadsUnsupportedFeature> {
    let mut unsupported_features = Vec::new();

    if system
        .design_specification_outdoor_air_object_name
        .is_some()
        || system.outdoor_air_inlet_node_name.is_some()
    {
        unsupported_features.push(IdealLoadsUnsupportedFeature::OutdoorAir);
    }
    if system.demand_controlled_ventilation_type != DemandControlledVentilationType::None {
        unsupported_features.push(IdealLoadsUnsupportedFeature::DemandControlledVentilation);
    }
    if system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer {
        unsupported_features.push(IdealLoadsUnsupportedFeature::Economizer);
    }
    if system.heat_recovery_type != HeatRecoveryType::None {
        unsupported_features.push(IdealLoadsUnsupportedFeature::HeatRecovery);
    }
    if system.humidification_control_type != HumidificationControlType::None {
        unsupported_features.push(IdealLoadsUnsupportedFeature::Humidification);
    }
    if matches!(
        system.dehumidification_control_type,
        DehumidificationControlType::Humidistat
            | DehumidificationControlType::ConstantSupplyHumidityRatio
    ) {
        unsupported_features.push(IdealLoadsUnsupportedFeature::Dehumidification);
    }

    unsupported_features
}

fn limit_fields_are_numeric(
    limit: IdealLoadsLimit,
    flow_limit_m3_per_s: Option<AutosizeOrNumber>,
    capacity_limit_w: Option<AutosizeOrNumber>,
) -> bool {
    match limit {
        IdealLoadsLimit::NoLimit => true,
        IdealLoadsLimit::LimitFlowRate => is_numeric(flow_limit_m3_per_s),
        IdealLoadsLimit::LimitCapacity => is_numeric(capacity_limit_w),
        IdealLoadsLimit::LimitFlowRateAndCapacity => {
            is_numeric(flow_limit_m3_per_s) && is_numeric(capacity_limit_w)
        }
    }
}

fn is_numeric(value: Option<AutosizeOrNumber>) -> bool {
    matches!(value, Some(AutosizeOrNumber::Value(_)))
}
