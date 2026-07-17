//! Standalone glazing spectral datasets consumed by spectral window materials.

use crate::{GlazingSpectralDataId, NormalizedName};

/// One wavelength quartet from `MaterialProperty:GlazingSpectralData`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlazingSpectralPoint {
    /// Wavelength in microns.
    pub wavelength_microns: f64,
    /// Normal-incidence spectral transmittance after the source 0.001 clamp.
    pub transmittance: f64,
    /// Front-side normal-incidence spectral reflectance.
    pub front_reflectance: f64,
    /// Back-side normal-incidence spectral reflectance.
    pub back_reflectance: f64,
}

/// Validated standalone glazing spectral dataset.
#[derive(Clone, Debug, PartialEq)]
pub struct GlazingSpectralData {
    /// Typed ID in the separate glazing-spectral-data namespace.
    pub id: GlazingSpectralDataId,
    /// Normalized dataset name.
    pub name: NormalizedName,
    /// Source-ordered wavelength quartets.
    pub points: Vec<GlazingSpectralPoint>,
}
