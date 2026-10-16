//! Genuine EconomicLifeCycleCost input wrapper on one retained native module slice.
//! Positive object getters stop at their first actual unported operation.
mod input;
mod state;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use state::{
    SourceNonrecurringCost, SourceRecurringCost, SourceUseAdjustment, SourceUsePriceEscalation,
};

/// Actual reached EconomicLifeCycleCostData members, not CostEstimate or Tariff.
/// The remaining monetary/computation fields are outside this input prefix.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceEconomicLifeCycleCost {
    get_input: bool,
    parameters_present: bool,
    length_study_years: i32,
    number_recurring_costs: i32,
    number_nonrecurring_cost: i32,
    number_use_price_escalation: i32,
    number_use_adjustment: i32,
    recurring_costs: Vec<SourceRecurringCost>,
    nonrecurring_cost: Vec<SourceNonrecurringCost>,
    use_price_escalation: Option<Vec<SourceUsePriceEscalation>>,
    use_adjustment: Option<Vec<SourceUseAdjustment>>,
}
impl Default for SourceEconomicLifeCycleCost {
    fn default() -> Self {
        Self {
            get_input: true,
            parameters_present: false,
            length_study_years: 0,
            number_recurring_costs: 0,
            number_nonrecurring_cost: 0,
            number_use_price_escalation: 0,
            number_use_adjustment: 0,
            recurring_costs: Vec::new(),
            nonrecurring_cost: Vec::new(),
            use_price_escalation: None,
            use_adjustment: None,
        }
    }
}
impl SourceEconomicLifeCycleCost {
    /// Actual SimulationManager354 child. All five calls precede the once-flag clear.
    /// Shared blank/name arrays are borrowed but untouched before a real getter.
    pub(crate) fn get_source_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.get_input {
            self.get_parameters(input, shortcut, backend)?;
            self.get_recurring_costs(input, shortcut)?;
            self.get_nonrecurring_cost(input, shortcut)?;
            self.get_use_price_escalation(input, shortcut)?;
            self.get_use_adjustment(input, shortcut)?;
            self.get_input = false;
        }
        Ok(())
    }
}
