//! Actual NonZoneEquipmentManager86–136 caller and same shared WaterUse child.
use super::{Error, SourceHvacManager, SourceHvacOwners};
impl SourceHvacManager {
    pub(crate) fn manage_source_non_zone_equipment(
        &mut self,
        first_hvac_iteration: bool,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.globals.non_zone_equipment.count_equipment {
            let mixed = o.input.get_num_objects_found("WaterHeater:Mixed");
            let stratified = o.input.get_num_objects_found("WaterHeater:Stratified");
            o.globals.non_zone_equipment.number_water_heaters = mixed
                .checked_add(stratified)
                .and_then(|count| i32::try_from(count).ok())
                .ok_or(Error::Unavailable(
                    "actual NonZone water-heater count int domain115",
                ))?;
            o.globals.non_zone_equipment.count_equipment = false;
        }
        o.gain_modules.water_use.simulate_source_water_use(
            first_hvac_iteration,
            &mut o.gain_modules.number_water_equipment,
            o.clock.begin_envrn_flag,
            o.input,
            o.shortcut,
        )?;
        if !o.globals.zone_sizing_calculation
            && o.globals.non_zone_equipment.number_water_heaters > 0
        {
            return Err(Error::Unavailable(
                "actual SimulateWaterHeaterStandAlone124",
            ));
        }
        if first_hvac_iteration {
            o.hvac.sim_non_zone_equipment = true;
        } else {
            o.hvac.sim_non_zone_equipment = false;
        }
        Ok(())
    }
}
