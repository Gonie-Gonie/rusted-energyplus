//! Model compiler stage contracts.

use ep_model::TypedModel;
use ep_raw_model::RawModel;

/// Ordered model compiler stages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompileStage {
    /// Parse epJSON into raw object storage.
    Parse,
    /// Validate against schema and required fields.
    SchemaValidation,
    /// Resolve defaults and canonical ordering.
    Normalize,
    /// Convert raw values to typed structs.
    TypedConversion,
    /// Resolve names to typed IDs.
    ReferenceResolution,
    /// Build model graphs.
    GraphBuild,
    /// Generate runtime execution plan.
    ExecutionPlan,
    /// Initialize runtime state and output handles.
    RuntimeInit,
}

/// Minimal report for a compiler pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompileReport {
    /// Stages that completed.
    pub completed_stages: Vec<CompileStage>,
    /// Raw object count observed at parse stage.
    pub raw_object_count: usize,
}

/// Placeholder compile function for v0.1 workspace validation.
#[must_use]
pub fn compile_raw_model(raw_model: &RawModel) -> (TypedModel, CompileReport) {
    let completed_stages = vec![CompileStage::Parse];
    let report = CompileReport {
        completed_stages,
        raw_object_count: raw_model.object_count(),
    };

    (TypedModel::default(), report)
}

#[cfg(test)]
mod tests {
    use super::{CompileStage, compile_raw_model};
    use ep_raw_model::RawModel;

    #[test]
    fn compile_report_records_parse_stage() {
        let raw_model = RawModel::default();
        let (_typed_model, report) = compile_raw_model(&raw_model);

        assert_eq!(report.completed_stages, vec![CompileStage::Parse]);
        assert_eq!(report.raw_object_count, 0);
    }
}
