//! Actual four SystemReports facility ventilation time fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceSystemReportsOutputField {
    AnyZoneBelowTargetTime,
    AllZonesAtTargetTime,
    AnyZoneAboveTargetTime,
    AnyZoneUnoccupiedVentilationTime,
}
