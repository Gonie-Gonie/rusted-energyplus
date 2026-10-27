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
                    let zone = model.zones.iter().find(|z| z.id == surface.zone);
                    let other_zone = model.zones.iter().find(|z| z.id == other.zone);
                    other.outside_boundary_condition == OutsideBoundaryCondition::Surface
                        && other.outside_boundary_condition_object.as_ref() == Some(&surface.name)
                        && other.id != surface.id
                        && other.zone != surface.zone
                        && other.space != surface.space
                        && surface.surface_type == SurfaceType::Wall
                        && other.surface_type == SurfaceType::Wall
                        && surface.construction == other.construction
                        && construction.is_some_and(|c| {
                            c.layers.len() == 1 && c.internal_heat_source.is_none()
                        })
                        && zone.zip(other_zone).is_some_and(|(a, b)| {
                            a.multiplier == b.multiplier && a.list_multiplier == b.list_multiplier
                        })
                        && reversed_coincident(&surface.vertices, &other.vertices)
                });
        if !valid {
            violations.push(format!("{} requires the bounded reciprocal Source A Wall pair with identical single-layer Construction, equal Zone factors and reversed coincident World vertices", surface.name.0));
        }
    }
}
