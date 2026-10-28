//! Input admission only: compare already-compiled canonical World operands.
//! Actual Source geometry and pairing remain genuine getter-owned results.
use super::PortingScope;
use ep_model::{OutsideBoundaryCondition, Point3, SurfaceType, TypedModel};
use std::collections::BTreeMap;

fn reversed_coincident(a: &[Point3], b: &[Point3]) -> bool {
    if a.len() < 3 || a.len() != b.len() {
        return false;
    }
    (0..a.len()).any(|start| {
        a.iter()
            .enumerate()
            .all(|(i, p)| *p == b[(start + a.len() - i) % a.len()])
    })
}
pub(super) fn validate(model: &TypedModel, scope: PortingScope, violations: &mut Vec<String>) {
    let surfaces = model
        .surfaces
        .iter()
        .map(|s| (s.name.0.as_str(), s))
        .collect::<BTreeMap<_, _>>();
    for surface in &model.surfaces {
        if surface.outside_boundary_condition != OutsideBoundaryCondition::Surface {
            continue;
        }
        let valid = scope == PortingScope::A
            && model.zones.len() > 1
            && surface
                .outside_boundary_condition_object
                .as_ref()
                .and_then(|name| surfaces.get(name.0.as_str()))
                .is_some_and(|other| {
                    let construction = model
                        .constructions
                        .iter()
                        .find(|c| c.id == surface.construction);
                    let adjacent_construction = model
                        .constructions
                        .iter()
                        .find(|c| c.id == other.construction);
                    let zone = model.zones.iter().find(|z| z.id == surface.zone);
                    let other_zone = model.zones.iter().find(|z| z.id == other.zone);
                    other.outside_boundary_condition == OutsideBoundaryCondition::Surface
                        && other.outside_boundary_condition_object.as_ref() == Some(&surface.name)
                        && other.id != surface.id
                        && other.zone != surface.zone
                        && other.space != surface.space
                        && surface.surface_type == SurfaceType::Wall
                        && other.surface_type == SurfaceType::Wall
                        && construction
                            .zip(adjacent_construction)
                            .is_some_and(|(a, b)| {
                                let left = a.effective_layers();
                                let right = b.effective_layers();
                                let exact_reverse =
                                    left.len() == right.len() && left.iter().eq(right.iter().rev());
                                let normal = left.iter().chain(right).all(|id| {
                                    model
                                        .materials
                                        .iter()
                                        .find(|material| material.id == *id)
                                        .is_some_and(|material| {
                                            material.kind()
                                                != ep_model::MaterialKind::InfraredTransparent
                                        })
                                });
                                let relation = exact_reverse || normal;
                                a.is_ordinary_opaque()
                                    && b.is_ordinary_opaque()
                                    && !a.has_internal_heat_source()
                                    && !b.has_internal_heat_source()
                                    && (1..=10).contains(&left.len())
                                    && (1..=10).contains(&right.len())
                                    && relation
                            })
                        && zone.zip(other_zone).is_some_and(|(a, b)| {
                            a.multiplier == b.multiplier && a.list_multiplier == b.list_multiplier
                        })
                        && reversed_coincident(&surface.vertices, &other.vertices)
                });
        if !valid {
            violations.push(format!("{} requires the bounded reciprocal Source A Wall pair with exactly reversed IRT layers or normal opaque layers checked against actual Source groups and NominalU, equal Zone factors and reversed coincident World vertices", surface.name.0));
        }
    }
}
