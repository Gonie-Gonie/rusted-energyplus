use crate::{
    MaterialHeatAndMoistureTransferDiffusionId, MaterialHeatAndMoistureTransferSorptionIsothermId,
    MaterialId, NormalizedName,
};

/// One relative-humidity/water-vapor-resistance point in a HAMT diffusion table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferDiffusionPoint {
    /// Relative humidity entered as a fraction.
    pub relative_humidity_fraction: f64,
    /// Dimensionless water-vapor diffusion resistance factor.
    pub water_vapor_diffusion_resistance_factor: f64,
}

/// Typed `MaterialProperty:HeatAndMoistureTransfer:Diffusion` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferDiffusion {
    /// Stable typed object ID.
    pub id: MaterialHeatAndMoistureTransferDiffusionId,
    /// Normalized epJSON instance key; the source object has no semantic name.
    pub name: NormalizedName,
    /// Referenced regular mass material.
    pub reference_material: MaterialId,
    /// Sorption attachment whose indexed last source-effective RH point is appended.
    pub reference_sorption_isotherm: MaterialHeatAndMoistureTransferSorptionIsothermId,
    /// Declared data-pair count, constrained to one through twenty-five.
    pub number_of_data_pairs: u8,
    /// Count-selected pairs after source blank numeric fields become zero.
    pub input_points: Vec<MaterialHeatAndMoistureTransferDiffusionPoint>,
    /// Source-effective pairs after appending the indexed last Sorption RH point.
    pub effective_points: Vec<MaterialHeatAndMoistureTransferDiffusionPoint>,
}
