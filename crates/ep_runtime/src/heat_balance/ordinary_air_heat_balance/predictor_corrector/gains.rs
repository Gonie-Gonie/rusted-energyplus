//! IHG8114-8147 and8216-8254: actual cached live device sums.
use super::*;
impl SelectedInternalHeatGains {
    pub(crate) fn source_space_convective_gain(
        &self,
        space: i32,
        return_air: bool,
    ) -> Result<f64, Error> {
        let row = self
            .allocations
            .space_devices
            .as_deref()
            .and_then(|rows| rows.get(index(space).ok()?))
            .ok_or(Error::Unavailable("actual SpaceIntGainDevices"))?;
        let mut result = 0.0;
        for number in 0..row.number_of_devices {
            let device = row
                .devices
                .as_deref()
                .and_then(|rows| rows.get(number))
                .ok_or(Error::Unavailable("actual internal-gain cached device"))?;
            result += device.cached_rates[usize::from(return_air)]; //returnNode0 includes every device
        }
        Ok(result)
    }
    pub(crate) fn source_zone_convective_gain(
        &self,
        zones: &SelectedZoneInputOwners,
        zone_number: i32,
        return_air: bool,
    ) -> Result<f64, Error> {
        let row = zones
            .zones
            .as_deref()
            .and_then(|rows| rows.get(index(zone_number).ok()?))
            .ok_or(Error::Unavailable("actual IHG Zone"))?;
        let mut result = 0.0;
        for &space in &row.space_indexes {
            let device_owner = self
                .allocations
                .space_devices
                .as_deref()
                .and_then(|rows| rows.get(index(space).ok()?))
                .ok_or(Error::Unavailable("actual SpaceIntGainDevices"))?;
            if device_owner.number_of_devices == 0 {
                continue;
            }
            result += self.source_space_convective_gain(space, return_air)?;
        }
        Ok(result)
    }
}
