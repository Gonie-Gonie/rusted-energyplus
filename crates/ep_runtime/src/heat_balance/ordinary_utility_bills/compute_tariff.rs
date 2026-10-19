//! Actual EconomicTariff2665-2703 prefix on the sole retained economic owner.
//! Positive economics records and native tariff variables remain explicit boundaries.
use super::{Error, SourceEconomicTariff};
use crate::heat_balance::ordinary_output_control::{
    SourceOutputControl, SourceResultsFrameworkOptions,
};
use crate::heat_balance::ordinary_tabular_reports::SourceTabularReports;

impl SourceEconomicTariff {
    /// Actual SimulationManager587 child. Input is not replayed at endrun.
    // Keep the explicit retained work-vector allocation and indexing used by the Original tariff algorithm.
    #[allow(clippy::useless_vec)]
    pub(crate) fn compute_source_tariff(
        &mut self,
        output_control: &SourceOutputControl,
        results: &SourceResultsFrameworkOptions,
        tabular: &mut SourceTabularReports,
    ) -> Result<(), Error> {
        // Native2678-2681 allocate four local Array1D<Real64>(NumMonths).
        // None marks allocated-unwritten cells; no numerical zero is supplied.
        let _a = vec![None::<f64>; 12];
        let _b = vec![None::<f64>; 12];
        let _c = vec![None::<f64>; 12];
        let _d = vec![None::<f64>; 12];
        const _NO_VAR: i32 = 0;
        let _annual_aggregate: Option<f64> = None;

        if !output_control.source_write_tabular(results) {
            *tabular.source_write_tabular_files_mut() = false;
            return Ok(());
        }

        // ObjexxFCL::HUGE_(Real64()) is numeric_limits<double>::max().
        let _huge_value = f64::MAX;
        // Native2694-2696 first reads econVar(nVar) then clears isEvaluated.
        // No positive EconVar record/allocator is implemented by this prefix.
        if self.number_economics_variables > 0 {
            return Err(Error::Unavailable(
                "actual ComputeTariff first econVar(nVar).isEvaluated2695",
            ));
        }

        if self.number_tariffs == 0 {
            return Ok(());
        }

        *tabular.source_write_tabular_files_mut() = true;
        Err(Error::Unavailable(
            "actual ComputeTariff setNativeVariables2703 entry",
        ))
    }
}
