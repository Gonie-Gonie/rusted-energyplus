//! Actual five getter prefixes; Parameters absence returns the later four at their guards.
use super::*;
fn counts(
    input: &mut SelectedInputProcessor,
    family: &str,
) -> Result<crate::heat_balance::selected_input_processor::SourceArgumentCounts, Error> {
    let args = input.get_object_def_max_args(family).map_err(|_| {
        Error::Unavailable("actual LifeCycleCost getObjectDefMaxArgs schema/size domain")
    })?;
    let total = args
        .total()
        .map_err(|_| Error::Unavailable("actual LifeCycleCost NumFields signed/storage domain"))?;
    let _number_fields = i32::try_from(total)
        .map_err(|_| Error::Unavailable("actual LifeCycleCost NumFields signed i32 domain"))?;
    Ok(args)
}
fn count(input: &SelectedInputProcessor, family: &str) -> Result<i32, Error> {
    i32::try_from(input.get_num_objects_found(family)).map_err(|_| {
        Error::Unavailable("actual LifeCycleCost getNumObjectsFound signed i32 domain")
    })
}
fn size(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .map_err(|_| Error::Unavailable("actual LifeCycleCost allocation extent"))
}
impl SourceEconomicLifeCycleCost {
    pub(super) fn get_parameters(
        &mut self,
        input: &mut SelectedInputProcessor,
        _shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // CurrentModuleObject is a native local, not dataIPShortCut's string.
        let current_module_object = "LifeCycleCost:Parameters";
        let args = counts(input, current_module_object)?;
        let _numeric_arguments = vec![None::<f64>; args.numerics];
        let _alpha_arguments = vec![String::new(); args.alphas];
        let number_objects = count(input, current_module_object)?;
        if number_objects == 0 {
            self.parameters_present = false;
        } else if number_objects == 1 {
            self.parameters_present = true;
            return Err(Error::Unavailable(
                "actual LifeCycleCost Parameters181 first getObjectItem/local data and shared blank/name writes",
            ));
        } else {
            backend.show_warning_error(&format!("{current_module_object}: Only one instance of this object is allowed. No life-cycle cost reports will be generated."))?;
            self.parameters_present = false;
        }
        Ok(())
    }
    pub(super) fn get_recurring_costs(
        &mut self,
        input: &mut SelectedInputProcessor,
        _shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        if !self.parameters_present {
            return Ok(());
        }
        let current_module_object = "LifeCycleCost:RecurringCosts";
        let args = counts(input, current_module_object)?;
        let _numeric_arguments = vec![None::<f64>; args.numerics];
        let _alpha_arguments = vec![String::new(); args.alphas];
        self.number_recurring_costs = count(input, current_module_object)?;
        self.recurring_costs.resize_with(
            size(self.number_recurring_costs)?,
            SourceRecurringCost::default,
        );
        if self.number_recurring_costs > 0 {
            return Err(Error::Unavailable(
                "actual LifeCycleCost RecurringCosts471 first getObjectItem/local data and shared blank/name writes",
            ));
        }
        Ok(())
    }
    pub(super) fn get_nonrecurring_cost(
        &mut self,
        input: &mut SelectedInputProcessor,
        _shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        if !self.parameters_present {
            return Ok(());
        }
        let current_module_object = "LifeCycleCost:NonrecurringCost";
        let args = counts(input, current_module_object)?;
        let _numeric_arguments = vec![None::<f64>; args.numerics];
        let _alpha_arguments = vec![String::new(); args.alphas];
        self.number_nonrecurring_cost = count(input, current_module_object)?;
        let number_component_cost_line_items = count(input, "ComponentCost:LineItem")?;
        if number_component_cost_line_items > 0 {
            let extent = self
                .number_nonrecurring_cost
                .checked_add(1)
                .ok_or(Error::Unavailable(
                    "actual LifeCycleCost numNonrecurringCost+1 signed domain",
                ))?;
            self.nonrecurring_cost
                .resize_with(size(extent)?, SourceNonrecurringCost::default);
        } else {
            self.nonrecurring_cost.resize_with(
                size(self.number_nonrecurring_cost)?,
                SourceNonrecurringCost::default,
            );
        }
        if self.number_nonrecurring_cost > 0 {
            return Err(Error::Unavailable(
                "actual LifeCycleCost NonrecurringCost696 first getObjectItem/local data and shared blank/name writes",
            ));
        }
        Ok(())
    }
    pub(super) fn get_use_price_escalation(
        &mut self,
        input: &mut SelectedInputProcessor,
        _shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        if !self.parameters_present {
            return Ok(());
        }
        let current_module_object = "LifeCycleCost:UsePriceEscalation";
        let args = counts(input, current_module_object)?;
        let _numeric_arguments = vec![None::<f64>; args.numerics];
        let _alpha_arguments = vec![String::new(); args.alphas];
        self.number_use_price_escalation = count(input, current_module_object)?;
        self.use_price_escalation = Some(
            (0..self.number_use_price_escalation)
                .map(|_| SourceUsePriceEscalation::default())
                .collect(),
        );
        for row in self
            .use_price_escalation
            .as_mut()
            .ok_or(Error::Unavailable(
                "actual LifeCycleCost UsePriceEscalation allocation",
            ))?
        {
            row.escalation = Some(vec![None; size(self.length_study_years)?]);
        }
        if self.number_use_price_escalation > 0 {
            return Err(Error::Unavailable(
                "actual LifeCycleCost UsePriceEscalation841 first getObjectItem/local data and shared blank/name writes",
            ));
        }
        Ok(())
    }
    pub(super) fn get_use_adjustment(
        &mut self,
        input: &mut SelectedInputProcessor,
        _shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        if !self.parameters_present {
            return Ok(());
        }
        let current_module_object = "LifeCycleCost:UseAdjustment";
        let args = counts(input, current_module_object)?;
        let _numeric_arguments = vec![None::<f64>; args.numerics];
        let _alpha_arguments = vec![String::new(); args.alphas];
        self.number_use_adjustment = count(input, current_module_object)?;
        self.use_adjustment = Some(
            (0..self.number_use_adjustment)
                .map(|_| SourceUseAdjustment::default())
                .collect(),
        );
        for row in self.use_adjustment.as_mut().ok_or(Error::Unavailable(
            "actual LifeCycleCost UseAdjustment allocation",
        ))? {
            row.adjustment = Some(vec![None; size(self.length_study_years)?]);
        }
        if self.number_use_adjustment > 0 {
            return Err(Error::Unavailable(
                "actual LifeCycleCost UseAdjustment993 first getObjectItem/local data and shared blank/name writes",
            ));
        }
        Ok(())
    }
}
