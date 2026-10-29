//! Bind actual Source Zone and active Space rows by real typed identities.
//! Their independent array orders and unused allocation tail are retained.
use super::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use ep_model::{NormalizedName, SpaceId, SpaceOrigin, TypedModel, ZoneId};
use std::collections::{BTreeMap, BTreeSet};

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
        let allocation = self
            .spaces
            .as_deref()
            .ok_or(Error::Unavailable("actual equipment Space allocation"))?;
        let active = usize::try_from(globals.num_spaces)
            .map_err(|_| Error::Unavailable("actual active numSpaces"))?;
        let spaces = allocation
            .get(..active)
            .ok_or(Error::Unavailable("actual active Space extent"))?;
        let authored = model
            .spaces
            .iter()
            .filter(|space| space.origin == SpaceOrigin::Authored)
            .count();
        if zones.is_empty()
            || zones.len() != spaces.len()
            || zones.len() != model.zones.len()
            || spaces.len() != model.spaces.len()
            || i32::try_from(zones.len()).ok() != Some(globals.num_of_zones)
            || (allocation.len() != spaces.len() && allocation.len() != zones.len() + authored)
            || self.num_zone_groups != 0
            || allocation[active..].iter().any(|row| {
                row.typed_space_id.is_some() || row.zone_num != 0 || !row.name.is_empty()
            })
        {
            return Err(Error::Unavailable(
                "selected actual whole-Zone Space allocation operands",
            ));
        }
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
        let mut zone_ids = Vec::with_capacity(zones.len());
        let mut space_ids = Vec::with_capacity(spaces.len());
        let mut assigned = BTreeSet::new();
        for (index, zone) in zones.iter().enumerate() {
            let zone_num = super::signed(index + 1)?;
            let zone_id = zone
                .typed_zone_id
                .ok_or(Error::Unavailable("actual Source typed Zone identity"))?;
            let typed = typed_zones
                .remove(&zone_id)
                .ok_or(Error::Unavailable("Source Zone typed identity bijection"))?;
            if typed.name != NormalizedName::new(&zone.name)
                || zone.num_spaces != 1
                || zone.space_indexes.len() != 1
            {
                return Err(Error::Unavailable(
                    "Source Zone/whole-Space identity bijection",
                ));
            }
            let number = zone.space_indexes[0];
            let at = number
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or(Error::Unavailable("positive actual Zone Space pointer"))?;
            let space = spaces
                .get(at)
                .ok_or(Error::Unavailable("actual Zone Space pointer bounds"))?;
            let space_id = space
                .typed_space_id
                .ok_or(Error::Unavailable("actual Source typed Space identity"))?;
            if !assigned.insert(number)
                || space.zone_num != zone_num
                || typed.spaces.as_slice() != [space_id]
            {
                return Err(Error::Unavailable(
                    "Source Zone actual ordered Space membership",
                ));
            }
            zone_ids.push(zone_id);
        }
        for space in spaces {
            let id = space
                .typed_space_id
                .ok_or(Error::Unavailable("actual Source typed Space identity"))?;
            let typed = typed_spaces
                .remove(&id)
                .ok_or(Error::Unavailable("Source Space typed identity bijection"))?;
            let at = space
                .zone_num
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or(Error::Unavailable("positive actual Space Zone pointer"))?;
            let zone = zones
                .get(at)
                .ok_or(Error::Unavailable("actual Space Zone pointer bounds"))?;
            let name_matches = match typed.origin {
                SpaceOrigin::Authored => typed.name.0 == space.name,
                SpaceOrigin::AutoZoneDefault => {
                    typed.name == NormalizedName::new(&space.name) && space.name == zone.name
                }
                SpaceOrigin::AutoZoneRemainder => false,
            };
            if Some(typed.zone) != zone.typed_zone_id || !name_matches {
                return Err(Error::Unavailable(
                    "Source Space/Zone typed identity bijection",
                ));
            }
            space_ids.push(id);
        }
        if !typed_zones.is_empty() || !typed_spaces.is_empty() || assigned.len() != active {
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
