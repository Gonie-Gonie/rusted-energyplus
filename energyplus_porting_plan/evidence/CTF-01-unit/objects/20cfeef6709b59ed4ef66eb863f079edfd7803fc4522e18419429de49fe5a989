//! Construction.cc 201-401 layer preprocessing; no CTF coefficient solution.

use ep_model::{ConstructionId, Material, MaterialDefinition, MaterialId};

const CFC: f64 = 4.1868;
const CFL: f64 = 0.3048;
const CFM: f64 = 0.45359237;
const CFA: f64 = CFL * CFL;
const CFT: f64 = 5.0 / 9.0;
const CFV: f64 = CFA * CFL;
const CFE: f64 = CFC * CFM * CFT / 3.6;
const CFD: f64 = CFM / CFV;
const CFK: f64 = CFE / (CFL * CFT);
const CFU: f64 = CFK / CFL;

/// Actual material members, in original outside-to-inside layer order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfLayerInput {
    /// Original material identity; merger never rewrites the construction stack.
    pub material_id: MaterialId,
    /// MaterialBase::Thickness, in metres.
    pub thickness: f64,
    /// MaterialBase::Conductivity, in W/m-K.
    pub conductivity: f64,
    /// MaterialBase::Density, in kg/m3.
    pub density: f64,
    /// MaterialBase::SpecHeat, copied without an extra factor of 1000.
    pub specific_heat: f64,
    /// Stored loader-owned MaterialBase::Resistance, in m2-K/W.
    pub resistance: f64,
    /// MaterialBase::ROnly; distinct from the calculated ResLayer flag.
    pub resistance_only: bool,
}

/// Caller state for the selected normal one-dimensional/no-source construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CtfLayerContext {
    /// Actual construction use by a selected CTF opaque surface.
    pub is_used_ctf: bool,
    /// Incoming shared flag, which remains sticky across ordered constructions.
    pub errors_found: bool,
    /// Actual internal-source declaration; true is outside this selected port.
    pub source_sink_present: bool,
    /// Actual selected construction dimension; only one is admitted here.
    pub solution_dimensions: i32,
}

/// Scope failure, kept separate from a genuine source input error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtfLayerScopeError {
    /// The supplied family has no selected Regular/NoMass owner mapping.
    UnsupportedMaterial(MaterialId),
    /// Source/sink checks and their state mutations are outside CTF-01's scope.
    InternalSource,
    /// The perpendicular two-dimensional branch is outside the selected scope.
    UnsupportedDimensions(i32),
    /// The source fixed-capacity active-prefix scope is one through eleven.
    LayerCount(usize),
    /// The source's fatal adjacent-layer invariant failed; no fallback is used.
    AdjacentResistiveInvariant,
}

/// A source error encountered during loading; loading still visits every layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtfLayerIssue {
    /// Original one-based material layer has thickness greater than three metres.
    MaterialTooThick(usize),
    /// Original one-based classified resistive layer has stored R below 0.001.
    ResistanceBelowMinimum(usize),
}

/// One local active layer; its numerical fields change units between checkpoints.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfNormalizedLayer {
    /// Local dl, initially metres and finally feet.
    pub dl: f64,
    /// Local rk, initially SI and finally English conductivity units.
    pub rk: f64,
    /// Local rho, initially SI and finally lb/ft3.
    pub rho: f64,
    /// Local cp, copied as supplied before the exact source conversion.
    pub cp: f64,
    /// Local lr, initially SI and finally English resistance units.
    pub lr: f64,
    /// Calculated ResLayer; this is not the material's ROnly flag.
    pub res_layer: bool,
}

/// Active-prefix checkpoint, without unused fixed-array tail claims.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfLayerCheckpoint {
    /// Actual local active layers; their count may shrink after merger.
    pub layers: Vec<CtfNormalizedLayer>,
    /// Actual NumResLayers, updated after each merger.
    pub num_res_layers: usize,
}

/// Post-conversion fields reached after Construction.cc 401.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfConvertedLayers {
    /// Converted active-prefix arrays.
    pub active: CtfLayerCheckpoint,
    /// Actual one-dimensional dyn value.
    pub dyn_spacing: f64,
    /// Ordered English-unit resistance sum rs.
    pub total_resistance: f64,
    /// Unguarded source cnd = 1.0 / rs, including IEEE nonfinite outcomes.
    pub conductance: f64,
}

/// Reached preprocessing checkpoints; absent later phases are not fabricated.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfLayerPreprocessing {
    /// True only for the source's early !IsUsedCTF return.
    pub skipped_unused: bool,
    /// Outgoing sticky shared flag from the loading phase.
    pub errors_found: bool,
    /// Source-defined loading issues, in original material order.
    pub issues: Vec<CtfLayerIssue>,
    /// Present only when the complete original loading loop was reached.
    pub after_load: Option<CtfLayerCheckpoint>,
    /// Present only after the ErrorsFound early-return test passed.
    pub after_merge: Option<CtfLayerCheckpoint>,
    /// Present only after conversion, dyn and ordered rs/cnd assignment.
    pub after_conversion: Option<CtfConvertedLayers>,
}

/// Construction-owned result kept in actual initializer order.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfLayerPreprocessing {
    /// Original typed construction identity.
    pub construction_id: ConstructionId,
    /// Actual surface-derived selected use flag.
    pub is_used_ctf: bool,
    /// Actual Rust preprocessing result or explicit scope gap.
    pub result: Result<CtfLayerPreprocessing, CtfLayerScopeError>,
}

/// Copy selected actual compiled material owners; no resistance accessor is used.
pub fn material_layer_inputs(
    materials: &[&Material],
) -> Result<Vec<CtfLayerInput>, CtfLayerScopeError> {
    materials
        .iter()
        .map(|material| {
            let (thickness, conductivity, density, specific_heat, resistance, resistance_only) =
                match material.definition {
                    MaterialDefinition::Regular(value) => (
                        value.thickness_m,
                        value.conductivity_w_per_m_k,
                        value.density_kg_per_m3,
                        value.specific_heat_j_per_kg_k,
                        value.thermal_resistance_m2_k_per_w,
                        false,
                    ),
                    // MaterialBase defaults are genuine zero members for NoMass;
                    // its actual loader stores Resistance and sets ROnly=true.
                    MaterialDefinition::NoMass(value) => (
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        value.thermal_resistance_m2_k_per_w,
                        true,
                    ),
                    _ => return Err(CtfLayerScopeError::UnsupportedMaterial(material.id)),
                };
            Ok(CtfLayerInput {
                material_id: material.id,
                thickness,
                conductivity,
                density,
                specific_heat,
                resistance,
                resistance_only,
            })
        })
        .collect()
}

/// Execute only the selected original layer preprocessing, in original order.
///
/// This does not generate coefficients, run the later solver, or assert anything
/// about later-solver ErrorsFound/DoCTFErrorReport mutations.
pub fn preprocess_ctf_layers(
    inputs: &[CtfLayerInput],
    context: CtfLayerContext,
) -> Result<CtfLayerPreprocessing, CtfLayerScopeError> {
    let mut result = CtfLayerPreprocessing {
        skipped_unused: !context.is_used_ctf,
        errors_found: context.errors_found,
        issues: Vec::new(),
        after_load: None,
        after_merge: None,
        after_conversion: None,
    };
    if !context.is_used_ctf {
        return Ok(result);
    }
    if context.source_sink_present {
        return Err(CtfLayerScopeError::InternalSource);
    }
    if context.solution_dimensions != 1 {
        return Err(CtfLayerScopeError::UnsupportedDimensions(
            context.solution_dimensions,
        ));
    }
    if inputs.is_empty() || inputs.len() > 11 {
        return Err(CtfLayerScopeError::LayerCount(inputs.len()));
    }
    let mut layers = Vec::with_capacity(inputs.len());
    let mut num_res_layers = 0;
    for (index, input) in inputs.iter().enumerate() {
        let mut layer = CtfNormalizedLayer {
            dl: input.thickness,
            rk: input.conductivity,
            rho: input.density,
            cp: input.specific_heat,
            lr: 0.0,
            res_layer: false,
        };
        if input.thickness > 3.0 {
            result
                .issues
                .push(CtfLayerIssue::MaterialTooThick(index + 1));
            result.errors_found = true;
        }
        if layer.rk <= 1.0e-6 {
            layer.res_layer = true;
        } else {
            layer.lr = layer.dl / layer.rk;
            layer.res_layer = (layer.dl * (layer.rho * layer.cp / layer.rk).sqrt()) < 1.0e-6;
        }
        if layer.res_layer {
            num_res_layers += 1;
            layer.lr = input.resistance;
            if layer.lr < 1.0e-3 {
                result
                    .issues
                    .push(CtfLayerIssue::ResistanceBelowMinimum(index + 1));
                result.errors_found = true;
            } else if index == 0 || index + 1 == inputs.len() || !input.resistance_only {
                layer.cp = 1.007;
                layer.rho = 1.1614;
                layer.rk = 0.0263;
                layer.dl = layer.rk * layer.lr;
            } else {
                layer.cp = 0.0;
                layer.rho = 0.0;
                layer.rk = 1.0;
                layer.dl = layer.lr;
            }
        }
        layers.push(layer);
    }
    result.after_load = Some(CtfLayerCheckpoint {
        layers: layers.clone(),
        num_res_layers,
    });
    if result.errors_found {
        return Ok(result);
    }
    if layers.len() > 3 && num_res_layers > 1 {
        let mut adjacent = Vec::new();
        for index in 1..layers.len() - 2 {
            if layers[index].res_layer && layers[index + 1].res_layer {
                // Native one-based Layer + 1 - NumAdjResLayers, mapped to zero.
                adjacent.push(index - adjacent.len());
            }
        }
        for index in adjacent {
            if !layers[index].res_layer || !layers[index + 1].res_layer {
                return Err(CtfLayerScopeError::AdjacentResistiveInvariant);
            }
            layers[index].cp = 0.0;
            layers[index].rho = 0.0;
            layers[index].rk = 1.0;
            let next_lr = layers[index + 1].lr;
            layers[index].lr += next_lr;
            layers[index].dl = layers[index].lr;
            num_res_layers -= 1;
            // Shift only the active prefix. No original layer IDs are mutated,
            // and no unused numeric or ResLayer tail is part of this contract.
            layers.remove(index + 1);
        }
    }
    result.after_merge = Some(CtfLayerCheckpoint {
        layers: layers.clone(),
        num_res_layers,
    });
    for layer in &mut layers {
        layer.lr *= CFU;
        layer.dl /= CFL;
        layer.rk /= CFK;
        layer.rho /= CFD;
        layer.cp /= CFC * 1000.0;
    }
    let mut rs = 0.0;
    for layer in &layers {
        rs += layer.lr;
    }
    let cnd = 1.0 / rs;
    result.after_conversion = Some(CtfConvertedLayers {
        active: CtfLayerCheckpoint {
            layers,
            num_res_layers,
        },
        dyn_spacing: 0.0,
        total_resistance: rs,
        conductance: cnd,
    });
    Ok(result)
}

#[cfg(test)]
mod tests;
