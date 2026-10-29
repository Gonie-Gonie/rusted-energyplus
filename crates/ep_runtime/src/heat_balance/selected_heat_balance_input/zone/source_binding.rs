//! Bind retained Source Zone/Space rows by their actual typed identities.
//! No typed vector position, generated ordinal ID or reordered model is used.
use super::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use ep_model::{NormalizedName, SpaceId, SpaceOrigin, TypedModel, ZoneId};
use std::collections::BTreeMap;

impl SelectedZoneInputOwners {
    pub(crate) fn source_implicit_space_ids(
        &self,
        model: &TypedModel,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(Vec<ZoneId>, Vec<SpaceId>), Error> {
        let zones = self
            .zones
            .as_deref()
            .ok_or(Error::Unavailable("actual equipment Zone allocation"))?;
        let spaces = self
            .spaces
            .as_deref()
            .ok_or(Error::Unavailable("actual equipment Space allocation"))?;
        if zones.is_empty()
            || zones.len() != spaces.len()
            || zones.len() != model.zones.len()
            || spaces.len() != model.spaces.len()
            || i32::try_from(zones.len()).ok() != Some(globals.num_of_zones)
            || i32::try_from(spaces.len()).ok() != Some(globals.num_spaces)
            || self.num_zone_groups != 0
        {
            return Err(Error::Unavailable(
                "selected actual implicit-Space allocation operands",
            ));
        }
        let (mut zone_ids, mut space_ids) = (
            Vec::with_capacity(zones.len()),
            Vec::with_capacity(spaces.len()),
        );
        // Maps provide identity lookups only; returned order is the retained
        // Source traversal, independent of typed-vector/map ordering.
        let mut typed_zones = model
            .zones
            .iter()
            .map(|row| (row.id, row))
            .collect::<BTreeMap<_, _>>();
        let mut typed_spaces = model
            .spaces
            .iter()
            .map(|row| (row.id, row))
            .collect::<BTreeMap<_, _>>();
        if typed_zones.len() != model.zones.len() || typed_spaces.len() != model.spaces.len() {
            return Err(Error::Unavailable("duplicate typed Zone/Space identity"));
        }
        for (index, zone) in zones.iter().enumerate() {
            let source_num = i32::try_from(index)
                .ok()
                .and_then(|n| n.checked_add(1))
                .ok_or(Error::Unavailable("Source Zone/Space signed index"))?;
            let zone_id = zone
                .typed_zone_id
                .ok_or(Error::Unavailable("actual Source typed Zone identity"))?;
            let typed_zone = typed_zones
                .remove(&zone_id)
                .ok_or(Error::Unavailable("Source Zone typed identity bijection"))?;
            if typed_zone.name != NormalizedName::new(&zone.name)
                || zone.num_spaces != 1
                || zone.space_indexes.as_slice() != [source_num]
            {
                return Err(Error::Unavailable(
                    "Source Zone/implicit-Space identity bijection",
                ));
            }
            // Genuine GetSpaceData appends one whole-Zone default in this
            // Source Zone traversal. Check that retained association; never
            // infer a typed ID from this Source array number.
            let space = &spaces[index];
            let space_id = space
                .typed_space_id
                .ok_or(Error::Unavailable("actual Source typed Space identity"))?;
            let typed_space = typed_spaces
                .remove(&space_id)
                .ok_or(Error::Unavailable("Source Space typed identity bijection"))?;
            if space.zone_num != source_num
                || space.name != zone.name
                || typed_zone.spaces.as_slice() != [space_id]
                || typed_space.zone != zone_id
                || typed_space.name != NormalizedName::new(&space.name)
                || typed_space.origin != SpaceOrigin::AutoZoneDefault
            {
                return Err(Error::Unavailable(
                    "Source Space/Zone typed identity bijection",
                ));
            }
            zone_ids.push(zone_id);
            space_ids.push(space_id);
        }
        if !typed_zones.is_empty() || !typed_spaces.is_empty() {
            return Err(Error::Unavailable(
                "incomplete Source typed identity bijection",
            ));
        }
        let list_ids = self.source_zone_list_ids(model)?;
        if self
            .zone_lists
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .zip(list_ids)
            .any(|(list, id)| list.typed_zone_list_id != Some(id))
        {
            return Err(Error::Unavailable(
                "unbound actual Source ZoneList identity",
            ));
        }
        Ok((zone_ids, space_ids))
    }
}
