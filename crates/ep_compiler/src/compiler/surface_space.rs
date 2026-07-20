use ep_model::{
    AutoOrNumber, NormalizedName, Space, SpaceId, SpaceOrigin, SurfaceId, TypedModel, ZoneId,
};
use ep_raw_model::RawObject;

use super::Compiler;

const SURFACE_OBJECT_TYPE: &str = "BuildingSurface:Detailed";
const SPACE_OBJECT_TYPE: &str = "Space";
const GENERAL_SPACE_TYPE: &str = "General";

impl Compiler<'_> {
    pub(super) fn resolve_surface_space(
        &mut self,
        model: &TypedModel,
        surface_name: &str,
        object: &RawObject,
        zone: ZoneId,
    ) -> Option<(SpaceId, bool)> {
        let space_name = self.optional_reference_name_checked(
            SURFACE_OBJECT_TYPE,
            surface_name,
            object,
            "space_name",
        )?;

        if let Some(space_name) = space_name {
            let normalized_space_name = NormalizedName::new(&space_name);
            let Some(space) = model
                .spaces
                .iter()
                .find(|space| space.name == normalized_space_name)
            else {
                self.error(
                    "MissingReference",
                    SURFACE_OBJECT_TYPE,
                    Some(surface_name),
                    Some("space_name"),
                    format!(
                        "{SURFACE_OBJECT_TYPE}/{surface_name} field space_name references missing {SPACE_OBJECT_TYPE} '{space_name}'"
                    ),
                );
                return None;
            };
            if space.zone != zone {
                self.error(
                    "SurfaceSpaceZoneMismatch",
                    SURFACE_OBJECT_TYPE,
                    Some(surface_name),
                    Some("space_name"),
                    format!(
                        "{SURFACE_OBJECT_TYPE}/{surface_name} field space_name references {SPACE_OBJECT_TYPE} '{space_name}' in a different Zone"
                    ),
                );
                return None;
            }
            return Some((space.id, true));
        }

        let Ok(zone_index) = usize::try_from(zone.0) else {
            self.internal_surface_space_error(
                surface_name,
                format!("resolved an unavailable Zone id {}", zone.0),
            );
            return None;
        };
        let Some(zone_state) = model.zones.get(zone_index) else {
            self.internal_surface_space_error(
                surface_name,
                format!("resolved an unavailable Zone id {}", zone.0),
            );
            return None;
        };
        let Some(space) = zone_state.spaces.last().copied() else {
            self.internal_surface_space_error(
                surface_name,
                format!("Zone '{}' has no Space", zone_state.name.0),
            );
            return None;
        };
        if model.spaces.get(space.0 as usize).is_none() {
            self.internal_surface_space_error(
                surface_name,
                format!(
                    "Zone '{}' references unavailable Space id {}",
                    zone_state.name.0, space.0
                ),
            );
            return None;
        }

        Some((space, false))
    }

    pub(super) fn create_missing_spaces(
        &mut self,
        model: &mut TypedModel,
        explicit_space_assignments: &[(SurfaceId, bool)],
    ) {
        let mut any_explicit = vec![false; model.zones.len()];
        let mut any_blank = vec![false; model.zones.len()];

        for &(surface_id, explicitly_assigned) in explicit_space_assignments {
            let Some(surface) = model.surfaces.get(surface_id.0 as usize) else {
                self.internal_surface_space_error(
                    &surface_id.0.to_string(),
                    format!("resolved an unavailable Surface id {}", surface_id.0),
                );
                return;
            };
            let Ok(zone_index) = usize::try_from(surface.zone.0) else {
                self.internal_surface_space_error(
                    &surface.name.0,
                    format!("resolved an unavailable Zone id {}", surface.zone.0),
                );
                return;
            };
            if model.zones.get(zone_index).is_none() {
                self.internal_surface_space_error(
                    &surface.name.0,
                    format!("resolved an unavailable Zone id {}", surface.zone.0),
                );
                return;
            }
            if explicitly_assigned {
                any_explicit[zone_index] = true;
            } else {
                any_blank[zone_index] = true;
            }
        }

        let remainder_zone_indexes = (0..model.zones.len())
            .filter(|&zone_index| any_explicit[zone_index] && any_blank[zone_index])
            .collect::<Vec<_>>();
        if remainder_zone_indexes.is_empty() {
            return;
        }

        let mut remainder_ids = Vec::with_capacity(remainder_zone_indexes.len());
        for (offset, &zone_index) in remainder_zone_indexes.iter().enumerate() {
            let name = format!("{}-REMAINDER", model.zones[zone_index].name.0);
            let Some(index) = model.spaces.len().checked_add(offset) else {
                self.internal_surface_space_error(&name, "Space arena length overflow".to_string());
                return;
            };
            let Some(id_value) = self.checked_id(SPACE_OBJECT_TYPE, &name, index) else {
                return;
            };
            remainder_ids.push(SpaceId(id_value));
        }

        let first_remainder_name = format!(
            "{}-REMAINDER",
            model.zones[remainder_zone_indexes[0]].name.0
        );
        let Some(space_type_id) =
            self.register_space_type(model, &first_remainder_name, GENERAL_SPACE_TYPE)
        else {
            return;
        };
        let mut remainder_by_zone = vec![None; model.zones.len()];
        for ((zone_index, id), expected_index) in remainder_zone_indexes
            .into_iter()
            .zip(remainder_ids)
            .zip(model.spaces.len()..)
        {
            debug_assert_eq!(id.0 as usize, expected_index);
            let name = format!("{}-REMAINDER", model.zones[zone_index].name.0);
            model.spaces.push(Space {
                id,
                name: NormalizedName::new(&name),
                zone: model.zones[zone_index].id,
                ceiling_height: AutoOrNumber::AutoCalculate,
                volume: AutoOrNumber::AutoCalculate,
                floor_area: AutoOrNumber::AutoCalculate,
                space_type: NormalizedName::new(GENERAL_SPACE_TYPE),
                space_type_id,
                tags: Vec::new(),
                origin: SpaceOrigin::AutoZoneRemainder,
            });
            model.zones[zone_index].spaces.push(id);
            remainder_by_zone[zone_index] = Some(id);
        }

        for &(surface_id, explicitly_assigned) in explicit_space_assignments {
            if explicitly_assigned {
                continue;
            }
            let surface = &mut model.surfaces[surface_id.0 as usize];
            let zone_index = surface.zone.0 as usize;
            if let Some(remainder) = remainder_by_zone[zone_index] {
                surface.space = remainder;
            }
        }
    }

    fn internal_surface_space_error(&mut self, surface_name: &str, detail: String) {
        self.error(
            "InternalReferenceError",
            SURFACE_OBJECT_TYPE,
            Some(surface_name),
            None,
            format!("{SURFACE_OBJECT_TYPE}/{surface_name} {detail}"),
        );
    }
}
