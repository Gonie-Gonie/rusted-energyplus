//! Actual InitConductionTransferFunctions6153–6203, fresh at HB2622.
//! Scientific public storage moves from the same construction allocation; no
//! old cache, completed phases, coefficient overrides or Native data are inputs.
use crate::heat_balance::ctf_first_assembly_owner::initialize_construction_ctf_first_assembly;
use crate::heat_balance::ctf_init_postlude::{CtfInitPostludeError, CtfInitPostludeProgress};
use crate::heat_balance::ctf_initial_owner::{
    ConstructionCtfInitialDiscretization, CtfSourcePriorConstruction,
    initialize_construction_ctf_initial_from_source,
};
use crate::heat_balance::ctf_public_initialization::{
    ConstructionCtfPublicInitialization, CtfPublicConstructionReturn,
    CtfPublicInitializationUnavailable, CtfPublicReturnRoute, OrderedCtfPublicInitialization,
    initialize_public_suffix,
};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ctf_retry_initialization::{
    ConstructionCtfRetryInitialization, CtfRetryChainStop, try_initialize_construction_ctf_retry,
};
use crate::heat_balance::ctf_source_diagnostics::{
    CtfSourceDiagnosticStop, emit_ctf_caller_diagnostic, emit_ctf_loading_diagnostic,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_material_data::{SelectedMaterialData, SourceMaterialGroup};
use crate::heat_balance::surface_manager::ctf_all_resistive::{
    ConstructionCtfAllResistive, CtfAllResistiveUnavailable, generate_all_resistive_ctf,
};
use crate::heat_balance::surface_manager::ctf_caller_stability::{
    CtfCallerDiagnosticNames, CtfCallerSharedFlags,
};
use crate::heat_balance::surface_manager::ctf_layer_preprocessing::{
    ConstructionCtfLayerPreprocessing, CtfLayerContext, CtfLayerFailure, CtfLayerInput,
    CtfLayerPreprocessing, CtfLayerScopeError, material_layer_inputs,
    try_preprocess_ctf_layers_retained,
};
use crate::heat_balance::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage;
use crate::weather::day::WeatherGlobalState;
use ep_model::{Construction, ConstructionId, MaterialDefinition, ReportInputs, TypedModel};
use std::io::Write;
mod reports;

pub(crate) struct SourceCtfInitializationOwners<'a, 'b> {
    pub(crate) model: &'a TypedModel,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) materials: &'a SelectedMaterialData,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) backend: &'a mut SourceErrorBackend<'b>,
    pub(crate) eio: &'a mut dyn Write,
}
#[derive(Debug)]
/// Actual CTF scientific scope, diagnostic and report causes cross HB unchanged.
pub enum SourceCtfInitializationError {
    /// An actual owner/identity required by this Source route is unavailable.
    Unavailable(&'static str),
    /// The actual selected layer loader stopped outside its represented scope.
    Loading(CtfLayerScopeError),
    /// A real Source diagnostic/backend failure retains its complete cause.
    Diagnostic(CtfSourceDiagnosticStop),
    /// The invoked real retry driver stopped before public coefficient stores.
    Retry(CtfRetryChainStop),
    /// Actual selected SI storage/report association is unavailable.
    Public(CtfPublicInitializationUnavailable),
    /// The real predefined/Scan/EIO/backend/fatal suffix failed.
    Postlude(CtfInitPostludeError),
}
impl std::fmt::Display for SourceCtfInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(v) => write!(f, "actual Source CTF owner unavailable: {v}"),
            Self::Loading(v) => write!(f, "actual Source CTF loading: {v:?}"),
            Self::Diagnostic(v) => std::fmt::Display::fmt(v, f),
            Self::Retry(v) => write!(f, "actual Source CTF retry: {v:?}"),
            Self::Public(v) => write!(f, "actual Source CTF public storage: {v:?}"),
            Self::Postlude(v) => std::fmt::Display::fmt(v, f),
        }
    }
}
impl std::error::Error for SourceCtfInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Diagnostic(v) => Some(v),
            Self::Postlude(v) => Some(v),
            _ => None,
        }
    }
}
type Error = SourceCtfInitializationError;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceCtfPhase {
    pub(crate) declared_index: usize,
    pub(crate) construction_id: ConstructionId,
    pub(crate) loading: Option<CtfLayerPreprocessing>,
    pub(crate) preprocessing: Option<ConstructionCtfLayerPreprocessing>,
    pub(crate) all_resistive: Option<ConstructionCtfAllResistive>,
    pub(crate) initial: Option<ConstructionCtfInitialDiscretization>,
    pub(crate) retry_invoked: bool,
    pub(crate) retry: Option<ConstructionCtfRetryInitialization>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceCtfInitialization {
    /// Same DataHeatBalance SimpleCTFOnly/MaxCTFTerms and construction stores.
    pub(crate) public: OrderedCtfPublicInitialization,
    pub(crate) shared_flags: CtfCallerSharedFlags,
    pub(crate) phases: Vec<SourceCtfPhase>,
    /// A reached reset/loading/method failure retains this moved original store.
    pub(crate) active_storage: Option<CtfPublicCoefficientStorage>,
    pub(crate) active_index: Option<usize>,
    pub(crate) progress: CtfInitPostludeProgress,
    pub(crate) init_returned: bool,
}
impl Default for SourceCtfInitialization {
    fn default() -> Self {
        Self {
            public: OrderedCtfPublicInitialization::begin(true, 0),
            shared_flags: CtfCallerSharedFlags {
                errors_found: false,
                do_ctf_error_report: false,
            },
            phases: Vec::new(),
            active_storage: None,
            active_index: None,
            progress: CtfInitPostludeProgress::default(),
            init_returned: false,
        }
    }
}
fn reset(storage: &mut CtfPublicCoefficientStorage) {
    // Exact Construction146–161 member-write order on the same live allocation.
    storage.cross.fill(0.0);
    storage.flux.fill(0.0);
    storage.inside.fill(0.0);
    storage.outside.fill(0.0);
    storage.source_in.fill(0.0);
    storage.source_out.fill(0.0);
    storage.ctf_time_step = 0.0;
    storage.temperature_source_out.fill(0.0);
    storage.temperature_source_in.fill(0.0);
    storage.temperature_source_q.fill(0.0);
    storage.temperature_user_out.fill(0.0);
    storage.temperature_user_in.fill(0.0);
    storage.temperature_user_source.fill(0.0);
    storage.num_histories = 0;
    storage.num_ctf_terms = 0;
    storage.u_value = 0.0;
}
fn slot(value: i32) -> Result<usize, Error> {
    usize::try_from(value)
        .ok()
        .and_then(|v| v.checked_sub(1))
        .ok_or(Error::Unavailable("Source positive MaterialNum/LayerPoint"))
}

impl SourceCtfInitialization {
    /// All calculations run now, once per actual declared Construction call.
    /// Repeated Init moves the SAME coefficient owners back from prior public
    /// rows before traversal; unreached later stores retain their prior bytes.
    pub(crate) fn initialize_source_conduction_transfer_functions(
        &mut self,
        owners: &mut SourceCtfInitializationOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.shared_flags = CtfCallerSharedFlags {
            errors_found: false,
            do_ctf_error_report: false,
        }; //6155/56
        self.init_returned = false;
        self.progress = CtfInitPostludeProgress::default();
        // Rust ownership moves only: no Source numeric value changes here.
        for row in self.public.constructions.drain(..) {
            let target = owners
                .control
                .source_ctf_coefficients
                .get_mut(row.declared_index)
                .ok_or(Error::Unavailable("same Construction coefficient slot"))?;
            if target.is_some() {
                return Err(Error::Unavailable(
                    "duplicate public Construction coefficient owner",
                ));
            }
            *target = Some(row.result.map_err(Error::Public)?.storage);
        }
        if let Some(index) = self.active_index.take() {
            let target = owners
                .control
                .source_ctf_coefficients
                .get_mut(index)
                .ok_or(Error::Unavailable(
                    "same partial Construction coefficient slot",
                ))?;
            if target.is_some() {
                return Err(Error::Unavailable(
                    "duplicate partial Construction coefficient owner",
                ));
            }
            *target = Some(self.active_storage.take().ok_or(Error::Unavailable(
                "actual retained partial coefficient allocation",
            ))?);
        }
        self.phases.clear();
        self.public
            .aggregate_after_returned_calls
            .returned_constructions = 0;
        self.public.selected_stop = None;
        self.public.selected_declared_loop_completed = false;
        self.public.final_shared_flags = None;
        for index in 0..owners.control.construction_usage.len() {
            let usage = &owners.control.construction_usage[index];
            let id = usage
                .construction_id
                .ok_or(Error::Unavailable("Source Construction identity"))?;
            let construction = owners
                .model
                .constructions
                .iter()
                .find(|v| v.id == id)
                .ok_or(Error::Unavailable("same physical Construction identity"))?;
            self.active_index = Some(index);
            self.active_storage = Some(
                owners
                    .control
                    .source_ctf_coefficients
                    .get_mut(index)
                    .and_then(Option::take)
                    .ok_or(Error::Unavailable(
                        "sole Construction coefficient allocation",
                    ))?,
            );
            reset(
                self.active_storage
                    .as_mut()
                    .ok_or(Error::Unavailable("active coefficient owner"))?,
            );
            let invoked_flags = self.shared_flags;
            if !usage.is_used_ctf {
                self.finish_row(
                    index,
                    construction,
                    false,
                    CtfPublicReturnRoute::UnusedReset,
                    invoked_flags,
                    None,
                )?;
                continue;
            }
            self.active_storage
                .as_mut()
                .ok_or(Error::Unavailable("active CTFTimeStep owner"))?
                .ctf_time_step = owners.clock.time_step_zone; //201
            let inputs = source_layer_inputs(usage, owners.materials, owners.model)?;
            let names = inputs
                .iter()
                .map(|v| {
                    crate::heat_balance::ctf_initialization_context::original_material_name(
                        owners.model,
                        v.material_id,
                    )
                    .map_err(|_| Error::Unavailable("actual original Material Name"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let context = CtfLayerContext {
                is_used_ctf: usage.is_used_ctf,
                errors_found: self.shared_flags.errors_found,
                source_sink_present: usage.source_sink_present,
                solution_dimensions: usage.solution_dimensions,
            };
            self.phases.push(SourceCtfPhase {
                declared_index: index,
                construction_id: id,
                loading: None,
                preprocessing: None,
                all_resistive: None,
                initial: None,
                retry_invoked: false,
                retry: None,
            });
            let phase = self
                .phases
                .last_mut()
                .ok_or(Error::Unavailable("actual phase owner"))?;
            let loaded =
                try_preprocess_ctf_layers_retained(&inputs, context, &mut phase.loading, |call| {
                    emit_ctf_loading_diagnostic(owners.backend, &names, call)
                });
            if let Some(actual) = &phase.loading {
                self.shared_flags.errors_found = actual.errors_found;
            }
            loaded.map_err(|e| match e {
                CtfLayerFailure::Unavailable(v) => Error::Loading(v),
                CtfLayerFailure::Diagnostic(v) => Error::Diagnostic(v),
            })?;
            let preprocessing = ConstructionCtfLayerPreprocessing {
                construction_id: id,
                is_used_ctf: usage.is_used_ctf,
                result: Ok(phase
                    .loading
                    .take()
                    .ok_or(Error::Unavailable("actual returned preprocessing"))?),
            };
            phase.preprocessing = Some(preprocessing);
            let preprocessing = phase
                .preprocessing
                .as_ref()
                .ok_or(Error::Unavailable("retained preprocessing"))?;
            phase.all_resistive = Some(ConstructionCtfAllResistive {
                construction_id: id,
                result: match &preprocessing.result {
                    Ok(v) => generate_all_resistive_ctf(v, owners.clock.time_step_zone),
                    Err(v) => Err(CtfAllResistiveUnavailable::PreprocessingScope(*v)),
                },
            });
            let previous = owners.control.construction_usage[..index]
                .iter()
                .map(|v| {
                    let prior_id = v
                        .construction_id
                        .ok_or(Error::Unavailable("actual prior Construction identity"))?;
                    let prior = owners
                        .model
                        .constructions
                        .iter()
                        .find(|c| c.id == prior_id)
                        .ok_or(Error::Unavailable(
                            "prior Construction metadata association",
                        ))?;
                    validate_layer_ids(v, owners.materials, prior)?;
                    Ok(CtfSourcePriorConstruction {
                        construction_id: prior_id,
                        layer_ids: prior.effective_layers(),
                        is_used_ctf: v.is_used_ctf,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            phase.initial = Some(initialize_construction_ctf_initial_from_source(
                construction,
                &previous,
                preprocessing,
                context,
                owners.clock.time_step_zone,
            ));
            let first = initialize_construction_ctf_first_assembly(
                phase
                    .initial
                    .as_ref()
                    .ok_or(Error::Unavailable("retained initial phase"))?,
                preprocessing,
            );
            let diagnostic_names = CtfCallerDiagnosticNames {
                construction_name: &usage.name,
                original_tot_layers: usage.total_layers,
                original_layer_material_names: &names,
            };
            phase.retry_invoked = true;
            let retry = try_initialize_construction_ctf_retry(
                first,
                self.shared_flags,
                diagnostic_names,
                |call| emit_ctf_caller_diagnostic(owners.backend, call),
            )
            .map_err(Error::Diagnostic)?;
            self.shared_flags = retry.flags_on_return;
            let chain_stop = retry.selected_chain_stop;
            phase.retry = Some(retry);
            if let Some(stop) = chain_stop {
                return Err(Error::Retry(stop));
            }
            let returned = initialize_public_suffix(
                construction,
                preprocessing,
                phase.initial.as_ref(),
                phase
                    .all_resistive
                    .as_ref()
                    .ok_or(Error::Unavailable("actual all-resistive route owner"))?,
                phase
                    .retry
                    .as_ref()
                    .ok_or(Error::Unavailable("actual retry owner"))?,
                &self.public.constructions,
            )
            .map_err(Error::Public)?;
            // Move the genuine return storage once. Retry method contexts retain
            // actual evolving dt/history; reset-prefix storage is not read back
            // into those calculations or reported as their live partial header.
            self.active_storage = Some(returned.storage);
            let retry_flags = phase
                .retry
                .as_ref()
                .map(|v| (v.flags_on_entry, v.flags_on_return));
            self.finish_row(
                index,
                construction,
                true,
                returned.route,
                invoked_flags,
                retry_flags,
            )?;
        }
        self.public.finish_declared_loop_with_flags(
            owners.control.construction_usage.len(),
            self.shared_flags,
        );
        reports::complete(self, owners).map_err(Error::Postlude)?;
        self.init_returned = true;
        Ok(())
    }
    fn finish_row(
        &mut self,
        index: usize,
        construction: &Construction,
        is_used: bool,
        route: CtfPublicReturnRoute,
        unused_flags: CtfCallerSharedFlags,
        retry_flags: Option<(CtfCallerSharedFlags, CtfCallerSharedFlags)>,
    ) -> Result<(), Error> {
        let storage = self
            .active_storage
            .take()
            .ok_or(Error::Unavailable("returned sole public storage"))?;
        if storage.num_histories > 1 {
            self.public.aggregate_after_returned_calls.simple_ctf_only = false;
        }
        if storage.num_ctf_terms > self.public.aggregate_after_returned_calls.max_ctf_terms {
            self.public.aggregate_after_returned_calls.max_ctf_terms = storage.num_ctf_terms;
        }
        self.public
            .aggregate_after_returned_calls
            .returned_constructions += 1;
        self.public
            .constructions
            .push(ConstructionCtfPublicInitialization {
                construction_id: construction.id,
                declared_index: index,
                original_material_ids: construction.effective_layers().to_vec(),
                is_used_ctf: is_used,
                public_initializer_invoked: true,
                retry_flags_on_entry: retry_flags.map(|v| v.0),
                retry_flags_on_return: retry_flags.map(|v| v.1),
                unused_reset_flags: (!is_used).then_some(unused_flags),
                result: Ok(CtfPublicConstructionReturn { route, storage }),
            });
        self.active_index = None;
        Ok(())
    }
    /// Borrow the actual phase-owned header; never read stale reset-prefix
    /// dt/history after the initial/retry contexts became their live owners.
    #[allow(
        dead_code,
        reason = "This retained CTF phase-header borrowing entry preserves the genuine active coefficient owner API; production now reads the phase-owned values through its narrower caller views."
    )]
    pub(crate) fn active_header(&self) -> Result<(&f64, &i32, &i32), Error> {
        let index = self
            .active_index
            .ok_or(Error::Unavailable("no active Construction call"))?;
        let storage = self
            .active_storage
            .as_ref()
            .ok_or(Error::Unavailable("same active coefficient arrays"))?;
        if let Some(phase) = self.phases.iter().find(|v| v.declared_index == index) {
            if phase.retry_invoked {
                let retry=phase.retry.as_ref().ok_or(Error::Unavailable("fallible retry diagnostic API did not retain its in-flight private scratch; header unpaired"))?;
                let driver = retry
                    .result
                    .as_ref()
                    .map_err(|_| Error::Unavailable("actual returned retry driver"))?;
                let context = driver
                    .postcheck_context
                    .as_ref()
                    .or_else(|| driver.retained_assembly.as_ref().map(|v| &v.context));
                if let Some(context) = context {
                    let terms=driver.attempts.last().and_then(|v|v.final_coefficients.as_ref()).and_then(|v|v.as_ref().ok()).and_then(|v|match v{
                        crate::heat_balance::surface_manager::ctf_final_coefficients::CtfFinalCoefficientObservation::Coefficients(value)=>Some(&value.num_ctf_terms),_=>None,
                    }).unwrap_or(&storage.num_ctf_terms);
                    return Ok((&context.ctf_time_step, &context.num_histories, terms));
                }
            }
            if let Some(Ok(initial)) = phase.initial.as_ref().map(|v| &v.result) {
                return Ok((
                    &initial.initial_ctf_time_step_hours,
                    &initial.initial_num_histories,
                    &storage.num_ctf_terms,
                ));
            }
        }
        Ok((
            &storage.ctf_time_step,
            &storage.num_histories,
            &storage.num_ctf_terms,
        ))
    }
}
fn validate_layer_ids(
    usage: &crate::heat_balance::selected_heat_balance_input::SelectedConstructionUsage,
    materials: &SelectedMaterialData,
    construction: &Construction,
) -> Result<(), Error> {
    let count =
        usize::try_from(usage.total_layers).map_err(|_| Error::Unavailable("actual TotLayers"))?;
    let points = usage
        .layer_points
        .get(..count)
        .ok_or(Error::Unavailable("actual fixed LayerPoint capacity"))?;
    let ids = points
        .iter()
        .map(|&n| {
            materials
                .materials
                .get(slot(n)?)
                .copied()
                .ok_or(Error::Unavailable("same actual Source MaterialNum"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if ids != construction.effective_layers() {
        return Err(Error::Unavailable(
            "actual Source LayerPoint/physical owner association",
        ));
    }
    Ok(())
}
fn source_layer_inputs(
    usage: &crate::heat_balance::selected_heat_balance_input::SelectedConstructionUsage,
    materials: &SelectedMaterialData,
    model: &TypedModel,
) -> Result<Vec<CtfLayerInput>, Error> {
    let construction = model
        .constructions
        .iter()
        .find(|v| Some(v.id) == usage.construction_id)
        .ok_or(Error::Unavailable("same Source Construction"))?;
    validate_layer_ids(usage, materials, construction)?;
    let mut rows = Vec::new();
    for &id in construction.effective_layers() {
        let material = model
            .materials
            .iter()
            .find(|v| v.id == id)
            .ok_or(Error::Unavailable("sole physical Material owner"))?;
        match (materials.material_groups.get(&id), material.definition) {
            (Some(SourceMaterialGroup::Regular), MaterialDefinition::Regular(_))
            | (Some(SourceMaterialGroup::Regular), MaterialDefinition::NoMass(_))
            | (Some(SourceMaterialGroup::AirGap), MaterialDefinition::AirGap(_)) => {}
            _ => {
                return Err(Error::Unavailable(
                    "actual opaque Material group/physical property view",
                ));
            }
        }
        rows.push(material);
    }
    let mut inputs = material_layer_inputs(&rows).map_err(Error::Loading)?;
    for input in &mut inputs {
        // Construction.cc298 consumes the actual MaterialBase::ROnly written
        // by GetMaterialData, independently of the later ResLayer predicate.
        input.resistance_only = materials
            .resistance_only
            .get(&input.material_id)
            .copied()
            .ok_or(Error::Unavailable("actual Material ROnly owner"))?;
    }
    Ok(inputs)
}
