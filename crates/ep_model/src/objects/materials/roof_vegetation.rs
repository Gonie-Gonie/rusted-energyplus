use super::{MaterialSurfaceRoughness, OpaqueSurfaceProperties};

/// Moisture redistribution method for a `Material:RoofVegetation` object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoofVegetationMoistureDiffusionMethod {
    /// Original simplified moisture-diffusion model.
    Simple,
    /// Schaap-Genuchten moisture redistribution, exposed as `Advanced` by
    /// EnergyPlus.
    Advanced,
}

impl RoofVegetationMoistureDiffusionMethod {
    /// Parses an EnergyPlus moisture-diffusion method token.
    #[must_use]
    pub fn from_energyplus_name(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "SIMPLE" => Some(Self::Simple),
            "ADVANCED" => Some(Self::Advanced),
            _ => None,
        }
    }
}

/// Source-effective fields for a `Material:RoofVegetation` object.
///
/// EnergyPlus reads the legacy `Soil Layer Name` field but does not store or
/// consume it, so it is intentionally absent from this payload.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofVegetationMaterial {
    /// Plant height in meters.
    pub height_of_plants_m: f64,
    /// Leaf area index.
    pub leaf_area_index: f64,
    /// Leaf solar reflectivity.
    pub leaf_reflectivity: f64,
    /// Leaf thermal emissivity.
    pub leaf_emissivity: f64,
    /// Minimum stomatal resistance in s/m.
    pub minimum_stomatal_resistance_s_per_m: f64,
    /// Dry-soil surface roughness used by exterior convection algorithms.
    pub roughness: MaterialSurfaceRoughness,
    /// Soil-layer thickness in meters.
    pub thickness_m: f64,
    /// Dry-soil thermal conductivity in W/m-K.
    pub dry_soil_conductivity_w_per_m_k: f64,
    /// Dry-soil density in kg/m3.
    pub dry_soil_density_kg_per_m3: f64,
    /// Dry-soil specific heat in J/kg-K.
    pub dry_soil_specific_heat_j_per_kg_k: f64,
    /// Shared opaque soil-surface absorptances.
    pub surface: OpaqueSurfaceProperties,
    /// Saturation volumetric moisture content (soil porosity).
    pub saturation_volumetric_moisture_content: f64,
    /// Residual volumetric moisture content.
    pub residual_volumetric_moisture_content: f64,
    /// Source-effective initial volumetric moisture content.
    pub initial_volumetric_moisture_content: f64,
    /// Moisture redistribution method.
    pub moisture_diffusion_method: RoofVegetationMoistureDiffusionMethod,
}
