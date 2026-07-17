use crate::{
    MaterialHeatAndMoistureTransferSorptionIsothermId,
    MaterialHeatAndMoistureTransferThermalConductivityId, MaterialId, NormalizedName,
};

/// One moisture-content/thermal-conductivity point in a HAMT conductivity table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferThermalConductivityPoint {
    /// Volumetric moisture content in kg/m3.
    pub moisture_content_kg_per_m3: f64,
    /// Moisture-dependent thermal conductivity in W/(m K).
    pub thermal_conductivity_w_per_m_k: f64,
}

/// Typed `MaterialProperty:HeatAndMoistureTransfer:ThermalConductivity` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferThermalConductivity {
    /// Stable typed object ID.
    pub id: MaterialHeatAndMoistureTransferThermalConductivityId,
    /// Normalized epJSON instance key; the source object has no semantic name.
    pub name: NormalizedName,
    /// Referenced regular mass material.
    pub reference_material: MaterialId,
    /// Sorption attachment whose last source-effective moisture point is appended.
    pub reference_sorption_isotherm: MaterialHeatAndMoistureTransferSorptionIsothermId,
    /// Declared thermal-coordinate count, constrained to one through twenty-five.
    pub number_of_thermal_coordinates: u8,
    /// Count-selected coordinates after source blank numeric fields become zero.
    pub input_points: Vec<MaterialHeatAndMoistureTransferThermalConductivityPoint>,
    /// Source-effective coordinates after appending the indexed last Sorption moisture point.
    pub effective_points: Vec<MaterialHeatAndMoistureTransferThermalConductivityPoint>,
}
