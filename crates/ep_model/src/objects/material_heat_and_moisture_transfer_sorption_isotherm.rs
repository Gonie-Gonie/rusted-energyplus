use crate::{
    MaterialHeatAndMoistureTransferSettingsId, MaterialHeatAndMoistureTransferSorptionIsothermId,
    MaterialId, NormalizedName,
};

/// One relative-humidity/moisture-content point in a HAMT sorption table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferSorptionPoint {
    /// Relative humidity as a fraction.
    pub relative_humidity_fraction: f64,
    /// Volumetric moisture content in kg/m3.
    pub moisture_content_kg_per_m3: f64,
}

/// Typed `MaterialProperty:HeatAndMoistureTransfer:SorptionIsotherm` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferSorptionIsotherm {
    /// Stable typed object ID.
    pub id: MaterialHeatAndMoistureTransferSorptionIsothermId,
    /// Normalized epJSON instance key; the source object has no semantic name.
    pub name: NormalizedName,
    /// Referenced regular mass material.
    pub reference_material: MaterialId,
    /// Settings attachment whose porosity supplies the source upper endpoint.
    pub reference_settings: MaterialHeatAndMoistureTransferSettingsId,
    /// Declared coordinate count, constrained to one through twenty-five.
    pub number_of_isotherm_coordinates: u8,
    /// Count-selected coordinates after source blank numeric fields become zero.
    pub input_points: Vec<MaterialHeatAndMoistureTransferSorptionPoint>,
    /// Source-effective coordinates after endpoint insertion, sorting, and correction.
    pub effective_points: Vec<MaterialHeatAndMoistureTransferSorptionPoint>,
    /// Whether source-equivalent adjacent averaging changed moisture contents.
    pub moisture_content_was_adjusted: bool,
}
