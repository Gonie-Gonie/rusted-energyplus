//! Typed scheduled inside-surface incident-solar declarations.

use crate::{ConstructionId, NormalizedName, ScheduleId, SurfaceId, SurfaceSolarIncidentId};

/// A validated `SurfaceProperty:SolarIncidentInside` declaration.
///
/// The record retains the source-selected surface/construction pair and
/// schedule. Representative-surface mutation and runtime application remain
/// outside this bounded typed-input phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceSolarIncident {
    /// Typed declaration ID within the arena.
    pub id: SurfaceSolarIncidentId,
    /// Required semantic declaration name.
    pub name: NormalizedName,
    /// Resolved opaque building-surface target.
    pub surface: SurfaceId,
    /// Resolved construction selected by the declaration.
    pub construction: ConstructionId,
    /// Resolved incident-solar schedule.
    pub schedule: ScheduleId,
}
