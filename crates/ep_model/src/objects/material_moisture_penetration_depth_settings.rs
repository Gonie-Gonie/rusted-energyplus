use crate::{AutoOrNumber, MaterialId, MaterialMoisturePenetrationDepthSettingsId, NormalizedName};

/// Typed `MaterialProperty:MoisturePenetrationDepth:Settings` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialMoisturePenetrationDepthSettings {
    /// Stable typed object ID.
    pub id: MaterialMoisturePenetrationDepthSettingsId,
    /// Normalized object key, which is also the referenced material name.
    pub name: NormalizedName,
    /// Referenced regular mass material.
    pub reference_material: MaterialId,
    /// Ratio of stagnant-air to material water-vapor permeability.
    pub water_vapor_diffusion_resistance_factor: f64,
    /// Moisture sorption equation coefficient a.
    pub moisture_equation_coefficient_a: f64,
    /// Moisture sorption equation coefficient b.
    pub moisture_equation_coefficient_b: f64,
    /// Moisture sorption equation coefficient c.
    pub moisture_equation_coefficient_c: f64,
    /// Moisture sorption equation coefficient d.
    pub moisture_equation_coefficient_d: f64,
    /// Explicit surface-layer penetration depth in m, or source autocalculation.
    pub surface_layer_penetration_depth_m: AutoOrNumber,
    /// Explicit deep-layer penetration depth in m, or source autocalculation.
    pub deep_layer_penetration_depth_m: AutoOrNumber,
    /// Coating-layer thickness in m.
    pub coating_layer_thickness_m: f64,
    /// Coating-layer water-vapor diffusion resistance factor.
    pub coating_layer_water_vapor_diffusion_resistance_factor: f64,
}
