//! Direct one-Regular-material/one-construction Init over actual caller owners.
//!
//! This entry does not construct a TypedModel, replay calculations, or import
//! observations. It shares the real preprocessing, all-resistive and postlude
//! bodies with ordinary initialization. Other reached routes remain unavailable.

use super::ctf_init_postlude::{
    CtfInitPostludeError, CtfInitPostludeProgress, CtfInitReportProvider,
    complete_ctf_init_postlude_from_provider,
};
use super::ctf_initialization_context::CtfInitializationContext;
use super::ctf_public_initialization::{CtfPublicInitAggregate, CtfPublicReturnRoute};
use super::ctf_reports::{CtfConstructionReport, CtfMaterialReport, CtfReportUnavailable};
use super::ctf_source_diagnostics::{CtfSourceDiagnosticStop, emit_ctf_loading_diagnostic};
use super::surface_manager::ctf_all_resistive::{
    CtfAllResistiveCoefficients, CtfAllResistiveUnavailable, generate_all_resistive_ctf,
};
use super::surface_manager::ctf_caller_stability::CtfCallerSharedFlags;
use super::surface_manager::ctf_layer_preprocessing::{
    CtfLayerContext, CtfLayerFailure, CtfLayerInput, CtfLayerPreprocessing, CtfLayerScopeError,
    try_preprocess_ctf_layers_retained,
};
use super::surface_manager::ctf_public_storage::{
    CtfPublicCoefficientStorage, CtfPublicStorageUnavailable,
    copy_all_resistive_public_coefficients,
};
use ep_model::{ConstructionId, MaterialSurfaceRoughness, ReportInputs};

/// Actual direct MaterialBase::Group::Regular and ConstructionProps members.
/// ROnly is the independent supplied `layer.resistance_only` member: it does
/// not convert the actual Regular material into a NoMass model definition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfDirectRegularConstructionInput<'a> {
    /// Original construction identity, independent of report counter one.
    pub construction_id: ConstructionId,
    /// Actual construction Name, without normalization or inferred loader spelling.
    pub construction_name: &'a str,
    /// Actual sole MaterialBase Name.
    pub material_name: &'a str,
    /// Actual sole original layer and its independent physical/R/ROnly members.
    pub layer: CtfLayerInput,
    /// Actual ConstructionProps::IsUsedCTF.
    pub is_used_ctf: bool,
    /// Actual ConstructionProps::SourceSinkPresent.
    pub source_sink_present: bool,
    /// Actual ConstructionProps::SolutionDimensions.
    pub solution_dimensions: i32,
    /// Actual outside thermal, inside thermal, outside solar, inside solar owners.
    pub absorptance: [f64; 4],
    /// Actual construction OutsideRoughness, not a material-property inference.
    pub outside_roughness: MaterialSurfaceRoughness,
    /// Actual global TimeStepZone; no input Timestep or reciprocal is substituted.
    pub time_step_zone: f64,
    /// Actual global AnyInternalHeatSourceInInput after the caller's real setup.
    pub any_internal_heat_source_in_input: bool,
}

/// Caller-retained calculation state, including genuine partial failure owners.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfDirectInitializationState {
    /// Actual construction storage reset/assigned at reached callpoints.
    pub storage: CtfPublicCoefficientStorage,
    /// Actual partial preprocessing owner; later absent checkpoints stay absent.
    pub preprocessing: Option<CtfLayerPreprocessing>,
    /// Actual generated all-resistive owner, absent if that producer was unreached.
    pub all_resistive: Option<CtfAllResistiveCoefficients>,
    /// Fresh actual whole-Init locals, retained even when a later effect fails.
    pub shared_flags: CtfCallerSharedFlags,
    /// Incoming global aggregate owners and the actual returned-call prefix.
    pub aggregate: CtfPublicInitAggregate,
    /// Set when this real selected construction call begins.
    pub construction_invoked: bool,
    /// Set only after the actual selected construction suffix returns.
    pub construction_return_route: Option<CtfPublicReturnRoute>,
    /// True only past the actual shared postlude/final fatal test.
    pub init_returned: bool,
}

impl CtfDirectInitializationState {
    /// Supply the actual incoming global aggregates, including on repeated Init.
    /// Fresh constructor owners are Source true/zero, not values from coefficients.
    #[must_use]
    pub fn new(simple_ctf_only: bool, max_ctf_terms: i32) -> Self {
        Self {
            storage: CtfPublicCoefficientStorage::reset(),
            preprocessing: None,
            all_resistive: None,
            shared_flags: CtfCallerSharedFlags {
                errors_found: false,
                do_ctf_error_report: false,
            },
            aggregate: CtfPublicInitAggregate {
                simple_ctf_only,
                max_ctf_terms,
                returned_constructions: 0,
            },
            construction_invoked: false,
            construction_return_route: None,
            init_returned: false,
        }
    }
}

/// Actual direct calls may stop without becoming a Native success/fatal claim.
#[derive(Debug)]
pub enum CtfDirectInitializationError {
    /// An actually reached selected preprocessing scope is unavailable.
    Preprocessing(CtfLayerScopeError),
    /// An actual loading diagnostic/backend failed at its original callpoint.
    Diagnostic(CtfSourceDiagnosticStop),
    /// The genuine converted owner selected an unrepresented later mass route.
    AllResistive(CtfAllResistiveUnavailable),
    /// An actual public copy is not representable; no clipped owner is supplied.
    Storage(CtfPublicStorageUnavailable),
    /// The shared report/Scan/backend suffix failed or reached typed Source fatal.
    Postlude(CtfInitPostludeError),
}
impl std::fmt::Display for CtfDirectInitializationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Preprocessing(value) => {
                write!(formatter, "direct CTF preprocessing unavailable: {value:?}")
            }
            Self::Diagnostic(value) => std::fmt::Display::fmt(value, formatter),
            Self::AllResistive(value) => {
                write!(formatter, "direct CTF all-resistive unavailable: {value:?}")
            }
            Self::Storage(value) => write!(
                formatter,
                "direct CTF public storage unavailable: {value:?}"
            ),
            Self::Postlude(value) => std::fmt::Display::fmt(value, formatter),
        }
    }
}
impl std::error::Error for CtfDirectInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Diagnostic(value) => Some(value),
            Self::Postlude(value) => Some(value),
            Self::Preprocessing(_) | Self::AllResistive(_) | Self::Storage(_) => None,
        }
    }
}

struct DirectReportProvider<'a> {
    input: &'a CtfDirectRegularConstructionInput<'a>,
    storage: &'a CtfPublicCoefficientStorage,
}
impl CtfInitReportProvider for DirectReportProvider<'_> {
    fn construction_count(&self) -> usize {
        1
    }
    fn is_used_ctf(&self, index: usize) -> Result<bool, CtfReportUnavailable> {
        if index != 0 {
            return Err(CtfReportUnavailable::Association);
        }
        Ok(self.input.is_used_ctf)
    }
    fn prepare_report(
        &self,
        index: usize,
    ) -> Result<CtfConstructionReport<'_>, CtfReportUnavailable> {
        if index != 0 {
            return Err(CtfReportUnavailable::Association);
        }
        let input = self.input;
        let layer = input.layer;
        Ok(CtfConstructionReport {
            construction_id: input.construction_id,
            declared_counter: 1,
            name: input.construction_name,
            layers: vec![CtfMaterialReport {
                material_id: layer.material_id,
                name: input.material_name,
                air_gap: false,
                values: [
                    layer.thickness,
                    layer.conductivity,
                    layer.density,
                    layer.specific_heat,
                    layer.resistance,
                ],
            }],
            public: self.storage,
            absorptance: input.absorptance,
            outside_roughness: Some(input.outside_roughness),
        })
    }
}

/// Invoke one actual direct construction and the shared whole-Init postlude.
///
/// The caller first performs its genuine registry/setup/optional Scan calls.
/// `report_inputs` must be produced by the real parser/compiler from actual input,
/// not a cache/result projection. Neither geometry nor a TypedModel is created.
/// All state, report/table/cache owners and sinks remain borrowed across Err.
pub fn initialize_direct_regular_ctf(
    input: &CtfDirectRegularConstructionInput<'_>,
    report_inputs: &ReportInputs,
    state: &mut CtfDirectInitializationState,
    context: &mut CtfInitializationContext<'_>,
) -> Result<(), CtfDirectInitializationError> {
    use CtfDirectInitializationError as Error;
    // Literal fresh Init locals; prior registry/cache/global aggregates are retained.
    state.shared_flags = CtfCallerSharedFlags {
        errors_found: false,
        do_ctf_error_report: false,
    };
    state.aggregate.returned_constructions = 0;
    state.preprocessing = None;
    state.all_resistive = None;
    state.construction_return_route = None;
    state.init_returned = false;
    context.reports.progress = CtfInitPostludeProgress::default();
    state.construction_invoked = true;
    state.storage = CtfPublicCoefficientStorage::reset();
    let route = if !input.is_used_ctf {
        // Native returns before inspecting layers/dimensions or assigning dt.
        CtfPublicReturnRoute::UnusedReset
    } else {
        state.storage.ctf_time_step = input.time_step_zone;
        let result = try_preprocess_ctf_layers_retained(
            &[input.layer],
            CtfLayerContext {
                is_used_ctf: input.is_used_ctf,
                errors_found: state.shared_flags.errors_found,
                source_sink_present: input.source_sink_present,
                solution_dimensions: input.solution_dimensions,
            },
            &mut state.preprocessing,
            |call| emit_ctf_loading_diagnostic(&mut context.backend, &[input.material_name], call),
        );
        // Read the genuinely retained flag, including earlier completed writes on Err.
        if let Some(owner) = &state.preprocessing {
            state.shared_flags.errors_found = owner.errors_found;
        }
        result.map_err(|failure| match failure {
            CtfLayerFailure::Unavailable(value) => Error::Preprocessing(value),
            CtfLayerFailure::Diagnostic(value) => Error::Diagnostic(value),
        })?;
        let owner = state.preprocessing.as_ref().ok_or(Error::AllResistive(
            CtfAllResistiveUnavailable::PostConversionUnavailable,
        ))?;
        if owner.errors_found {
            CtfPublicReturnRoute::LoadingErrorReset
        } else {
            let generated = generate_all_resistive_ctf(owner, input.time_step_zone)
                .map_err(Error::AllResistive)?;
            state.all_resistive = Some(generated);
            state.storage =
                copy_all_resistive_public_coefficients(&generated).map_err(Error::Storage)?;
            CtfPublicReturnRoute::AllResistive
        }
    };
    state.construction_return_route = Some(route);
    // Exact actual caller postreturn order; no update occurs on a failed call.
    if state.storage.num_histories > 1 {
        state.aggregate.simple_ctf_only = false;
    }
    if state.storage.num_ctf_terms > state.aggregate.max_ctf_terms {
        state.aggregate.max_ctf_terms = state.storage.num_ctf_terms;
    }
    state.aggregate.returned_constructions += 1;
    let reports = DirectReportProvider {
        input,
        storage: &state.storage,
    };
    complete_ctf_init_postlude_from_provider(
        report_inputs,
        &reports,
        state.shared_flags,
        input.any_internal_heat_source_in_input,
        &mut state.aggregate.simple_ctf_only,
        context.reports.registry.opaque_layer_columns(),
        &mut context.reports.table,
        &mut context.reports.scan,
        context.eio,
        &mut context.backend,
        &mut context.reports.progress,
    )
    .map_err(Error::Postlude)?;
    state.init_returned = true;
    Ok(())
}

#[cfg(test)]
#[path = "ctf_direct_initialization_tests.rs"]
mod tests;
