//! Pure copies of actual public preprocessing return owners; no CTF arithmetic.
use ep_runtime::heat_balance::ctf_first_assembly_owner::{
    ConstructionCtfFirstAssembly, CtfFirstAssemblyUnavailable,
};
use ep_runtime::heat_balance::ctf_first_exponential_owner::{
    ConstructionCtfFirstExponential, CtfFirstExponentialUnavailable,
};
use ep_runtime::heat_balance::ctf_first_inverse_owner::CtfFirstInverseUnavailable;
use ep_runtime::heat_balance::ctf_first_matrix_owner::{
    ConstructionCtfFirstInverseAfterExponential, PriorExponentialUnavailable,
};
use ep_runtime::heat_balance::ctf_initial_owner::{
    ConstructionCtfInitialDiscretization, CtfInitialUnavailable,
};
use ep_runtime::heat_balance::surface_manager::ctf_exponential_matrix::CtfExponentialObservation;
use ep_runtime::heat_balance::surface_manager::ctf_layer_preprocessing::{
    ConstructionCtfLayerPreprocessing, CtfLayerCheckpoint, CtfLayerInput, CtfLayerPreprocessing,
    CtfLayerScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_state_space_assembly::CtfAssemblyContext;
use ep_runtime::heat_balance::surface_manager::ctf_state_space_assembly::CtfAssemblyObservation;
use serde_json::{Value, json};

pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value.is_infinite() {
        if value.is_sign_negative() {
            "negative_infinity"
        } else {
            "positive_infinity"
        }
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite() { Some(value) } else { None },
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}
pub(super) fn unpaired_later_flag() -> Value {
    json!({"owner_available":false,"value":null,"counted_as_PASS":false,
        "reason":"Native whole-method DoCTFErrorReport/solver mutations are outside the selected Rust API"})
}
pub(super) fn input(source: &CtfLayerInput) -> Value {
    json!({"material_id":source.material_id.0,"Thickness":scalar(source.thickness),
        "Conductivity":scalar(source.conductivity),"Density":scalar(source.density),
        "SpecHeat":scalar(source.specific_heat),"Resistance":scalar(source.resistance),"ROnly":source.resistance_only})
}
fn checkpoint(source: Option<&CtfLayerCheckpoint>, errors: bool, reason: &str) -> Value {
    let Some(actual) = source else {
        return json!({"reached":false,"available":false,"snapshot":null,
            "unavailable_reason":reason,"counted_as_PASS":false});
    };
    json!({"reached":true,"available":true,"unavailable_reason":null,"snapshot":{
        "LayersInConstruct":actual.layers.len(),"NumResLayers":actual.num_res_layers,
        "ErrorsFound":errors,"DoCTFErrorReport":null,"DoCTFErrorReport_paired":false,
        "active_prefix_available":true,"copied_layer_count":actual.layers.len(),
        "active_prefix":actual.layers.iter().map(|v|json!({"dl":scalar(v.dl),"rk":scalar(v.rk),
            "rho":scalar(v.rho),"cp":scalar(v.cp),"lr":scalar(v.lr),"ResLayer":v.res_layer})).collect::<Vec<_>>(),
        "dyn":null,"rs":null,"cnd":null}})
}
pub(super) fn result(
    source: &std::result::Result<CtfLayerPreprocessing, CtfLayerScopeError>,
) -> Value {
    match source {
        Err(error) => {
            json!({"status":"scope-error-returned","actual_scope_error":format!("{error:?}"),
            "returned_preprocessing_owner":null,"observations":null,"counted_as_PASS":false,
            "selected_ErrorsFound_mutation_available":false,"Native_fatal_or_exception_inferred":false})
        }
        Ok(actual) => {
            let reason = if actual.skipped_unused {
                "actual API returned unused without a checkpoint"
            } else {
                "actual API returned before this checkpoint"
            };
            let mut converted = checkpoint(
                actual.after_conversion.as_ref().map(|v| &v.active),
                actual.errors_found,
                reason,
            );
            if let Some(value) = &actual.after_conversion {
                converted["snapshot"]["dyn"] = scalar(value.dyn_spacing);
                converted["snapshot"]["rs"] = scalar(value.total_resistance);
                converted["snapshot"]["cnd"] = scalar(value.conductance);
            }
            json!({"status":"source-returned","skipped_unused":actual.skipped_unused,
                "selected_ErrorsFound":actual.errors_found,"issues_in_actual_order":format!("{:?}",actual.issues),
                "observations":{"PostLoad":checkpoint(actual.after_load.as_ref(),actual.errors_found,reason),
                    "PostMerge":checkpoint(actual.after_merge.as_ref(),actual.errors_found,reason),
                    "PostConversion":converted,"unused_fixed_array_tail_emitted":false,
                    "normalized_layer_material_IDs_fabricated":false},
                "Native_whole_method_or_coefficients_observed":false,"numerical_PASS_claimed":false})
        }
    }
}

pub(super) fn initial(
    source: &ConstructionCtfInitialDiscretization,
    preprocessing: Option<&ConstructionCtfLayerPreprocessing>,
) -> Value {
    let phases = preprocessing.and_then(|owner| owner.result.as_ref().ok());
    let converted = phases.and_then(|owner| owner.after_conversion.as_ref());
    let context = json!({"is_used_ctf":source.context.is_used_ctf,
        "errors_found":source.context.errors_found,
        "source_sink_present":source.context.source_sink_present,
        "solution_dimensions":source.context.solution_dimensions});
    let (available, route, reverse_id, reason, snapshot) = match &source.result {
        Err(reason) => {
            let (route, reverse_id) = match reason {
                CtfInitialUnavailable::PreprocessingScope(_) => ("PreprocessingScope", None),
                CtfInitialUnavailable::UnusedConstruction => ("UnusedConstruction", None),
                CtfInitialUnavailable::PreprocessingErrorReturn => {
                    ("PreprocessingErrorReturn", None)
                }
                CtfInitialUnavailable::PostConversionUnavailable => {
                    ("PostConversionUnavailable", None)
                }
                CtfInitialUnavailable::AllResistiveBranch => ("AllResistiveBranch", None),
                CtfInitialUnavailable::ReverseConstruction { construction_id } => {
                    ("ReverseConstruction", Some(construction_id.0))
                }
                CtfInitialUnavailable::InitialScope(_) => ("InitialScope", None),
            };
            (
                false,
                route,
                reverse_id,
                Some(format!("{reason:?}")),
                Value::Null,
            )
        }
        Ok(actual) => (
            true,
            "InitialOwnerReturned",
            None,
            None,
            json!({
            "LayersInConstruct":converted.map(|owner|owner.active.layers.len()),
            "NumResLayers":converted.map(|owner|owner.active.num_res_layers),
            "ErrorsFound":phases.map(|owner|owner.errors_found),
            "DoCTFErrorReport":null,"DoCTFErrorReport_paired":false,
            "active_prefix_available":true,"copied_layer_count":actual.active.len(),
            "active_prefix":actual.active.iter().map(|layer|json!({"Nodes":layer.nodes,"dx":scalar(layer.dx)})).collect::<Vec<_>>(),
            "rcmax":actual.rcmax,"NodeSource":actual.node_source,"NodeUserTemp":actual.node_user_temp,
            "CTFTimeStep":scalar(actual.initial_ctf_time_step_hours),
            "NumHistories":actual.initial_num_histories,"TimeStepZone":scalar(actual.time_step_zone_hours)}),
        ),
    };
    json!({"protocol":"ctf03-initial-discretization.v1","owner_available":available,
        "selected_initial_operands":available,"actual_source_route":route,
        "actual_reverse_construction_id":reverse_id,"unavailable_reason":reason,"snapshot":snapshot,
        "construction_id":source.construction_id.0,"actual_context":context,
        "actual_wrapper_TimeStepZone":scalar(source.time_step_zone_hours),
        "same_preprocessing_owner_available":preprocessing.is_some(),
        "same_post_conversion_handoff_available":converted.is_some(),
        "same_preprocessing_construction_id":preprocessing.map(|owner|owner.construction_id.0),
        "initial_state_distinct_from_retry_and_whole_return":true,
        "observer_nodal_or_route_arithmetic_performed":false,
        "Native_whole_method_flags_or_callback_reachability_inferred":false,"counted_as_PASS":false})
}

pub(super) fn first_assembly(
    source: &ConstructionCtfFirstAssembly,
    initial: Option<&ConstructionCtfInitialDiscretization>,
    preprocessing: Option<&ConstructionCtfLayerPreprocessing>,
) -> Value {
    let phases = preprocessing.and_then(|owner| owner.result.as_ref().ok());
    let converted = phases.and_then(|owner| owner.after_conversion.as_ref());
    let (available, route, reason, snapshot) = match &source.result {
        Err(reason) => {
            let route = match reason {
                CtfFirstAssemblyUnavailable::InitialUnavailable(_) => "InitialUnavailable",
                CtfFirstAssemblyUnavailable::ConstructionIdentityMismatch => {
                    "ConstructionIdentityMismatch"
                }
                CtfFirstAssemblyUnavailable::PreprocessingScope(_) => "PreprocessingScope",
                CtfFirstAssemblyUnavailable::PreprocessingRouteMismatch => {
                    "PreprocessingRouteMismatch"
                }
                CtfFirstAssemblyUnavailable::PostConversionUnavailable => {
                    "PostConversionUnavailable"
                }
                CtfFirstAssemblyUnavailable::CallerTimeStepMismatch => "CallerTimeStepMismatch",
                CtfFirstAssemblyUnavailable::AssemblyScope(_) => "AssemblyScope",
            };
            (false, route, Some(format!("{reason:?}")), Value::Null)
        }
        Ok(CtfAssemblyObservation::Unavailable(reason)) => (
            false,
            "AssemblyUnavailable",
            Some(format!("{reason:?}")),
            Value::Null,
        ),
        Ok(CtfAssemblyObservation::Assembled(actual)) => {
            let values = |source: &[f64]| source.iter().copied().map(scalar).collect::<Vec<_>>();
            (
                true,
                "FirstAssemblyReturned",
                None,
                json!({
                "LayersInConstruct":actual.layers.len(),
                "NumResLayers":converted.map(|owner|owner.active.num_res_layers),
                "ErrorsFound":phases.map(|owner|owner.errors_found),
                "DoCTFErrorReport":null,"DoCTFErrorReport_paired":false,
                "IsUsedCTF":initial.map(|owner|owner.context.is_used_ctf),
                "SourceSinkPresent":actual.context.source_sink_present,
                "SolutionDimensions":actual.context.solution_dimensions,
                "active_prefix_available":true,"copied_layer_count":actual.layers.len(),
                "active_prefix":actual.layers.iter().enumerate().map(|(index,layer)|json!({
                    "dl":scalar(layer.dl),"rk":scalar(layer.rk),"rho":scalar(layer.rho),
                    "cp":scalar(layer.cp),"lr":scalar(layer.lr),"ResLayer":layer.res_layer,
                    "Nodes":actual.nodes.get(index).copied(),
                    "dx":actual.dx.get(index).copied().map(scalar)})).collect::<Vec<_>>(),
                "actual_nodes_count":actual.nodes.len(),"actual_dx_count":actual.dx.len(),
                "rcmax":actual.rcmax,"NodeSource":actual.context.node_source,
                "NodeUserTemp":actual.context.node_user_temp,
                "CTFTimeStep":scalar(actual.context.ctf_time_step),
                "NumHistories":actual.context.num_histories,
                "TimeStepZone":scalar(actual.context.time_step_zone),
                "attempt_ordinal":actual.context.attempt_ordinal,
                "actual_context_construction_id":actual.context.construction_id.0,
                "actual_route":format!("{:?}",actual.context.route),
                "matrices_available":true,"matrices":{
                    "AMat":values(&actual.a_mat),"IdenMatrix":values(&actual.iden_matrix),
                    "BMat":values(&actual.b_mat),"CMat":values(&actual.c_mat),"DMat":values(&actual.d_mat)},
                "storage_order":"source_i1_outer_i2_inner_row_first",
                "Native_Objexx_bounds_not_Rust_owner":true}),
            )
        }
    };
    json!({"protocol":"ctf04-matrix-assembly.v1","owner_available":available,
        "selected_first_assembly_operands":available,"actual_source_route":route,
        "unavailable_reason":reason,"snapshot":snapshot,"construction_id":source.construction_id.0,
        "same_initial_owner_available":initial.is_some(),
        "same_initial_construction_id":initial.map(|owner|owner.construction_id.0),
        "same_preprocessing_owner_available":preprocessing.is_some(),
        "same_preprocessing_construction_id":preprocessing.map(|owner|owner.construction_id.0),
        "same_post_conversion_handoff_available":converted.is_some(),
        "only_first_Rust_assembly_invocation_owned":true,
        "later_Native_attempts_available_in_Rust":false,"later_Native_attempts_counted_as_PASS":false,
        "retry_driver_available":false,"Native_retry_state_or_counts_supplied_to_Rust":false,
        "observer_matrix_identity_or_route_arithmetic_performed":false,
        "Native_whole_method_outcome_or_flags_inferred":false,"counted_as_PASS":false})
}

fn matrix_values(source: &[f64]) -> Vec<Value> {
    source.iter().copied().map(scalar).collect()
}
fn attempt_context(source: &CtfAssemblyContext) -> Value {
    json!({"construction_id":source.construction_id.0,"route":format!("{:?}",source.route),
        "SourceSinkPresent":source.source_sink_present,"SolutionDimensions":source.solution_dimensions,
        "NodeSource":source.node_source,"NodeUserTemp":source.node_user_temp,
        "attempt_ordinal":source.attempt_ordinal,"TimeStepZone":scalar(source.time_step_zone),
        "CTFTimeStep":scalar(source.ctf_time_step),"NumHistories":source.num_histories})
}
fn original_assembly(source: Option<&ConstructionCtfFirstAssembly>) -> Value {
    let Some(owner) = source else {
        return json!({"owner_available":false,"snapshot":null,
            "unavailable_reason":"matching original first assembly owner not supplied","counted_as_PASS":false});
    };
    match &owner.result {
        Ok(CtfAssemblyObservation::Assembled(actual)) => json!({"owner_available":true,
            "construction_id":owner.construction_id.0,"snapshot":{"rcmax":actual.rcmax,
                "context":attempt_context(&actual.context),"AMat":matrix_values(&actual.a_mat),
                "IdenMatrix":matrix_values(&actual.iden_matrix),
                "actual_AMat_value_count":actual.a_mat.len(),"actual_IdenMatrix_value_count":actual.iden_matrix.len()},
            "unavailable_reason":null,"storage_order":"source_i1_outer_i2_inner_row_first","counted_as_PASS":false}),
        _ => {
            json!({"owner_available":false,"construction_id":owner.construction_id.0,"snapshot":null,
            "unavailable_reason":format!("{:?}",owner.result),"counted_as_PASS":false})
        }
    }
}
pub(super) fn first_exponential(
    source: &ConstructionCtfFirstExponential,
    assembly: Option<&ConstructionCtfFirstAssembly>,
) -> Value {
    let (available, route, reason, snapshot) = match &source.result {
        Err(reason) => {
            let route = match reason {
                CtfFirstExponentialUnavailable::FirstAssemblyUnavailable(_) => {
                    "FirstAssemblyUnavailable"
                }
                CtfFirstExponentialUnavailable::AssemblyUnavailable(_) => "AssemblyUnavailable",
                CtfFirstExponentialUnavailable::ConstructionIdentityMismatch => {
                    "ConstructionIdentityMismatch"
                }
                CtfFirstExponentialUnavailable::AssemblyRouteMismatch => "AssemblyRouteMismatch",
                CtfFirstExponentialUnavailable::NotFirstAttempt(_) => "NotFirstAttempt",
                CtfFirstExponentialUnavailable::ExponentialScope(_) => "ExponentialScope",
            };
            (false, route, Some(format!("{reason:?}")), Value::Null)
        }
        Ok(CtfExponentialObservation::Unavailable(reason)) => (
            false,
            "ExponentialUnavailable",
            Some(format!("{reason:?}")),
            Value::Null,
        ),
        Ok(CtfExponentialObservation::Exponential(actual)) => (
            true,
            "FirstExponentialReturned",
            None,
            json!({"context":attempt_context(&actual.context),"rcmax":actual.rcmax,
                "AMatRowNormMax":scalar(actual.a_mat_row_norm_max),"k":actual.k,
                "fact":scalar(actual.fact),"CheckVal":scalar(actual.check_val),"l":actual.l,
                "final_i":actual.final_power_i,
                "PowerControl":actual.power_steps.iter().map(|event|json!({"i":event.i,"SigFigLimit":event.sig_fig_limit})).collect::<Vec<_>>(),
                "SquareControl":actual.square_steps.iter().map(|event|json!({"isq":event.isq,"Backup":event.backup})).collect::<Vec<_>>(),
                "actual_power_control_count":actual.power_steps.len(),"actual_square_control_count":actual.square_steps.len(),
                "matrices":{"AExp":matrix_values(&actual.a_exp),"AMat1":matrix_values(&actual.a_mat1),
                    "AMato":matrix_values(&actual.a_mato),"AMatN":matrix_values(&actual.a_mat_n)},
                "actual_matrix_value_counts":{"AExp":actual.a_exp.len(),"AMat1":actual.a_mat1.len(),
                    "AMato":actual.a_mato.len(),"AMatN":actual.a_mat_n.len()},
                "storage_order":"source_i1_outer_i2_inner_row_first"}),
        ),
    };
    json!({"protocol":"ctf06-first-exponential.v1","construction_id":source.construction_id.0,
        "owner_available":available,"selected_first_exponential_operands":available,
        "actual_source_route":route,"unavailable_reason":reason,"snapshot":snapshot,
        "actual_public_API":"initialize_construction_ctf_first_exponential",
        "actual_public_API_invoked":true,"original_first_assembly":original_assembly(assembly),
        "only_first_Rust_method_invocation_owned":true,"retry_driver_available":false,
        "later_Native_method_visits_paired":false,"later_Native_method_visits_counted_as_PASS":false,
        "Native_stage_callback_or_return_counts_inferred":false,
        "observer_matrix_control_or_timestep_arithmetic_performed":false,"counted_as_PASS":false})
}
pub(super) fn first_inverse(
    source: &ConstructionCtfFirstInverseAfterExponential,
    assembly: Option<&ConstructionCtfFirstAssembly>,
) -> Value {
    let (invoked, available, route, reason, snapshot) = match &source.result {
        Err(reason) => {
            let route = match reason {
                PriorExponentialUnavailable::Error(_) => "PriorExponentialError",
                PriorExponentialUnavailable::Route(_) => "PriorExponentialRouteUnavailable",
            };
            (
                false,
                false,
                route,
                Some(format!("{reason:?}")),
                Value::Null,
            )
        }
        Ok(owner) => match &owner.result {
            Err(reason) => {
                let route = match reason {
                    CtfFirstInverseUnavailable::FirstAssemblyUnavailable(_) => {
                        "FirstAssemblyUnavailable"
                    }
                    CtfFirstInverseUnavailable::AssemblyRouteUnavailable(_) => {
                        "AssemblyRouteUnavailable"
                    }
                    CtfFirstInverseUnavailable::ConstructionIdentityMismatch => {
                        "ConstructionIdentityMismatch"
                    }
                    CtfFirstInverseUnavailable::NonFirstAttempt(_) => "NonFirstAttempt",
                    CtfFirstInverseUnavailable::AssemblyContext => "AssemblyContext",
                    CtfFirstInverseUnavailable::InverseScope(_) => "InverseScope",
                };
                (true, false, route, Some(format!("{reason:?}")), Value::Null)
            }
            Ok(actual) => (
                true,
                true,
                "FirstInverseReturned",
                None,
                json!({"actual_public_owner_construction_id":owner.construction_id.0,
                    "context":attempt_context(&actual.context),"rcmax":actual.inverse.rcmax,
                    "matrices":{"AInv":matrix_values(&actual.inverse.a_inv),"AMat1":matrix_values(&actual.inverse.a_mat1)},
                    "actual_matrix_value_counts":{"AInv":actual.inverse.a_inv.len(),"AMat1":actual.inverse.a_mat1.len()},
                    "storage_order":"source_i1_outer_i2_inner_row_first"}),
            ),
        },
    };
    json!({"protocol":"ctf05-first-inverse-after-exponential.v1","construction_id":source.construction_id.0,
        "owner_available":available,"selected_first_inverse_operands":available,
        "actual_source_route":route,"unavailable_reason":reason,"snapshot":snapshot,
        "actual_public_API":"initialize_construction_ctf_first_inverse","actual_public_API_invoked":invoked,
        "not_invoked_after_prior_exponential_unavailable":!invoked,
        "actual_orchestration_assembly_context":source.assembly_context.as_ref().map(attempt_context),
        "actual_public_owner_construction_id":source.result.as_ref().ok().map(|owner|owner.construction_id.0),
        "original_first_assembly":original_assembly(assembly),
        "inverse_input_is_original_AMat_and_IdenMatrix":true,"AExp_supplied_as_inverse_input":false,
        "only_first_Rust_method_invocation_owned":true,"retry_driver_available":false,
        "later_Native_method_visits_paired":false,"later_Native_method_visits_counted_as_PASS":false,
        "Native_stage_callback_or_return_counts_inferred":false,
        "observer_inverse_or_timestep_arithmetic_performed":false,"counted_as_PASS":false})
}

pub(super) const OBSERVER_PATHS: &[&str] = &[
    "crates/ep_run/examples/ctf06_candidate_unit_observer.rs",
    "crates/ep_run/examples/ctf06_candidate_unit_support/fields.rs",
];
// Exact path inventory; approved formatted SHAs are runtime operands and must
// also match Root's actual Cargo source capture before results are admitted.
pub(super) const API_PATHS: &[&str] = &[
    "crates/ep_run/Cargo.toml",
    "crates/ep_model/src/lib.rs",
    "crates/ep_model/src/ids.rs",
    "crates/ep_runtime/Cargo.toml",
    "crates/ep_runtime/src/lib.rs",
    "crates/ep_runtime/src/heat_balance/mod.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_layer_preprocessing.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_initial_discretization.rs",
    "crates/ep_runtime/src/heat_balance/ctf_initial_owner.rs",
    "crates/ep_model/src/objects/construction.rs",
    "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_state_space_assembly.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_assembly_owner.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_exponential_matrix.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_exponential_owner.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_inverse_matrix.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_inverse_owner.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_matrix_owner.rs",
];
