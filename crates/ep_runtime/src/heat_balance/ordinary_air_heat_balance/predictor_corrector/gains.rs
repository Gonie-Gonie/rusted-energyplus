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
    /// Native IHG8216-8254: per-space reduction, then zone reduction.
    pub(crate) fn source_zone_return_air_convective_gain(
        &self,
        zones: &SelectedZoneInputOwners,
        number: i32,
        return_node: i32,
    ) -> Result<f64, Error> {
        let zone = zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual return convection Zone association",
            ))?;
        let mut total = 0.0;
        for &space in &zone.space_indexes {
            let owner = self
                .allocations
                .space_devices
                .as_ref()
                .and_then(|rows| rows.get(index(space).ok()?))
                .ok_or(Error::Unavailable(
                    "actual return convection SpaceIntGainDevices",
                ))?;
            if owner.number_of_devices == 0 {
                continue;
            }
            let mut space_total = 0.0;
            for number in 0..owner.number_of_devices {
                let device = owner
                    .devices
                    .as_ref()
                    .and_then(|rows| rows.get(number))
                    .ok_or(Error::Unavailable("actual cached return convection device"))?;
                if return_node == 0 || return_node == device.return_air_node {
                    space_total += device.cached_rates[1];
                }
            }
            total += space_total;
        }
        Ok(total)
    }
    /// Native IHG8506-8535: one reduction over ordered spaces and devices.
    pub(crate) fn source_zone_return_air_latent_gain(
        &self,
        zones: &SelectedZoneInputOwners,
        number: i32,
        return_node: i32,
    ) -> Result<f64, Error> {
        let zone = zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable("actual return latent Zone association"))?;
        let mut total = 0.0;
        for &space in &zone.space_indexes {
            let owner = self
                .allocations
                .space_devices
                .as_ref()
                .and_then(|rows| rows.get(index(space).ok()?))
                .ok_or(Error::Unavailable(
                    "actual return latent SpaceIntGainDevices",
                ))?;
            if owner.number_of_devices == 0 {
                continue;
            }
            for number in 0..owner.number_of_devices {
                let device = owner
                    .devices
                    .as_ref()
                    .and_then(|rows| rows.get(number))
                    .ok_or(Error::Unavailable("actual cached return latent device"))?;
                if return_node == 0 || return_node == device.return_air_node {
                    total += device.cached_rates[4];
                }
            }
        }
        Ok(total)
    }
}
