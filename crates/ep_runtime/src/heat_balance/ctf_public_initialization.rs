//! Selected ordered public stores and Init's postreturn aggregate prefix.
//!
//! Reporting, the final fatal backend and surface activation are not invoked.

use super::ctf_initial_owner::{ConstructionCtfInitialDiscretization, CtfInitialUnavailable};
use super::ctf_retry_driver::{CtfRetryDriverOutcome, CtfRetryDriverStop};
use super::ctf_retry_initialization::{
    ConstructionCtfRetryInitialization, CtfRetryInitializationUnavailable,
};
use super::surface_manager::ctf_all_resistive::{
    ConstructionCtfAllResistive, CtfAllResistiveUnavailable,
};
use super::surface_manager::ctf_caller_stability::{
    CtfCallerSharedFlags, CtfCallerStabilityOutcome,
};
use super::surface_manager::ctf_final_coefficients::CtfFinalCoefficientObservation;
use super::surface_manager::ctf_layer_preprocessing::{
    ConstructionCtfLayerPreprocessing, CtfLayerScopeError,
};
use super::surface_manager::ctf_public_storage::{
    CtfPublicCoefficientStorage, CtfPublicStorageUnavailable,
    copy_all_resistive_public_coefficients, copy_reversed_public_coefficients,
    store_selected_massive_public_coefficients,
};
use ep_model::{Construction, ConstructionId, MaterialId};

/// A retained source route with actual fixed public storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfPublicReturnRoute {
    /// Construction.cc163 returns before assigning the used timestep.
    UnusedReset,
    /// Construction.cc313 returns after assigning TimeStepZone.
    LoadingErrorReset,
    /// Reuse the existing actually generated CTF02 owner.
    AllResistive,
    /// The initial owner selected this actual earlier original-ID provider.
    Reversed {
        /// Actual earlier original typed construction provider.
        construction_id: ConstructionId,
        /// Actual earlier provider position in declared construction order.
        declared_index: usize,
    },
    /// Actual last private coefficients precede a converged caller exit.
    MassiveConverged,
    /// Severe caller exit still reaches common public stores.
    MassiveSevereBreak,
}

/// Storage returned by this selected Rust construction suffix, not Native success.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfPublicConstructionReturn {
    /// Actual selected route; reset returns are distinct from generation.
    pub route: CtfPublicReturnRoute,
    /// All twelve fixed19 owners and true metadata, including reset tails.
    pub storage: CtfPublicCoefficientStorage,
}

/// Selected-prefix stops are never substituted for a Native process outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfPublicInitStop {
    /// The actual09 caller fatal boundary precedes this construction's stores.
    Fatal {
        /// Actual construction whose selected caller reached the fatal boundary.
        construction_id: ConstructionId,
    },
    /// The fixed Rust storage cannot represent an actual inclusive source count.
    Unrepresentable {
        /// Actual construction whose public storage exceeds the selected capacity.
        construction_id: ConstructionId,
        /// Actual inclusive history count, retained without clipping.
        num_ctf_terms: i32,
    },
    /// A scope/custody failure prevents further selected Init completion claims.
    Scope {
        /// Actual construction at the selected scope or custody boundary.
        construction_id: ConstructionId,
    },
}

/// Missing owners and source outcomes are deliberately separate cases.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfPublicInitializationUnavailable {
    /// This declared construction family has no selected whole-call owner here.
    ExcludedConstruction,
    /// Declared vector order or original construction custody is inconsistent.
    Association,
    /// The selected construction was not invoked after an earlier selected stop.
    PriorSelectedStop(CtfPublicInitStop),
    /// The genuine09 initializer itself was not invoked after an earlier stop.
    RetryInitialization(CtfRetryInitializationUnavailable),
    /// No initial route owner exists for this actually supplied construction.
    MissingInitialOwner,
    /// Actual preprocessing stopped outside its selected domain.
    Preprocessing(CtfLayerScopeError),
    /// The actual initial owner returned a selected-domain/route failure.
    Initial(CtfInitialUnavailable),
    /// The accepted CTF02 producer did not return its selected public owner.
    AllResistive(CtfAllResistiveUnavailable),
    /// The selected prior original-ID provider lacks actual public storage.
    ReverseProvider(ConstructionId),
    /// No actual converted current conductance exists.
    MissingConvertedOwner,
    /// The actual retry driver stopped outside the represented suffix.
    Driver(CtfRetryDriverStop),
    /// The actual caller fatal boundary bypasses stores and aggregate updates.
    FatalNoCtfs,
    /// Actual last private/caller/postcheck custody is missing or mismatched.
    MissingTerminalOwner,
    /// Storage scope/shape/representability error, not a Native capacity guard.
    Storage(CtfPublicStorageUnavailable),
}

/// One actual declared construction row; later selected absence is explicit.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfPublicInitialization {
    /// Original typed identity and actual vector index; neither is sorted.
    pub construction_id: ConstructionId,
    /// Actual declaration position, also used for reverse-provider custody.
    pub declared_index: usize,
    /// Original outside-to-inside material IDs, never normalized layers.
    pub original_material_ids: Vec<MaterialId>,
    /// The actual selected use owner supplied to preprocessing.
    pub is_used_ctf: bool,
    /// Actual Rust public initializer invocation, including an error/fatal return.
    /// This does not assert that Native common stores or a whole call were reached.
    pub public_initializer_invoked: bool,
    /// Actual retry flags; None when this suffix was not invoked.
    pub retry_flags_on_entry: Option<CtfCallerSharedFlags>,
    /// Actual retry return flags; no synthetic Native diagnostic backend.
    pub retry_flags_on_return: Option<CtfCallerSharedFlags>,
    /// Genuine stored/reset owner or honest unavailable boundary.
    pub result: Result<CtfPublicConstructionReturn, CtfPublicInitializationUnavailable>,
}

/// Source-owned aggregate values updated only after a selected suffix returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CtfPublicInitAggregate {
    /// Actual prefix value of HeatBalanceManager.cc6159-6160.
    pub simple_ctf_only: bool,
    /// Actual prefix maximum at6162-6163, including reset-return term counts.
    pub max_ctf_terms: i32,
    /// Number of selected returned suffixes, not assumed Native whole returns.
    pub returned_constructions: usize,
}

/// Ordinary ordered cache ingress; no report/final-fatal completion is inferred.
#[derive(Clone, Debug, PartialEq)]
pub struct OrderedCtfPublicInitialization {
    /// Every declared construction row, including excluded and prior-stop rows.
    pub constructions: Vec<ConstructionCtfPublicInitialization>,
    /// Genuine selected aggregate prefix; preserved at an unavailable boundary.
    pub aggregate_after_returned_calls: CtfPublicInitAggregate,
    /// Actual selected fatal, unrepresentability, or scope boundary.
    pub selected_stop: Option<CtfPublicInitStop>,
    /// True only after every declared row has a selected returned suffix.
    /// This excludes the later AnyInternalHeatSource/report/fatal stages.
    pub selected_declared_loop_completed: bool,
}

impl OrderedCtfPublicInitialization {
    /// Supply actual incoming aggregate owners. Fresh ordinary initialization
    /// uses DataHeatBalance.hh defaults true/0; repeated Init must retain its owners.
    #[must_use]
    pub fn begin(simple_ctf_only: bool, max_ctf_terms: i32) -> Self {
        Self {
            constructions: Vec::new(),
            aggregate_after_returned_calls: CtfPublicInitAggregate {
                simple_ctf_only,
                max_ctf_terms,
                returned_constructions: 0,
            },
            selected_stop: None,
            selected_declared_loop_completed: false,
        }
    }

    fn append(&mut self, mut row: ConstructionCtfPublicInitialization) {
        if row.declared_index != self.constructions.len() {
            row.result = Err(CtfPublicInitializationUnavailable::Association);
        }
        if let Ok(returned) = &row.result {
            // Literal Init postreturn order; never applied to a missing/fatal owner.
            if returned.storage.num_histories > 1 {
                self.aggregate_after_returned_calls.simple_ctf_only = false;
            }
            if returned.storage.num_ctf_terms > self.aggregate_after_returned_calls.max_ctf_terms {
                self.aggregate_after_returned_calls.max_ctf_terms = returned.storage.num_ctf_terms;
            }
            self.aggregate_after_returned_calls.returned_constructions += 1;
        } else if self.selected_stop.is_none() {
            self.selected_stop = Some(match &row.result {
                Err(CtfPublicInitializationUnavailable::FatalNoCtfs) => CtfPublicInitStop::Fatal {
                    construction_id: row.construction_id,
                },
                Err(CtfPublicInitializationUnavailable::Storage(
                    CtfPublicStorageUnavailable::PublicCapacity(num_ctf_terms),
                )) => CtfPublicInitStop::Unrepresentable {
                    construction_id: row.construction_id,
                    num_ctf_terms: *num_ctf_terms,
                },
                Err(CtfPublicInitializationUnavailable::RetryInitialization(
                    CtfRetryInitializationUnavailable::PriorFatal { construction_id },
                )) => CtfPublicInitStop::Fatal {
                    construction_id: *construction_id,
                },
                _ => CtfPublicInitStop::Scope {
                    construction_id: row.construction_id,
                },
            });
        }
        self.constructions.push(row);
    }

    /// Preserve a family lacking a selected whole-call implementation. This
    /// stops only the new public/aggregate prefix, not the legacy09 cache work.
    pub fn record_excluded(
        &mut self,
        declared_index: usize,
        construction: &Construction,
        is_used_ctf: bool,
    ) {
        let result = Err(self.selected_stop.map_or(
            CtfPublicInitializationUnavailable::ExcludedConstruction,
            CtfPublicInitializationUnavailable::PriorSelectedStop,
        ));
        self.append(ConstructionCtfPublicInitialization {
            construction_id: construction.id,
            declared_index,
            original_material_ids: construction.effective_layers().to_vec(),
            is_used_ctf,
            public_initializer_invoked: false,
            retry_flags_on_entry: None,
            retry_flags_on_return: None,
            result,
        });
    }

    /// Borrow the real completed09 chain once, store its selected suffix once,
    /// then immediately update this actual Init-prefix owner. No method replay.
    pub fn record_ordinary(
        &mut self,
        declared_index: usize,
        construction: &Construction,
        preprocessing: &ConstructionCtfLayerPreprocessing,
        initial: Option<&ConstructionCtfInitialDiscretization>,
        all_resistive: &ConstructionCtfAllResistive,
        retry: &ConstructionCtfRetryInitialization,
    ) {
        let invoked = self.selected_stop.is_none();
        let result = if let Some(stop) = self.selected_stop {
            Err(CtfPublicInitializationUnavailable::PriorSelectedStop(stop))
        } else {
            initialize_public_suffix(
                construction,
                preprocessing,
                initial,
                all_resistive,
                retry,
                &self.constructions,
            )
        };
        self.append(ConstructionCtfPublicInitialization {
            construction_id: construction.id,
            declared_index,
            original_material_ids: construction.effective_layers().to_vec(),
            is_used_ctf: preprocessing.is_used_ctf,
            public_initializer_invoked: invoked,
            retry_flags_on_entry: invoked.then_some(retry.flags_on_entry),
            retry_flags_on_return: invoked.then_some(retry.flags_on_return),
            result,
        });
    }

    /// Complete only the selected declared-call prefix. AnyInternalHeatSource,
    /// ScanForReports, reports and final fatal backend remain uninvoked here.
    pub fn finish_declared_loop(&mut self, actual_declared_count: usize) {
        self.selected_declared_loop_completed =
            self.selected_stop.is_none() && self.constructions.len() == actual_declared_count;
    }
}

fn initialize_public_suffix(
    construction: &Construction,
    preprocessing: &ConstructionCtfLayerPreprocessing,
    initial: Option<&ConstructionCtfInitialDiscretization>,
    all_resistive: &ConstructionCtfAllResistive,
    retry: &ConstructionCtfRetryInitialization,
    previous: &[ConstructionCtfPublicInitialization],
) -> Result<CtfPublicConstructionReturn, CtfPublicInitializationUnavailable> {
    use CtfPublicInitializationUnavailable as Error;
    use CtfPublicReturnRoute as Route;
    if [
        preprocessing.construction_id,
        all_resistive.construction_id,
        retry.construction_id,
    ]
    .iter()
    .any(|id| *id != construction.id)
    {
        return Err(Error::Association);
    }
    let driver = retry
        .result
        .as_ref()
        .map_err(|reason| Error::RetryInitialization(*reason))?;
    let initial = initial.ok_or(Error::MissingInitialOwner)?;
    if initial.construction_id != construction.id
        || initial.context.is_used_ctf != preprocessing.is_used_ctf
        || driver.construction_id != construction.id
    {
        return Err(Error::Association);
    }
    if let Err(reason) = &initial.result {
        let actual_route = CtfRetryDriverOutcome::ScopeStopped(CtfRetryDriverStop::FirstAssembly(
            super::ctf_first_assembly_owner::CtfFirstAssemblyUnavailable::InitialUnavailable(
                *reason,
            ),
        ));
        if driver.outcome != actual_route || !driver.attempts.is_empty() {
            return Err(Error::Association);
        }
    }
    let phases = preprocessing
        .result
        .as_ref()
        .map_err(|reason| Error::Preprocessing(*reason))?;
    if phases.skipped_unused {
        if initial.result != Err(CtfInitialUnavailable::UnusedConstruction) {
            return Err(Error::Association);
        }
        return Ok(CtfPublicConstructionReturn {
            route: Route::UnusedReset,
            storage: CtfPublicCoefficientStorage::reset(),
        });
    }
    if phases.errors_found {
        if initial.result != Err(CtfInitialUnavailable::PreprocessingErrorReturn) {
            return Err(Error::Association);
        }
        let mut storage = CtfPublicCoefficientStorage::reset();
        storage.ctf_time_step = initial.time_step_zone_hours;
        return Ok(CtfPublicConstructionReturn {
            route: Route::LoadingErrorReset,
            storage,
        });
    }
    let converted = phases
        .after_conversion
        .as_ref()
        .ok_or(Error::MissingConvertedOwner)?;
    let (route, storage) = match &initial.result {
        Err(CtfInitialUnavailable::AllResistiveBranch) => {
            let actual = all_resistive
                .result
                .as_ref()
                .map_err(|reason| Error::AllResistive(*reason))?;
            if actual.ctf_time_step.to_bits() != initial.time_step_zone_hours.to_bits() {
                return Err(Error::Association);
            }
            (
                Route::AllResistive,
                copy_all_resistive_public_coefficients(actual),
            )
        }
        Err(CtfInitialUnavailable::ReverseConstruction { construction_id }) => {
            // Provider precedence was actually selected by the03 original-ID loop.
            // Join that same earlier owner; do not rerun selection from numeric layers.
            let provider = previous
                .iter()
                .find(|row| row.construction_id == *construction_id)
                .filter(|row| {
                    row.is_used_ctf
                        && construction
                            .effective_layers()
                            .iter()
                            .eq(row.original_material_ids.iter().rev())
                })
                .ok_or(Error::ReverseProvider(*construction_id))?;
            let actual = provider
                .result
                .as_ref()
                .map_err(|_| Error::ReverseProvider(*construction_id))?;
            (
                Route::Reversed {
                    construction_id: *construction_id,
                    declared_index: provider.declared_index,
                },
                copy_reversed_public_coefficients(&actual.storage, converted.conductance),
            )
        }
        Err(reason) => return Err(Error::Initial(*reason)),
        Ok(_) => {
            let first_context = driver.initial_context.ok_or(Error::MissingTerminalOwner)?;
            if first_context.construction_id != construction.id
                || first_context.attempt_ordinal != 1
                || first_context.time_step_zone.to_bits() != initial.time_step_zone_hours.to_bits()
            {
                return Err(Error::MissingTerminalOwner);
            }
            let route = match driver.outcome {
                CtfRetryDriverOutcome::ConvergedLoopExit => Route::MassiveConverged,
                CtfRetryDriverOutcome::SevenHourSevereBreak => Route::MassiveSevereBreak,
                CtfRetryDriverOutcome::FatalNoCtfs => return Err(Error::FatalNoCtfs),
                CtfRetryDriverOutcome::ScopeStopped(reason) => return Err(Error::Driver(reason)),
            };
            let last = driver.attempts.last().ok_or(Error::MissingTerminalOwner)?;
            let Some(Ok(CtfFinalCoefficientObservation::Coefficients(private))) =
                &last.final_coefficients
            else {
                return Err(Error::MissingTerminalOwner);
            };
            let caller = last
                .caller
                .as_ref()
                .and_then(|value| value.as_ref().ok())
                .ok_or(Error::MissingTerminalOwner)?;
            let postcheck = driver
                .postcheck_context
                .ok_or(Error::MissingTerminalOwner)?;
            let expected_outcome = match route {
                Route::MassiveConverged => CtfCallerStabilityOutcome::ConvergedLoopExit,
                _ => CtfCallerStabilityOutcome::SevenHourSevereBreak,
            };
            if caller.outcome != expected_outcome
                || !same_context_bits(caller.postcheck_context, postcheck)
                || !same_context_bits(caller.input_context, private.context)
                || !same_context_bits(last.assembly.context, private.context)
                || caller.num_ctf_terms != private.num_ctf_terms
            {
                return Err(Error::MissingTerminalOwner);
            }
            (
                route,
                store_selected_massive_public_coefficients(
                    private,
                    postcheck,
                    converted.conductance,
                ),
            )
        }
    };
    Ok(CtfPublicConstructionReturn {
        route,
        storage: storage.map_err(Error::Storage)?,
    })
}

fn same_context_bits(
    left: super::surface_manager::ctf_state_space_assembly::CtfAssemblyContext,
    right: super::surface_manager::ctf_state_space_assembly::CtfAssemblyContext,
) -> bool {
    left.construction_id == right.construction_id
        && left.route == right.route
        && left.solution_dimensions == right.solution_dimensions
        && left.source_sink_present == right.source_sink_present
        && left.node_source == right.node_source
        && left.node_user_temp == right.node_user_temp
        && left.attempt_ordinal == right.attempt_ordinal
        && left.num_histories == right.num_histories
        && left.time_step_zone.to_bits() == right.time_step_zone.to_bits()
        && left.ctf_time_step.to_bits() == right.ctf_time_step.to_bits()
}

#[cfg(test)]
#[path = "ctf_public_initialization_tests.rs"]
mod tests;
