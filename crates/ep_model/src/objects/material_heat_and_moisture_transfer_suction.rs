use crate::{
    MaterialHeatAndMoistureTransferSorptionIsothermId, MaterialHeatAndMoistureTransferSuctionId,
    MaterialId, NormalizedName,
};

/// One moisture-content/liquid-transport point in a HAMT suction table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferSuctionPoint {
    /// Volumetric moisture content in kg/m3.
    pub moisture_content_kg_per_m3: f64,
    /// Suction liquid transport coefficient in m2/s.
    pub liquid_transport_coefficient_m2_per_s: f64,
}

/// Typed `MaterialProperty:HeatAndMoistureTransfer:Suction` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferSuction {
    /// Stable typed object ID.
    pub id: MaterialHeatAndMoistureTransferSuctionId,
    /// Normalized epJSON instance key; the source object has no semantic name.
    pub name: NormalizedName,
    /// Referenced regular mass material.
    pub reference_material: MaterialId,
    /// Sorption attachment whose last source-effective moisture point is appended.
    pub reference_sorption_isotherm: MaterialHeatAndMoistureTransferSorptionIsothermId,
    /// Declared suction-point count, constrained to one through twenty-five.
    pub number_of_suction_points: u8,
    /// Count-selected coordinates after source blank numeric fields become zero.
    pub input_points: Vec<MaterialHeatAndMoistureTransferSuctionPoint>,
    /// Source-effective coordinates after appending the sorption high-water endpoint.
    pub effective_points: Vec<MaterialHeatAndMoistureTransferSuctionPoint>,
}
