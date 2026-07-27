//! Numeric flow and capacity limit helpers for IdealLoads calculations.

use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsLimit};

use crate::ideal_loads::PurchasedAirSizedLimits;

use super::psychrometrics::{
    DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3, STANDARD_PRESSURE_SEA_LEVEL_PA,
    energyplus_standard_air_density_kg_per_m3, standard_pressure_elevation_base,
};

/// Runtime context needed for numeric IdealLoads flow limits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsSensibleLimitContext {
    /// Standard air density in kg/m3 used to convert volumetric limits to mass limits.
    pub standard_air_density_kg_per_m3: f64,
    /// Barometric pressure in Pa used by supply-air saturation checks.
    pub barometric_pressure_pa: f64,
    /// Begin-environment cached maximum heating mass flow, when initialized.
    pub initialized_heating_air_mass_flow_limit_kg_per_s: Option<f64>,
    /// Begin-environment cached maximum cooling mass flow, when initialized.
    pub initialized_cooling_air_mass_flow_limit_kg_per_s: Option<f64>,
    /// Runtime-owned four-field `SizePurchasedAir` overlay, when initialized.
    pub purchased_air_sized_limits: Option<PurchasedAirSizedLimits>,
}

impl Default for IdealLoadsSensibleLimitContext {
    fn default() -> Self {
        Self {
            standard_air_density_kg_per_m3: DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3,
            barometric_pressure_pa: STANDARD_PRESSURE_SEA_LEVEL_PA,
            initialized_heating_air_mass_flow_limit_kg_per_s: None,
            initialized_cooling_air_mass_flow_limit_kg_per_s: None,
            purchased_air_sized_limits: None,
        }
    }
}

impl IdealLoadsSensibleLimitContext {
    /// Builds the limit context from EnergyPlus `StdRhoAir` source-order inputs.
    #[must_use]
    pub fn from_site_elevation_m(elevation_m: f64) -> Option<Self> {
        let base = standard_pressure_elevation_base(elevation_m)?;
        let barometric_pressure_pa = STANDARD_PRESSURE_SEA_LEVEL_PA * base.powf(5.2559);
        energyplus_standard_air_density_kg_per_m3(elevation_m).map(
            |standard_air_density_kg_per_m3| Self {
                standard_air_density_kg_per_m3,
                barometric_pressure_pa,
                initialized_heating_air_mass_flow_limit_kg_per_s: None,
                initialized_cooling_air_mass_flow_limit_kg_per_s: None,
                purchased_air_sized_limits: None,
            },
        )
    }

    /// Returns a copy using the supplied timestep barometric pressure.
    #[must_use]
    pub fn with_barometric_pressure_pa(mut self, barometric_pressure_pa: f64) -> Self {
        self.barometric_pressure_pa = barometric_pressure_pa;
        self
    }

    /// Returns a copy backed by `InitPurchasedAir` begin-environment flow caches.
    #[must_use]
    pub fn with_initialized_flow_limits(
        mut self,
        standard_air_density_kg_per_m3: f64,
        heating_kg_per_s: f64,
        cooling_kg_per_s: f64,
    ) -> Self {
        self.standard_air_density_kg_per_m3 = standard_air_density_kg_per_m3;
        self.initialized_heating_air_mass_flow_limit_kg_per_s = Some(heating_kg_per_s);
        self.initialized_cooling_air_mass_flow_limit_kg_per_s = Some(cooling_kg_per_s);
        self
    }

    /// Returns a copy backed by the persistent four-field sizing overlay.
    #[must_use]
    pub fn with_purchased_air_sized_limits(
        mut self,
        sized_limits: PurchasedAirSizedLimits,
    ) -> Self {
        self.purchased_air_sized_limits = Some(sized_limits);
        self
    }

    pub(super) fn sized_limits_or_system(
        self,
        system: &IdealLoadsAirSystem,
    ) -> PurchasedAirSizedLimits {
        self.purchased_air_sized_limits
            .unwrap_or_else(|| PurchasedAirSizedLimits::from_system(system))
    }
}

pub(super) fn flow_limit_kg_per_s(
    limit: IdealLoadsLimit,
    flow_limit_m3_per_s: Option<AutosizeOrNumber>,
    initialized_mass_flow_limit_kg_per_s: Option<f64>,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Option<f64> {
    if !limit_includes_flow_rate(limit) {
        return None;
    }

    initialized_mass_flow_limit_kg_per_s.or_else(|| {
        numeric_autosize_value(flow_limit_m3_per_s).map(|flow_limit_m3_per_s| {
            flow_limit_m3_per_s * limit_context.standard_air_density_kg_per_m3
        })
    })
}

pub(super) fn capacity_limit_w(
    limit: IdealLoadsLimit,
    capacity_limit_w: Option<AutosizeOrNumber>,
) -> Option<f64> {
    if !limit_includes_capacity(limit) {
        return None;
    }

    numeric_autosize_value(capacity_limit_w)
}

pub(super) fn cooling_capacity_limit_is_zero(
    system: &IdealLoadsAirSystem,
    context: IdealLoadsSensibleLimitContext,
) -> bool {
    let sized_limits = context.sized_limits_or_system(system);
    matches!(
        capacity_limit_w(
            system.cooling_limit,
            sized_limits.maximum_total_cooling_capacity_w,
        ),
        Some(capacity_limit_w) if capacity_limit_w <= 0.0
    )
}

pub(super) fn heating_capacity_limit_is_zero(
    system: &IdealLoadsAirSystem,
    context: IdealLoadsSensibleLimitContext,
) -> bool {
    let sized_limits = context.sized_limits_or_system(system);
    matches!(
        capacity_limit_w(
            system.heating_limit,
            sized_limits.maximum_sensible_heating_capacity_w,
        ),
        Some(capacity_limit_w) if capacity_limit_w <= 0.0
    )
}

fn numeric_autosize_value(value: Option<AutosizeOrNumber>) -> Option<f64> {
    match value {
        Some(AutosizeOrNumber::Value(value)) => Some(value),
        Some(AutosizeOrNumber::Autosize) | None => None,
    }
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
