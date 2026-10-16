//! Borrowed sole Source Space AirHB records for genuine InitIntConvCoeff.
use super::SourceAirHeatBalance;
use crate::heat_balance::inside_convection_dispatcher::{
    InsideConvectionUnavailable as Error, SpaceAirTemperatureBorrow, SpaceAirTemperatureOwners,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

/// Actual static metadata remains outside re-initialized AirHB records, as in
/// Original ZoneSpaceHeatBalanceData. Both inputs are the caller's live owners.
pub(crate) struct SourceSpaceAirView<'a> {
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) metadata: &'a SelectedZoneInputOwners,
}

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or(Error::Association)
}

impl SpaceAirTemperatureOwners for SourceSpaceAirView<'_> {
    fn allocated_space_count(&self) -> Result<usize, Error> {
        let air = self.air.spaces.as_ref().ok_or(Error::Association)?;
        let metadata = self.metadata.spaces.as_ref().ok_or(Error::Association)?;
        if air.len() != metadata.len() {
            return Err(Error::Association);
        }
        Ok(air.len())
    }
    fn borrow_space_at(&self, number: usize) -> Result<SpaceAirTemperatureBorrow<'_>, Error> {
        let metadata = self
            .metadata
            .spaces
            .as_ref()
            .and_then(|rows| rows.get(number))
            .ok_or(Error::Association)?;
        let zone = self
            .metadata
            .zones
            .as_ref()
            .and_then(|rows| rows.get(index(metadata.zone_num).ok()?))
            .ok_or(Error::Association)?;
        let air = &self
            .air
            .spaces
            .as_ref()
            .and_then(|rows| rows.get(number))
            .ok_or(Error::Association)?
            .air;
        Ok(SpaceAirTemperatureBorrow {
            space_id: metadata.typed_space_id.ok_or(Error::Association)?,
            zone_id: zone.typed_zone_id.ok_or(Error::Association)?,
            mat: &air.mat,
            zt: &air.zt,
            ztav: &air.ztav,
        })
    }
}
