//! Sole native HVACSizingSimMgrData pointer; admitted constructor-null path only.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
/// Opaque positive pointer identity; no absent sizing logger state is fabricated.
/// The genuine make_unique at ManageHVACSizingSimulation196 remains unavailable.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceHvacSizingSimulationManager {
    _unported: (),
}
/// HVACSizingSimulationManager.hh108 unique_ptr constructor-null; not a KindOfSim inference.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHvacSizingSimulationData {
    pub(crate) manager: Option<Box<SourceHvacSizingSimulationManager>>,
}
impl SourceHvacSizingSimulationData {
    /// The actual caller tests KindOfSim4/5 before this actual pointer guard.
    pub(crate) fn update_source_zone_step_logs(&self) -> Result<(), Error> {
        if self.manager.is_some() {
            return Err(Error::Unavailable(
                "actual HVACSizingSimulationManager UpdateSizingLogsZoneStep sizingLogger.UpdateSizingLogValuesZoneStep184",
            ));
        }
        Ok(())
    }
}
