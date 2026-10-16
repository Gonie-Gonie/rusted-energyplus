//! InitThermalComfort166-497 / GetAngleFactorList1937-2073.
use super::state::{
    SourceAngleFactorData, SourceThermalComfortData, SourceThermalComfortInAsh55,
    SourceThermalComfortSetpoint,
};
use super::*;
use crate::heat_balance::source_output_registration::SourceComfortOutputField as Field;
impl SourceThermalComfort {
    pub(super) fn initialize<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.thermal_comfort_data = Some(vec![
            SourceThermalComfortData::default();
            count(o.gains.totals[0])?
        ]);
        for number in 1..=o.gains.totals[0] {
            let p = o
                .gains
                .allocations
                .people
                .get(index(number)?)
                .ok_or(Error::Unavailable("actual InitThermalComfort People"))?;
            if p.fanger {
                return Err(Error::Unavailable("actual positive Fanger registration184"));
            }
            if p.pierce {
                return Err(Error::Unavailable("actual positive Pierce registration209"));
            }
            if p.ksu {
                return Err(Error::Unavailable("actual positive KSU registration249"));
            }
            if p.fanger || p.pierce || p.ksu {
                return Err(Error::Unavailable(
                    "actual positive MRT/operative registration259",
                ));
            }
            if p.adaptive_ash55 {
                return Err(Error::Unavailable(
                    "actual positive adaptiveASH55 registration284",
                ));
            }
            if p.adaptive_cen15251 {
                return Err(Error::Unavailable(
                    "actual positive adaptiveCEN registration317",
                ));
            }
            if p.cooling_effect_ash55 {
                return Err(Error::Unavailable(
                    "actual positive CoolingEffect registration357",
                ));
            }
            if p.ankle_draft_ash55 {
                return Err(Error::Unavailable(
                    "actual positive AnkleDraft registration369",
                ));
            }
        }
        self.thermal_comfort_in_ash55 = Some(vec![
            SourceThermalComfortInAsh55::default();
            count(o.globals.num_of_zones)?
        ]);
        for number in 1..=o.gains.totals[0] {
            let p = o
                .gains
                .allocations
                .people
                .get(index(number)?)
                .ok_or(Error::Unavailable("actual Show55Warning People"))?;
            if p.show55_warning {
                self.thermal_comfort_in_ash55
                    .as_mut()
                    .and_then(|v| v.get_mut(index(p.zone_ptr).ok()?))
                    .ok_or(Error::Unavailable("actual Show55Warning Zone"))?
                    .enable55_warning = true;
            }
        }
        for number in 1..=o.globals.num_of_zones {
            let key = zone_name(o.zones, number)?;
            self.register(
                o,
                Field::ZoneSimpleSummer,
                number,
                "Zone Thermal Comfort ASHRAE 55 Simple Model Summer Clothes Not Comfortable Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneSimpleWinter,
                number,
                "Zone Thermal Comfort ASHRAE 55 Simple Model Winter Clothes Not Comfortable Time",
                key,
            )?;
            self.register(o,Field::ZoneSimpleEither,number,"Zone Thermal Comfort ASHRAE 55 Simple Model Summer or Winter Clothes Not Comfortable Time",key)?;
        }
        self.register(
            o,
            Field::FacilitySimpleSummer,
            0,
            "Facility Thermal Comfort ASHRAE 55 Simple Model Summer Clothes Not Comfortable Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilitySimpleWinter,
            0,
            "Facility Thermal Comfort ASHRAE 55 Simple Model Winter Clothes Not Comfortable Time",
            "Facility",
        )?;
        self.register(o,Field::FacilitySimpleEither,0,"Facility Thermal Comfort ASHRAE 55 Simple Model Summer or Winter Clothes Not Comfortable Time","Facility")?;
        self.thermal_comfort_setpoint = Some(vec![
            SourceThermalComfortSetpoint::default();
            count(o.globals.num_of_zones)?
        ]);
        for number in 1..=o.globals.num_of_zones {
            let key = zone_name(o.zones, number)?;
            self.register(
                o,
                Field::ZoneNotMetHeating,
                number,
                "Zone Heating Setpoint Not Met Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneNotMetHeatingOccupied,
                number,
                "Zone Heating Setpoint Not Met While Occupied Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneNotMetCooling,
                number,
                "Zone Cooling Setpoint Not Met Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneNotMetCoolingOccupied,
                number,
                "Zone Cooling Setpoint Not Met While Occupied Time",
                key,
            )?;
        }
        self.register(
            o,
            Field::FacilityNotMetHeating,
            0,
            "Facility Heating Setpoint Not Met Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilityNotMetCooling,
            0,
            "Facility Cooling Setpoint Not Met Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilityNotMetHeatingOccupied,
            0,
            "Facility Heating Setpoint Not Met While Occupied Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilityNotMetCoolingOccupied,
            0,
            "Facility Cooling Setpoint Not Met While Occupied Time",
            "Facility",
        )?;
        self.get_source_angle_factor_list(o)?;
        self.zone_occ_hrs = Some(vec![0.0; count(o.globals.num_of_zones)?]);
        Ok(())
    }
    fn get_source_angle_factor_list<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        o.shortcut.current_module_object = "ComfortViewFactorAngles".into();
        let number = o
            .input
            .get_num_objects_found(&o.shortcut.current_module_object);
        self.angle_factor_list = Some(vec![SourceAngleFactorData::default(); number]);
        if number > 0 {
            return Err(Error::Unavailable(
                "actual ComfortViewFactorAngles first getObjectItem1959",
            ));
        }
        for people in o.gains.allocations.people.iter() {
            if people.mrt_calc_type != 2 {
                continue;
            } // Actual CalcMRT::AngleFactor.
            return Err(Error::Unavailable(
                "actual People AngleFactorList FindItemInList2041",
            ));
        }
        Ok(())
    }
}
