//! Defined first Source IRT use domain; no physical properties are fabricated.
use super::{text, text_is};
use ep_raw_model::{ObjectType, RawModel};
use std::collections::BTreeSet;
pub(super) fn validate_defined_uses(
    raw: &RawModel,
    validated_paired_wall_stage: bool,
) -> Result<(), String> {
    let Some(materials) = raw
        .objects
        .get(&ObjectType("Material:InfraredTransparent".into()))
    else {
        return Ok(());
    };
    if materials.is_empty() {
        return Ok(());
    }
    let names = materials
        .keys()
        .map(|name| name.0.to_ascii_uppercase())
        .collect::<BTreeSet<_>>();
    let report = |kind: &str| {
        raw.objects
            .get(&ObjectType("Output:Constructions".into()))
            .is_some_and(|rows| {
                rows.values().any(|row| {
                    ["details_type_1", "details_type_2"].iter().any(|field| {
                        row.fields
                            .get(&ep_raw_model::FieldName((*field).into()))
                            .is_some_and(|value| text_is(value, &[kind]))
                    })
                })
            })
    };
    // Native Material.cc2789 indexes a six-element name array at Invalid=-1.
    if report("Materials") {
        return Err(
            "Source IRT Materials report reaches undefined Native roughness-name indexing".into(),
        );
    }
    let constructions = raw.objects.get(&ObjectType("Construction".into()))
        .map(|rows| rows.iter().filter(|(_, row)| row.fields.iter().any(|(field, value)| {
            (field.0 == "outside_layer" || field.0.starts_with("layer_"))
                && matches!(value, ep_raw_model::RawValue::String(name) if names.contains(&name.to_ascii_uppercase()))
        })).map(|(name, _)| name.0.to_ascii_uppercase()).collect::<BTreeSet<_>>()).unwrap_or_default();
    let mut used = false;
    if let Some(surfaces) = raw
        .objects
        .get(&ObjectType("BuildingSurface:Detailed".into()))
    {
        for surface in surfaces.values() {
            if text(surface, "construction_name")
                .is_none_or(|name| !constructions.contains(&name.to_ascii_uppercase()))
            {
                continue;
            }
            used = true;
            // Keep the existing one-Zone A/B Ground/self-Adiabatic route.
            // Newly admitted Source A2+ IRT uses are ONLY the proven reciprocal
            // Surface pairs; do not broaden multi-Zone Ground/Adiabatic uses.
            // This flag comes after the caller's existing raw-pair proof.
            if !text(surface, "outside_boundary_condition").is_some_and(|value| {
                if validated_paired_wall_stage {
                    value.eq_ignore_ascii_case("Surface")
                } else {
                    ["Ground", "Adiabatic"]
                        .iter()
                        .any(|kind| value.eq_ignore_ascii_case(kind))
                }
            }) {
                return Err("Source used IRT requires the existing one-Zone Ground/Adiabatic route or a validated Source A paired Wall use".into());
            }
        }
    }
    if used && report("Constructions") {
        return Err(
            "Source used IRT Constructions report reaches undefined Native roughness-name indexing"
                .into(),
        );
    }
    Ok(())
}
