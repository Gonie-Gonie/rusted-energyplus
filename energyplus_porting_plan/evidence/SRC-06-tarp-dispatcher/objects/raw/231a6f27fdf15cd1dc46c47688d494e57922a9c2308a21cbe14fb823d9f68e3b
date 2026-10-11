//! Ordinary public CTF generation and the actual fixed19 history lifecycle.
//!
//! This owner reuses the accepted construction/binding/history algorithms. It
//! does not supply Native observations, diagnostic overrides or a second solver.

use super::ctf_history_capture::{
    self as capture, CtfHistoryBoundary as Boundary, CtfHistoryObservation, CtfHistoryOwnerView,
};
use super::ctf_public_initialization::OrderedCtfPublicInitialization;
use super::ctf_surface_binding::{
    CtfSurfaceBindingUnavailable, CtfSurfaceGenerationContext, CtfSurfaceGenerationKey,
    CtfSurfaceProducerRevision, capture_public_ctf_surface_generation,
};
use super::ctf_thermal_history::{
    CtfThermalHistoryAlgorithm, CtfThermalHistoryArrays, CtfThermalHistoryContext,
    CtfThermalHistoryFluxReports, CtfThermalHistoryMasters, CtfThermalHistoryState,
    CtfThermalHistorySurface, CtfThermalHistoryUnavailable,
    initialize_selected_ctf_thermal_histories, update_selected_ctf_thermal_histories,
};
use super::state::{HeatBalanceState, SurfaceHeatBalanceState};
use super::surface_manager::ctf_caller_stability::CtfCallerSharedFlags;
use crate::weather::day::ProductionCtfHistoryClock;
#[cfg(test)]
use ep_model::SurfaceType;
use ep_model::{SurfaceId, TypedModel};

// Exact compiler-owned source witnesses. They are neither an answer digest nor
// runtime filesystem reads, and are automatically the actual compiled bytes.
const PRODUCER_SOURCES: &[(&str, &str)] = &[
    ("caller_clock", include_str!("ctf_caller_clock.rs")),
    ("timestep", include_str!("timestep.rs")),
    (
        "purchased_air_step",
        include_str!("timestep/purchased_air.rs"),
    ),
    ("warmup", include_str!("warmup.rs")),
    ("run_period", include_str!("run_period.rs")),
    (
        "run_period_standalone",
        include_str!("run_period/standalone.rs"),
    ),
    (
        "production_weather",
        include_str!("../weather_day/production.rs"),
    ),
    (
        "construction_cache",
        include_str!("surface_manager/construction_cache.rs"),
    ),
    (
        "layers",
        include_str!("surface_manager/ctf_layer_preprocessing.rs"),
    ),
    (
        "all_resistive",
        include_str!("surface_manager/ctf_all_resistive.rs"),
    ),
    (
        "initial",
        include_str!("surface_manager/ctf_initial_discretization.rs"),
    ),
    ("initial_owner", include_str!("ctf_initial_owner.rs")),
    (
        "assembly",
        include_str!("surface_manager/ctf_state_space_assembly.rs"),
    ),
    (
        "assembly_owner",
        include_str!("ctf_first_assembly_owner.rs"),
    ),
    (
        "exponential",
        include_str!("surface_manager/ctf_exponential_matrix.rs"),
    ),
    (
        "inverse",
        include_str!("surface_manager/ctf_inverse_matrix.rs"),
    ),
    ("gamma", include_str!("surface_manager/ctf_gamma_matrix.rs")),
    (
        "private",
        include_str!("surface_manager/ctf_final_coefficients.rs"),
    ),
    (
        "caller",
        include_str!("surface_manager/ctf_caller_stability.rs"),
    ),
    ("retry", include_str!("ctf_retry_driver.rs")),
    (
        "retry_initialization",
        include_str!("ctf_retry_initialization.rs"),
    ),
    (
        "public_storage",
        include_str!("surface_manager/ctf_public_storage.rs"),
    ),
    (
        "public_initialization",
        include_str!("ctf_public_initialization.rs"),
    ),
    ("binding", include_str!("ctf_surface_binding.rs")),
    ("history", include_str!("ctf_thermal_history.rs")),
    ("lifecycle", include_str!("ctf_surface_lifecycle.rs")),
    (
        "inside_convection_layout",
        include_str!("inside_convection_layout.rs"),
    ),
    ("surface_copy", include_str!("ctf.rs")),
    ("initializer", include_str!("initialization.rs")),
    (
        "initializer_shell",
        include_str!("initialization/state_shell.rs"),
    ),
];

/// A selected Rust ownership failure, never a substitute Native fatal outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CtfSurfaceLifecycleUnavailable {
    /// The existing binding rejected actual public generation or custody.
    Binding(CtfSurfaceBindingUnavailable),
    /// The accepted history function rejected actual supplied state/operands.
    History(CtfThermalHistoryUnavailable),
    /// Actual zones, spaces, surfaces or caller-owner associations are inconsistent.
    Association,
    /// The existing actual weather caller did not supply its lifecycle clock.
    MissingCallerClock,
    /// No genuine BeginEnvrn call has initialized this allocated owner.
    EnvironmentNotInitialized,
    /// The actual producer-input/public-source key differs from the retained key.
    StaleGeneration,
    /// The same actual zone-step clock was already committed.
    DuplicateCommit,
    /// Current caller zone timestep differs from the generation input.
    TimeStepAssociation,
    /// No actual public initialization owner exists at the consumer handoff.
    MissingPublicOwner,
    /// A mutable legacy consumer view no longer matches its real public owner.
    ConsumerView,
}

impl From<CtfSurfaceBindingUnavailable> for CtfSurfaceLifecycleUnavailable {
    fn from(value: CtfSurfaceBindingUnavailable) -> Self {
        Self::Binding(value)
    }
}
impl From<CtfThermalHistoryUnavailable> for CtfSurfaceLifecycleUnavailable {
    fn from(value: CtfThermalHistoryUnavailable) -> Self {
        Self::History(value)
    }
}

/// One actual ordinary allocated owner. Public coefficients remain in their
/// original moved construction owner; normal/master/scratch/report storage is
/// retained here across warmup and run-period calls without an alternate push.
#[derive(Clone, Debug, PartialEq)]
pub struct CtfSurfaceLifecycle {
    generation_key: CtfSurfaceGenerationKey,
    generation_context: CtfSurfaceGenerationContext,
    heat_transfer_groups: Vec<Vec<SurfaceId>>,
    opaque_groups: Vec<Vec<SurfaceId>>,
    all_heat_transfer_surfaces: Vec<SurfaceId>,
    history: CtfThermalHistoryState,
    initialized: bool,
    current_clock: Option<ProductionCtfHistoryClock>,
    last_committed_clock: Option<ProductionCtfHistoryClock>,
    failure: Option<CtfSurfaceLifecycleUnavailable>,
}

fn allocated_arrays(count: usize) -> CtfThermalHistoryArrays {
    CtfThermalHistoryArrays {
        inside_temperature: std::array::from_fn(|_| vec![0.0; count]),
        outside_temperature: std::array::from_fn(|_| vec![0.0; count]),
        inside_flux: std::array::from_fn(|_| vec![0.0; count]),
        outside_flux: std::array::from_fn(|_| vec![0.0; count]),
    }
}

fn ordinary_orders(
    model: &TypedModel,
) -> Result<Vec<Vec<SurfaceId>>, CtfSurfaceLifecycleUnavailable> {
    let layout = super::inside_convection_layout::InsideConvectionLayout::from_typed_model(model)
        .map_err(|_| CtfSurfaceLifecycleUnavailable::Association)?;
    Ok(layout
        .groups()
        .iter()
        .map(|group| group.surfaces.clone())
        .collect())
}

// Borrow-only observation packs keep the private hook interface bounded.
type CaptureOwners<'a> = (
    &'a TypedModel,
    Option<&'a OrderedCtfPublicInitialization>,
    &'a [SurfaceHeatBalanceState],
);
type CaptureSelected<'a> = (
    Option<&'a [CtfThermalHistorySurface<'a>]>,
    Option<CtfThermalHistoryContext<'a>>,
    Option<super::ctf_thermal_history::CtfThermalHistoryUpdateRoute>,
);

impl CtfSurfaceLifecycle {
    pub(crate) fn allocate(
        model: &TypedModel,
        public: &OrderedCtfPublicInitialization,
        context: CtfSurfaceGenerationContext,
    ) -> Result<Self, CtfSurfaceLifecycleUnavailable> {
        let generation = capture_public_ctf_surface_generation(model, public, context)?;
        for surface in &model.surfaces {
            generation.bind_surface(surface.id)?;
        }
        let groups = ordinary_orders(model)?;
        let all = groups.iter().flatten().copied().collect::<Vec<_>>();
        let count = model.surfaces.len();
        let aggregate = public.aggregate_after_returned_calls;
        let masters = if !aggregate.simple_ctf_only || context.any_ems_in_model {
            Some(CtfThermalHistoryMasters {
                arrays: allocated_arrays(count),
                current_num_histories: vec![0; count],
            })
        } else {
            None
        };
        Ok(Self {
            generation_key: generation.key().clone(),
            generation_context: context,
            heat_transfer_groups: groups.clone(),
            opaque_groups: groups,
            all_heat_transfer_surfaces: all,
            history: CtfThermalHistoryState {
                surface_ids: model.surfaces.iter().map(|value| value.id).collect(),
                normal: allocated_arrays(count),
                masters,
                scratch: None,
                update_first_time: true,
                flux_reports: CtfThermalHistoryFluxReports {
                    inside_conduction: vec![0.0; count],
                    inside_flux: vec![0.0; count],
                    outside_flux: vec![0.0; count],
                    outside_conduction: vec![0.0; count],
                },
            },
            initialized: false,
            current_clock: None,
            last_committed_clock: None,
            failure: None,
        })
    }

    /// Actual allocated owners; an uninitialized allocation is not initialized history.
    #[must_use]
    pub fn history(&self) -> &CtfThermalHistoryState {
        &self.history
    }

    /// Complete original primitive/public/source equality key captured by the producer.
    #[must_use]
    pub fn generation_key(&self) -> &CtfSurfaceGenerationKey {
        &self.generation_key
    }

    /// The first actual unavailable boundary, retained without steady fallback.
    #[must_use]
    pub fn failure(&self) -> Option<CtfSurfaceLifecycleUnavailable> {
        self.failure
    }

    fn observe<'a>(
        &'a self,
        boundary: Boundary,
        (model, public, surfaces): CaptureOwners<'a>,
        (clock, begin_timestep_seconds_argument, outdoor): (
            Option<ProductionCtfHistoryClock>,
            Option<f64>,
            f64,
        ),
        (inputs, context, route): CaptureSelected<'a>,
        error: Option<CtfSurfaceLifecycleUnavailable>,
    ) {
        if !capture::is_active() {
            return;
        }
        capture::record(CtfHistoryObservation {
            boundary,
            model,
            public,
            consumer_surfaces: surfaces,
            caller_clock: clock,
            begin_timestep_seconds_argument,
            outdoor_dry_bulb_c: outdoor,
            selected_inputs: inputs,
            selected_context: context,
            update_route: route,
            error,
            owner: CtfHistoryOwnerView {
                generation_key: &self.generation_key,
                generation_context: self.generation_context,
                history: &self.history,
                heat_transfer_groups: &self.heat_transfer_groups,
                opaque_groups: &self.opaque_groups,
                all_heat_transfer_surfaces: &self.all_heat_transfer_surfaces,
                initialized: self.initialized,
                current_clock: self.current_clock,
                last_committed_clock: self.last_committed_clock,
                failure: self.failure,
            },
        });
    }

    fn inputs<'a>(
        &self,
        model: &'a TypedModel,
        public: &'a OrderedCtfPublicInitialization,
        surfaces: &[SurfaceHeatBalanceState],
        outdoor_dry_bulb: f64,
    ) -> Result<Vec<CtfThermalHistorySurface<'a>>, CtfSurfaceLifecycleUnavailable> {
        let generation =
            capture_public_ctf_surface_generation(model, public, self.generation_context)?;
        if generation.key() != &self.generation_key {
            return Err(CtfSurfaceLifecycleUnavailable::StaleGeneration);
        }
        self.history
            .surface_ids
            .iter()
            .map(|&id| {
                let current = surfaces
                    .iter()
                    .find(|value| value.surface_id == id)
                    .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
                let binding = generation.bind_surface(id)?;
                if current.construction_id != binding.construction_id {
                    return Err(CtfSurfaceLifecycleUnavailable::Association);
                }
                let original = model
                    .surfaces
                    .iter()
                    .find(|surface| surface.id == id)
                    .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
                let coefficients = binding.coefficients;
                let count = usize::try_from(coefficients.num_ctf_terms)
                    .map_err(|_| CtfSurfaceLifecycleUnavailable::ConsumerView)?;
                if [
                    current.ctf.inside_temperature_history_c.len(),
                    current.ctf.outside_temperature_history_c.len(),
                    current.ctf.inside_flux_history_w_per_m2.len(),
                    current.ctf.outside_flux_history_w_per_m2.len(),
                ]
                .into_iter()
                .any(|length| length != count)
                {
                    return Err(CtfSurfaceLifecycleUnavailable::ConsumerView);
                }
                let same_values = |left: &[f64], right: &[f64]| {
                    left.len() == right.len()
                        && left
                            .iter()
                            .zip(right)
                            .all(|(a, b)| a.to_bits() == b.to_bits())
                };
                let index = self
                    .history
                    .surface_ids
                    .iter()
                    .position(|value| *value == id)
                    .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
                for (term, value) in current.ctf.inside_temperature_history_c.iter().enumerate() {
                    if value.to_bits()
                        != self.history.normal.inside_temperature[term + 1][index].to_bits()
                        || current.ctf.outside_temperature_history_c[term].to_bits()
                            != self.history.normal.outside_temperature[term + 1][index].to_bits()
                        || current.ctf.inside_flux_history_w_per_m2[term].to_bits()
                            != self.history.normal.inside_flux[term + 1][index].to_bits()
                        || current.ctf.outside_flux_history_w_per_m2[term].to_bits()
                            != self.history.normal.outside_flux[term + 1][index].to_bits()
                    {
                        return Err(CtfSurfaceLifecycleUnavailable::ConsumerView);
                    }
                }
                if current.outside_boundary_condition != original.outside_boundary_condition
                    || current.ctf.public_num_ctf_terms != Some(coefficients.num_ctf_terms)
                    || current.ctf.outside_0_w_per_m2_k.to_bits()
                        != coefficients.outside[0].to_bits()
                    || current.ctf.cross_0_w_per_m2_k.to_bits() != coefficients.cross[0].to_bits()
                    || current.ctf.inside_0_w_per_m2_k.to_bits() != coefficients.inside[0].to_bits()
                    || current.ctf.flux_0.map(f64::to_bits) != Some(coefficients.flux[0].to_bits())
                    || !same_values(
                        &current.ctf.outside_history_w_per_m2_k,
                        &coefficients.outside[1..count + 1],
                    )
                    || !same_values(
                        &current.ctf.cross_history_w_per_m2_k,
                        &coefficients.cross[1..count + 1],
                    )
                    || !same_values(
                        &current.ctf.inside_history_w_per_m2_k,
                        &coefficients.inside[1..count + 1],
                    )
                    || !same_values(&current.ctf.flux_history, &coefficients.flux[1..count + 1])
                {
                    return Err(CtfSurfaceLifecycleUnavailable::ConsumerView);
                }
                Ok(CtfThermalHistorySurface {
                    surface_id: id,
                    construction_id: binding.construction_id,
                    public_owner_construction_id: binding.construction_id,
                    coefficients: binding.coefficients,
                    algorithm: CtfThermalHistoryAlgorithm::Ctf,
                    ordinary_opaque: true,
                    source_sink_present: false,
                    outside_boundary: current.outside_boundary_condition,
                    outside_dry_bulb:
                        super::convection::energyplus_stored_surface_outdoor_air_temperature_c(
                            current,
                            outdoor_dry_bulb,
                        ),
                    area: current.area_m2,
                    inside_surface_temperature: current.inside_face_temperature_c,
                    inside_constant: current.ctf.const_in_part_w_per_m2,
                    outside_constant: current.ctf.const_out_part_w_per_m2,
                })
            })
            .collect()
    }

    fn copy_lagged_views(
        &self,
        surfaces: &mut [SurfaceHeatBalanceState],
    ) -> Result<(), CtfSurfaceLifecycleUnavailable> {
        for (index, &id) in self.history.surface_ids.iter().enumerate() {
            let surface = surfaces
                .iter_mut()
                .find(|value| value.surface_id == id)
                .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
            let terms = surface
                .ctf
                .public_num_ctf_terms
                .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
            let count =
                usize::try_from(terms).map_err(|_| CtfSurfaceLifecycleUnavailable::Association)?;
            if count >= 19
                || [
                    surface.ctf.inside_temperature_history_c.len(),
                    surface.ctf.outside_temperature_history_c.len(),
                    surface.ctf.inside_flux_history_w_per_m2.len(),
                    surface.ctf.outside_flux_history_w_per_m2.len(),
                ]
                .into_iter()
                .any(|length| length != count)
            {
                return Err(CtfSurfaceLifecycleUnavailable::ConsumerView);
            }
            for (term, target) in surface
                .ctf
                .inside_temperature_history_c
                .iter_mut()
                .enumerate()
            {
                *target = self.history.normal.inside_temperature[term + 1][index];
                surface.ctf.outside_temperature_history_c[term] =
                    self.history.normal.outside_temperature[term + 1][index];
                surface.ctf.inside_flux_history_w_per_m2[term] =
                    self.history.normal.inside_flux[term + 1][index];
                surface.ctf.outside_flux_history_w_per_m2[term] =
                    self.history.normal.outside_flux[term + 1][index];
            }
        }
        Ok(())
    }

    fn begin_step(
        &mut self,
        model: &TypedModel,
        public: &OrderedCtfPublicInitialization,
        surfaces: &mut [SurfaceHeatBalanceState],
        clock: ProductionCtfHistoryClock,
        outdoor: f64,
    ) -> Result<(), CtfSurfaceLifecycleUnavailable> {
        if let Some(error) = self.failure {
            return Err(error);
        }
        if self.current_clock.is_some() {
            return Err(CtfSurfaceLifecycleUnavailable::DuplicateCommit);
        }
        if clock.time_step_zone.to_bits() != self.generation_context.time_step_zone_hours.to_bits()
        {
            return Err(CtfSurfaceLifecycleUnavailable::TimeStepAssociation);
        }
        if self
            .last_committed_clock
            .is_some_and(|value| value.same_zone_step(clock))
        {
            return Err(CtfSurfaceLifecycleUnavailable::DuplicateCommit);
        }
        let inputs = self.inputs(model, public, surfaces, outdoor)?;
        if clock.begin_environment {
            // Borrow disjoint fields explicitly; the accepted arithmetic is unchanged.
            let aggregate = public.aggregate_after_returned_calls;
            let context = CtfThermalHistoryContext {
                simple_ctf_only: aggregate.simple_ctf_only,
                any_ems_in_model: self.generation_context.any_ems_in_model,
                any_construction_overrides: self.generation_context.any_construction_overrides,
                any_internal_heat_source: false,
                global_max_ctf_terms: aggregate.max_ctf_terms,
                time_step_zone_hours: clock.time_step_zone,
                heat_transfer_groups: &self.heat_transfer_groups,
                opaque_groups: &self.opaque_groups,
                all_heat_transfer_surfaces: &self.all_heat_transfer_surfaces,
            };
            // Actual selected bulk report reset precedes the history suffix;
            // this is initialization, not an expected endpoint or a flux fill.
            self.history.flux_reports.inside_conduction.fill(0.0);
            self.history.flux_reports.inside_flux.fill(0.0);
            self.history.flux_reports.outside_flux.fill(0.0);
            self.history.flux_reports.outside_conduction.fill(0.0);
            self.observe(
                Boundary::InitializationInput,
                (model, Some(public), surfaces),
                (Some(clock), None, outdoor),
                (Some(&inputs), Some(context), None),
                None,
            );
            initialize_selected_ctf_thermal_histories(&mut self.history, &inputs, context)?;
            self.observe(
                Boundary::InitializationReturned,
                (model, Some(public), surfaces),
                (Some(clock), None, outdoor),
                (Some(&inputs), Some(context), None),
                None,
            );
            self.initialized = true;
            for (index, &id) in self.history.surface_ids.iter().enumerate() {
                let surface = surfaces
                    .iter_mut()
                    .find(|value| value.surface_id == id)
                    .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
                surface.inside_face_temperature_c =
                    self.history.normal.inside_temperature[0][index];
                // Native SurfTempOut starts at zero; TH(1,1) already owns the
                // resolved boundary temperature. These are distinct owners.
                surface.outside_face_temperature_c = 0.0;
                surface.inside_ctf_outside_temperature_c =
                    self.history.normal.outside_temperature[0][index];
                surface.inside_reference_air_temperature_c =
                    super::ctf_thermal_history::CTF_SURFACE_INITIAL_TEMPERATURE;
                surface.inside_convection_coefficient_w_per_m2_k = 3.076;
                surface.ctf.const_in_part_w_per_m2 = 0.0;
                surface.ctf.const_out_part_w_per_m2 = 0.0;
                surface.heat_gain_to_zone_w = 0.0;
            }
        }
        if !self.initialized {
            return Err(CtfSurfaceLifecycleUnavailable::EnvironmentNotInitialized);
        }
        self.copy_lagged_views(surfaces)?;
        self.current_clock = Some(clock);
        Ok(())
    }

    fn commit_step(
        &mut self,
        model: &TypedModel,
        public: &OrderedCtfPublicInitialization,
        surfaces: &mut [SurfaceHeatBalanceState],
        outdoor: f64,
    ) -> Result<(), CtfSurfaceLifecycleUnavailable> {
        let clock = self
            .current_clock
            .ok_or(CtfSurfaceLifecycleUnavailable::MissingCallerClock)?;
        // Verify producer freshness and all duplicated lagged consumer bits
        // before synchronizing current slots or mutating any update owner.
        let inputs = self.inputs(model, public, surfaces, outdoor)?;
        for (index, &id) in self.history.surface_ids.iter().enumerate() {
            let surface = surfaces
                .iter()
                .find(|value| value.surface_id == id)
                .ok_or(CtfSurfaceLifecycleUnavailable::Association)?;
            self.history.normal.inside_temperature[0][index] = surface.inside_face_temperature_c;
            self.history.normal.outside_temperature[0][index] =
                surface.inside_ctf_outside_temperature_c;
        }
        let aggregate = public.aggregate_after_returned_calls;
        let context = CtfThermalHistoryContext {
            simple_ctf_only: aggregate.simple_ctf_only,
            any_ems_in_model: self.generation_context.any_ems_in_model,
            any_construction_overrides: self.generation_context.any_construction_overrides,
            any_internal_heat_source: false,
            global_max_ctf_terms: aggregate.max_ctf_terms,
            time_step_zone_hours: clock.time_step_zone,
            heat_transfer_groups: &self.heat_transfer_groups,
            opaque_groups: &self.opaque_groups,
            all_heat_transfer_surfaces: &self.all_heat_transfer_surfaces,
        };
        self.observe(
            Boundary::HistoryUpdateInput,
            (model, Some(public), surfaces),
            (Some(clock), None, outdoor),
            (Some(&inputs), Some(context), None),
            None,
        );
        let route = update_selected_ctf_thermal_histories(&mut self.history, &inputs, context)?;
        self.observe(
            Boundary::HistoryUpdateReturned,
            (model, Some(public), surfaces),
            (Some(clock), None, outdoor),
            (Some(&inputs), Some(context), Some(route)),
            None,
        );
        self.copy_lagged_views(surfaces)?;
        self.last_committed_clock = Some(clock);
        self.observe(
            Boundary::HistoryCommitBeforeClockRelease,
            (model, Some(public), surfaces),
            (Some(clock), None, outdoor),
            (None, None, Some(route)),
            None,
        );
        self.current_clock = None;
        Ok(())
    }
}

pub(crate) fn fresh_generation_context(time_step_zone_hours: f64) -> CtfSurfaceGenerationContext {
    CtfSurfaceGenerationContext {
        producer_revision: CtfSurfaceProducerRevision::Compiled(PRODUCER_SOURCES),
        time_step_zone_hours,
        initial_flags: CtfCallerSharedFlags {
            errors_found: false,
            do_ctf_error_report: false,
        },
        initial_simple_ctf_only: true,
        initial_max_ctf_terms: 0,
        any_ems_in_model: false,
        any_construction_overrides: false,
    }
}

pub(crate) fn begin_step(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    clock: Option<ProductionCtfHistoryClock>,
    outdoor: f64,
    timestep_seconds: f64,
) -> Result<(), CtfSurfaceLifecycleUnavailable> {
    let Some(lifecycle) = state.ctf_surface_lifecycle.as_mut() else {
        return Ok(());
    };
    lifecycle.observe(
        Boundary::BeginEntry,
        (
            model,
            state.construction_ctf10_public_initialization.as_ref(),
            &state.surfaces,
        ),
        (clock, Some(timestep_seconds), outdoor),
        (None, None, None),
        None,
    );
    let result = match (&state.construction_ctf10_public_initialization, clock) {
        (Some(public), Some(clock))
            if clock.time_step_zone_seconds.to_bits() == timestep_seconds.to_bits() =>
        {
            lifecycle.begin_step(model, public, &mut state.surfaces, clock, outdoor)
        }
        (Some(_), Some(_)) => Err(CtfSurfaceLifecycleUnavailable::TimeStepAssociation),
        (None, _) => Err(CtfSurfaceLifecycleUnavailable::MissingPublicOwner),
        (_, None) => Err(CtfSurfaceLifecycleUnavailable::MissingCallerClock),
    };
    if let Err(error) = result {
        lifecycle.failure.get_or_insert(error);
    }
    lifecycle.observe(
        if result.is_ok() {
            Boundary::BeginReturned
        } else {
            Boundary::BeginFailed
        },
        (
            model,
            state.construction_ctf10_public_initialization.as_ref(),
            &state.surfaces,
        ),
        (clock, Some(timestep_seconds), outdoor),
        (None, None, None),
        result.err(),
    );
    result
}

pub(crate) fn commit_step(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    outdoor: f64,
) -> Result<(), CtfSurfaceLifecycleUnavailable> {
    let Some(lifecycle) = state.ctf_surface_lifecycle.as_mut() else {
        return Ok(());
    };
    let clock = lifecycle.current_clock;
    let result = match &state.construction_ctf10_public_initialization {
        Some(public) => lifecycle.commit_step(model, public, &mut state.surfaces, outdoor),
        None => Err(CtfSurfaceLifecycleUnavailable::MissingPublicOwner),
    };
    if let Err(error) = result {
        lifecycle.failure.get_or_insert(error);
    }
    lifecycle.observe(
        if result.is_ok() {
            Boundary::CommitReturned
        } else {
            Boundary::CommitFailed
        },
        (
            model,
            state.construction_ctf10_public_initialization.as_ref(),
            &state.surfaces,
        ),
        (clock, None, outdoor),
        (None, None, None),
        result.err(),
    );
    result
}

pub(crate) fn ensure_available(state: &HeatBalanceState) -> Result<(), crate::error::RuntimeError> {
    match state
        .ctf_surface_lifecycle
        .as_ref()
        .and_then(CtfSurfaceLifecycle::failure)
    {
        Some(error) => Err(crate::error::RuntimeError::CtfSurfaceLifecycle(error)),
        None => Ok(()),
    }
}

#[cfg(test)]
#[path = "ctf_surface_lifecycle_tests.rs"]
mod tests;
