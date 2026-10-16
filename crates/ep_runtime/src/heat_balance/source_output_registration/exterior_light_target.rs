//! Actual fields of ExteriorLightUsage, resolved against the sole live row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceExteriorLightOutputField {
    Power,
    CurrentUse,
}
