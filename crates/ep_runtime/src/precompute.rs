//! Runtime precomputation artifacts shared by setup and execution planning.

use crate::{ExecutionPlan, RuntimeOutputRegistry, build_execution_plan_with_output_registry};
use ep_model::SimulationModel;

/// Runtime data resolved once from a compiled simulation model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimePrecomputedData {
    /// Deterministic source-order execution plan.
    pub execution_plan: ExecutionPlan,
    /// Output registry cached for the run.
    pub output_registry: RuntimeOutputRegistry,
}

impl RuntimePrecomputedData {
    /// Builds runtime precomputed data for a compiled simulation model.
    #[must_use]
    pub fn from_model(model: &SimulationModel) -> Self {
        let output_registry = RuntimeOutputRegistry::from_model(model);
        let execution_plan = build_execution_plan_with_output_registry(model, &output_registry);
        Self {
            execution_plan,
            output_registry,
        }
    }
}

/// Precomputes runtime data for one compiled simulation model.
#[must_use]
pub fn precompute_runtime_data(model: &SimulationModel) -> RuntimePrecomputedData {
    RuntimePrecomputedData::from_model(model)
}
