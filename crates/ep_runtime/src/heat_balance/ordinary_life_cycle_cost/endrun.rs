//! Actual ComputeLifeCycleCostAndReport119-138; GetInput remains at caller354.
use super::{Error, SourceEconomicLifeCycleCost, SourceErrorBackend};
use crate::heat_balance::error_backend::SourceDiagnosticFlags;

impl SourceEconomicLifeCycleCost {
    pub(crate) fn compute_source_life_cycle_cost_and_report(
        &self,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        if self.parameters_present {
            backend.display_source_string("Computing Life Cycle Costs and Reporting", flags)?;
            return Err(Error::Unavailable(
                "actual ComputeLifeCycleCostAndReport ExpressAsCashFlows132",
            ));
        }
        Ok(())
    }
}
