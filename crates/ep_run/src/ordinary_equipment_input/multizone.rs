//! Source A's producer-free opaque multi-Zone and explicit paired-Wall stage.
//! This is input admission only; it supplies no scientific owner values.
use super::{number, text};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

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
                | "Material:InfraredTransparent"
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
                "Source A bounded multi-Zone excludes positive family {}",
                family.0
            ));
        }
    }
    validate_surface_pairs(raw)
}

// Input-only checks; canonical World coordinates are checked by the typed scope.
// Native Source getters still produce and validate every scientific owner.
fn validate_surface_pairs(raw: &RawModel) -> Result<(), String> {
    let Some(surfaces) = raw
        .objects
        .get(&ObjectType("BuildingSurface:Detailed".into()))
    else {
        return Ok(());
    };
    let constructions = raw.objects.get(&ObjectType("Construction".into()));
    let zones = raw.objects.get(&ObjectType("Zone".into()));
    for (name, row) in surfaces {
        if !text(row, "outside_boundary_condition")
            .is_some_and(|value| value.eq_ignore_ascii_case("Surface"))
        {
            continue;
        }
        let target_name = text(row, "outside_boundary_condition_object")
            .ok_or("paired Surface requires an actual target name")?;
        let matches: Vec<_> = surfaces
            .iter()
            .filter(|(name, _)| name.0.eq_ignore_ascii_case(target_name))
            .collect();
        if matches.len() != 1 || matches[0].0.0.eq_ignore_ascii_case(&name.0) {
            return Err("paired Surface requires one distinct actual target".into());
        }
        let (_, other) = matches[0];
        if !text(other, "outside_boundary_condition")
            .is_some_and(|value| value.eq_ignore_ascii_case("Surface"))
            || !text(other, "outside_boundary_condition_object")
                .is_some_and(|value| value.eq_ignore_ascii_case(&name.0))
        {
            return Err("paired Surface names must be explicitly reciprocal".into());
        }
        for surface in [row, other] {
            if !text(surface, "surface_type").is_some_and(|v| v.eq_ignore_ascii_case("Wall"))
                || !text(surface, "sun_exposure").is_some_and(|v| v.eq_ignore_ascii_case("NoSun"))
                || !text(surface, "wind_exposure").is_some_and(|v| v.eq_ignore_ascii_case("NoWind"))
            {
                return Err("paired Source Wall requires explicit NoSun and NoWind".into());
            }
        }
        let zone_name = text(row, "zone_name").ok_or("paired Wall actual Zone")?;
        let other_zone_name = text(other, "zone_name").ok_or("paired Wall adjacent Zone")?;
        if zone_name.eq_ignore_ascii_case(other_zone_name) {
            return Err("paired Walls require distinct Zones/implicit Spaces".into());
        }
        let multiplier = |zone_name: &str| {
            zones
                .and_then(|rows| {
                    rows.iter()
                        .find(|(name, _)| name.0.eq_ignore_ascii_case(zone_name))
                })
                .and_then(|(_, row)| {
                    row.fields
                        .get(&FieldName("multiplier".into()))
                        .map_or(Some(1.0), number)
                })
        };
        if multiplier(zone_name).is_none() || multiplier(zone_name) != multiplier(other_zone_name) {
            return Err("paired Walls require equal actual Zone multipliers".into());
        }
        let construction =
            text(row, "construction_name").ok_or("paired Wall actual Construction")?;
        if !text(other, "construction_name").is_some_and(|v| v.eq_ignore_ascii_case(construction)) {
            return Err("paired Walls require the same single-layer Construction".into());
        }
        let Some((_, construction)) = constructions.and_then(|rows| {
            rows.iter()
                .find(|(name, _)| name.0.eq_ignore_ascii_case(construction))
        }) else {
            return Err("paired Wall actual Construction target".into());
        };
        if text(construction, "outside_layer").is_none()
            || construction.fields.iter().any(|(field, value)| {
                field.0.starts_with("layer_")
                    && !matches!(value, RawValue::String(v) if v.trim().is_empty())
                    && !matches!(value, RawValue::Null)
            })
        {
            return Err("paired Walls require one actual construction layer".into());
        }
    }
    Ok(())
}
