//! Actual SystemReports facility and indexed controlled-zone ventilation fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceSystemReportsOutputField {
    Zone {
        index: i32,
        field: SourceZoneVentReportOutputField,
    },
    AnyZoneBelowTargetTime,
    AllZonesAtTargetTime,
    AnyZoneAboveTargetTime,
    AnyZoneUnoccupiedVentilationTime,
}

/// Native ZoneVentRepVars fields, resolved on their sole live SystemReports owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceZoneVentReportOutputField {
    NoLoadCoolingByVent,
    CoolingLoadAddedByVent,
    OverheatingByVent,
    CoolingLoadMetByVent,
    NoLoadHeatingByVent,
    HeatingLoadAddedByVent,
    OvercoolingByVent,
    HeatingLoadMetByVent,
    OaMassFlow,
    OaMass,
    OaVolFlowStdRho,
    OaVolStdRho,
    OaVolFlowCrntRho,
    OaVolCrntRho,
    MechAch,
    TargetVentilationFlowVoz,
    TimeBelowVozDyn,
    TimeAtVozDyn,
    TimeAboveVozDyn,
    TimeVentUnocc,
}
