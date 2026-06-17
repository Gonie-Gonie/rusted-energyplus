//! Shared IdealLoads calculation input and result types.

/// Operating mode selected by the first IdealLoads sensible subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsSensibleMode {
    /// Unit is unavailable.
    Off,
    /// No sensible load is active.
    Deadband,
    /// Cooling branch is active.
    Cooling,
    /// Heating branch is active.
    Heating,
}

/// Zone-side inputs to `CalcPurchAirLoads`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsZoneState {
    /// Zone air temperature in C.
    pub air_temperature_c: f64,
    /// Zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
}

/// Result of the no-OA sensible calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsSensibleResult {
    /// Selected operating mode.
    pub mode: IdealLoadsSensibleMode,
    /// EnergyPlus `PsyCpAirFnW` value in J/kg-K.
    pub cp_air_j_per_kg_k: f64,
    /// Final supply temperature in C.
    pub supply_temperature_c: f64,
    /// Final supply humidity ratio in kgWater/kgDryAir.
    pub supply_humidity_ratio: f64,
    /// Final supply enthalpy in J/kg.
    pub supply_enthalpy_j_per_kg: f64,
    /// Final supply mass flow in kg/s.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Sensible heating flow contribution in kg/s.
    pub heating_mass_flow_rate_kg_per_s: f64,
    /// Sensible cooling flow contribution in kg/s.
    pub cooling_mass_flow_rate_kg_per_s: f64,
    /// Zone total heating rate in W.
    pub zone_total_heating_rate_w: f64,
    /// Zone total cooling rate in W.
    pub zone_total_cooling_rate_w: f64,
    /// Zone sensible heating rate in W.
    pub zone_sensible_heating_rate_w: f64,
    /// Zone sensible cooling rate in W.
    pub zone_sensible_cooling_rate_w: f64,
    /// Zone latent heating rate in W.
    pub zone_latent_heating_rate_w: f64,
    /// Zone latent cooling rate in W.
    pub zone_latent_cooling_rate_w: f64,
    /// Supply air sensible heating rate in W.
    pub supply_air_sensible_heating_rate_w: f64,
    /// Supply air sensible cooling rate in W.
    pub supply_air_sensible_cooling_rate_w: f64,
    /// Supply air latent heating rate in W.
    pub supply_air_latent_heating_rate_w: f64,
    /// Supply air latent cooling rate in W.
    pub supply_air_latent_cooling_rate_w: f64,
    /// Supply air total heating rate in W.
    pub supply_air_total_heating_rate_w: f64,
    /// Supply air total cooling rate in W.
    pub supply_air_total_cooling_rate_w: f64,
}
