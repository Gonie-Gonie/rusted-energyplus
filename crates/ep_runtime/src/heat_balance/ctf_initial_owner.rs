//! Construction-owned initial CTF phase; no retry or coefficient driver.

use super::surface_manager::ctf_initial_discretization::{
    CtfInitialDiscretization, CtfInitialDiscretizationScopeError,
    discretize_selected_ctf_1d_initial,
};
use super::surface_manager::ctf_layer_preprocessing::{
    ConstructionCtfLayerPreprocessing, CtfLayerContext, CtfLayerScopeError,
};
use ep_model::{Construction, ConstructionId};
use std::collections::BTreeSet;

/// A source route or selected input-domain stop supplies no initial operands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfInitialUnavailable {
    /// The accepted preprocessing owner cannot represent this source context.
    PreprocessingScope(CtfLayerScopeError),
    /// The genuine unused-construction return precedes layer processing.
    UnusedConstruction,
    /// Loading returned with the shared ErrorsFound owner set.
    PreprocessingErrorReturn,
    /// No actual post-conversion active-prefix owner was reached.
    PostConversionUnavailable,
    /// The literal source dispatch does not enter the massive branch.
    AllResistiveBranch,
    /// An earlier used construction has the exact reversed original ID stack.
    ReverseConstruction {
        /// Actual earlier typed construction identity, without coefficient copying.
        construction_id: ConstructionId,
    },
    /// The accepted pure phase stopped outside its defined input domain.
    InitialScope(CtfInitialDiscretizationScopeError),
}

impl std::fmt::Display for CtfInitialUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "initial CTF construction owner unavailable: {self:?}"
        )
    }
}

impl std::error::Error for CtfInitialUnavailable {}

/// Actual ordered initialization owner, separate from final CTF storage.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfInitialDiscretization {
    /// Original typed construction identity; normalized layers introduce no IDs.
    pub construction_id: ConstructionId,
    /// The actual context supplied to this construction's single preprocessing call.
    pub context: CtfLayerContext,
    /// Actual caller timestep from the compiled model's timestep count, in hours.
    pub time_step_zone_hours: f64,
    /// Reached initial state or explicit unselected/unavailable source route.
    pub result: Result<CtfInitialDiscretization, CtfInitialUnavailable>,
}

/// Initialize only the selected pre-matrix phase from an existing preprocessing owner.
///
/// `previous_constructions` is the actual construction-vector prefix before
/// `construction`; `used_ctf_constructions` contains the real referenced owner IDs.
/// The caller supplies the same context and timestep used for this construction.
/// Original material IDs determine reverse eligibility before the pure phase runs.
/// This function neither repeats preprocessing nor generates reverse coefficients.
#[allow(
    clippy::nonminimal_bool,
    reason = "Preserve pinned Construction.cc 405 source predicate."
)]
pub fn initialize_construction_ctf_initial(
    construction: &Construction,
    previous_constructions: &[Construction],
    used_ctf_constructions: &BTreeSet<u32>,
    preprocessing: &ConstructionCtfLayerPreprocessing,
    context: CtfLayerContext,
    time_step_zone_hours: f64,
) -> ConstructionCtfInitialDiscretization {
    use CtfInitialUnavailable as Unavailable;

    let result = (|| {
        let phases = preprocessing
            .result
            .as_ref()
            .map_err(|error| Unavailable::PreprocessingScope(*error))?;
        if phases.skipped_unused {
            return Err(Unavailable::UnusedConstruction);
        }
        if context.source_sink_present {
            return Err(Unavailable::PreprocessingScope(
                CtfLayerScopeError::InternalSource,
            ));
        }
        if context.solution_dimensions != 1 {
            return Err(Unavailable::PreprocessingScope(
                CtfLayerScopeError::UnsupportedDimensions(context.solution_dimensions),
            ));
        }
        if phases.errors_found {
            return Err(Unavailable::PreprocessingErrorReturn);
        }
        let converted = phases
            .after_conversion
            .as_ref()
            .ok_or(Unavailable::PostConversionUnavailable)?;
        // Exact Construction.cc405 predicate; the other route belongs to CTF02.
        if !(converted.active.layers.len() > converted.active.num_res_layers) {
            return Err(Unavailable::AllResistiveBranch);
        }
        // Construction.cc417-455 uses original LayerPoint IDs and prior use,
        // never merged layers, coefficient values, or numerical similarity.
        let layers = construction.effective_layers();
        for previous in previous_constructions {
            let other = previous.effective_layers();
            if layers.len() == other.len()
                && layers.iter().eq(other.iter().rev())
                && used_ctf_constructions.contains(&previous.id.0)
            {
                return Err(Unavailable::ReverseConstruction {
                    construction_id: previous.id,
                });
            }
        }
        discretize_selected_ctf_1d_initial(converted, time_step_zone_hours)
            .map_err(Unavailable::InitialScope)
    })();
    ConstructionCtfInitialDiscretization {
        construction_id: construction.id,
        context,
        time_step_zone_hours,
        result,
    }
}

#[cfg(test)]
#[path = "ctf_initial_owner_tests.rs"]
mod tests;
