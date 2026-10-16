//! Genuine FillPredefinedTableOnThermostatSetpoints6558 zero-array route.
use super::*;
impl SourceZoneAirPredictorCorrector {
    pub(crate) fn fill_source_predefined_table_on_thermostat_setpoints(&self) -> Result<(), Error> {
        let mut unique_schedules = Vec::<i32>::new();
        let heating = i32::try_from(self.flags.number_temperature_controls[1]).map_err(|_| {
            Error::Unavailable("actual thermostat NumTempControls integer domain6565")
        })?;
        let cooling = i32::try_from(self.flags.number_temperature_controls[2]).map_err(|_| {
            Error::Unavailable("actual thermostat NumTempControls integer domain6565")
        })?;
        let single = i32::try_from(self.flags.number_temperature_controls[3]).map_err(|_| {
            Error::Unavailable("actual thermostat NumTempControls integer domain6566")
        })?;
        let dual = i32::try_from(self.flags.number_temperature_controls[4]).map_err(|_| {
            Error::Unavailable("actual thermostat NumTempControls integer domain6566")
        })?;
        let first = heating.checked_add(cooling).ok_or(Error::Unavailable(
            "actual thermostat reserve first native int sum6565",
        ))?;
        let second = first.checked_add(single).ok_or(Error::Unavailable(
            "actual thermostat reserve second native int sum6566",
        ))?;
        let twice_dual = dual.checked_mul(2).ok_or(Error::Unavailable(
            "actual thermostat reserve dual native int product6566",
        ))?;
        let capacity = second.checked_add(twice_dual).ok_or(Error::Unavailable(
            "actual thermostat reserve final native int sum6566",
        ))?;
        unique_schedules.reserve_exact(usize::try_from(capacity).map_err(|_| {
            Error::Unavailable("actual thermostat reserve nonnegative size_t domain")
        })?);
        // Native real/int scratch is unwritten and never read on this route.
        let _month_assumed = String::new();
        let _month_assumed_2 = String::new();
        for (kind, first_read) in [
            (1, "actual thermostat SingleHeat heatSched->Num6575"),
            (2, "actual thermostat SingleCool coolSched->Num6594"),
            (3, "actual thermostat SingleHeatCool heatSched->Num6613"),
            (4, "actual thermostat DualHeatCool heatSched->Num6642"),
        ] {
            if let Some(_setpoint) = self.flags.temperature_setpoint_schedules[kind]
                .as_deref()
                .and_then(|rows| rows.first())
            {
                return Err(Error::Unavailable(first_read));
            }
        }
        Ok(())
    }
}

impl SourceZoneAirPredictorCorrector {
    pub(crate) fn fill_source_predefined_table_on_thermostat_schedules(&self) -> Result<(), Error> {
        // Native aliases/type/lambda declarations before this loop create no
        // runtime vector, allocation or report entry. infos is inside the loop.
        if self.controls.number_temperature_zones > 0 {
            return Err(Error::Unavailable(
                "actual thermostat schedules TempControlledZone(idx) first row6716 after actual count loop6715",
            ));
        }
        Ok(())
    }
}
