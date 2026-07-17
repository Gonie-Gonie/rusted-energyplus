use crate::{MaterialId, MaterialPhaseChangeId, NormalizedName};

/// One ordered temperature-enthalpy point for a CondFD phase-change table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseChangeTemperatureEnthalpyPoint {
    /// Table temperature in C.
    pub temperature_c: f64,
    /// Specific enthalpy in J/kg.
    pub enthalpy_j_per_kg: f64,
}

/// Typed `MaterialProperty:PhaseChange` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialPhaseChange {
    /// Stable typed object ID.
    pub id: MaterialPhaseChangeId,
    /// Normalized object key, which is also the referenced material name.
    pub name: NormalizedName,
    /// Referenced regular or no-mass material.
    pub reference_material: MaterialId,
    /// Thermal-conductivity change per degree from the 20 C base temperature.
    pub temperature_coefficient_for_thermal_conductivity_w_per_m_k2: f64,
    /// Source-ordered temperature-enthalpy function points.
    pub temperature_enthalpy_points: Vec<PhaseChangeTemperatureEnthalpyPoint>,
}
