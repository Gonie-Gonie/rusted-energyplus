//! IdealLoads input boundary checks for the first conformance candidate.

use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsLimit, OutdoorAirEconomizerType,
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
}

/// Classification for the first no-OA/no-limit sensible compatibility branch.
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

/// Classifies an IdealLoads system for the first diagnostic candidate.
#[must_use]
pub fn classify_no_oa_no_limit_sensible_subset(
    system: &IdealLoadsAirSystem,
) -> IdealLoadsSubsetBoundary {
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
    if system.heating_limit != IdealLoadsLimit::NoLimit {
        unsupported_features.push(IdealLoadsUnsupportedFeature::HeatingLimit);
    }
    if system.cooling_limit != IdealLoadsLimit::NoLimit {
        unsupported_features.push(IdealLoadsUnsupportedFeature::CoolingLimit);
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

    IdealLoadsSubsetBoundary {
        unsupported_features,
    }
}
