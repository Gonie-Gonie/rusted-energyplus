//! Genuine demand BeginEnvironment and predicted-sensible actual writes.
use super::*;
impl super::super::demand::SourceSystemSensibleDemand {
    pub(super) fn begin_source_environment(&mut self) -> Result<(), Error> {
        self.base.remaining_output_required = 0.0;
        self.base.total_output_required = 0.0;
        if !self.sequenced_output_required.is_empty() {
            for number in 0..usize::try_from(self.base.num_zone_equipment)
                .map_err(|_| Error::Unavailable("actual sensible NumZoneEquipment"))?
            {
                *self
                    .sequenced_output_required
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual sequenced sensible demand"))? = 0.0;
                *self
                    .sequenced_output_to_heating_sp
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual sequenced heating demand"))? = 0.0;
                *self
                    .sequenced_output_to_cooling_sp
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual sequenced cooling demand"))? = 0.0;
            }
        }
        self.air_sys_heat_energy = 0.0;
        self.air_sys_cool_energy = 0.0;
        self.air_sys_heat_rate = 0.0;
        self.air_sys_cool_rate = 0.0;
        self.predicted_rate = 0.0;
        self.predicted_heating_sp_rate = 0.0;
        self.predicted_cooling_sp_rate = 0.0;
        Ok(())
    }
    pub(super) fn report_source_predicted_sensible(
        &mut self,
        load: f64,
        heating: f64,
        cooling: f64,
        correction: f64,
        multiplier: i32,
        list_multiplier: i32,
        controlled: bool,
    ) -> Result<(), Error> {
        self.predicted_rate = load * correction;
        self.predicted_heating_sp_rate = heating * correction;
        self.predicted_cooling_sp_rate = cooling * correction;
        let mult = f64::from(
            multiplier
                .checked_mul(list_multiplier)
                .ok_or(Error::Unavailable(
                    "defined Source predicted Zone multiplier338",
                ))?,
        );
        self.base.total_output_required = self.predicted_rate * mult;
        self.output_required_to_heating_sp = self.predicted_heating_sp_rate * mult;
        self.output_required_to_cooling_sp = self.predicted_cooling_sp_rate * mult;
        if controlled && self.base.num_zone_equipment > 0 {
            for number in 0..usize::try_from(self.base.num_zone_equipment)
                .map_err(|_| Error::Unavailable("actual predicted NumZoneEquipment"))?
            {
                *self
                    .sequenced_output_required
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual predicted sequenced demand"))? =
                    self.base.total_output_required;
                *self
                    .sequenced_output_to_heating_sp
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual predicted sequenced heating"))? =
                    self.output_required_to_heating_sp;
                *self
                    .sequenced_output_to_cooling_sp
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual predicted sequenced cooling"))? =
                    self.output_required_to_cooling_sp;
            }
        }
        Ok(())
    }
}
impl super::super::demand::SourceSystemMoistureDemand {
    pub(super) fn begin_source_environment(&mut self) -> Result<(), Error> {
        self.base.remaining_output_required = 0.0;
        self.base.total_output_required = 0.0;
        if !self.sequenced_output_required.is_empty() {
            for number in 0..usize::try_from(self.base.num_zone_equipment)
                .map_err(|_| Error::Unavailable("actual moisture NumZoneEquipment"))?
            {
                *self
                    .sequenced_output_required
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual sequenced moisture demand"))? = 0.0;
                *self
                    .sequenced_output_to_humid_sp
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual sequenced humidifying"))? = 0.0;
                *self
                    .sequenced_output_to_dehumid_sp
                    .get_mut(number)
                    .ok_or(Error::Unavailable("actual sequenced dehumidifying"))? = 0.0;
            }
        }
        self.air_sys_heat_energy = 0.0;
        self.air_sys_cool_energy = 0.0;
        self.air_sys_heat_rate = 0.0;
        self.air_sys_cool_rate = 0.0;
        self.air_sys_sensible_heat_ratio = 0.0;
        self.vapor_pressure_difference = 0.0;
        self.predicted_rate = 0.0;
        self.predicted_humid_sp_rate = 0.0;
        self.predicted_dehumid_sp_rate = 0.0;
        Ok(())
    }
}
