use crate::{MaterialId, MaterialVariableThermalConductivityId, NormalizedName};

/// One ordered temperature-conductivity point for a CondFD material table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialVariableThermalConductivityPoint {
    /// Table temperature in C.
    pub temperature_c: f64,
    /// Thermal conductivity in W/m-K.
    pub thermal_conductivity_w_per_m_k: f64,
}

/// Typed `MaterialProperty:VariableThermalConductivity` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialVariableThermalConductivity {
    /// Stable typed object ID.
    pub id: MaterialVariableThermalConductivityId,
    /// Normalized object key, which is also the referenced material name.
    pub name: NormalizedName,
    /// Referenced regular or no-mass material.
    pub reference_material: MaterialId,
    /// Source-ordered temperature-conductivity function points.
    pub temperature_conductivity_points: Vec<MaterialVariableThermalConductivityPoint>,
}
