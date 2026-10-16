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
impl SourceHvacManager {
    pub(crate) fn update_source_zone_list_and_group_loads(
        &self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=o.zones.num_zone_lists {
            let reports = &mut o.control.zone_air_group_loads;
            reset(&mut reports.list_heat_energy, number)?;
            reset(&mut reports.list_cool_energy, number)?;
            reset(&mut reports.list_heat_rate, number)?;
            reset(&mut reports.list_cool_rate, number)?;
        }
        if o.zones.num_zone_lists > 0 {
            return Err(Error::Unavailable(
                "actual ZoneList record2217: positive GetZoneData list body unported",
            ));
        }
        if o.zones.num_zone_groups > 0 {
            return Err(Error::Unavailable(
                "actual ZoneGroup record2229: positive GetZoneData group body unported",
            ));
        }
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
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if *o.equipment.source_number_of_nodes() > 0 {
            return Err(Error::Unavailable(
                "actual Node TempLastTimestep/EnthalpyLastTimestep firstrow2958",
            ));
        }
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
