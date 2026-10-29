//! Source A's bounded opaque multi-Zone, direct-gain and paired-surface stage.
//! This is input admission only; it supplies no scientific owner values.
use super::{number, text};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawObject, RawValue};

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
        // other one-Zone gain, comfort and HVAC paths from leaking into this
        // topology. Existing per-field/reference checks still run next.
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
                | "ZoneList"
                | "ZoneGroup"
                | "Space"
                | "SpaceList"
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
                | "OtherEquipment"
                | "ElectricEquipment"
                | "People"
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

// Empty exposure strings and absent fields select the schema defaults. Preserve
// authored presence for the actual getter; other JSON kinds/whitespace are not
// members of the schema enum and must not pass through a generic blank alias.
fn legal_exposure(row: &RawObject, field: &str, values: &[&str]) -> bool {
    match row.fields.get(&FieldName(field.into())) {
        None => true,
        Some(RawValue::String(value)) => {
            value.is_empty()
                || values
                    .iter()
                    .any(|choice| value.eq_ignore_ascii_case(choice))
        }
        _ => false,
    }
}

// Actual contiguous input references, not a construction/property producer.
// Native opaque reversal compares material identity; equal thermal values of
// differently named materials never substitute for the same layer reference.
fn construction_layers(row: &RawObject) -> Result<Vec<&str>, String> {
    let mut layers = vec![
        text(row, "outside_layer")
            .ok_or("paired opaque surface actual outside Material reference")?,
    ];
    let mut ended = false;
    for number in 2..=10 {
        match row.fields.get(&FieldName(format!("layer_{number}"))) {
            None => ended = true,
            Some(RawValue::String(value)) if value.is_empty() => ended = true,
            Some(RawValue::String(value)) if !ended && !value.trim().is_empty() => {
                layers.push(value);
            }
            _ => {
                return Err(
                    "paired opaque surfaces require contiguous actual Material references".into(),
                );
            }
        }
    }
    Ok(layers)
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
    let infrared = raw
        .objects
        .get(&ObjectType("Material:InfraredTransparent".into()));
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
            if !text(surface, "surface_type").is_some_and(|value| {
                ["Wall", "Floor", "Roof", "Ceiling"]
                    .iter()
                    .any(|kind| value.eq_ignore_ascii_case(kind))
            }) || !legal_exposure(surface, "sun_exposure", &["NoSun", "SunExposed"])
                || !legal_exposure(surface, "wind_exposure", &["NoWind", "WindExposed"])
            {
                return Err(
                    "paired Source opaque surface requires schema-valid class, Sun and Wind exposure fields".into(),
                );
            }
        }
        let current_class =
            text(row, "surface_type").ok_or("paired opaque surface actual class")?;
        let other_class =
            text(other, "surface_type").ok_or("paired opaque surface adjacent class")?;
        let is_roof = |value: &str| {
            ["Roof", "Ceiling"]
                .iter()
                .any(|kind| value.eq_ignore_ascii_case(kind))
        };
        if !((current_class.eq_ignore_ascii_case("Wall")
            && other_class.eq_ignore_ascii_case("Wall"))
            || (is_roof(current_class) && other_class.eq_ignore_ascii_case("Floor"))
            || (current_class.eq_ignore_ascii_case("Floor") && is_roof(other_class)))
        {
            return Err(
                "paired opaque class-mismatch warnings remain outside this Source stage".into(),
            );
        }
        let zone_name = text(row, "zone_name").ok_or("paired opaque surface actual Zone")?;
        let other_zone_name =
            text(other, "zone_name").ok_or("paired opaque surface adjacent Zone")?;
        if zone_name.eq_ignore_ascii_case(other_zone_name) {
            return Err("paired opaque surfaces require distinct Zones/implicit Spaces".into());
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
            return Err("paired opaque surfaces require equal actual Zone multipliers".into());
        }
        let mut layers = Vec::with_capacity(2);
        for surface in [row, other] {
            let name = text(surface, "construction_name")
                .ok_or("paired opaque surface actual Construction")?;
            let (_, construction) = constructions
                .and_then(|rows| {
                    rows.iter()
                        .find(|(key, _)| key.0.eq_ignore_ascii_case(name))
                })
                .ok_or("paired opaque surface actual Construction target")?;
            layers.push(construction_layers(construction)?);
        }
        // Only normal opaque classes expand here. The existing IRT route
        // remains Wall-only even though its separate use guard accepts Surface.
        let uses_irt = layers.iter().any(|stack| {
            infrared.is_some_and(|rows| {
                stack.iter().any(|material| {
                    rows.keys()
                        .any(|name| name.0.eq_ignore_ascii_case(material))
                })
            })
        });
        if uses_irt
            && [row, other].iter().any(|surface| {
                !text(surface, "surface_type")
                    .is_some_and(|value| value.eq_ignore_ascii_case("Wall"))
            })
        {
            return Err("IRT paired surfaces retain the existing Wall-only domain".into());
        }
        if layers.iter().any(|stack| {
            stack.len() > 1
                && infrared.is_some_and(|rows| {
                    stack.iter().any(|material| {
                        rows.keys()
                            .any(|name| name.0.eq_ignore_ascii_case(material))
                    })
                })
        }) {
            return Err("paired IRT Construction must retain its sole Material layer".into());
        }
        if layers[0].len() != layers[1].len()
            && layers.iter().any(|stack| {
                infrared.is_some_and(|rows| {
                    stack.iter().any(|material| {
                        rows.keys()
                            .any(|name| name.0.eq_ignore_ascii_case(material))
                    })
                })
            })
        {
            return Err(
                "unequal-count paired opaque surfaces require normal opaque materials".into(),
            );
        }
        // Only the established IRT path still requires exact identity here.
        // Normal opaque mismatches defer BOTH real groups and U to Source.
        let exact_reverse = layers[0].len() == layers[1].len()
            && layers[0]
                .iter()
                .zip(layers[1].iter().rev())
                .all(|(left, right)| left.eq_ignore_ascii_case(right));
        if !exact_reverse
            && layers.iter().any(|stack| {
                infrared.is_some_and(|rows| {
                    stack.iter().any(|material| {
                        rows.keys()
                            .any(|name| name.0.eq_ignore_ascii_case(material))
                    })
                })
            })
        {
            return Err(
                "IRT paired opaque surfaces require the existing exact reverse Material identity"
                    .into(),
            );
        }
    }
    Ok(())
}
