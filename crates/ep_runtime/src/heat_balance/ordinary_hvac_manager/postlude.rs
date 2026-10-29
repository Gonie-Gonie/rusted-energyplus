//! Actual UpdateZoneListAndGroupLoads2199-2236 using the sole existing load arrays.
use super::{Error, SourceHvacManager, SourceHvacOwners};
mod zero_modules;
pub(crate) use zero_modules::SourceHvacPostludeModules;
fn reset(values: &mut Option<Vec<f64>>, number: i32) -> Result<(), Error> {
    let at = number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual ZoneList report ordinal"))?;
    *values
        .as_deref_mut()
        .and_then(|rows| rows.get_mut(at))
        .ok_or(Error::Unavailable("actual ZoneList report allocation2210"))? = 0.0;
    Ok(())
}
fn add(values: &mut Option<Vec<f64>>, number: i32, value: f64) -> Result<(), Error> {
    let at = number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual ZoneList report ordinal"))?;
    *values
        .as_deref_mut()
        .and_then(|rows| rows.get_mut(at))
        .ok_or(Error::Unavailable("actual ZoneList report allocation2221"))? += value;
    Ok(())
}
impl SourceHvacManager {
    /// The same ordered HVACManager resets/sums, on the original live owners.
    pub(crate) fn sum_source_zone_list_loads(
        zones: &crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners,
        demand: &crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirDemand,
        reports: &mut crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneListGroupLoadReports,
    ) -> Result<(), Error> {
        for number in 1..=zones.num_zone_lists {
            reset(&mut reports.list_heat_energy, number)?;
            reset(&mut reports.list_cool_energy, number)?;
            reset(&mut reports.list_heat_rate, number)?;
            reset(&mut reports.list_cool_rate, number)?;
        }
        for number in 1..=zones.num_zone_lists {
            let at = number
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or(Error::Unavailable("actual ZoneList report ordinal"))?;
            let list = zones
                .zone_lists
                .as_deref()
                .and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual ZoneList record2217"))?;
            let members = list
                .zone
                .as_deref()
                .ok_or(Error::Unavailable("actual ZoneList member allocation2218"))?;
            for member in 0..list.num_of_zones {
                let zone_number = *usize::try_from(member)
                    .ok()
                    .and_then(|i| members.get(i))
                    .ok_or(Error::Unavailable("actual ZoneList member2218"))?;
                let zone_at = zone_number
                    .checked_sub(1)
                    .and_then(|n| usize::try_from(n).ok())
                    .ok_or(Error::Unavailable("actual ZoneList member Zone ordinal"))?;
                let demand = demand
                    .zones
                    .as_deref()
                    .and_then(|rows| rows.get(zone_at))
                    .ok_or(Error::Unavailable("actual ZoneSysEnergyDemand2219"))?;
                let multiplier = f64::from(
                    zones
                        .zones
                        .as_deref()
                        .and_then(|rows| rows.get(zone_at))
                        .ok_or(Error::Unavailable("actual ZoneList member Multiplier2220"))?
                        .multiplier,
                );
                add(
                    &mut reports.list_heat_energy,
                    number,
                    demand.air_sys_heat_energy * multiplier,
                )?;
                add(
                    &mut reports.list_cool_energy,
                    number,
                    demand.air_sys_cool_energy * multiplier,
                )?;
                add(
                    &mut reports.list_heat_rate,
                    number,
                    demand.air_sys_heat_rate * multiplier,
                )?;
                add(
                    &mut reports.list_cool_rate,
                    number,
                    demand.air_sys_cool_rate * multiplier,
                )?;
            }
        }
        // Native2229-2235 follows all resets and ordered list sums. Each Group
        // reads its real referenced List cell; ListMultiplier never enters2220.
        for number in 1..=zones.num_zone_groups {
            let at = number
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or(Error::Unavailable("actual Group report ordinal2229"))?;
            let group = zones
                .zone_groups
                .as_deref()
                .and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual Group record2229"))?;
            let list_at = group
                .zone_list
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or(Error::Unavailable("actual Group referenced List2229"))?;
            let factor = f64::from(group.multiplier);
            for (list, target) in [
                (&reports.list_heat_energy, &mut reports.group_heat_energy),
                (&reports.list_cool_energy, &mut reports.group_cool_energy),
                (&reports.list_heat_rate, &mut reports.group_heat_rate),
                (&reports.list_cool_rate, &mut reports.group_cool_rate),
            ] {
                let value = *list
                    .as_deref()
                    .and_then(|rows| rows.get(list_at))
                    .ok_or(Error::Unavailable("actual Group referenced List cell2232"))?;
                *target
                    .as_deref_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual Group report allocation2232"))? =
                    value * factor;
            }
        }
        Ok(())
    }
    pub(crate) fn update_source_zone_list_and_group_loads(
        &self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        Self::sum_source_zone_list_loads(
            o.zones,
            &o.controller.demand,
            &mut o.control.zone_air_group_loads,
        )?;

        Ok(())
    }
}

impl SourceHvacManager {
    pub(crate) fn update_source_ice_fractions(&self) -> Result<(), Error> {
        self.ancillary
            .postlude_modules
            .ice
            .update_source_ice_fractions()
    }
    pub(crate) fn check_source_concurrent_eir_heat_pumps(&self) -> Result<(), Error> {
        self.ancillary
            .postlude_modules
            .eir
            .check_source_concurrent_operation()
    }
    pub(crate) fn update_source_demand_managers(&self) -> Result<(), Error> {
        self.ancillary
            .postlude_modules
            .demand
            .update_source_demand_managers()
    }
    pub(crate) fn update_source_plant_node_thermal_history(
        &self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        o.equipment.update_source_node_thermal_history()?;
        if self.simulation.plant.total_loops > 0 && !o.clock.warmup_flag {
            return Err(Error::Unavailable(
                "actual PlantLoop thermal history firstrow2964",
            ));
        }
        Ok(())
    }
}

impl SourceHvacManager {
    pub(crate) fn manage_source_demand(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.ancillary
            .postlude_modules
            .demand
            .manage_source_demand(o.globals, o.clock, o.input)
    }
}
