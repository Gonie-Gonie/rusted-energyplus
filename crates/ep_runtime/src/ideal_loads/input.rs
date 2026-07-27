//! IdealLoads input boundary checks for sensible IdealLoads candidates.

use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsLimit,
    OutdoorAirEconomizerType,
};

/// Compile-stage IdealLoads feature flags used to choose compatibility branches.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IdealLoadsFeatureFlags {
    /// Outdoor-air design object or inlet node is present.
    pub has_outdoor_air: bool,
    /// Outdoor-air economizer is active.
    pub has_economizer: bool,
    /// Heat recovery is active.
    pub has_heat_recovery: bool,
    /// Demand controlled ventilation is active.
    pub has_dcv: bool,
    /// Humidistat humidity control is active.
    pub has_humidistat: bool,
    /// ConstantSensibleHeatRatio dehumidification branch is active.
    pub has_constant_shr: bool,
    /// ConstantSupplyHumidityRatio humidity branch is active.
    pub has_constant_supply_humidity: bool,
    /// Heating or cooling flow limit is active.
    pub has_flow_limit: bool,
    /// Heating or cooling capacity limit is active.
    pub has_capacity_limit: bool,
    /// Autosize appears in any IdealLoads flow or capacity limit field.
    pub has_autosize: bool,
}

impl IdealLoadsFeatureFlags {
    /// Builds feature flags from a typed IdealLoads system.
    #[must_use]
    pub fn from_system(system: &IdealLoadsAirSystem) -> Self {
        Self {
            has_outdoor_air: system
                .design_specification_outdoor_air_object_name
                .is_some()
                || system.outdoor_air_inlet_node_name.is_some(),
            has_economizer: system.outdoor_air_economizer_type
                != OutdoorAirEconomizerType::NoEconomizer,
            has_heat_recovery: system.heat_recovery_type != HeatRecoveryType::None,
            has_dcv: system.demand_controlled_ventilation_type
                != DemandControlledVentilationType::None,
            has_humidistat: system.dehumidification_control_type
                == DehumidificationControlType::Humidistat
                || system.humidification_control_type == HumidificationControlType::Humidistat,
            has_constant_shr: system.dehumidification_control_type
                == DehumidificationControlType::ConstantSensibleHeatRatio,
            has_constant_supply_humidity: system.dehumidification_control_type
                == DehumidificationControlType::ConstantSupplyHumidityRatio
                || system.humidification_control_type
                    == HumidificationControlType::ConstantSupplyHumidityRatio,
            has_flow_limit: limit_includes_flow_rate(system.heating_limit)
                || limit_includes_flow_rate(system.cooling_limit),
            has_capacity_limit: limit_includes_capacity(system.heating_limit)
                || limit_includes_capacity(system.cooling_limit),
            has_autosize: is_autosize(system.maximum_heating_air_flow_rate_m3_per_s)
                || is_autosize(system.maximum_sensible_heating_capacity_w)
                || is_autosize(system.maximum_cooling_air_flow_rate_m3_per_s)
                || is_autosize(system.maximum_total_cooling_capacity_w),
        }
    }
}

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
    /// Heating limit is active but a required hard size is missing, autosized,
    /// negative, or nonfinite.
    UnresolvedHeatingLimit,
    /// Cooling limit is active but a required hard size is missing, autosized,
    /// negative, or nonfinite.
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
    let feature_flags = IdealLoadsFeatureFlags::from_system(system);

    if feature_flags.has_outdoor_air {
        unsupported_features.push(IdealLoadsUnsupportedFeature::OutdoorAir);
    }
    if feature_flags.has_dcv {
        unsupported_features.push(IdealLoadsUnsupportedFeature::DemandControlledVentilation);
    }
    if feature_flags.has_economizer {
        unsupported_features.push(IdealLoadsUnsupportedFeature::Economizer);
    }
    if feature_flags.has_heat_recovery {
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
    matches!(value, Some(AutosizeOrNumber::Value(value)) if value.is_finite() && value >= 0.0)
}

fn is_autosize(value: Option<AutosizeOrNumber>) -> bool {
    matches!(value, Some(AutosizeOrNumber::Autosize))
}

fn limit_includes_flow_rate(limit: IdealLoadsLimit) -> bool {
    matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
}

fn limit_includes_capacity(limit: IdealLoadsLimit) -> bool {
    matches!(
        limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
}
