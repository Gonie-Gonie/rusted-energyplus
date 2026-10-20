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
            _ => None,
        };
        value.ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}
