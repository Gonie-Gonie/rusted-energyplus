//! Scheduled absorbed-solar declarations for complex-fenestration layers.

use crate::{ConstructionId, FenestrationSolarAbsorbedRequestId, NormalizedName, ScheduleId};

/// A validated `ComplexFenestrationProperty:SolarAbsorbedLayers` request.
///
/// The fenestration surface remains a normalized name until the corresponding
/// typed surface family is available. Construction and layer schedules are
/// already resolved against their typed namespaces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FenestrationSolarAbsorbedRequest {
    /// Dense identity within this request family.
    pub id: FenestrationSolarAbsorbedRequestId,
    /// Normalized semantic declaration name.
    pub name: NormalizedName,
    /// Unresolved normalized fenestration-surface target.
    pub fenestration_surface_name: NormalizedName,
    /// Resolved complex-fenestration construction.
    pub construction: ConstructionId,
    /// Resolved per-solid-layer schedules in outside-to-inside order.
    pub layer_schedules: Vec<ScheduleId>,
}
