//! Actual Which references to the same internal-gain object/Zone/Space owners.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceRealOutputOwners,
    SourceRealOutputTarget,
};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceInternalGainOutputField {
    OtherPower,
    OtherConsumption,
    // Field is the actual interleaved Energy/Rate report ordinal, never a value.
    OtherThermal { field: u8 },
    ZoneOtherThermal { field: u8 },
    SpaceOtherThermal { field: u8 },
    ZoneOtherFuel { fuel: u8, energy: bool },
    SpaceOtherFuel { fuel: u8, energy: bool },
    ZoneTotalInternal { field: u8 },
    SpaceTotalInternal { field: u8 },
    PeopleObject(SourcePeopleOutputField),
    PeopleZone(SourcePeopleOutputField),
    PeopleSpace(SourcePeopleOutputField),
}
impl SourceRealOutputOwners for SelectedInternalHeatGains {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::InternalGain { field, index } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let i = index
            .checked_sub(1)
            .and_then(|x| usize::try_from(x).ok())
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        let other = || {
            self.allocations
                .other_equipment
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)
        };
        let zone = || {
            self.allocations
                .zone_report
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)
        };
        let space = || {
            self.allocations
                .space_report
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(OrdinaryRadiationUnavailable::StorageDomain)
        };
        use SourceInternalGainOutputField::*;
        use SourcePeopleOutputField as P;
        let value = match field {
            OtherPower => Some(&other()?.rates[0]),
            OtherConsumption => Some(&other()?.energies[0]),
            OtherThermal { field } if field < 10 => {
                let row = other()?;
                let at = usize::from(field / 2) + 1;
                if field.is_multiple_of(2) {
                    row.energies.get(at)
                } else {
                    row.rates.get(at)
                }
            }
            ZoneOtherThermal { field } => zone()?.other.get(usize::from(field)),
            SpaceOtherThermal { field } => space()?.other.get(usize::from(field)),
            ZoneTotalInternal { field } => zone()?.total.get(usize::from(field)),
            SpaceTotalInternal { field } => space()?.total.get(usize::from(field)),
            ZoneOtherFuel { fuel, energy } if fuel < 13 => {
                let row = zone()?;
                if energy {
                    row.other_consumption.get(usize::from(fuel))
                } else {
                    row.other_power.get(usize::from(fuel))
                }
            }
            SpaceOtherFuel { fuel, energy } if fuel < 13 => {
                let row = space()?;
                if energy {
                    row.other_consumption.get(usize::from(fuel))
                } else {
                    row.other_power.get(usize::from(fuel))
                }
            }
            PeopleObject(field) => {
                let row = self
                    .allocations
                    .people
                    .rows
                    .get(i)
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                Some(match field {
                    P::Count => &row.num_occ,
                    P::RadiantEnergy => &row.rad_gain_energy,
                    P::RadiantRate => &row.rad_gain_rate,
                    P::ConvectiveEnergy => &row.con_gain_energy,
                    P::ConvectiveRate => &row.con_gain_rate,
                    P::SensibleEnergy => &row.sen_gain_energy,
                    P::SensibleRate => &row.sen_gain_rate,
                    P::LatentEnergy => &row.lat_gain_energy,
                    P::LatentRate => &row.lat_gain_rate,
                    P::TotalEnergy => &row.tot_gain_energy,
                    P::TotalRate => &row.tot_gain_rate,
                    P::AirTemperature => &row.temperature_in_zone,
                    P::AirRelativeHumidity => &row.relative_humidity_in_zone,
                })
            }
            PeopleZone(field) | PeopleSpace(field) => {
                let rows = if matches!(
                    target,
                    SourceRealOutputTarget::InternalGain {
                        field: PeopleZone(_),
                        ..
                    }
                ) {
                    &self.allocations.zone_report
                } else {
                    &self.allocations.space_report
                };
                let row = rows
                    .as_ref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                Some(match field {
                    P::Count => &row.people_num_occ,
                    P::RadiantEnergy => &row.people_rad_gain,
                    P::RadiantRate => &row.people_rad_gain_rate,
                    P::ConvectiveEnergy => &row.people_con_gain,
                    P::ConvectiveRate => &row.people_con_gain_rate,
                    P::SensibleEnergy => &row.people_sen_gain,
                    P::SensibleRate => &row.people_sen_gain_rate,
                    P::LatentEnergy => &row.people_lat_gain,
                    P::LatentRate => &row.people_lat_gain_rate,
                    P::TotalEnergy => &row.people_tot_gain,
                    P::TotalRate => &row.people_tot_gain_rate,
                    P::AirTemperature | P::AirRelativeHumidity => {
                        return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
                    }
                })
            }
            _ => None,
        };
        value.ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}

/// Native People Which member identities; Zone/Space have the first eleven.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourcePeopleOutputField {
    Count,
    RadiantEnergy,
    RadiantRate,
    ConvectiveEnergy,
    ConvectiveRate,
    SensibleEnergy,
    SensibleRate,
    LatentEnergy,
    LatentRate,
    TotalEnergy,
    TotalRate,
    AirTemperature,
    AirRelativeHumidity,
}
