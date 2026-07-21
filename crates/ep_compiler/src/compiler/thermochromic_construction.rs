use super::{Compiler, DiagnosticSeverity, construction_layer_field};
use ep_model::{
    ConstructionThermochromicChild, ConstructionThermochromicSeries, MaterialDefinition,
    NormalizedName, ThermochromicConstructionChildId, TypedModel,
};

const FIXED_OUTPUT_THRESHOLD: f64 = f64::from_bits(0x3fb9_9999_9999_9999);
const F64_MAX_DIGITS_10: i32 = 17;

impl Compiler<'_> {
    /// Projects the source `CreateTCConstructions` child series without consuming global
    /// construction identities. Both derived arenas publish together only after a valid pass.
    pub(super) fn create_thermochromic_construction_projections(&mut self, model: &mut TypedModel) {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        let diagnostics_before_pass = self.diagnostics.len();
        let mut masters = model
            .constructions
            .iter()
            .filter(|construction| construction.thermochromic_master.is_some())
            .cloned()
            .collect::<Vec<_>>();
        masters.sort_by_key(|construction| construction.id);

        let mut series = Vec::with_capacity(masters.len());
        let mut children = Vec::new();
        for construction in masters {
            let Some(metadata) = construction.thermochromic_master else {
                continue;
            };
            let layer_index = metadata.layer_index as usize;
            let field = construction_layer_field(layer_index);
            let Some(parent_material) = model.materials.get(metadata.parent_material.0 as usize)
            else {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its thermochromic parent material is unavailable",
                );
                continue;
            };
            let MaterialDefinition::WindowGlazingThermochromicGroup(group) =
                parent_material.definition
            else {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its retained parent material is not a thermochromic glazing group",
                );
                continue;
            };
            let Some(states) = model.window_glazing_thermochromic_states(group) else {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its thermochromic glazing state range is unavailable",
                );
                continue;
            };
            let Some(first_state) = states.first() else {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its thermochromic glazing state range is empty",
                );
                continue;
            };
            if construction.layers.is_empty() {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its effective master layer stack is empty",
                );
                continue;
            }
            if construction.layers.get(layer_index) != Some(&first_state.glazing_material) {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its effective master layer does not contain the first thermochromic state",
                );
                continue;
            }
            if states
                .iter()
                .any(|state| !state.optical_data_temperature_c.is_finite())
            {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its thermochromic glazing state temperature is not finite",
                );
                continue;
            }
            if states.iter().any(|state| {
                !model
                    .materials
                    .get(state.glazing_material.0 as usize)
                    .is_some_and(|material| {
                        matches!(
                            material.definition,
                            MaterialDefinition::WindowGlazingSpectralAverage(_)
                                | MaterialDefinition::WindowGlazingRefractionExtinction(_)
                        )
                    })
            }) {
                self.invalid_thermochromic_projection(
                    &construction.name.0,
                    &field,
                    "its state range contains an unavailable or non-glazing material",
                );
                continue;
            }

            let Some(first_child) =
                self.checked_id("Construction", &construction.name.0, children.len())
            else {
                continue;
            };
            let Some(child_count) =
                self.checked_id("Construction", &construction.name.0, states.len())
            else {
                continue;
            };
            series.push(ConstructionThermochromicSeries {
                master_construction: construction.id,
                initial_specification_temperature_c: first_state.optical_data_temperature_c,
                first_child: ThermochromicConstructionChildId(first_child),
                child_count,
            });

            for (state_index, state) in states.iter().enumerate() {
                let Some(id) =
                    self.checked_id("Construction", &construction.name.0, children.len())
                else {
                    break;
                };
                let Some(state_index) =
                    self.checked_id("Construction", &construction.name.0, state_index)
                else {
                    break;
                };
                let mut layers = construction.layers.clone();
                layers[layer_index] = state.glazing_material;
                let generated_name = format!(
                    "{}_TC_{}",
                    construction.name.0,
                    format_energyplus_round_zero(state.optical_data_temperature_c)
                );
                children.push(ConstructionThermochromicChild {
                    id: ThermochromicConstructionChildId(id),
                    master_construction: construction.id,
                    state_index,
                    name: NormalizedName::new(&generated_name),
                    specification_temperature_c: state.optical_data_temperature_c,
                    outside_layer: layers[0],
                    layers,
                });
            }
        }

        if self.diagnostics[diagnostics_before_pass..]
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }
        model.construction_thermochromic_series = series;
        model.construction_thermochromic_children = children;
    }

    fn invalid_thermochromic_projection(
        &mut self,
        construction_name: &str,
        field: &str,
        reason: &str,
    ) {
        self.error(
            "InvalidThermochromicConstructionProjection",
            "Construction",
            Some(construction_name),
            Some(field),
            format!(
                "Construction/{construction_name} cannot project thermochromic children because {reason}"
            ),
        );
    }
}

/// Source-shaped precision-zero `R` formatting with pinned thermochromic-name boundaries.
///
/// This intentionally does not claim parity for every finite IEEE-754 bit pattern across the
/// source's fmt 8 formatter and Rust's formatter.
pub(super) fn format_energyplus_round_zero(value: f64) -> String {
    debug_assert!(value.is_finite());
    if value == 0.0 {
        return "0".to_string();
    }

    if value >= FIXED_OUTPUT_THRESHOLD || value <= -FIXED_OUTPUT_THRESHOLD {
        // The source formatter skips its ULP nudge for large positive values and retains a
        // trailing decimal point once a precision-zero fixed rendering exhausts max_digits10.
        if value > 100_000.0 {
            let rendered = format!("{value:.0}");
            if (value.log10() as i32) >= F64_MAX_DIGITS_10 {
                return format!("{rendered}.");
            }
            return rendered;
        }

        let nudged = (0..3).fold(value, |value, _| next_float_away_from_zero(value));
        return format!("{nudged:.0}");
    }

    let nudged = next_float_away_from_zero(value);
    let rendered = format!("{nudged:.0E}");
    let Some((mantissa, exponent)) = rendered.split_once('E') else {
        return rendered;
    };
    let Ok(exponent) = exponent.parse::<i32>() else {
        return rendered;
    };
    format!("{mantissa}E{exponent:+04}")
}

fn next_float_away_from_zero(value: f64) -> f64 {
    debug_assert!(value.is_finite() && value != 0.0);
    if value == f64::MAX || value == f64::MIN {
        return value;
    }
    f64::from_bits(value.to_bits() + 1)
}
