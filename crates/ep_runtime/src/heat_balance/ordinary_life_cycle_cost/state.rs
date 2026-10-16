//! Native EconomicLifeCycleCost.hh312-376 row constructors used by input allocation.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub(super) enum SourceCostCategory {
    Maintenance = 0,
    Construction = 10,
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub(super) enum SourceStartCosts {
    ServicePeriod = 0,
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub(super) enum SourceResource {
    Invalid = -1,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceRecurringCost {
    name: String,
    line_item: String,
    category: SourceCostCategory,
    cost: f64,
    start_of_costs: SourceStartCosts,
    years_from_start: i32,
    months_from_start: i32,
    total_months_from_start: i32,
    repeat_period_years: i32,
    repeat_period_months: i32,
    total_repeat_period_months: i32,
    annual_escalation_rate: f64,
}
impl Default for SourceRecurringCost {
    fn default() -> Self {
        Self {
            name: String::new(),
            line_item: String::new(),
            category: SourceCostCategory::Maintenance,
            cost: 0.0,
            start_of_costs: SourceStartCosts::ServicePeriod,
            years_from_start: 0,
            months_from_start: 0,
            total_months_from_start: 0,
            repeat_period_years: 0,
            repeat_period_months: 0,
            total_repeat_period_months: 0,
            annual_escalation_rate: 0.0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceNonrecurringCost {
    name: String,
    line_item: String,
    category: SourceCostCategory,
    cost: f64,
    start_of_costs: SourceStartCosts,
    years_from_start: i32,
    months_from_start: i32,
    total_months_from_start: i32,
}
impl Default for SourceNonrecurringCost {
    fn default() -> Self {
        Self {
            name: String::new(),
            line_item: String::new(),
            category: SourceCostCategory::Construction,
            cost: 0.0,
            start_of_costs: SourceStartCosts::ServicePeriod,
            years_from_start: 0,
            months_from_start: 0,
            total_months_from_start: 0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceUsePriceEscalation {
    name: String,
    resource: SourceResource,
    escalation_start_year: i32,
    escalation_start_month: i32,
    // Native Array1D<Real64> is constructor-unallocated; allocate does not fill.
    pub(super) escalation: Option<Vec<Option<f64>>>,
}
impl Default for SourceUsePriceEscalation {
    fn default() -> Self {
        Self {
            name: String::new(),
            resource: SourceResource::Invalid,
            escalation_start_year: 0,
            escalation_start_month: 0,
            escalation: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceUseAdjustment {
    name: String,
    resource: SourceResource,
    pub(super) adjustment: Option<Vec<Option<f64>>>,
}
impl Default for SourceUseAdjustment {
    fn default() -> Self {
        Self {
            name: String::new(),
            resource: SourceResource::Invalid,
            adjustment: None,
        }
    }
}
