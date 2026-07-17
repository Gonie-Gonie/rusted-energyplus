use crate::{MaterialHeatAndMoistureTransferSettingsId, MaterialId, NormalizedName};

/// Typed `MaterialProperty:HeatAndMoistureTransfer:Settings` attachment.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialHeatAndMoistureTransferSettings {
    /// Stable typed object ID.
    pub id: MaterialHeatAndMoistureTransferSettingsId,
    /// Normalized epJSON instance key; the source object has no semantic name.
    pub name: NormalizedName,
    /// Referenced regular mass material.
    pub reference_material: MaterialId,
    /// Open pore volume divided by total material volume.
    pub porosity: f64,
    /// Initial water mass divided by dry material mass for each run period.
    pub initial_water_content_ratio: f64,
}
