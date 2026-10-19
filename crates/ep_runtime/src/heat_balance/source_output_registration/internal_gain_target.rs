//! Actual Which references to same OtherEquipment rate/energy owners.
use super::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable, SourceRealOutputOwners,
    SourceRealOutputTarget,
};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceInternalGainOutputField {
    OtherPower,
    OtherConsumption,
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
        let row = self
            .allocations
            .other_equipment
            .as_ref()
            .and_then(|rows| rows.get(i))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        Ok(match field {
            SourceInternalGainOutputField::OtherPower => &row.rates[0],
            SourceInternalGainOutputField::OtherConsumption => &row.energies[0],
        })
    }
}
