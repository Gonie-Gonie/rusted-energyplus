//! Construction.cc 985-1077: selected all-resistive coefficient storage.

use super::ctf_layer_preprocessing::{CFU, CtfLayerPreprocessing, CtfLayerScopeError};
use ep_model::ConstructionId;

/// Construction.hh MaxCTFTerms, including the coefficient at index zero.
pub const CTF_COEFFICIENT_SLOTS: usize = 19;

/// Public coefficient storage reached only through the all-resistive branch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfAllResistiveCoefficients {
    /// Actual source CTFOutside storage, in W/m2-K.
    pub outside: [f64; CTF_COEFFICIENT_SLOTS],
    /// Actual source CTFCross storage, in W/m2-K.
    pub cross: [f64; CTF_COEFFICIENT_SLOTS],
    /// Actual source CTFInside storage, in W/m2-K.
    pub inside: [f64; CTF_COEFFICIENT_SLOTS],
    /// Actual source CTFFlux storage, dimensionless.
    pub flux: [f64; CTF_COEFFICIENT_SLOTS],
    /// Actual supplied zone timestep copied by the source, in hours.
    pub ctf_time_step: f64,
    /// Source NumHistories, including one for this steady branch.
    pub num_histories: usize,
    /// Source history-term count; index zero is not included in this count.
    pub num_ctf_terms: usize,
    /// Source UValue = assigned English cnd times CFU, in W/m2-K.
    pub u_value: f64,
}

impl CtfAllResistiveCoefficients {
    fn reset() -> Self {
        Self {
            outside: [0.0; CTF_COEFFICIENT_SLOTS],
            cross: [0.0; CTF_COEFFICIENT_SLOTS],
            inside: [0.0; CTF_COEFFICIENT_SLOTS],
            flux: [0.0; CTF_COEFFICIENT_SLOTS],
            ctf_time_step: 0.0,
            num_histories: 0,
            num_ctf_terms: 0,
            u_value: 0.0,
        }
    }
}

/// An unavailable or unselected branch never supplies fabricated coefficients.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfAllResistiveUnavailable {
    /// The accepted layer interface cannot represent this source input family.
    PreprocessingScope(CtfLayerScopeError),
    /// The genuine source returned at !IsUsedCTF before layer loading.
    UnusedConstruction,
    /// The genuine loading-phase shared ErrorsFound return was reached.
    PreprocessingErrorReturn,
    /// The converted prefix/cnd owner was not reached or is unavailable.
    PostConversionUnavailable,
    /// Native's LayersInConstruct > NumResLayers selects later mass algorithms.
    MassiveBranchNotSelected,
}

/// Coefficients retained with their actual typed construction identity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstructionCtfAllResistive {
    /// Original typed construction identity; no normalized layer IDs replace it.
    pub construction_id: ConstructionId,
    /// Available only for the selected, reached all-resistive source branch.
    pub result: Result<CtfAllResistiveCoefficients, CtfAllResistiveUnavailable>,
}

/// Consume the assigned CTF-01 cnd and the caller's actual zone timestep.
///
/// This neither sums SI resistance nor reruns layer classification. It preserves
/// Native's literal dispatch predicate and applies no area/R/finite-value guard.
/// Source/sink and dimension exclusions are carried by preprocessing scope.
pub fn generate_all_resistive_ctf(
    preprocessing: &CtfLayerPreprocessing,
    time_step_zone_hours: f64,
) -> Result<CtfAllResistiveCoefficients, CtfAllResistiveUnavailable> {
    if preprocessing.skipped_unused {
        return Err(CtfAllResistiveUnavailable::UnusedConstruction);
    }
    if preprocessing.errors_found {
        return Err(CtfAllResistiveUnavailable::PreprocessingErrorReturn);
    }
    let converted = preprocessing
        .after_conversion
        .as_ref()
        .ok_or(CtfAllResistiveUnavailable::PostConversionUnavailable)?;
    // Exact Construction.cc405: the else branch handles all resistive layers.
    if converted.active.layers.len() > converted.active.num_res_layers {
        return Err(CtfAllResistiveUnavailable::MassiveBranchNotSelected);
    }

    let mut storage = CtfAllResistiveCoefficients::reset();
    storage.ctf_time_step = time_step_zone_hours;
    storage.num_histories = 1;
    storage.num_ctf_terms = 1;
    let cnd = converted.conductance;
    let mut s0 = [[0.0; 2]; 2];
    s0[0][0] = cnd;
    s0[1][0] = -cnd;
    s0[0][1] = cnd;
    s0[1][1] = -cnd;

    // Source allocates one history term and fills both e and s with +0.0.
    let e = [0.0; 1];
    let s = [[[0.0; 1]; 2]; 2];
    storage.outside[0] = s0[0][0] * CFU;
    storage.cross[0] = s0[0][1] * CFU;
    storage.inside[0] = -s0[1][1] * CFU;
    for history in 1..=storage.num_ctf_terms {
        storage.outside[history] = s[0][0][history - 1] * CFU;
        storage.cross[history] = s[0][1][history - 1] * CFU;
        storage.inside[history] = -s[1][1][history - 1] * CFU;
        storage.flux[history] = -e[history - 1];
    }
    storage.u_value = cnd * CFU;
    Ok(storage)
}

#[cfg(test)]
#[path = "ctf_all_resistive_tests.rs"]
mod tests;
