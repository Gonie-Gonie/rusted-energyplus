//! Source A's disconnected, producer-free opaque multi-Zone stage.
//! This is input admission only; it supplies no scientific owner values.
use ep_raw_model::{ObjectType, RawModel};

pub(super) fn has_multiple_zones(raw: &RawModel) -> bool {
    raw.objects
        .get(&ObjectType("Zone".into()))
        .is_some_and(|rows| rows.len() > 1)
}

pub(super) fn validate(raw: &RawModel) -> Result<(), String> {
    for (family, rows) in &raw.objects {
        if rows.is_empty() {
            continue;
        }
        // An explicit positive-family boundary prevents previously admitted
        // one-Zone gains, comfort and equipment paths from leaking into this
        // new topology. Existing per-field/reference checks still run next.
        if !matches!(
            family.0.as_str(),
            "Version"
                | "Timestep"
                | "Building"
                | "HeatBalanceAlgorithm"
                | "SurfaceConvectionAlgorithm:Inside"
                | "SurfaceConvectionAlgorithm:Outside"
                | "SimulationControl"
                | "RunPeriod"
                | "Site:Location"
                | "Site:GroundTemperature:BuildingSurface"
                | "Site:GroundTemperature:FCfactorMethod"
                | "Site:GroundTemperature:Shallow"
                | "Site:GroundTemperature:Deep"
                | "SizingPeriod:DesignDay"
                | "Material"
                | "Material:NoMass"
                | "Material:AirGap"
                | "Construction"
                | "Zone"
                | "GlobalGeometryRules"
                | "BuildingSurface:Detailed"
                | "ZoneAirHeatBalanceAlgorithm"
                | "RunPeriodControl:DaylightSavingTime"
                | "RunPeriodControl:SpecialDays"
                | "ScheduleTypeLimits"
                | "Schedule:Constant"
                | "Schedule:Compact"
                | "Schedule:File"
                | "Schedule:Day:Hourly"
                | "Schedule:Day:Interval"
                | "Schedule:Day:List"
                | "Schedule:Week:Compact"
                | "Schedule:Week:Daily"
                | "Schedule:Year"
                | "Output:Schedules"
                | "Output:Diagnostics"
                | "Output:VariableDictionary"
                | "Output:Variable"
                | "Output:Surfaces:Drawing"
                | "Output:Surfaces:List"
                | "Output:Constructions"
                | "Output:Meter"
                | "Output:Meter:MeterFileOnly"
                | "OutputControl:Table:Style"
                | "Output:Table:SummaryReports"
        ) {
            return Err(format!(
                "Source A disconnected multi-Zone excludes positive family {}",
                family.0
            ));
        }
    }
    Ok(())
}
