//! Typed request state for incident-solar multipliers on window surfaces.

use crate::{NormalizedName, ScheduleId, SurfaceIncidentSolarMultiplierRequestId};

/// Validated declaration awaiting fenestration-surface resolution.
///
/// The source routine ultimately mutates window-surface state. Until that
/// surface family is typed, this record retains only the validated input
/// request and deliberately does not manufacture a `SurfaceId`.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceIncidentSolarMultiplierRequest {
    /// Typed request ID within the request arena.
    pub id: SurfaceIncidentSolarMultiplierRequestId,
    /// Normalized, nonsemantic epJSON outer-key snapshot.
    pub declaration_name: NormalizedName,
    /// Normalized target window-surface name awaiting typed resolution.
    pub surface_name: NormalizedName,
    /// Source incident-solar multiplier in the inclusive range `[0, 1]`.
    pub multiplier: f64,
    /// Optional schedule resolved through the shared schedule namespace.
    pub schedule: Option<ScheduleId>,
}
